// THis is very experimental..
use std::collections;
use super::super::models::options::ModelOptions;

#[derive(Debug)]
pub enum QueryType {
	DELETE,
	INSERT,
	SELECT,
	UPDATE,
}

// Query represents structurea about sql query
#[derive(Debug)]
pub struct Query {
	query_type: QueryType,
	columns: Vec<String>,
	column_aliases: collections::BTreeMap<String, String>,
	namespace: String,
}

// implementation of Query
impl Query {

	pub fn new(query_type:QueryType) -> Query {
		Query{
			query_type: query_type,
			columns: vec![],
			column_aliases: collections::BTreeMap::new(),
			namespace: "".to_string(),
		}
	}

	pub fn delete() -> Query {
		Self::new(QueryType::DELETE)
	}

	pub fn select() -> Query {
		Self::new(QueryType::SELECT)
	}

//	pub fn from_model_options(&mut self, mo:ModelOptions) {
//		for column in mo.columns {
//			self.add_column(column);
//		}
//	}
//
//	pub fn add_column(&mut self, column:String) {
//		self.columns.push(column);
//	}
//
//	pub fn add_column_alias(&mut self, alias:String, column:String) {
//		self.column_aliases.insert(alias, column);
//	}

}