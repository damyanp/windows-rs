//! A Task Manager replacement built on `windows-reactor`.
//!
//! The library exposes the data layer (`monitor`), the pure presentation logic
//! (`view`), and the reactor UI (`app`, `pages`) so that both the binary and the
//! integration self-test harness can drive them.

pub mod app;
pub mod monitor;
pub mod pages;
pub mod view;
