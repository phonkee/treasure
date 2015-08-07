/*
Dialect trait
All sql dialects must implement this trait
 */

pub mod postgres;

pub trait Dialect {

	// transactions handling
	fn begin(&self);
	fn commit(&self);
	fn rollback(&self);

	// querying
	fn query(&self);
	fn execute(&self);

	// query_builder performs builder query.
	fn query_builder(&self);
}
