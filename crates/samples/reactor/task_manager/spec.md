# Task Manager (reactor sample) — Specification

A replacement for the Windows Task Manager, built in Rust on top of
`windows-reactor` (WinUI 3) and using Windows APIs directly via `windows-rs`.

This document is the working spec. It is detailed for the **MVP milestone** and
carries a lighter **roadmap** for later milestones toward full Task Manager
parity. It lives with the app crate and is updated as we go.

## 1. Vision & goals

This project serves several purposes at once (all of them, deliberately):

- **Showcase** what `windows-reactor` can do with a real, non-trivial app.
- **Daily-driver** replacement for Windows Task Manager that is genuinely useful.
- **Stress test** that exercises reactor with live, high-frequency data and
  surfaces gaps / rough edges to feed back into the framework.
- **Benchmark** for reactor's performance when rendering frequently-updating lists.

North star: **full parity with the Windows 11 Task Manager** (all tabs). We get
there iteratively; the MVP is the first vertical slice.

### Non-goals (for now)

- Cross-platform support. This is Windows-only and leans into `windows-rs`.
- Remote / multi-machine monitoring.
- Historical logging or persistence of metrics across runs.

## 2. Guiding principles

- **Use Windows APIs directly via `windows-rs`** (Toolhelp, `NtQuerySystemInformation`,
  PDH, etc.). Do not pull in a cross-platform crate like `sysinfo`. Discovering
  which APIs the projection exposes (and any gaps) is part of the value.
- **Run as a standard user.** Show everything we can; tolerate access-denied for
  processes we cannot open, and never crash or hang because of it. An elevation
  path can come later.
- **Match Task Manager conventions** where a choice is otherwise arbitrary (e.g.
  CPU% is the share of total capacity across all logical processors, 0–100%
  overall, the same number Task Manager shows).
- **Separate the data layer from the UI.** The data layer is a plain-Rust,
  testable module behind a trait so tests can inject fake data; the UI is a
  reactor render tree that consumes snapshots.

## 3. Architecture overview

```
+-------------------------------------------------------------+
|  reactor UI (render fns, hooks, NavigationView shell)       |
|    - Processes page  (MVP)                                  |
|    - Performance / Users / Details / Services … (later)     |
+-------------------------------------------------------------+
                |  consumes immutable Snapshot
                v
+-------------------------------------------------------------+
|  Data layer (plain Rust, no reactor deps)                   |
|    trait SystemMonitor { fn sample(&mut self) -> Snapshot } |
|      - WindowsMonitor  (windows-rs: Toolhelp / NtQuery…)    |
|      - FakeMonitor     (deterministic, for tests)           |
|    Snapshot { processes: Vec<ProcessInfo>, taken_at, … }    |
+-------------------------------------------------------------+
```

- The UI never calls Windows APIs directly. It asks the `SystemMonitor` for a
  `Snapshot` on a timer and renders it. This keeps the UI testable (headless
  reactor tests drive a `FakeMonitor`) and the data layer unit-testable in
  isolation.
- Sampling that is cheap and non-blocking runs on the UI thread via reactor's
  `DispatcherTimer`. If a sample turns out to be expensive it is moved to a
  worker thread and marshalled back with `use_ui_marshaller` / `use_async_state`.
  (Start simple; measure before adding threads.)

### Reactor building blocks we rely on

- `NavigationView` + `TitleBar` for the Win11-style shell with a left nav pane.
- `list_view` (templated list) for the process list.
- `DispatcherTimer` hook (`crates/libs/reactor/src/hooks.rs`) for the refresh loop.
- `use_state` / `use_ref` / `use_memo` for view state (sort key, expanded groups).
- `use_ui_marshaller` / `use_async_state` if/when sampling moves off-thread.

## 4. Crate layout

Location: `crates/samples/reactor/task_manager/` (alongside other reactor samples).

```
task_manager/
  Cargo.toml
  build.rs                # windows-reactor-setup (like sibling samples)
  spec.md                 # this document
  src/
    main.rs               # App::new().render(app)
    app.rs                # top-level shell (NavigationView, title bar)
    monitor/
      mod.rs              # SystemMonitor trait, Snapshot, ProcessInfo types
      windows.rs          # WindowsMonitor — windows-rs implementation
      fake.rs             # FakeMonitor — deterministic test double
      cpu.rs              # CPU% delta accounting
    pages/
      processes.rs        # Processes page (MVP)
  tests/                  # data-layer unit tests + headless reactor self-tests
```

Crate name: `reactor_task_manager` (follows sibling `reactor_gallery`).
`publish = false`, `edition = "2024"`, `[lints] workspace = true`.

## 5. MVP milestone — Live process list

### 5.1 Scope

A single **Processes** page showing a live-updating, grouped, sortable list of
processes. Read-only (no end-task yet). Win11 Task Manager look via
`NavigationView` shell (the nav pane lists the future tabs, but only Processes is
implemented; others are placeholders).

### 5.2 Columns

Per row (a process):

| Column   | Source / notes                                                        |
|----------|-----------------------------------------------------------------------|
| Icon     | Process executable icon (fallback to a generic icon on failure).      |
| Name     | Friendly name where available (e.g. product/description), else image name. |
| Status   | Running / Suspended (e.g. UWP suspended apps).                          |
| PID      | Process id.                                                            |
| CPU %    | Share of total CPU across all logical processors (Task Manager convention). |
| Memory   | Working set (private working set preferred; document which we use).     |

### 5.3 Grouping

Rows are grouped like Task Manager:

- **Apps** — processes with a visible top-level window / that the shell treats as
  foreground apps.
- **Background processes** — everything else the user can see.
- (Optional, later within MVP if cheap: **Windows processes**.)

Groups are **expandable/collapsible** with a header row. Group headers may show
an aggregate (sum of child CPU% / memory) — nice-to-have, not required for the
first cut.

### 5.4 Sorting

- Clicking a column header sorts by that column; clicking again toggles
  ascending/descending.
- Default sort: **CPU % descending** (most active first).
- Sorting is applied within the flat set and grouping is preserved (sort within
  each group, or sort groups by aggregate — decide during implementation; default
  is sort rows within their group).

### 5.5 Refresh cadence

- Configurable refresh interval; **default 1 second** (Task Manager "Normal").
- Interval options exposed in a simple control (e.g. Low/Normal/High or an
  explicit seconds value). Changing it re-arms the `DispatcherTimer`.
- CPU% is computed from the **delta** in process kernel+user time between the two
  most recent samples divided by elapsed wall-clock × logical-processor count.
  The first sample after launch has no prior baseline, so CPU% shows `—`/0 until
  the second sample.

### 5.6 Privilege / access-denied behaviour

- Runs unelevated. For processes we cannot `OpenProcess` (access denied), we
  still show the row with whatever data is available (name/PID from the snapshot
  enumeration) and leave unavailable fields blank or `—`.
- No crashes, no hangs, no error dialogs for expected access-denied cases.

### 5.7 Visual / UX

- Win11 Task Manager styling: left `NavigationView` pane with tab entries
  (Processes selected; Performance, App history, Startup apps, Users, Details,
  Services as disabled/placeholder items), a title bar, and the process list
  filling the content area.
- Live updates must not visually "flicker" or reset scroll position / expansion
  state on each tick. Row identity is keyed by PID so reactor can diff in place.

### 5.8 MVP acceptance criteria

Functional:

1. **Launches** as a standard (non-elevated) user and shows a window with a
   NavigationView shell and the Processes page selected.
2. **Enumerates processes** using Windows APIs via `windows-rs` (no third-party
   system-info crate) and lists them.
3. Each row shows **icon, friendly name, status, PID, CPU%, memory**.
4. The list **updates live** at the configured interval (default 1s) without
   flicker and **without losing scroll position, sort order, or group
   expand/collapse state** across ticks.
5. Processes are **grouped** into Apps / Background processes with
   **expandable/collapsible** group headers.
6. **CPU%** reflects the Task Manager convention (0–100% across all logical
   processors) and is derived from sample deltas; values are sane (sum across
   processes ≈ overall CPU usage, within reason).
7. **Sorting**: clicking a column header sorts by it; default sort is CPU%
   descending; toggling direction works.
8. The **refresh interval is configurable** and takes effect immediately.
9. **Access-denied** processes appear without crashing; unavailable fields are
   shown as blank/`—`.
10. Newly started processes appear, and exited processes disappear, on the next
    refresh.

Quality / non-functional:

11. `cargo fmt --all` is clean and `cargo clippy -p reactor_task_manager
    --all-targets` passes with **no warnings** (CI uses `-D warnings`).
12. `cargo check -p reactor_task_manager --quiet` succeeds.
13. No generated files are hand-edited; if any codegen is touched, the relevant
    `tool_*` is re-run and committed.
14. Idle CPU overhead of the app itself at the 1s interval is modest (target:
    the app is not a top CPU consumer on an otherwise idle machine). Exact
    threshold to be set once we can measure.

### 5.9 Testing (MVP)

Required (per stakeholder: full rigor):

- **Data-layer unit tests** (`tests/`), driving `FakeMonitor` and pure functions:
  - CPU% delta math: given two synthetic samples with known busy-time deltas and
    elapsed time and N logical processors, the computed CPU% matches expected
    values (including the first-sample-has-no-baseline case → 0/`—`).
  - Grouping: a synthetic snapshot is partitioned into Apps vs Background as
    expected.
  - Sorting: a synthetic set sorts correctly by each column, both directions,
    with grouping preserved.
  - Diffing/identity: process add/remove/update between two snapshots is detected
    by PID.
- **Headless reactor self-tests**: mount the Processes page with a `FakeMonitor`
  and assert the render tree (row count, cell text, group headers, that changing
  the fake snapshot and ticking updates the tree, that sort/expand state is
  preserved across a tick). Runs in CI headless (mirrors
  `cargo test -p test_reactor` / `--headless` conventions).
- **Runnable integration selftest harness** (`examples/selftest.rs`): a
  `--headless`/`--filter`-capable binary that exercises the real `WindowsMonitor`
  against live OS state — samples at least twice, asserts a non-empty process
  list, and confirms CPU%/memory baselines are established on the second sample.
  Exits non-zero on failure. (A heavier variant that also launches the real WinUI
  window and asserts it comes up — mirroring `test_reactor_selftest` — is deferred
  as a follow-up; see the roadmap.)

The `WindowsMonitor` itself is validated by the integration harness on a real
machine (its output is inherently machine-dependent, so it is not asserted
value-by-value in CI unit tests). Unit tests target the pure logic and the
`FakeMonitor`-driven paths.

## 6. Data model (initial sketch)

Subject to refinement during implementation.

```rust
pub struct Snapshot {
    pub taken_at: std::time::Instant,
    pub logical_processors: u32,
    pub processes: Vec<ProcessInfo>,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,           // friendly name, falls back to image name
    pub image_name: String,     // e.g. "chrome.exe"
    pub status: ProcessStatus,  // Running | Suspended
    pub group: ProcessGroup,    // App | Background (| Windows later)
    pub cpu_percent: Option<f32>,   // None until a baseline exists / access denied
    pub memory_bytes: Option<u64>,  // working set; None if unavailable
    pub icon: Option<IconHandle>,   // resolved lazily/cached
    // busy-time counters retained internally for the next delta
}

pub trait SystemMonitor {
    fn sample(&mut self) -> Snapshot;
}
```

Notes:
- The monitor is stateful: it keeps the previous per-PID busy-time counters to
  compute CPU% deltas.
- Icons are relatively expensive; resolve lazily and cache by executable path.

## 7. Roadmap (lighter detail — refine as we reach each)

Ordered roughly by likely implementation order. Each becomes a detailed section
(with its own acceptance criteria) when we start it.

1. **Processes — actions.** End task / kill process, context menu (open file
   location, properties, go to details, set priority/affinity). Requires handling
   confirmation and access-denied/elevation gracefully.
2. **Performance tab.** Live line graphs (CPU overall + per-core, Memory, Disk,
   Ethernet/Wi‑Fi, GPU). Likely a good `windows-canvas` showcase for the charts.
   Data via PDH / performance counters.
3. **Details tab.** Flat, dense, sortable process table with many columns
   (PID, status, user name, CPU, memory variants, handles, threads, etc.),
   column chooser.
4. **Startup apps.** Enumerate and enable/disable startup entries; "startup
   impact".
5. **Services.** List services, status, start/stop/restart, link to Details.
6. **Users.** Per-session grouping of processes by signed-in user.
7. **App history.** Resource usage over time for packaged apps.
8. **Settings / chrome.** Refresh-rate presets, theme, always-on-top, start
   minimized / tray, default tab, "Efficiency mode".

Cross-cutting later work: optional **elevation / "run as admin"** path to reveal
data denied to a standard user; column customization/persistence; search/filter.

Deferred from the MVP (small, tracked here so they aren't lost):

- **WinUI-window integration self-test.** A `test_reactor_selftest`-style harness
  that launches the real window and asserts it comes up (the current
  `examples/selftest.rs` validates the data layer only).

Landed after the initial MVP:

- **Suspended-state detection.** A process is reported `Suspended` when it has
  threads and every thread is waiting with a `Suspended` wait reason — the same
  heuristic Task Manager and Process Hacker use, covering both `SuspendThread`
  suspension and OS-frozen packaged (UWP) apps. Implemented via
  `NtQuerySystemInformation(SystemProcessInformation)` in `monitor/suspend.rs`.
- **Per-executable icons.** Each process's small icon is extracted (`SHGetFileInfo`
  → `GetDIBits` to premultiplied BGRA8) and cached by executable path in the
  monitor. The UI turns the pixels into an in-memory WinUI `WriteableBitmap` via
  the reactor `RasterImageSource` (no temp files), memoised by icon identity.
  Rows fall back to the app/background Segoe Fluent Icons glyph when extraction
  fails.

## 8. Open questions (to resolve as they come up)

- Which exact enumeration + timing API gives the best cost/accuracy tradeoff for
  CPU%: Toolhelp snapshot + `OpenProcess`/`GetProcessTimes`, vs a single
  `NtQuerySystemInformation(SystemProcessInformation)` pass (which returns times,
  threads, and memory in one call and avoids per-process opens). Prototype both;
  the single-call approach is likely cheaper and more access-denied friendly.
- "Friendly name" source: version-info product/description vs. shell display name.
- Precise memory metric shown in the Memory column (working set vs private
  working set vs commit) — match Task Manager's default.
- App vs Background classification heuristic (visible top-level window on the
  desktop, package identity, etc.).

---

*Assisted-by: copilot*
