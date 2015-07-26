/*
Attrs module handles annotation attributes in more user friendly values than MetaItems.
Attrs can generate source code of data from instance.
 */
//use syntax::attr::AttrMetaMethods;
use std::collections;
use std::fmt;
use syntax::ast;

// Attr is recursive structure how to represent annotation values in somehow more friendly than MetaItems.
// Attr has implements trait fmt::Display in that way that it can generate source code from instance.
#[derive(Debug)]
pub enum Attr {
	ListAttr(String, Vec<Attr>),
	NamedAttr(String, Box<Attr>),
	StringAttr(String),
}

impl fmt::Display for Attr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Attr::StringAttr(ref s) => write!(f, "attrs:Attr::Attr(\"{}\".to_string())", s),
			&Attr::NamedAttr(ref n, ref v) => {
				write!(f, "attrs:Attr::NamedAttr(\"{}\".to_string(), Box::new({}))", n, v)
			}
			&Attr::ListAttr(ref n, ref v) => {
				let mut items = vec![];
				for i in v.iter() {
					items.push(format!("{}", i))
				}
				write!(f, "attrs:Attr::ListAttr(\"{}\".to_string(), vec![{}])", n, items.join(", "))
			}
		}
    }
}

// Errors that can occur at reading from meta attributes and applying for callback closure.
pub enum AttrError {
	UnknownAttr(String),
	UnusedAttr,
}

// implement Display trait to nice error messages
impl fmt::Display for AttrError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			UnknownAttr => write!(f, "unknown attribute"),
			UnusedAttr => write!(f, "unused attribute"),
		}
    }
}

/*
Attributes struct to hold all attributes
 */
#[derive(Debug)]
pub struct Attrs(collections::HashMap<String, Attr>);

/*
Attrs is a set of all attributes for given column.
 */
impl Attrs {

	// Create new instance of Attrs
	pub fn new() -> Attrs {
		Attrs(collections::HashMap::new())
	}

	// insert new attribute
	pub fn insert(&mut self, name: &String, attr:Attr) -> &Self {
		self.0.insert((*name).clone(), attr);
		self
	}

	// read meta item and populate attributes
	// closure is callback function that can correct these attributes to valid values, or
	// refuse attribute at all.
	pub fn read_meta_item<F>(&mut self, _mi:&ast::MetaItem, _closure: F) -> Result<Attrs, AttrError>
		where F: Fn(String, Result<Attr, AttrError>) -> Result<Attr, AttrError>
	{

		Err(AttrError::UnknownAttr)
	}

}

/*
@TODO: Implement fmt::Display trait that will similarly to Attr generate source code representation of data it holds.
 */
impl fmt::Display for Attrs {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "attrs:Attrs")
    }
}

#[test]
fn attrs() {
}

