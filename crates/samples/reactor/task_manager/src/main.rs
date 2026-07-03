#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    let _ = App::new()
        .title("Task Manager (Rust reactor)")
        .inner_size(900.0, 640.0)
        .backdrop(Backdrop::Mica)
        .eager_templated_realization(true)
        .render(reactor_task_manager::app::app);
}
