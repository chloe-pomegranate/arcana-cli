//! Terminal User Interface using ratatui

pub mod app;
pub mod screens;
pub mod widgets;

#[allow(unused_imports)]
pub use app::{App, run_tui};
