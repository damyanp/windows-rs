//! Pure presentation logic: turning a [`Snapshot`] into a flat, grouped,
//! sorted list of rows for the UI. No reactor or OS dependencies, so it is
//! fully unit tested.

use crate::monitor::{ProcessGroup, ProcessInfo, Snapshot};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortColumn {
    Name,
    Status,
    Pid,
    Cpu,
    Memory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sort {
    pub column: SortColumn,
    pub dir: SortDir,
}

impl Default for Sort {
    fn default() -> Self {
        // Task Manager default: most active first.
        Self {
            column: SortColumn::Cpu,
            dir: SortDir::Desc,
        }
    }
}

impl Sort {
    /// Toggles direction if the same column is clicked, otherwise switches to
    /// the new column with a sensible default direction.
    pub fn on_header_click(self, column: SortColumn) -> Self {
        if self.column == column {
            Self {
                column,
                dir: match self.dir {
                    SortDir::Asc => SortDir::Desc,
                    SortDir::Desc => SortDir::Asc,
                },
            }
        } else {
            let dir = match column {
                // Numeric columns feel natural descending first.
                SortColumn::Cpu | SortColumn::Memory => SortDir::Desc,
                _ => SortDir::Asc,
            };
            Self { column, dir }
        }
    }
}

/// A single display row: either a group header or a process.
#[derive(Clone, Debug, PartialEq)]
pub enum Row {
    Group {
        group: ProcessGroup,
        count: usize,
        expanded: bool,
    },
    Process(ProcessInfo),
}

impl Row {
    /// Stable identity for list diffing.
    pub fn key(&self) -> String {
        match self {
            Self::Group { group, .. } => format!("g:{}", group_order(*group)),
            Self::Process(p) => format!("p:{}", p.pid),
        }
    }
}

fn group_order(group: ProcessGroup) -> u8 {
    match group {
        ProcessGroup::App => 0,
        ProcessGroup::Background => 1,
    }
}

pub fn group_label(group: ProcessGroup) -> &'static str {
    match group {
        ProcessGroup::App => "Apps",
        ProcessGroup::Background => "Background processes",
    }
}

fn cmp_processes(a: &ProcessInfo, b: &ProcessInfo, sort: Sort) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let ord = match sort.column {
        SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        SortColumn::Status => (a.status as u8).cmp(&(b.status as u8)),
        SortColumn::Pid => a.pid.cmp(&b.pid),
        SortColumn::Cpu => a
            .cpu_percent
            .unwrap_or(-1.0)
            .partial_cmp(&b.cpu_percent.unwrap_or(-1.0))
            .unwrap_or(Ordering::Equal),
        SortColumn::Memory => a
            .memory_bytes
            .unwrap_or(0)
            .cmp(&b.memory_bytes.unwrap_or(0)),
    };
    // Stable tie-breaker so equal keys keep a deterministic order.
    let ord = ord.then_with(|| a.pid.cmp(&b.pid));
    match sort.dir {
        SortDir::Asc => ord,
        SortDir::Desc => ord.reverse(),
    }
}

/// Builds the flat list of rows: a header per group followed by that group's
/// processes (unless collapsed). Groups are ordered Apps then Background.
pub fn build_rows(snapshot: &Snapshot, sort: Sort, collapsed: &[ProcessGroup]) -> Vec<Row> {
    let mut rows = Vec::new();
    for group in [ProcessGroup::App, ProcessGroup::Background] {
        let mut members: Vec<ProcessInfo> = snapshot
            .processes
            .iter()
            .filter(|p| p.group == group)
            .cloned()
            .collect();
        members.sort_by(|a, b| cmp_processes(a, b, sort));

        let expanded = !collapsed.contains(&group);
        rows.push(Row::Group {
            group,
            count: members.len(),
            expanded,
        });
        if expanded {
            rows.extend(members.into_iter().map(Row::Process));
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitor::ProcessStatus;
    use std::time::Instant;

    fn proc(
        pid: u32,
        name: &str,
        cpu: Option<f32>,
        mem: Option<u64>,
        group: ProcessGroup,
    ) -> ProcessInfo {
        ProcessInfo {
            pid,
            name: name.to_string(),
            image_name: format!("{name}.exe"),
            status: ProcessStatus::Running,
            group,
            cpu_percent: cpu,
            memory_bytes: mem,
            icon: None,
        }
    }

    fn snapshot(processes: Vec<ProcessInfo>) -> Snapshot {
        Snapshot {
            taken_at: Instant::now(),
            logical_processors: 4,
            processes,
        }
    }

    #[test]
    fn groups_apps_before_background_with_headers() {
        let snap = snapshot(vec![
            proc(2, "bg", Some(1.0), Some(10), ProcessGroup::Background),
            proc(1, "app", Some(2.0), Some(20), ProcessGroup::App),
        ]);
        let rows = build_rows(&snap, Sort::default(), &[]);
        assert!(matches!(
            rows[0],
            Row::Group {
                group: ProcessGroup::App,
                count: 1,
                ..
            }
        ));
        assert!(matches!(rows[1], Row::Process(ref p) if p.pid == 1));
        assert!(matches!(
            rows[2],
            Row::Group {
                group: ProcessGroup::Background,
                count: 1,
                ..
            }
        ));
        assert!(matches!(rows[3], Row::Process(ref p) if p.pid == 2));
    }

    #[test]
    fn collapsed_group_hides_members() {
        let snap = snapshot(vec![proc(1, "app", Some(2.0), Some(20), ProcessGroup::App)]);
        let rows = build_rows(&snap, Sort::default(), &[ProcessGroup::App]);
        // Only the two headers, no process rows.
        assert_eq!(rows.len(), 2);
        assert!(matches!(
            rows[0],
            Row::Group {
                expanded: false,
                ..
            }
        ));
    }

    #[test]
    fn sorts_by_cpu_descending_by_default() {
        let snap = snapshot(vec![
            proc(1, "a", Some(5.0), None, ProcessGroup::App),
            proc(2, "b", Some(50.0), None, ProcessGroup::App),
            proc(3, "c", Some(20.0), None, ProcessGroup::App),
        ]);
        let rows = build_rows(&snap, Sort::default(), &[]);
        let pids: Vec<u32> = rows
            .iter()
            .filter_map(|r| match r {
                Row::Process(p) => Some(p.pid),
                _ => None,
            })
            .collect();
        assert_eq!(pids, vec![2, 3, 1]);
    }

    #[test]
    fn sorts_by_name_ascending() {
        let snap = snapshot(vec![
            proc(1, "Charlie", None, None, ProcessGroup::App),
            proc(2, "alpha", None, None, ProcessGroup::App),
            proc(3, "Bravo", None, None, ProcessGroup::App),
        ]);
        let sort = Sort {
            column: SortColumn::Name,
            dir: SortDir::Asc,
        };
        let rows = build_rows(&snap, sort, &[]);
        let names: Vec<String> = rows
            .iter()
            .filter_map(|r| match r {
                Row::Process(p) => Some(p.name.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(names, vec!["alpha", "Bravo", "Charlie"]);
    }

    #[test]
    fn unknown_cpu_sorts_last_when_descending() {
        let snap = snapshot(vec![
            proc(1, "a", None, None, ProcessGroup::App),
            proc(2, "b", Some(1.0), None, ProcessGroup::App),
        ]);
        let rows = build_rows(&snap, Sort::default(), &[]);
        let pids: Vec<u32> = rows
            .iter()
            .filter_map(|r| match r {
                Row::Process(p) => Some(p.pid),
                _ => None,
            })
            .collect();
        assert_eq!(pids, vec![2, 1]);
    }

    #[test]
    fn header_click_toggles_direction_on_same_column() {
        let s = Sort {
            column: SortColumn::Cpu,
            dir: SortDir::Desc,
        };
        assert_eq!(s.on_header_click(SortColumn::Cpu).dir, SortDir::Asc);
    }

    #[test]
    fn header_click_switches_column_with_default_dir() {
        let s = Sort::default();
        let s2 = s.on_header_click(SortColumn::Name);
        assert_eq!(s2.column, SortColumn::Name);
        assert_eq!(s2.dir, SortDir::Asc);
    }

    #[test]
    fn row_keys_are_stable_and_distinct() {
        let snap = snapshot(vec![proc(7, "x", None, None, ProcessGroup::App)]);
        let rows = build_rows(&snap, Sort::default(), &[]);
        assert_eq!(rows[0].key(), "g:0");
        assert_eq!(rows[1].key(), "p:7");
    }
}
