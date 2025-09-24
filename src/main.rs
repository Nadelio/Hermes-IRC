use std::io::{self, Write};

const RESET: &str = "\u{1b}[0m";
const NORMAL: &str = "\u{1b}[36m";
const QUIT: &str = "\u{1b}[33m";
const PING: &str = "\u{1b}[35m";

fn parse_input(input: &str) -> bool {
	if input.to_lowercase().trim() == "quit" || input.to_lowercase().trim() == "q" {
		print!("$ {QUIT}[Client] Closing Client...{RESET}");
		return true;
	}

	let at_sign = input.find('@');
	if let Some(at) = at_sign {
		let whitespace = input[at..]
			.find(|c: char| c.is_ascii_whitespace())
			.unwrap_or(input.len() - at)
			+ at;
		let split1 = &input[..at];
		let ping = &input[at..whitespace];
		let split2 = &input[whitespace..];

		print!("$ {NORMAL}{split1}{PING}{ping}{NORMAL}{split2}{RESET}");
	} else {
		print!("$ {NORMAL}{input}{RESET}");
	}

	false
}

fn main() {
	loop {
		print!("> ");
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		if parse_input(&input) {
			break;
		}
	}
}
