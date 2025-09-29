use std::io::{self, Write};

const ANSI: &str = "\u{1b}["; // ansi escape code
const RESET: &str = "\u{1b}[0m";
const NORMAL: &str = "\u{1b}[36m"; // cyan
const SYSTEM: &str = "\u{1b}[33m"; // yellow
const PING: &str = "\u{1b}[35m"; // magenta/pink
const CHANNEL: &str = "\u{1b}[94;47m"; // bright blue
const ERROR: &str = "\u{1b}[31m"; // red

// TODO: move trait and impl code to utils.rs
trait ToU8Vec {
	fn to_byte_vec(&self) -> Vec<u8>;
}

impl ToU8Vec for Vec<String> {
	fn to_byte_vec(&self) -> Vec<u8> {
		let mut vu8: Vec<u8> = Vec::new();
		for elem in self {
			vu8.extend_from_slice(elem.as_bytes());
		}
		vu8
	}
}

// TODO: Move to src/tui/tui.rs
fn parse_input(input: &str) -> (String, bool) {
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

// TODO: Move to src/tui/tui.rs
fn clear_term_screen() {
	print!("{esc}[2J{esc}[H", esc = 27 as char);
}

fn main() {
	let mut output_buffer: Vec<String> = Vec::new();
	loop {
		// TODO: Move tui code to src/tui/tui.rs
		clear_term_screen();
		// create user prompt section
		println!("> ");
		// write out message history
		let buf = &output_buffer.to_byte_vec();
		io::stdout().write_all(buf).unwrap();
		// move back to user prompt section
		print!("{ANSI}H{ANSI}2C");
		// get user prompt
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let result = parse_input(&input);
		output_buffer.push(result.0);
		//TODO: quit cmd (doesn't print quitting message, need to do that!)
		if result.1 {
			clear_term_screen();
			print!("> {ANSI}B\r");
			let buf = &output_buffer.to_byte_vec();
			io::stdout().write_all(buf).unwrap();
			print!("{ANSI}H{ANSI}2C");
			io::stdout().flush().unwrap();
			break;
		}
	}
}
