mod exports;

use exports::*;
use std::env::{args, current_dir};

fn main() {
	let input = match args().nth(1) {
		Some(val) => val,
		None => return eprintln!("linez: missing operand\nExample usage: \"linez hello.txt\""),
	};

	let path = clean_path(
		current_dir()
			.unwrap()
			.join(input)
			.as_os_str()
			.to_str()
			.unwrap()
	);

	let content = match read_file(&path) {
		Ok(val) => val,
		Err(e) => return match e.raw_os_error() {
			Some(_) => eprintln!("linez: invalid path\nCould not resolve \"{}\"", path),
			None => eprintln!("linez: invalid file content\nInvalid UTF-8 found in \"{}\"", path),
		},
	};

	let mut lines = content.matches('\n').count();
	if !content.ends_with('\n') {
		lines += 1;
	}

	print_results(&path, lines);
}
