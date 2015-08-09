Dialect
=======

Dialect is trait for database connector. Currently only postgres is developed, other will be implemeneted in the future.
Dialect is basically wrapper around connection to sql database. It provides methods for querying, formatting sql query,
but main method that treasure ORM will use is query_from_builder, which takes Builder instance and formats query and
executes. Resultset is analyzed and returned as Rows instance(?).

Value
=====

Value is enum value that can and knows how to comuniate with database. Model Column trait defines two methods, from_value
and to_value which can "read" value from database result.

Row
===

Custom database row.



@TODO: finalize Dialect trait.

Current trait definition is as following:

```rust

pub trait Dialect {
	// transactions handling
	fn begin(&self);
	fn commit(&self);
	fn rollback(&self);

    // query that returns resultset
	fn query(&self);
	
	// queries that don't return rows
	fn execute(&self);

	// query_builder performs builder query.
	fn query_from_builder(&self);

    // get_query formats sqsl query and returns also list of sql params
    get_query(&self, &builder)
}
```