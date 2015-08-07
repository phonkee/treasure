use std::collections;
use super::value::Value;

pub struct Row {
	values: collections::BTreeMap<String, Value>
}

impl Row {
	pub fn new() -> Row {
		Row{
			values: collections::BTreeMap::new()
		}
	}
}

pub struct Rows {
	rows: Vec<Row>
}