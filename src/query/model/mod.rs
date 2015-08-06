///
/// Model builder that reads model properties and generates low level query builder.
///

use ::models::options::ModelOptions;
use ::models::model::Model;
use ::query::column::{QueryColumn,ColumnValue};
use ::query::builder::filter::Operator;
use ::query::builder;
use std::collections;


pub fn select<T:Model>() -> ModelBuilder {
	ModelBuilder::new::<T>()
}

#[derive(Debug)]
pub struct ModelBuilder {
	options: ModelOptions,
}

pub enum Filter {
	And(Vec<Filter>),
	Or(Vec<Filter>),
	Q(QueryColumn)
}

/// ModelBuilder
/// reads information about given model, and provides api such as in django to filter
impl ModelBuilder {

	// returns new ModelBuilder instance for given model
	pub fn new<T:Model>() -> ModelBuilder {
		ModelBuilder{
			options: T::model_options_static()
		}
	}

	pub fn as_builder(&self) -> builder::Builder {
		let mut builder = builder::select();
		for column in self.options.columns.iter() {
			builder.column(*column);
		}
		builder
	}

	pub fn filter(self) -> ModelBuilder {

		self
	}

}