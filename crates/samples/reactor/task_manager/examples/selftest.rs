//! Runnable integration self-test harness for the Task Manager data layer.
//!
//! Unlike the headless reactor tests (which use a `FakeMonitor`), this exercises
//! the real `WindowsMonitor` against live OS state: it samples at least twice,
//! confirms a non-empty process list, and checks that CPU% baselines are
//! established on the second sample.
//!
//! Usage:
//!   cargo run -p reactor_task_manager --example selftest
//!   cargo run -p reactor_task_manager --example selftest -- --headless
//!   cargo run -p reactor_task_manager --example selftest -- --filter windows_monitor
//!
//! `--headless` is accepted for CI parity (this harness is already headless).
//! Exits with a non-zero code if any fixture fails.

use std::process::ExitCode;
use std::time::Duration;

use reactor_task_manager::monitor::MonitorKind;

type Fixture = (&'static str, fn() -> Result<String, String>);

const FIXTURES: &[Fixture] = &[("windows_monitor", windows_monitor)];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filter = args
        .iter()
        .position(|a| a == "--filter")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let selected: Vec<&Fixture> = FIXTURES
        .iter()
        .filter(|(name, _)| filter.as_deref().is_none_or(|f| name.contains(f)))
        .collect();

    if selected.is_empty() {
        eprintln!("no fixtures matched filter {filter:?}");
        return ExitCode::FAILURE;
    }

    let mut failures = 0;
    for (name, run) in &selected {
        match run() {
            Ok(detail) => println!("PASS  {name}  ({detail})"),
            Err(err) => {
                println!("FAIL  {name}  ({err})");
                failures += 1;
            }
        }
    }

    println!(
        "\n{} passed, {} failed",
        selected.len() - failures,
        failures
    );
    if failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Samples the real monitor twice and validates the live results.
fn windows_monitor() -> Result<String, String> {
    let mut monitor = MonitorKind::Real.create();

    let first = monitor.sample();
    if first.processes.is_empty() {
        return Err("first sample returned no processes".into());
    }
    if first.logical_processors < 1 {
        return Err("logical_processors must be >= 1".into());
    }
    // No prior baseline: CPU% must be unknown on the first sample.
    if first.processes.iter().any(|p| p.cpu_percent.is_some()) {
        return Err("first sample should not report CPU% (no baseline yet)".into());
    }

    // Give the delta something to measure.
    std::thread::sleep(Duration::from_millis(400));

    let second = monitor.sample();
    if second.processes.is_empty() {
        return Err("second sample returned no processes".into());
    }
    let with_cpu = second
        .processes
        .iter()
        .filter(|p| p.cpu_percent.is_some())
        .count();
    if with_cpu == 0 {
        return Err("second sample established no CPU% baselines".into());
    }
    let with_mem = second
        .processes
        .iter()
        .filter(|p| p.memory_bytes.is_some())
        .count();
    if with_mem == 0 {
        return Err("no process reported memory (expected at least our own)".into());
    }

    Ok(format!(
        "{} processes, {} logical CPUs, {with_cpu} with CPU%, {with_mem} with memory",
        second.processes.len(),
        second.logical_processors,
    ))
}
