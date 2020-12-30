extern crate regex;

/**
 * Cleans the given path.\
 * Parameters: `path`\
 * Returns: `String`
 */
pub fn clean_path(path: &str) -> String {
	use self::regex::Regex;
	let single_regex = Regex::new("/(?:\\./)+").unwrap();
	let double_regex = Regex::new("[^/]+/\\.\\./").unwrap();

	let mut result = single_regex.replace_all(path, "/").to_string();
	while double_regex.is_match(&result) {
		result = double_regex.replace_all(&result, "").to_string();
	}

	result
}
