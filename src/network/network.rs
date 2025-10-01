use crate::tui::tui::{RESET, SYSTEM};

const PORT: u32 = 6667;

pub fn network_info() -> String {
	format!("$ {SYSTEM}[Server] Port: {PORT}, IP: Local Host{RESET}\n")
}
