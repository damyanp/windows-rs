//! Per-process CPU% accounting from busy-time deltas.
//!
//! CPU% follows the Task Manager convention: the share of total capacity across
//! all logical processors, so the values across all processes sum to roughly the
//! overall CPU usage (0-100).

use std::collections::HashMap;
use std::time::Duration;

/// Computes one process's CPU% from a busy-time delta.
///
/// * `busy_delta_100ns` — increase in kernel+user time (in 100ns units) since
///   the previous sample.
/// * `elapsed` — wall-clock time between the two samples.
/// * `logical_processors` — number of logical processors (>= 1).
pub fn cpu_percent(busy_delta_100ns: u64, elapsed: Duration, logical_processors: u32) -> f32 {
    let elapsed_100ns = (elapsed.as_nanos() / 100) as f64;
    let procs = logical_processors.max(1) as f64;
    if elapsed_100ns <= 0.0 {
        return 0.0;
    }
    let pct = (busy_delta_100ns as f64 / (elapsed_100ns * procs)) * 100.0;
    pct.clamp(0.0, 100.0) as f32
}

/// Tracks previous busy times per PID so successive samples yield CPU%.
///
/// Shared by the real and fake monitors; the delta math is pure and unit
/// tested independently of any OS calls.
#[derive(Default)]
pub struct CpuTracker {
    prev_busy: HashMap<u32, u64>,
    logical_processors: u32,
}

impl CpuTracker {
    pub fn new(logical_processors: u32) -> Self {
        Self {
            prev_busy: HashMap::new(),
            logical_processors: logical_processors.max(1),
        }
    }

    /// Feeds current raw busy times and the elapsed interval, returning CPU% per
    /// PID. A PID seen for the first time (no baseline) yields `None`. PIDs
    /// absent from `current` are pruned.
    pub fn update(
        &mut self,
        current: &[(u32, u64)],
        elapsed: Duration,
    ) -> HashMap<u32, Option<f32>> {
        let mut out = HashMap::with_capacity(current.len());
        let mut next = HashMap::with_capacity(current.len());

        for &(pid, busy) in current {
            let pct = match self.prev_busy.get(&pid) {
                Some(&prev) => Some(cpu_percent(
                    busy.saturating_sub(prev),
                    elapsed,
                    self.logical_processors,
                )),
                None => None,
            };
            out.insert(pid, pct);
            next.insert(pid, busy);
        }

        self.prev_busy = next;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_full_core_busy_over_one_second() {
        // 1s == 10_000_000 * 100ns. A single core fully busy on a 4-core box is
        // 25% of total capacity.
        let pct = cpu_percent(10_000_000, Duration::from_secs(1), 4);
        assert!((pct - 25.0).abs() < 0.001, "got {pct}");
    }

    #[test]
    fn all_cores_busy_is_capped_at_100() {
        // Busy time equal to elapsed * cores -> 100%.
        let pct = cpu_percent(40_000_000, Duration::from_secs(1), 4);
        assert!((pct - 100.0).abs() < 0.001, "got {pct}");
    }

    #[test]
    fn over_budget_busy_is_clamped() {
        let pct = cpu_percent(999_000_000, Duration::from_secs(1), 4);
        assert_eq!(pct, 100.0);
    }

    #[test]
    fn zero_elapsed_is_zero() {
        assert_eq!(cpu_percent(10_000_000, Duration::ZERO, 4), 0.0);
    }

    #[test]
    fn first_sample_has_no_baseline() {
        let mut t = CpuTracker::new(4);
        let out = t.update(&[(1, 1_000_000)], Duration::from_secs(1));
        assert_eq!(out.get(&1), Some(&None));
    }

    #[test]
    fn second_sample_uses_delta() {
        let mut t = CpuTracker::new(4);
        t.update(&[(1, 1_000_000)], Duration::from_secs(1));
        // +10_000_000 (100ns) over 1s on 4 cores == 25%.
        let out = t.update(&[(1, 11_000_000)], Duration::from_secs(1));
        assert_eq!(out.get(&1), Some(&Some(25.0)));
    }

    #[test]
    fn vanished_pid_is_pruned() {
        let mut t = CpuTracker::new(2);
        t.update(&[(1, 100), (2, 100)], Duration::from_secs(1));
        t.update(&[(1, 200)], Duration::from_secs(1));
        // PID 2 gone; reappearing later must be treated as a new baseline.
        let out = t.update(&[(2, 500)], Duration::from_secs(1));
        assert_eq!(out.get(&2), Some(&None));
    }
}
