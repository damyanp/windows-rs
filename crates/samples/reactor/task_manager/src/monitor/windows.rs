//! Real system monitor backed by Windows APIs via `windows-rs`.
//!
//! Enumeration uses a Toolhelp snapshot; per-process CPU times and memory come
//! from `OpenProcess` + `GetProcessTimes` / `GetProcessMemoryInfo`. Processes we
//! cannot open (access denied) still appear, just with missing fields.

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::{size_of, zeroed};
use std::rc::Rc;
use std::time::Instant;

use windows::Win32::Foundation::{CloseHandle, FILETIME, HWND, LPARAM, MAX_PATH};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};
use windows::Win32::System::Threading::{
    GetProcessTimes, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
    QueryFullProcessImageNameW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowThreadProcessId, IsWindowVisible,
};
use windows::core::{BOOL, PWSTR};

use super::icon::extract_icon_bgra;
use super::{
    CpuTracker, IconPixels, ProcessGroup, ProcessIcon, ProcessInfo, ProcessStatus, Snapshot,
    SystemMonitor,
};

pub struct WindowsMonitor {
    logical_processors: u32,
    cpu: CpuTracker,
    prev_instant: Option<Instant>,
    /// Extracted icons keyed by executable path. `None` marks a path we tried
    /// and failed, so we don't retry it every tick.
    icon_cache: HashMap<String, Option<ProcessIcon>>,
}

impl Default for WindowsMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsMonitor {
    pub fn new() -> Self {
        let logical_processors = logical_processor_count();
        Self {
            logical_processors,
            cpu: CpuTracker::new(logical_processors),
            prev_instant: None,
            icon_cache: HashMap::new(),
        }
    }

    /// Icon for an executable path, extracting and caching on first use.
    fn icon_for(&mut self, path: &str) -> Option<ProcessIcon> {
        if path.is_empty() {
            return None;
        }
        self.icon_cache
            .entry(path.to_string())
            .or_insert_with(|| {
                extract_icon_bgra(path).map(|px: IconPixels| ProcessIcon(Rc::new(px)))
            })
            .clone()
    }
}

impl SystemMonitor for WindowsMonitor {
    fn sample(&mut self) -> Snapshot {
        let now = Instant::now();
        let elapsed = self
            .prev_instant
            .map_or(std::time::Duration::from_secs(1), |p| now.duration_since(p));
        self.prev_instant = Some(now);

        let windowed = pids_with_visible_windows();
        let suspended = super::suspend::suspended_pids();
        let raw = enumerate_processes();

        let busy: Vec<(u32, u64)> = raw
            .iter()
            .filter_map(|p| p.busy_100ns.map(|b| (p.pid, b)))
            .collect();
        let cpu = self.cpu.update(&busy, elapsed);

        let processes = raw
            .into_iter()
            .map(|p| {
                let icon = self.icon_for(&p.exe_path);
                ProcessInfo {
                    pid: p.pid,
                    name: p.image_name.clone(),
                    image_name: p.image_name,
                    status: if suspended.contains(&p.pid) {
                        ProcessStatus::Suspended
                    } else {
                        ProcessStatus::Running
                    },
                    group: if windowed.contains(&p.pid) {
                        ProcessGroup::App
                    } else {
                        ProcessGroup::Background
                    },
                    cpu_percent: cpu.get(&p.pid).copied().flatten(),
                    memory_bytes: p.memory_bytes,
                    icon,
                }
            })
            .collect();

        Snapshot {
            taken_at: now,
            logical_processors: self.logical_processors,
            processes,
        }
    }
}

struct RawProcess {
    pid: u32,
    image_name: String,
    exe_path: String,
    busy_100ns: Option<u64>,
    memory_bytes: Option<u64>,
}

fn logical_processor_count() -> u32 {
    let mut info = SYSTEM_INFO::default();
    unsafe { GetSystemInfo(&mut info) };
    info.dwNumberOfProcessors.max(1)
}

fn filetime_to_u64(ft: FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | ft.dwLowDateTime as u64
}

fn enumerate_processes() -> Vec<RawProcess> {
    let mut out = Vec::new();
    unsafe {
        let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
            return out;
        };

        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                out.push(read_process(&entry));
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }

        let _ = CloseHandle(snapshot);
    }
    out
}

unsafe fn read_process(entry: &PROCESSENTRY32W) -> RawProcess {
    let pid = entry.th32ProcessID;
    let image_name = wide_to_string(&entry.szExeFile);
    let mut exe_path = String::new();
    let mut busy_100ns = None;
    let mut memory_bytes = None;

    // Access denied is expected for many processes as a standard user; skip
    // quietly and leave the fields as None.
    if let Ok(handle) = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }
        && !handle.is_invalid()
    {
        let (mut creation, mut exit, mut kernel, mut user) = (
            FILETIME::default(),
            FILETIME::default(),
            FILETIME::default(),
            FILETIME::default(),
        );
        if unsafe { GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user) }
            .is_ok()
        {
            busy_100ns = Some(filetime_to_u64(kernel) + filetime_to_u64(user));
        }

        let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { zeroed() };
        counters.cb = size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        if unsafe { K32GetProcessMemoryInfo(handle, &mut counters, counters.cb) }.as_bool() {
            memory_bytes = Some(counters.WorkingSetSize as u64);
        }

        let mut buf = [0u16; MAX_PATH as usize];
        let mut len = buf.len() as u32;
        if unsafe {
            QueryFullProcessImageNameW(
                handle,
                PROCESS_NAME_WIN32,
                PWSTR(buf.as_mut_ptr()),
                &mut len,
            )
        }
        .is_ok()
        {
            exe_path = String::from_utf16_lossy(&buf[..len as usize]);
        }

        let _ = unsafe { CloseHandle(handle) };
    }

    RawProcess {
        pid,
        image_name,
        exe_path,
        busy_100ns,
        memory_bytes,
    }
}

fn wide_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

fn pids_with_visible_windows() -> HashSet<u32> {
    let mut set: HashSet<u32> = HashSet::new();
    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut set as *mut HashSet<u32> as isize),
        );
    }
    set
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        if IsWindowVisible(hwnd).as_bool() && GetWindowTextLengthW(hwnd) > 0 {
            let set = &mut *(lparam.0 as *mut HashSet<u32>);
            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid != 0 {
                set.insert(pid);
            }
        }
    }
    BOOL(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_processor_count_is_positive() {
        assert!(logical_processor_count() >= 1);
    }

    #[test]
    fn filetime_combines_high_and_low() {
        let ft = FILETIME {
            dwLowDateTime: 0x0000_0002,
            dwHighDateTime: 0x0000_0001,
        };
        assert_eq!(filetime_to_u64(ft), (1u64 << 32) | 2);
    }

    #[test]
    fn wide_to_string_stops_at_nul() {
        let mut buf = [0u16; 8];
        for (i, c) in "abc".encode_utf16().enumerate() {
            buf[i] = c;
        }
        assert_eq!(wide_to_string(&buf), "abc");
    }
}
