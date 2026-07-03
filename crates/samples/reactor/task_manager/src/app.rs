//! Top-level app shell: a Win11-style `NavigationView` with the tab list. Only
//! the Processes tab is implemented; the rest are placeholders on the roadmap.

use windows_reactor::*;

use crate::monitor::MonitorKind;
use crate::pages::processes::{ProcessesProps, processes_page};

const TAB_PROCESSES: &str = "processes";

struct Tab {
    tag: &'static str,
    label: &'static str,
    icon: Symbol,
    implemented: bool,
}

const TABS: &[Tab] = &[
    Tab {
        tag: TAB_PROCESSES,
        label: "Processes",
        icon: Symbol::AllApps,
        implemented: true,
    },
    Tab {
        tag: "performance",
        label: "Performance",
        icon: Symbol::Globe,
        implemented: false,
    },
    Tab {
        tag: "app-history",
        label: "App history",
        icon: Symbol::List,
        implemented: false,
    },
    Tab {
        tag: "startup",
        label: "Startup apps",
        icon: Symbol::Home,
        implemented: false,
    },
    Tab {
        tag: "users",
        label: "Users",
        icon: Symbol::World,
        implemented: false,
    },
    Tab {
        tag: "details",
        label: "Details",
        icon: Symbol::List,
        implemented: false,
    },
    Tab {
        tag: "services",
        label: "Services",
        icon: Symbol::Setting,
        implemented: false,
    },
];

/// The app shell. `monitor` selects the data source, letting the self-test
/// harness swap in a `FakeMonitor`; the real entry point uses [`app`].
pub fn shell(cx: &mut RenderCx, monitor: MonitorKind) -> Element {
    let (selected, set_selected) = cx.use_state(String::from(TAB_PROCESSES));

    let nav_items: Vec<NavViewItem> = TABS
        .iter()
        .map(|t| NavViewItem::new(t.label).tag(t.tag).icon(t.icon))
        .collect();

    let tab = TABS.iter().find(|t| t.tag == selected);
    let content: Element = match tab {
        Some(t) if t.implemented => component(processes_page, ProcessesProps { monitor }),
        _ => placeholder(&selected),
    };

    NavigationView::new(nav_items, content)
        .selected_tag(&selected)
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .settings_visible(false)
        .on_selection_changed(move |tag: String| {
            if !tag.is_empty() {
                set_selected.call(tag);
            }
        })
        .into()
}

/// The real app entry point (Windows-backed monitor).
pub fn app(cx: &mut RenderCx) -> Element {
    shell(cx, MonitorKind::Real)
}

fn placeholder(tag: &str) -> Element {
    let label = TABS.iter().find(|t| t.tag == tag).map_or(tag, |t| t.label);
    vstack((
        text_block(label).font_size(28.0).bold(),
        text_block("Not implemented yet — coming in a later milestone.").opacity(0.6),
    ))
    .spacing(8.0)
    .padding(Thickness::uniform(40.0))
    .into()
}
