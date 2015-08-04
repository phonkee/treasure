// QueryTable
#[derive(Debug)]
pub struct QueryTable {
	name: String,
	alias: String,
}

// QueryTable is implementation for column in sql query
impl QueryTable {
	pub fn new(name:String) -> QueryTable {
		QueryTable{
			name:name,
			alias: "".to_string(),
		}
	}

	/// alias sets table alias
	pub fn alias<T:Into<String>>(&mut self, alias:T) -> &Self {
		self.alias = alias.into();
		self
	}
}

// conversion from &str to QueryTable
impl <'a> From<&'a str> for QueryTable {
    fn from(s: &'a str) -> QueryTable {
        QueryTable::new(s.to_string())
    }
}

// conversion from String to QueryTable
impl From<String> for QueryTable {
    fn from(s: String) -> QueryTable {
        QueryTable::new(s.clone())
    }
}