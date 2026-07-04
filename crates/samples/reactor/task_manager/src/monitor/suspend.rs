//! Detects suspended processes via `NtQuerySystemInformation`.
//!
//! A process is considered suspended when it has threads and every thread is
//! waiting with a `Suspended` wait reason — the same heuristic Task Manager and
//! Process Hacker use. This covers both classic `SuspendThread` suspension and
//! OS-frozen packaged (UWP) apps.

use std::collections::HashSet;
use std::mem::size_of;

use windows::Wdk::System::SystemInformation::{NtQuerySystemInformation, SystemProcessInformation};
use windows::Win32::System::WindowsProgramming::{
    SYSTEM_PROCESS_INFORMATION, SYSTEM_THREAD_INFORMATION,
};

/// `ThreadState` value for a thread in the waiting state.
const THREAD_STATE_WAITING: u32 = 5;
/// `KWAIT_REASON::Suspended`.
const WAIT_REASON_SUSPENDED: u32 = 5;

/// The set of process IDs whose every thread is suspended.
pub fn suspended_pids() -> HashSet<u32> {
    let mut out = HashSet::new();
    if let Some(buf) = query_process_information() {
        unsafe { collect_suspended(&buf, &mut out) };
    }
    out
}

/// Whether a process's threads indicate it is suspended.
fn is_suspended(threads: &[SYSTEM_THREAD_INFORMATION]) -> bool {
    !threads.is_empty()
        && threads
            .iter()
            .all(|t| t.ThreadState == THREAD_STATE_WAITING && t.WaitReason == WAIT_REASON_SUSPENDED)
}

/// Snapshot all process/thread information into an 8-byte-aligned buffer,
/// growing until it fits. Returns `None` on failure.
fn query_process_information() -> Option<Vec<u64>> {
    let mut cap_bytes = 1024 * 1024usize;
    loop {
        let mut buf = vec![0u64; cap_bytes.div_ceil(8)];
        let mut needed = 0u32;
        let status = unsafe {
            NtQuerySystemInformation(
                SystemProcessInformation,
                buf.as_mut_ptr() as *mut _,
                (buf.len() * 8) as u32,
                &mut needed,
            )
        };
        if status.0 >= 0 {
            return Some(buf);
        }
        // Any negative NTSTATUS here is effectively "buffer too small"; grow and
        // retry, bailing out if we can't make progress or it gets unreasonable.
        let next = (needed as usize).max(cap_bytes * 2) + 64 * 1024;
        if next <= cap_bytes || next > 128 * 1024 * 1024 {
            return None;
        }
        cap_bytes = next;
    }
}

/// Walk the `SYSTEM_PROCESS_INFORMATION` linked list, recording suspended PIDs.
unsafe fn collect_suspended(buf: &[u64], out: &mut HashSet<u32>) {
    let base = buf.as_ptr() as *const u8;
    let end = buf.len() * 8;
    let mut offset = 0usize;

    loop {
        let proc_ptr = unsafe { base.add(offset) } as *const SYSTEM_PROCESS_INFORMATION;
        let process = unsafe { &*proc_ptr };

        let count = process.NumberOfThreads as usize;
        if count > 0 {
            let threads_ptr =
                unsafe { (proc_ptr as *const u8).add(size_of::<SYSTEM_PROCESS_INFORMATION>()) }
                    as *const SYSTEM_THREAD_INFORMATION;
            let threads = unsafe { std::slice::from_raw_parts(threads_ptr, count) };
            if is_suspended(threads) {
                out.insert(process.UniqueProcessId.0 as usize as u32);
            }
        }

        let step = process.NextEntryOffset as usize;
        if step == 0 {
            break;
        }
        offset += step;
        if offset >= end {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thread(state: u32, reason: u32) -> SYSTEM_THREAD_INFORMATION {
        SYSTEM_THREAD_INFORMATION {
            ThreadState: state,
            WaitReason: reason,
            ..Default::default()
        }
    }

    #[test]
    fn no_threads_is_not_suspended() {
        assert!(!is_suspended(&[]));
    }

    #[test]
    fn all_waiting_suspended_is_suspended() {
        let threads = [thread(5, 5), thread(5, 5)];
        assert!(is_suspended(&threads));
    }

    #[test]
    fn any_running_thread_is_not_suspended() {
        let threads = [thread(5, 5), thread(2, 0)];
        assert!(!is_suspended(&threads));
    }

    #[test]
    fn waiting_for_other_reason_is_not_suspended() {
        // Waiting on a queue (reason 15), not suspended.
        let threads = [thread(5, 15)];
        assert!(!is_suspended(&threads));
    }

    #[test]
    fn live_query_walks_without_panicking() {
        // Exercises the real NtQuerySystemInformation walk on live data, which
        // catches buffer-sizing and struct-layout mistakes. We can't assert a
        // specific count, but print it for manual inspection.
        let pids = suspended_pids();
        println!("suspended pids: {} ({pids:?})", pids.len());
    }
}
