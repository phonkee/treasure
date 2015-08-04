use std::ops::Not;
use super::column::QueryColumn;
use ::models::columns::Column;

// Create new And expr
pub fn and(qf:Filter) -> Filter {
	Filter::And(vec![qf])
}

// Create new OR expr
pub fn or(qf:Filter) -> Filter {
	Filter::Or(vec![qf])
}

// create expression
pub fn q<T:Into<QueryColumn>>(column:T, op:Operator, value:String) -> Filter {
	Filter::Expr(column.into(), op, value)
}


#[derive(Debug,Clone)]
pub enum Operator {
	// equals to
	EQ,

	// greater than
	GT,

	// greater than or equal
	GTE,

	// in ..
	IN,

	// lower than
	LT,

	// lower than or equal
	LTE,

	// string like
	LIKE,

	// case insensitive like
	ILIKE,

	// is null
	ISNULL,
}

/// Filter implements WHERE clause parts in SQL query
#[derive(Debug,Clone)]
pub enum Filter {

	// No filter
	None,

	// AND clause
	And(Vec<Filter>),

	// OR clause
	Or(Vec<Filter>),

	// Expression "column <op> value"
	Expr(QueryColumn, Operator, String),

	// NOT negates everything in it
	Not(Box<Filter>),
}

impl Filter {

	// check if self is And/Or. If yes add value, if not wrap into And.
	pub fn and(&self, _f: Filter) -> Filter {
		match self {
			&Filter::And(ref vec) => {
				let mut newvec = vec.clone();
				newvec.push(_f);
				Filter::And(newvec)
			},
			&Filter::None => Filter::And(vec![]),
			other => Filter::And(vec![other.clone()])
		}
	}

	// check if self is And/Or. If yes add value, if not wrap into And.
	pub fn or(&self, _f: Filter) -> Filter {
		match self {
			&Filter::Or(ref vec) => {
				let mut newvec = vec.clone();
				newvec.push(_f);
				Filter::Or(newvec)
			},
			&Filter::None => Filter::Or(vec![]),
			other => Filter::Or(vec![other.clone()])
		}
	}

	// add next filter
	pub fn add(&self, _f: Filter) -> Filter {
		match self {
			&Filter::Or(_) => self.or(_f),
			other => self.and(other.clone())
		}
	}

}

// implement not so negation of filter can be writte as e.g: !and()
impl Not for Filter {
    type Output = Filter;

	// negation for NOT filter
    fn not(self) -> Filter {
		Filter::Not(Box::new(self))
    }
}

// Value - right value for Expr
// @TODO: implement Func which will have api Func("CONCAT").arg(c("this"))
// @TODO: implement c("some") + c("other") + 1  ????
pub enum Value {
	Column(Column),
}