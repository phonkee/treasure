/*
Modal trait that all models should satisfy. For now all methods for Model trait are generated.
 */
use super::options::ModelOptions;
use super::columns::options;


// Model trait
pub trait Model {

	// returns model options such as table name, list of all columns with info.
	// concrete implementation will be generated by treasure.
	// this method could be possibli inlined??
	fn model_options(&self) -> ModelOptions;
	fn model_options_static() -> ModelOptions;

	// returns column options for given column
//	fn get_column_options(&self, column: &'static str) -> Option<&options::ColumnOptions>;
//	fn get_column_options_static(column: &'static str) -> Option<&options::ColumnOptions>;

	// init function to create new model instance.
	fn init_new() -> Self;
}