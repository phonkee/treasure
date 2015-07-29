/*
Magical querying macros.

Please see readme.md there is more info.

@TODO: better error reporting, at least for blank arguments where is at least one required.
@TODO: better documentation
@TODO: attach to querybuilder (not yet implemented)
 */

/*
Macro for selecting data. currently supports two "types" of selecting
select!(many:<model>[...]) - selects multiple objects
select!(one:<model>[...]) - selects one object from database
 */
#[macro_export]
macro_rules! select {
	( many : $model:ident [ $($args:tt)* ]) => {
		{
			let _model_options = $model::model_options_static();
			// querybuilder where are you?
			let _qb = 1;
			query_parts!(qb, select, $($args)*);
			// add map method for model

			_qb
		}
	};
	( one : $model:ident [ $($args:tt)* ]) => {
		{
			let _model_options = $model::model_options_static();
			// querybuilder where are you?
			let _qb = 1;
			query_parts!(qb, select, $($args)*);

			// add map method for model
			_qb
		}
	};
}

// query_parts are e.g. filter, limit in future also group etc..
#[macro_export]
macro_rules! query_parts {
	($query_builder:ident, select, filter [ $($inner:tt)* ] $($rest:tt)*) => {
		{
			filter!($query_builder, select, $($inner)*);
			query_parts!($query_builder, select, $($rest)*);
		}
	};
}

/*
 expression is and[], or[], or single expression [expr => expr]
  */
#[macro_export]
macro_rules! expr {
	($query_builder:ident, and [ $($inner:tt)* ] $($rest_and:tt)* ) => {
		// @TODO: insert querybu
		expr!( $query_builder, $($inner)* );
		expr!( $query_builder, $($rest_and)* );
	};

	($query_builder:ident, or [ $($inner:tt)* ] $($rest_or:tt)* ) => {
		// we got and here
		// @TODO: insert querybuilder here
		expr!($query_builder, $($inner)*);
		expr!($query_builder, $($rest_or)*);
	};

	// here we need all possible expressions
	($query_builder:ident, [ $left:expr => $right:expr ] $($rest_expr:tt)* ) => {
		expr!($query_builder, $($rest_expr)*);
	};

	// blank one for last one
	($query_builder:ident, ) => {
	};
	// blank one for last one
	($query_builder:ident, [ ]) => {
	};

}

/*
Filter (where) query part (select, update)
filter!(qb, and[...] or[...] ["user" => "phonkee"]
 */
#[macro_export]
macro_rules! filter {
	($query_builder:ident, and [ $($inner:tt)* ] ) => {
		expr!($query_builder, and [ $($inner)* ]);
	};

	($query_builder:ident, or [ $($inner:tt)* ] ) => {
		expr!($query_builder, or [ $($inner)* ]);
	};

	($query_builder:ident, $($inner:tt)*) => {
		expr!($query_builder, and [ $($inner)* ]);
	};
}

/*
Limit clause (select, update(bulk), delete(bulk))

format is limit[limit, offset] or limit[limit] - offset will be set to 0.
 */
#[macro_export]
macro_rules! limit {
	// liit without offset
	($query_builder:ident, $limit:expr) => {
		limit!($query_builder, $limit, 0);
	};
	// limit with offset
	($query_builder:ident, $limit:expr, $offset:expr) => {
		// set limit to querybuilder
	};
}

/*
query_parts is dispatcher for parts of query
@TODO: right now it automatically calls macros, change it to call specific macros
 */
#[macro_export]
macro_rules! query_parts {
	($query_builder:ident, select, $($part:ident [ $($inner:tt)* ])* ) => {
		{
			// @TODO: change from dynamic to static
			$(
				$part!($query_builder, $($inner)*);
			)*
			// here goes special macro that sets map function
		}
	};
}
