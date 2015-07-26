#![allow(unused_variables)]
/*
Column implementations must be written for all POD data types and Option (nullable column) and also
ForeignKey, ManyToMany, OneToOne fields.
 */

use super::options::{ColumnInfo,ColumnOption,ColumnOptionError};

// trait Column is trait that all struct fields that will be persisted to db must implement.
// so all POD must support that
pub trait Column {

	// init function which will have parameter ColumnOptions so it can correctly create new value
	// with default value that can be provided in annotation
	fn init(&ColumnInfo) -> Self;

	// this function will have param ast::StructField and will populate ColumnOptions that are specific for
	// given type
	fn set_option(name:&str, co:&ColumnOption, ci:&mut ColumnInfo) -> Option<ColumnOptionError>;

	// set defaults for coluninfo
	fn set_defaults(&mut ColumnInfo);
}

// Column implementation for i32
impl Column for i32 {
	fn init(_ci:&ColumnInfo) -> Self {
		0
	}

	fn set_option(name:&str, co:&ColumnOption, ci:&mut ColumnInfo) -> Option<ColumnOptionError> {
		None
	}

	fn set_defaults(_ci:&mut ColumnInfo) {
	}
}

// Column implementation for String
impl Column for String {
	fn init(_ci:&ColumnInfo) -> Self {
		"".to_string()
	}

	fn set_option(name:&str, co:&ColumnOption, ci:&mut ColumnInfo) -> Option<ColumnOptionError> {
		None
	}

	fn set_defaults(_ci:&mut ColumnInfo) {
	}

}


// Column implementation for Option<T>
// In case ForeignKey is just type alias for Option<Model> wouldn't this clash?
impl <T> Column for Option<T> {
	fn init(_ci:&ColumnInfo) -> Self {
		None
	}

	fn set_option(name:&str, co:&ColumnOption, ci:&mut ColumnInfo) -> Option<ColumnOptionError> {
		None
	}

	fn set_defaults(_ci:&mut ColumnInfo) {
	}

}

// generic init method that calls concrete implementations
pub fn init_column<T:Column>(_ci:&ColumnInfo) -> T {
	T::init(_ci)
}


/*
These two implementations doesnt work:
If Itry
		Column::set_defaults<i32>(&mut ci);


 */

// generic set_option method
pub fn set_option<T:Column>(name:&str, co:&ColumnOption, ci:&mut ColumnInfo) -> Option<ColumnOptionError> {
	T::set_option(name, co, ci)
}

// generic set_defaults
pub fn set_defaults<T:Column>(_ci:&mut ColumnInfo) {
	T::set_defaults(_ci)
}
