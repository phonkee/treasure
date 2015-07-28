/*
Various string helpler methods
 */
use std::ascii::AsciiExt;

/// Converts CamelCase to snake_case
pub fn camel_to_snake(input:String) -> String {
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

#[test]
fn test_valid_camel_to_snake() {
	assert_eq!(camel_to_snake("HelloWorld"), "hello_world".to_string());
	assert_eq!(camel_to_snake("HWorld"), "h_world".to_string());
}
