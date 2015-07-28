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
#[derive(Debug,Clone)]
pub enum Attr {
	ListAttr(String, Vec<Attr>),
	NamedAttr(String, Box<Attr>),
	StringAttr(String),
}

impl fmt::Display for Attr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Attr::StringAttr(ref s) => write!(f, "treasure::Attr::StringAttr(\"{}\".to_string())", s),
			&Attr::NamedAttr(ref n, ref v) => {
				write!(f, "treasure::Attr::NamedAttr(\"{}\".to_string(), Box::new({}))", n, v)
			}
			&Attr::ListAttr(ref n, ref v) => {
				let mut items = vec![];
				for i in v.iter() {
					items.push(format!("{}", i))
				}
				write!(f, "treasure::Attr::ListAttr(\"{}\".to_string(), vec![{}])", n, items.join(", "))
			}
		}
    }
}

impl Attr {

	// returns attr structure from ast::MetaItem_
	pub fn new_from_meta_item(mi:&ast::MetaItem_) -> Attr {
		match mi {
			&ast::MetaList(ref name, ref list) => {
				let mut reslist:Vec<Attr> = vec![];
				for item in list.iter() {
					reslist.push(Attr::new_from_meta_item(&item.node))
				}
				Attr::ListAttr(name.to_string(), reslist)
			},
			&ast::MetaWord(ref word) => {
				Attr::StringAttr(word.to_string())
			},
			&ast::MetaNameValue(ref name, ref lit) => {
				match lit.node {
					ast::LitStr(ref word, _) => Attr::NamedAttr(
						name.to_string(),
						Box::new(Attr::StringAttr(word.to_string()))
					),
					_ => panic!("MetaNameValue is not string")
				}


			}
		}
	}

	pub fn new_from_attribute(a:&ast::Attribute) -> Attr {
		Attr::new_from_meta_item(&a.node.value.node)
	}

}

// Errors that can occur at reading from meta attributes and applying for callback closure.
pub enum AttrError {
	UnknownAttr,
	UnusedAttr,
}

// implement Display trait to nice error messages
impl fmt::Display for AttrError {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&AttrError::UnknownAttr => write!(f, "unknown attribute"),
			&AttrError::UnusedAttr => write!(f, "unused attribute"),
		}
    }

}

/*
Attributes struct to hold all attributes
 */
#[derive(Debug,Clone)]
pub struct Attrs(collections::HashMap<String, Attr>);

/*
Attrs is a set of all attributes for given column.
 */
impl <'a> Attrs {

	// Create new instance of Attrs
	pub fn new() -> Attrs {
		Attrs(collections::HashMap::new())
	}

	// insert new attribute
	pub fn insert(&mut self, name: &String, attr:Attr) -> &Self {
		self.0.insert((*name).clone(), attr);
		self
	}
}

/*
@TODO: Implement fmt::Display trait that will similarly to Attr generate source code representation of data it holds.
 */
impl fmt::Display for Attrs {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let mut lines:Vec<String> = vec!["let mut i = treasure::Attrs::new()".to_string()];
		for (k, v) in self.0.iter() {
			lines.push(format!("i.insert(&\"{}\".to_string(), {})", k, v));
		}
		lines.push("i".to_string());
		write!(f, r#"{{ {} }}"#, lines.join(";"))
    }
}

#[test]
fn attrs() {
}

