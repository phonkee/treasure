use std::str::FromStr;

// QueryColumn
#[derive(Debug,Clone)]
pub struct QueryColumn {
	name: String,
	alias: String,
}

/// helper function to return new column
pub fn c<T:Into<String>>(name:T) -> QueryColumn {
	QueryColumn::new(name.into())
}

/// QueryColumn is implementation for column in sql query
impl QueryColumn {
	pub fn new(name:String) -> QueryColumn {
		QueryColumn{
			name:name,
			alias: "".to_string(),
		}
	}

	// set alias
	pub fn alias<T:Into<String>>(&mut self, alias:T) -> &Self {
		self.alias = alias.into();
		self
	}
}

/// conversion from &str to QueryColumn to use with T:<Into<QueryColumn>>
impl <'a> From<&'a str> for QueryColumn {
    fn from(s: &'a str) -> QueryColumn {
        QueryColumn::new(s.to_string())
    }
}

/// conversion from String to QueryColumn to use with T:<Into<QueryColumn>>
impl From<String> for QueryColumn {
    fn from(s: String) -> QueryColumn {
        QueryColumn::new(s.clone())
    }
}