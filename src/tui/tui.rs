use terminal_size::{terminal_size, Height, Width};

pub const ANSI: &str = "\u{1b}["; // ansi escape code
pub const RESET: &str = "\u{1b}[0m";
const NORMAL: &str = "\u{1b}[36m"; // cyan
pub const SYSTEM: &str = "\u{1b}[33m"; // yellow
const PING: &str = "\u{1b}[35m"; // magenta/pink
const CHANNEL: &str = "\u{1b}[30;47m"; // bright blue
const ERROR: &str = "\u{1b}[31m"; // red

fn parse_cmd(cmd: &str) -> bool {
	if cmd[1..] == *"CLEAR" {
		return true;
	}
	false
}

pub fn parse_input(input: &str) -> (String, bool, bool) {
	if input.to_lowercase().trim() == "quit" || input.to_lowercase().trim() == "q" {
		return (
			format!("$ {SYSTEM}[Client] Closing Client...{RESET}"),
			true,
			false,
		);
	}

	let mut should_clear_buf: bool = false;

	let parts: Vec<String> = input
		.split_whitespace()
		.map(|part| {
			let mut part_str = part.to_string();

			match part_str.chars().next().unwrap() {
				'@' => part_str.insert_str(0, PING),
				'#' => part_str.insert_str(0, CHANNEL),
				'!' => part_str.insert_str(0, ERROR),
				'/' => should_clear_buf = parse_cmd(&part_str.clone()),
				_ => part_str.insert_str(0, NORMAL),
			}

			part_str.push_str(RESET);
			part_str
		})
		.collect();

	let highlighted = parts.join(" ");

	(format!("$ {highlighted}\n"), false, should_clear_buf)
}

pub fn clear_term_screen() {
	print!("{esc}[2J{esc}[H", esc = 27 as char);
}

pub fn build_tui() -> String {
	// Use this function to build the TUI frame
	// TODO: [ ] TUI::{ MessageBox, ChannelBox, UserInput, ModeBox, MemberBox, CurrentServerBox, ChatBox }

	// testing
	let size = terminal_size();
	if let Some((Width(w), Height(h))) = size {
		format!("$ {SYSTEM}[Client] Your terminal is {w} cols wide and {h} lines tall{RESET}\n")
	} else {
		format!("$ {ERROR}[FATAL] Unable to get terminal size{RESET}\n")
	}
}
