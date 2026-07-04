//! Data layer: system monitoring types and the `SystemMonitor` trait.
//!
//! Everything here is plain Rust with no reactor dependency so it can be unit
//! tested in isolation, and so the UI can be driven by a `FakeMonitor` in tests.

mod cpu;
mod fake;
mod icon;
mod suspend;
mod windows;

pub use cpu::CpuTracker;
pub use fake::FakeMonitor;
pub use windows::WindowsMonitor;

use std::rc::Rc;
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

/// Raw BGRA8 pixels for a process's icon, top-down and premultiplied so the UI
/// can hand them straight to a `WriteableBitmap`. Data-layer only — no WinUI
/// types, so it stays unit-testable and usable off the UI thread.
#[derive(Debug)]
pub struct IconPixels {
    pub width: i32,
    pub height: i32,
    pub bgra: Vec<u8>,
}

/// A shared, cheaply-cloned handle to a process icon. Equality is by identity
/// (pointer), so unchanged icons don't trigger snapshot churn and the UI can
/// cache the built image source by pointer.
#[derive(Clone)]
pub struct ProcessIcon(pub Rc<IconPixels>);

impl ProcessIcon {
    /// Stable identity for use as a cache key.
    pub fn id(&self) -> usize {
        Rc::as_ptr(&self.0) as usize
    }
}

impl PartialEq for ProcessIcon {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl std::fmt::Debug for ProcessIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProcessIcon({}x{})", self.0.width, self.0.height)
    }
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
    /// The executable's icon, when it could be extracted.
    pub icon: Option<ProcessIcon>,
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
