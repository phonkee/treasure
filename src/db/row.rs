use std::collections;
use super::value::Value;


pub struct Row {
	values: collections::BTreeMap<String, Value>
}
