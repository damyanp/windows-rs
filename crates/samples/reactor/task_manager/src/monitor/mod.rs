//! Data layer: system monitoring types and the `SystemMonitor` trait.
//!
//! Everything here is plain Rust with no reactor dependency so it can be unit
//! tested in isolation, and so the UI can be driven by a `FakeMonitor` in tests.

mod cpu;
mod fake;
mod windows;

pub use cpu::CpuTracker;
pub use fake::FakeMonitor;
pub use windows::WindowsMonitor;

use std::time::Instant;

/// A process's classification for grouping in the list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessGroup {
    /// Has a visible top-level window on the desktop.
    App,
    /// Everything else the user can see.
    Background,
}

/// Coarse run state shown in the Status column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessStatus {
    Running,
    Suspended,
}

/// One process in a snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessInfo {
    pub pid: u32,
    /// Friendly name where available, else the image name.
    pub name: String,
    /// Executable file name, e.g. "chrome.exe".
    pub image_name: String,
    pub status: ProcessStatus,
    pub group: ProcessGroup,
    /// Share of total CPU across all logical processors (0-100). `None` until a
    /// baseline exists or when the data is unavailable (access denied).
    pub cpu_percent: Option<f32>,
    /// Working-set bytes. `None` when unavailable.
    pub memory_bytes: Option<u64>,
}

/// An immutable point-in-time view of the system.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub taken_at: Instant,
    pub logical_processors: u32,
    pub processes: Vec<ProcessInfo>,
}

/// Selects which data source the UI is driven by. Kept as a small `Copy` enum
/// so it can live in reactor props (which must be `Clone + PartialEq`), while
/// still letting the self-test harness pick the deterministic `FakeMonitor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonitorKind {
    /// Real Windows-backed monitor.
    Real,
    /// Deterministic fake, for tests and demos.
    Fake,
}

impl MonitorKind {
    pub fn create(self) -> Box<dyn SystemMonitor> {
        match self {
            Self::Real => Box::new(WindowsMonitor::new()),
            Self::Fake => Box::new(FakeMonitor::new()),
        }
    }
}

/// Produces a fresh [`Snapshot`] each time it is polled. Implementations are
/// stateful (they retain the previous sample to compute CPU% deltas).
pub trait SystemMonitor {
    fn sample(&mut self) -> Snapshot;
}
