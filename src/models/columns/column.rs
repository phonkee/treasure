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
	fn init(&options::Options) -> Self;

	// this function will have param ast::StructField and will populate Options that are specific for
	// given type
	fn set_attr(name:&str, attr:&attrs::Attr, ci:&mut options::Options) -> Option<attrs::AttrError>;

	// set defaults for coluninfo
	fn set_defaults(&mut options::Options);
}

// Column implementation for i32
impl Column for i32 {
	fn init(_ci:&options::Options) -> Self {
		0
	}

	fn set_attr(name:&str, attr:&attrs::Attr, ci:&mut options::Options) -> Option<attrs::AttrError> {
		None
	}

	fn set_defaults(_ci:&mut options::Options) {

	}
}

// Column implementation for String
impl Column for String {
	fn init(_ci:&options::Options) -> Self {
		"".to_string()
	}

	fn set_attr(name:&str, attr:&attrs::Attr, ci:&mut options::Options) -> Option<attrs::AttrError> {
		None
	}

	fn set_defaults(_ci:&mut options::Options) {
	}

}


// Column implementation for Option<T>
// In case ForeignKey is just type alias for Option<Model> wouldn't this clash?
impl <T> Column for Option<T> {
	fn init(_ci:&options::Options) -> Self {
		None
	}

	fn set_attr(name:&str, attr:&attrs::Attr, ci:&mut options::Options) -> Option<attrs::AttrError> {
		None
	}

	fn set_defaults(_ci:&mut options::Options) {
	}

}

// generic init method that calls concrete implementations
pub fn init_column<T:Column>(_ci:&options::Options) -> T {
	T::init(_ci)
}


/*
These two implementations doesnt work:
If Itry
		Column::set_defaults<i32>(&mut ci);


 */

// generic set_attr method
pub fn set_attr<T:Column>(name:&str, attr:&attrs::Attr, ci:&mut options::Options) -> Option<attrs::AttrError> {
	T::set_attr(name, attr, ci)
}

// generic set_defaults
pub fn set_defaults<T:Column>(_ci:&mut options::Options) {
	T::set_defaults(_ci)
}
