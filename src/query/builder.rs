// THis is very unstable, mess..
// move builder to builder directory.
// create QueryBuilder and ModelQueryBuilder

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
pub struct QueryBuilder {
	query_type: QueryType,
	columns: Vec<String>,
	column_aliases: collections::BTreeMap<String, String>,
	namespace: String,
}

// implementation of Query
impl QueryBuilder {

	pub fn new(query_type:QueryType) -> QueryBuilder {
		QueryBuilder {
			query_type: query_type,
			columns: vec![],
			column_aliases: collections::BTreeMap::new(),
			namespace: "".to_string(),
		}
	}

	pub fn delete() -> QueryBuilder {
		Self::new(QueryType::DELETE)
	}

	pub fn select() -> QueryBuilder {
		Self::new(QueryType::SELECT)
	}

}