#[allow(unused_assignments)]
use std::collections;
use std::fmt;
use syntax::ast;

use super::column::Column;
use super::super::super::utils::attrs;

// errors
#[derive(Debug)]
pub enum OptionsError{
	ErrorPrivateField,
	UnspecifiedError,
}

// returns list of Options from structdef
pub fn get_columns(sd:&ast::StructDef) -> Result<collections::BTreeMap<String, String>, OptionsError> {

	let ue = OptionsError::UnspecifiedError;
	let mut result:collections::BTreeMap<String, String> = collections::BTreeMap::new();

	for field in sd.fields.iter() {
		match generate_column_options(field) {
			Ok((name, value)) => {
				result.insert(name, value);
			},
			_ => return Err(ue),
		};
	};

	Ok(result)
}


pub fn generate_column_options(sf:&ast::StructField) -> Result<(String, String), OptionsError>{

	let mut name;
	let mut db_name;
	let mut ty = "".to_string();
	let mut attrs = attrs::Attrs::new();

	match sf.node.kind {
		ast::StructFieldKind::NamedField(ref ident, vis) => {
			match vis {
				ast::Visibility::Public => {
					let iname = String::from(ident.name.as_str());
					name = iname.clone();
					db_name = iname.clone();

					// what if it's not path?
					if let ast::Ty_::TyPath(_, ref path) = sf.node.ty.node {
						ty = format!("{}", path);
					}
				},
				ast::Visibility::Inherited => return Err(OptionsError::ErrorPrivateField)
			}
		},
		_ => return Err(OptionsError::ErrorPrivateField)
	}

	for attr in sf.node.attrs.iter() {
		match attrs::Attr::new_from_meta_item(&attr.node.value.node) {
			attrs::Attr::ListAttr(ref name, ref list) => {
				if name.to_string() != "field".to_string() {
					continue
				}
				// @TODO: call here Column method to validate all attrs
				for value in list.iter() {
					match *value {
						attrs::Attr::ListAttr(ref n, _) | attrs::Attr::NamedAttr(ref n, _) | attrs::Attr::StringAttr(ref n) => {
							attrs.insert(n, value.clone())
						},
					};
				}
			},
			_ => ()
		}
	}

	Ok((name.clone(), format!(r#"treasure::ColumnOptions::new("{name}", "{db_name}", "{ty}", {attrs})"#, name=name, db_name=db_name, ty=ty, attrs=attrs)))
}


// ColumnOptions
// just trait nothing more, implementation will be generated
#[derive(Clone)]
pub struct ColumnOptions {
	pub name:&'static str,
	pub db_name: &'static str,
	pub ty: &'static str,
	pub attrs: attrs::Attrs,
}

impl ColumnOptions {
	pub fn new(name:&'static str, db_name:&'static str, ty:&'static str, attrs:attrs::Attrs) -> ColumnOptions {
		ColumnOptions {
			name: name,
			db_name: db_name,
			ty: ty,
			attrs: attrs,
		}
	}
}