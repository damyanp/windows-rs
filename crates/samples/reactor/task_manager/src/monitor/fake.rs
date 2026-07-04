//! A deterministic monitor used by tests and for running the UI without real
//! system data. Its output varies per tick so live updates are observable.

use std::time::Instant;

use super::{CpuTracker, ProcessGroup, ProcessInfo, ProcessStatus, Snapshot, SystemMonitor};

pub struct FakeMonitor {
    tick: u64,
    cpu: CpuTracker,
    logical_processors: u32,
}

impl Default for FakeMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeMonitor {
    pub fn new() -> Self {
        Self {
            tick: 0,
            cpu: CpuTracker::new(4),
            logical_processors: 4,
        }
    }

    fn roster() -> &'static [(u32, &'static str, &'static str, ProcessStatus, ProcessGroup)] {
        &[
            (
                1000,
                "Contoso Browser",
                "browser.exe",
                ProcessStatus::Running,
                ProcessGroup::App,
            ),
            (
                1200,
                "Text Editor",
                "editor.exe",
                ProcessStatus::Running,
                ProcessGroup::App,
            ),
            (
                1300,
                "Mail",
                "mail.exe",
                ProcessStatus::Suspended,
                ProcessGroup::App,
            ),
            (
                400,
                "System Service Host",
                "svchost.exe",
                ProcessStatus::Running,
                ProcessGroup::Background,
            ),
            (
                404,
                "Search Indexer",
                "indexer.exe",
                ProcessStatus::Running,
                ProcessGroup::Background,
            ),
            (
                408,
                "Update Agent",
                "updater.exe",
                ProcessStatus::Running,
                ProcessGroup::Background,
            ),
        ]
    }
}

impl SystemMonitor for FakeMonitor {
    fn sample(&mut self) -> Snapshot {
        self.tick += 1;
        let tick = self.tick;

        // Synthesize monotonically increasing busy times; each process gets a
        // different, tick-dependent load so the list visibly changes.
        let raw: Vec<(u32, u64)> = Self::roster()
            .iter()
            .enumerate()
            .map(|(i, (pid, ..))| {
                let load = (tick.wrapping_mul(7).wrapping_add(i as u64 * 13)) % 20;
                let busy = tick * 500_000 + load * 100_000;
                (*pid, busy)
            })
            .collect();

        let cpu = self.cpu.update(&raw, std::time::Duration::from_secs(1));

        let processes = Self::roster()
            .iter()
            .map(|(pid, name, image, status, group)| ProcessInfo {
                pid: *pid,
                name: (*name).to_string(),
                image_name: (*image).to_string(),
                status: *status,
                group: *group,
                cpu_percent: cpu.get(pid).copied().flatten(),
                memory_bytes: Some((*pid as u64) * 1_048_576),
                icon: None,
            })
            .collect();

        Snapshot {
            taken_at: Instant::now(),
            logical_processors: self.logical_processors,
            processes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sample_has_no_cpu_baseline() {
        let mut m = FakeMonitor::new();
        let snap = m.sample();
        assert_eq!(snap.processes.len(), FakeMonitor::roster().len());
        assert!(snap.processes.iter().all(|p| p.cpu_percent.is_none()));
    }

    #[test]
    fn later_samples_report_cpu() {
        let mut m = FakeMonitor::new();
        m.sample();
        let snap = m.sample();
        assert!(snap.processes.iter().all(|p| p.cpu_percent.is_some()));
    }
}
