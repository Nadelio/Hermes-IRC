#[allow(clippy::module_inception)]
pub mod tui;
pub use crate::tui::tui::{clear_term_screen, parse_input, ANSI};
