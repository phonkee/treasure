#![allow(unused_variables)]
/*
Column implementations must be written for all POD data types and Option (nullable column) and also
ForeignKey, ManyToMany, OneToOne fields.
 */

use super::options;
use super::super::super::utils::attrs;

// trait Column is trait that all struct fields that will be persisted to db must implement.
// so all POD must support that
pub trait Column {

	// init function which will have parameter Options so it can correctly create new value
	// with default value that can be provided in annotation
	fn init(&options::ColumnOptions) -> Self;

	fn from_sql();
	fn to_sql();
}

// Column implementation for i32
impl Column for i32 {
	fn init(_ci:&options::ColumnOptions) -> Self {
		0
	}
	fn from_sql() {

	}
	fn to_sql() {

	}
}

// Column implementation for String
impl Column for String {
	fn init(_ci:&options::ColumnOptions) -> Self {
		"".to_string()
	}
	fn from_sql() {

	}
	fn to_sql() {

	}
}


// Column implementation for Option<T>
// In case ForeignKey is just type alias for Option<Model> wouldn't this clash?
impl <T> Column for Option<T> {
	fn init(_ci:&options::ColumnOptions) -> Self {
		None
	}
	fn from_sql() {

	}
	fn to_sql() {

	}
}

// generic init method that calls concrete implementations
pub fn init_column<T:Column>(_ci:&options::ColumnOptions) -> T {
	T::init(_ci)
}

