/*

Attributes, parses attributes from annotations into nicer form. Also these attributes must have method that can generate
source code.

Attr
AttrValue






 */
use std::collections::HashMap;
use syntax::attr::AttrMetaMethods;
use syntax::ast::{Attribute,Item_,LitStr,MetaItem,MetaList,MetaWord,MetaNameValue,StructDef,StructField,StructFieldKind,Visibility};
use std::borrow::Cow;

#[derive(Debug)]
pub enum AttrError {
	AttrNotFound,
	InvalidAttr,
}

#[derive(Debug)]
pub enum Attr {
	StringAttr(String),
	StringVecAttr(Vec<String>),
	NoneAttr,
}

pub struct Attrs(HashMap<String, Attr>);

impl Attrs {

	pub fn new() -> Attrs {
		Attrs(HashMap::new())
	}

	// walks thru all attributes
	pub fn from_attribute_vec<F>(&mut self, attrs:&Vec<Attribute>, name:&str, closure: F) -> Result<Attr, AttrError>
		where F: Fn(String, Result<Attr, AttrError>) -> Result<Attr, AttrError>
	{
		for attr in attrs.iter() {

			if !attr.check_name(name) {
				continue
			}

			if let Some(v) = attr.meta_item_list() {
				for mii in v.iter() {
					match mii.node {
						MetaNameValue(ref n, ref v) => {
							let xn = (*n).clone().to_string();
							if let LitStr(ref vv, _) = v.node {
								let cr = closure(xn.clone(), Ok(Attr::StringAttr((*vv).clone().to_string())));
								match cr {
									Ok(z) => self.0.insert(xn, z),
									Err(err) => return Err(err),
								};
							}
						},
						MetaWord(ref n) => {
							let xn = (*n).clone().to_string();
							let cr = closure(xn.clone(), Ok(Attr::NoneAttr));
							match cr {
								Ok(z) => self.0.insert(xn, z),
								Err(err) => return Err(err),
							};
						},
//						MetaList(ref n, ref v) => {
//							let sva = Attr::StringVecAttr(Vec::new());
//							for i in v.iter() {
//								if let MetaWord(ref xv) = i.node {
//									sva.0.push(xv.clone().to_string())
//								} else {
//									return Err(AttrError::InvalidAttr)
//								}
//								let xn = (*n).clone().to_string();
//								let cr = closure(xn.clone(), Ok(sva));
//								match cr {
//									Ok(z) => self.0.insert(xn, z),
//									Err(err) => return Err(err),
//								};
//							}
//						},
						_ => (),
					};
				}
			}

	//			match attr.node.value.node {
//				MetaList(_, ref attrs) => {
//					for a in attrs.iter() {
//						let mn = a.name().to_string();
//						let mn = mn.as_str();
//
//						match closure(mn, Self::get_attr(a)) {
//							Err(e) => return Err(e),
//							_ => (),
//						}
//					}
//				},
//				_ => (),
//			}
		}

		Err(AttrError::AttrNotFound)
	}
}