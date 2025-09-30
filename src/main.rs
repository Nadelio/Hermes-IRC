use crate::network::network::*;
use crate::tui::tui::*;
use std::io::{self, Write};
pub mod network;
pub mod tui;

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

fn tui_process(output_buffer: &mut Vec<String>) -> bool {
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
	if result.2 {
		output_buffer.clear();
	}
	result.1
}

fn main() {
	let mut output_buffer: Vec<String> = Vec::new();
	output_buffer.push(build_tui());
	output_buffer.push(network_info());
	loop {
		let should_quit = tui_process(&mut output_buffer);
		//TODO: quit cmd (doesn't print quitting message, need to do that!)
		if should_quit {
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
