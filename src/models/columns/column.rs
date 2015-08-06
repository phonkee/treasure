#![allow(unused_variables)]
/*
Column implementations must be written for all POD data types and Option (nullable column) and also
ForeignKey, ManyToMany, OneToOne fields.
 */

use super::options;
use super::super::super::utils::attrs;

use ::db::value::Value;

// trait Column is trait that all struct fields that will be persisted to db must implement.
// so all POD must support that
pub trait Column {

	// init function which will have parameter Options so it can correctly create new value
	// with default value that can be provided in annotation
	fn init_column(&options::ColumnOptions) -> Self;
	fn default_attrs() -> attrs::Attrs;
	fn from_value(&self, value:&Value);
	fn to_value(&self) -> Value;
}

// Column implementation for i32
impl Column for i32 {
	fn init_column(_ci:&options::ColumnOptions) -> Self {
		0
	}
	fn from_value(&self, value:&Value) {
	}
	fn to_value(&self) -> Value {
		Value::I32(0)
	}
	fn default_attrs() -> attrs::Attrs {
		attrs::Attrs::new()
	}
}

// Column implementation for String
impl Column for String {
	fn init_column(_ci:&options::ColumnOptions) -> Self {
		"".to_string()
	}
	fn from_value(&self, value:&Value) {
	}
	fn to_value(&self) -> Value {
		Value::String(self.clone())
	}
	fn default_attrs() -> attrs::Attrs {
		attrs::Attrs::new()
	}
}

// Column implementation for Option<T>
// In case ForeignKey is just type alias for Option<Model> wouldn't this clash?
impl <T> Column for Option<T> {
	fn init_column(_ci:&options::ColumnOptions) -> Self {
		None
	}
	fn from_value(&self, value:&Value) {

	}
	fn to_value(&self) -> Value {
		Value::Null
	}
	fn default_attrs() -> attrs::Attrs {
		attrs::Attrs::new()
	}
}

// generic init method that calls concrete implementations
pub fn init_column<T:Column>(_ci:&options::ColumnOptions) -> T {
	T::init_column(_ci)
}

