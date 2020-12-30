use std::io::Result;

/**
 * Reads the content from a specified file.\
 * Parameters: `path`\
 * Returns: `Result<String>`
 */
pub fn read_file(path: &str) -> Result<String> {
	use std::fs::File;
	use std::io::prelude::Read;

	let mut file = File::open(path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	Ok(content)
}

/**
 * Specific function to print results.\
 * Parameters: `path`, `lines`
 */
pub fn print_results(path: &str, lines: usize) {
	use std::cmp::max;

	let path_str = format!("Path: {}", path);
	let line_str = format!("Lines: {}", lines);
	let times = max(path_str.chars().count(), line_str.chars().count());

	println!("\n{}", "=".repeat(times));

	println!("{}", path_str);
	println!("{}", line_str);

	println!("{}\n", "=".repeat(times));
}
