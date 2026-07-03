//! The Processes page: a live, grouped, sortable process list.

use std::time::{Duration, Instant};

use windows_reactor::*;

use crate::monitor::{MonitorKind, ProcessGroup, ProcessInfo, ProcessStatus, Snapshot};
use crate::view::{Row, Sort, SortColumn, build_rows, group_label};

const COL_ICON: f64 = 36.0;
const COL_NAME: f64 = 260.0;
const COL_STATUS: f64 = 100.0;
const COL_PID: f64 = 80.0;
const COL_CPU: f64 = 80.0;
const COL_MEM: f64 = 110.0;

const INTERVAL_OPTIONS: [(&str, u64); 3] = [
    ("High (0.5s)", 500),
    ("Normal (1s)", 1000),
    ("Low (4s)", 4000),
];

fn empty_snapshot() -> Snapshot {
    Snapshot {
        taken_at: Instant::now(),
        logical_processors: 1,
        processes: Vec::new(),
    }
}

/// Props for [`processes_page`]. `monitor` selects the data source, kept as a
/// small `Copy` enum so props stay `Clone + PartialEq` as components require.
#[derive(Clone, Copy, PartialEq)]
pub struct ProcessesProps {
    pub monitor: MonitorKind,
}

/// Renders the Processes page. The monitor is constructed once from the props.
pub fn processes_page(props: &ProcessesProps, cx: &mut RenderCx) -> Element {
    let kind = props.monitor;
    let (snapshot, set_snapshot) = cx.use_state(empty_snapshot());
    let (sort, set_sort) = cx.use_state(Sort::default());
    let (collapsed, set_collapsed) = cx.use_state(Vec::<ProcessGroup>::new());
    let (interval_ms, set_interval) = cx.use_state(1000_u64);

    let monitor = cx.use_ref::<Option<Box<dyn crate::monitor::SystemMonitor>>>(None);

    // Refresh loop: (re)armed whenever the interval changes. The monitor is
    // constructed once and retained across ticks in a ref.
    {
        let monitor = monitor.clone();
        let set_snapshot = set_snapshot;
        cx.use_effect_with_cleanup((interval_ms,), move || {
            if monitor.borrow().is_none() {
                monitor.set(Some(kind.create()));
            }
            let sample = {
                let monitor = monitor.clone();
                let set_snapshot = set_snapshot.clone();
                move || {
                    if let Some(m) = monitor.borrow_mut().as_mut() {
                        set_snapshot.call(m.sample());
                    }
                }
            };
            sample(); // paint immediately, don't wait a full interval
            let timer = DispatcherTimer::new(Duration::from_millis(interval_ms), sample).ok();
            Some(move || drop(timer))
        });
    }

    let rows = build_rows(&snapshot, sort, &collapsed);

    let toolbar = build_toolbar(&snapshot, interval_ms, set_interval);
    let header = build_header(sort, set_sort);

    let list = list_view(rows, move |row, _idx| {
        render_row(row, &collapsed, set_collapsed.clone())
    })
    .with_key_selector(|row: &Row| row.key());

    grid((
        toolbar.grid_row(0),
        header.grid_row(1),
        Element::from(list).grid_row(2),
    ))
    .rows([GridLength::Auto, GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Star(1.0)])
    .row_spacing(4.0)
    .padding(Thickness::uniform(12.0))
    .into()
}

fn build_toolbar(snapshot: &Snapshot, interval_ms: u64, set_interval: SetState<u64>) -> Element {
    let selected = INTERVAL_OPTIONS
        .iter()
        .position(|(_, ms)| *ms == interval_ms)
        .unwrap_or(1) as i32;

    let interval_picker = ComboBox::new(INTERVAL_OPTIONS.iter().map(|(label, _)| *label))
        .selected_index(selected)
        .on_selection_changed(move |i: i32| {
            if let Some((_, ms)) = INTERVAL_OPTIONS.get(i.max(0) as usize) {
                set_interval.call(*ms);
            }
        })
        .width(160.0);

    let count = text_block(format!(
        "{} processes  •  {} logical processors",
        snapshot.processes.len(),
        snapshot.logical_processors
    ))
    .opacity(0.7);

    hstack((
        text_block("Update rate").opacity(0.7),
        interval_picker,
        count,
    ))
    .spacing(12.0)
    .vertical_alignment(VerticalAlignment::Center)
    .into()
}

fn header_cell(
    label: &str,
    column: SortColumn,
    sort: Sort,
    set_sort: SetState<Sort>,
    width: f64,
) -> Element {
    let indicator = if sort.column == column {
        match sort.dir {
            crate::view::SortDir::Asc => " \u{25B2}",
            crate::view::SortDir::Desc => " \u{25BC}",
        }
    } else {
        ""
    };
    button(format!("{label}{indicator}"))
        .on_click(move || set_sort.call(sort.on_header_click(column)))
        .width(width)
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .into()
}

fn build_header(sort: Sort, set_sort: SetState<Sort>) -> Element {
    hstack((
        text_block("").width(COL_ICON),
        header_cell("Name", SortColumn::Name, sort, set_sort.clone(), COL_NAME),
        header_cell(
            "Status",
            SortColumn::Status,
            sort,
            set_sort.clone(),
            COL_STATUS,
        ),
        header_cell("PID", SortColumn::Pid, sort, set_sort.clone(), COL_PID),
        header_cell("CPU", SortColumn::Cpu, sort, set_sort.clone(), COL_CPU),
        header_cell("Memory", SortColumn::Memory, sort, set_sort, COL_MEM),
    ))
    .spacing(4.0)
    .into()
}

fn render_row(
    row: &Row,
    collapsed: &[ProcessGroup],
    set_collapsed: SetState<Vec<ProcessGroup>>,
) -> Element {
    match row {
        Row::Group {
            group,
            count,
            expanded,
        } => render_group_header(*group, *count, *expanded, collapsed.to_vec(), set_collapsed),
        Row::Process(p) => render_process_row(p),
    }
}

fn render_group_header(
    group: ProcessGroup,
    count: usize,
    expanded: bool,
    collapsed: Vec<ProcessGroup>,
    set_collapsed: SetState<Vec<ProcessGroup>>,
) -> Element {
    let chevron = if expanded { "\u{25BC}" } else { "\u{25B6}" };
    button(format!("{chevron}  {}  ({count})", group_label(group)))
        .on_click(move || {
            let mut next = collapsed.clone();
            if let Some(pos) = next.iter().position(|g| *g == group) {
                next.remove(pos);
            } else {
                next.push(group);
            }
            set_collapsed.call(next);
        })
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .into()
}

fn render_process_row(p: &ProcessInfo) -> Element {
    let status = match p.status {
        ProcessStatus::Running => "Running",
        ProcessStatus::Suspended => "Suspended",
    };
    let cpu = match p.cpu_percent {
        Some(v) => format!("{v:.1}%"),
        None => "\u{2014}".to_string(),
    };
    let mem = match p.memory_bytes {
        Some(b) => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
        None => "\u{2014}".to_string(),
    };

    // Placeholder icon; per-executable icons are a follow-up (see spec).
    let glyph = match p.group {
        ProcessGroup::App => "\u{E737}",
        ProcessGroup::Background => "\u{E115}",
    };

    hstack((
        text_block(glyph)
            .font_family("Segoe Fluent Icons")
            .width(COL_ICON),
        text_block(p.name.clone()).width(COL_NAME),
        text_block(status).width(COL_STATUS).opacity(0.8),
        text_block(p.pid.to_string()).width(COL_PID).opacity(0.8),
        text_block(cpu).width(COL_CPU),
        text_block(mem).width(COL_MEM).opacity(0.8),
    ))
    .spacing(4.0)
    .padding(Thickness {
        left: 4.0,
        top: 4.0,
        right: 4.0,
        bottom: 4.0,
    })
    .into()
}
