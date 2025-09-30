use crate::tui::tui;

pub const ANSI: &str = "\u{1b}["; // ansi escape code
const RESET: &str = "\u{1b}[0m";
const NORMAL: &str = "\u{1b}[36m"; // cyan
const SYSTEM: &str = "\u{1b}[33m"; // yellow
const PING: &str = "\u{1b}[35m"; // magenta/pink
const CHANNEL: &str = "\u{1b}[94;47m"; // bright blue
const ERROR: &str = "\u{1b}[31m"; // red

pub fn parse_input(input: &str) -> (String, bool) {
	if input.to_lowercase().trim() == "quit" || input.to_lowercase().trim() == "q" {
		return (format!("$ {SYSTEM}[Client] Closing Client...{RESET}"), true);
	}

	let parts: Vec<String> = input
		.split_whitespace()
		.map(|part| {
			let mut part_str = part.to_string();

			match part_str.chars().next().unwrap() {
				'@' => part_str.insert_str(0, PING),
				'#' => part_str.insert_str(0, CHANNEL),
				'!' => part_str.insert_str(0, ERROR),
				_ => part_str.insert_str(0, NORMAL),
			}

			part_str.push_str(RESET);
			part_str
		})
		.collect();

	let highlighted = parts.join(" ");

	(format!("$ {highlighted}\n"), false)
}

pub fn clear_term_screen() {
	print!("{esc}[2J{esc}[H", esc = 27 as char);
}
