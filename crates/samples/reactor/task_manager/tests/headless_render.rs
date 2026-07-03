//! Headless reactor self-tests: mount the Processes page with a `FakeMonitor`
//! and assert the resulting render tree, without launching a WinUI window.
//!
//! These drive the real reactor render + reconcile pipeline; the timer degrades
//! to a no-op off the UI thread, but the effect's immediate sample still feeds
//! the first snapshot into the tree.

use std::rc::Rc;

use reactor_task_manager::monitor::MonitorKind;
use reactor_task_manager::pages::processes::{ProcessesProps, processes_page};
use test_reactor::{Op, RecordingBackend};
use windows_reactor::{ControlKind, Element, Reconciler, RenderCx};

const EXPECTED_APP: usize = 3;
const EXPECTED_BACKGROUND: usize = 3;
// Two group headers plus every process row.
const EXPECTED_ROWS: usize = 2 + EXPECTED_APP + EXPECTED_BACKGROUND;

/// Renders the page once (empty snapshot), flushes the effect so the fake
/// monitor produces a snapshot, then renders again to capture the populated
/// tree — mirroring how a real render/effect/rerender cycle unfolds.
fn render_populated_page() -> (Element, Element) {
    let mut cx = RenderCx::for_test();
    let props = ProcessesProps {
        monitor: MonitorKind::Fake,
    };

    cx.begin_render();
    let first = processes_page(&props, &mut cx);
    cx.flush_effects();

    cx.begin_render();
    let second = processes_page(&props, &mut cx);
    (first, second)
}

fn reconcile(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, el, None, Rc::new(|| {}));
    r.drain_realizations();
    r
}

fn templated_item_count(ops: &[Op]) -> Option<usize> {
    ops.iter().find_map(|op| match op {
        Op::SetTemplatedItemCount { count, .. } => Some(*count),
        _ => None,
    })
}

#[test]
fn mounts_a_list_view() {
    let (_, page) = render_populated_page();
    let r = reconcile(&page);
    assert!(
        r.backend.ops.iter().any(|op| matches!(
            op,
            Op::Create {
                kind: ControlKind::ListView,
                ..
            }
        )),
        "expected a ListView to be created; ops: {:?}",
        r.backend.ops
    );
}

#[test]
fn first_render_is_empty_then_snapshot_populates_rows() {
    let (first, second) = render_populated_page();

    // Before the effect runs, only the two group headers exist.
    let first_count = templated_item_count(&reconcile(&first).backend.ops);
    assert_eq!(
        first_count,
        Some(2),
        "empty snapshot should show only headers"
    );

    // After the fake monitor samples, every process row is present.
    let second_count = templated_item_count(&reconcile(&second).backend.ops);
    assert_eq!(
        second_count,
        Some(EXPECTED_ROWS),
        "populated snapshot should list all processes"
    );
}

#[test]
fn does_not_panic_without_a_dispatcher() {
    // Simply exercising the render + effect path off the UI thread must not
    // panic even though the refresh timer cannot be created here.
    let _ = render_populated_page();
}
