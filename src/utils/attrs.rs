/*

 */
//use syntax::attr::AttrMetaMethods;
use std::collections;
use std::fmt;
use syntax::ast;

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

impl fmt::Display for Attrs {
	// @TODO: implement this
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "attrs:Attrs")
    }
}



#[test]
fn attrs() {
}
















//#[derive(Debug)]
//pub enum AttrError {
//	AttrNotFound,
//	InvalidAttr,
//}
//
//#[derive(Debug)]
//pub enum Attr {
//	StringAttr(String),
//	StringVecAttr(Vec<String>),
//	NoneAttr,
//}
//
//pub struct Attrs(HashMap<String, Attr>);
//
//impl Attrs {
//
//	pub fn new() -> Attrs {
//		Attrs(HashMap::new())
//	}
//
//	// walks thru all attributes
//	pub fn from_attribute_vec<F>(&mut self, attrs:&Vec<Attribute>, name:&str, closure: F) -> Result<Attr, AttrError>
//		where F: Fn(String, Result<Attr, AttrError>) -> Result<Attr, AttrError>
//	{
//		for attr in attrs.iter() {
//
//			if !attr.check_name(name) {
//				continue
//			}
//
//			if let Some(v) = attr.meta_item_list() {
//				for mii in v.iter() {
//					match mii.node {
//						MetaNameValue(ref n, ref v) => {
//							let xn = (*n).clone().to_string();
//							if let LitStr(ref vv, _) = v.node {
//								let cr = closure(xn.clone(), Ok(Attr::StringAttr((*vv).clone().to_string())));
//								match cr {
//									Ok(z) => self.0.insert(xn, z),
//									Err(err) => return Err(err),
//								};
//							}
//						},
//						MetaWord(ref n) => {
//							let xn = (*n).clone().to_string();
//							let cr = closure(xn.clone(), Ok(Attr::NoneAttr));
//							match cr {
//								Ok(z) => self.0.insert(xn, z),
//								Err(err) => return Err(err),
//							};
//						},
////						MetaList(ref n, ref v) => {
////							let sva = Attr::StringVecAttr(Vec::new());
////							for i in v.iter() {
////								if let MetaWord(ref xv) = i.node {
////									sva.0.push(xv.clone().to_string())
////								} else {
////									return Err(AttrError::InvalidAttr)
////								}
////								let xn = (*n).clone().to_string();
////								let cr = closure(xn.clone(), Ok(sva));
////								match cr {
////									Ok(z) => self.0.insert(xn, z),
////									Err(err) => return Err(err),
////								};
////							}
////						},
//						_ => (),
//					};
//				}
//			}
//
//	//			match attr.node.value.node {
////				MetaList(_, ref attrs) => {
////					for a in attrs.iter() {
////						let mn = a.name().to_string();
////						let mn = mn.as_str();
////
////						match closure(mn, Self::get_attr(a)) {
////							Err(e) => return Err(e),
////							_ => (),
////						}
////					}
////				},
////				_ => (),
////			}
//		}
//
//		Err(AttrError::AttrNotFound)
//	}
//}