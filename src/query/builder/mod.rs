/// Builder is low level SQL query builder implementation.
/// It doesn't know anything about models at all.
///
/// For querying models use future ModelBuilder.
///
/// builder module provides helper functions to return all query types:
/// ```rust
///	let qb = select();
/// ```
///
/// **WARNING!!**
/// This module is under development and its api may be subject to change.
pub mod filter;

pub use self::filter::{Filter,and,or,Operator,q};
pub use ::query::column::{QueryColumn,c};
pub use ::query::table::{QueryTable};

#[derive(Debug,PartialEq)]
pub enum QueryType {
	Delete,
	Insert,
	Select,
	Update,
}

/// shorthand to get select Builder
/// @TODO: make this call chainable. e.g.: select().from("table")
pub fn select() -> Builder {
	Builder::new(QueryType::Select)
}

/// @TODO: make this call chainable.
/// shorthand to get update Builder. e.g.: update().table("table")
pub fn update() -> Builder {
	Builder::new(QueryType::Update)
}

/// Main Builder implementation
/// Builder instance can hold one query type
#[derive(Debug)]
pub struct Builder {
	// type of query
	pub query_type:QueryType,

	// columns to select from
	pub query_columns: Vec<QueryColumn>,

	pub query_table:QueryTable,

	// filter for select, update, delete
	pub query_filter: Filter,
}

/// Builder is builder pattern to create new SQL query
/// all dialects will know hot to build concrete SQL query from builder
/// ```rust
/// let q = Builder::new()
/// ```
impl Builder {

	/// Constructor to return new Builder
	pub fn new(qt:QueryType) -> Builder {
		Builder{
			query_columns: vec![],
			query_filter:  Filter::None,
			query_table:   QueryTable::new("".to_string()),
			query_type:    qt,
		}
	}

	/// add column to query builder
	pub fn column<C: Into<QueryColumn>>(&mut self, column: C) -> &mut Self {
		debug_assert_eq!(self.query_type, QueryType::Select);
		self.query_columns.push(column.into());
		self
	}

	/// filter method sets filter to query. This makes sense only for QueryType: select, update
	pub fn filter(&mut self, qf:Filter) -> &mut Self {
		self.query_filter = qf;
		self
	}

	/// filter method sets filter to query. This makes sense only for QueryType: select, update
	pub fn filter_add(&mut self, qf:Filter) -> &mut Self {
		self.query_filter = self.query_filter.add(qf);
		self
	}

	/// Multiple aliases to set table.
	/// "table" sets table
	pub fn table<T: Into<QueryTable>>(&mut self, table:T) -> &mut Self {
		self.query_table = table.into();
		self
	}

	/// "from" sets table for select querytype
	pub fn from<T: Into<QueryTable>>(&mut self, table:T) -> &mut Self {
		debug_assert_eq!(self.query_type, QueryType::Select);
		self.query_table = table.into();
		self
	}

	/// "into" is alias for table for insert query type
	pub fn into<T: Into<QueryTable>>(&mut self, table:T) -> &mut Self {
		debug_assert_eq!(self.query_type, QueryType::Insert);
		self.query_table = table.into();
		self
	}

}
