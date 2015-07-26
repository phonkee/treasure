use std::ascii::AsciiExt;
use std::borrow::Cow;

/// Converts CamelCase to snake_case
pub fn camel_to_snake(input:&str) -> String {
	let mut result = String::new();

	for (i, c) in input.chars().enumerate() {
		let ch = c.to_ascii_lowercase();

		if ch != c && i > 0 {
			result.push_str("_")
		}
		result.push(ch)
	}

	return result;
}
