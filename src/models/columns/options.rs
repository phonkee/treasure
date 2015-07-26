use std::collections::HashMap;
use syntax::ast::{Item_,Lit,MetaItem,MetaList,MetaWord,StructDef,StructField,StructFieldKind,Visibility,Ty_};

use super::column::Column;

// errors
#[derive(Debug)]
pub enum ColumnInfoError{
	ErrorPrivateField,
}

// errors
#[derive(Debug)]
pub enum ColumnOptionError{
	ErrorUnknown,
}

type ColumnOptionString = String;
type ColumnOptionStringVec = Vec<String>;


/*
Not exactly correct, need to move attr to utils::attrs
 */
#[derive(Debug)]
pub enum ColumnOption {
	ColumnOptionString,
	ColumnOptionStringVec,
}

/*
Information about columns
 */
#[derive(Debug)]
pub struct ColumnInfo {
	pub name: String,
	pub db_name: String,
	pub options: Vec<ColumnOption>,
	pub ty: String,
}

/*
ColumnInfo - information about column
 */
impl ColumnInfo {

	pub fn new() -> ColumnInfo {
		ColumnInfo {
			name: "".to_string(),
			db_name: "".to_string(),
			ty: "".to_string(),
			options: vec![],
		}
	}

	// returns new ColumnInfo from struct field
	pub fn new_from_struct_field(sf:&StructField) -> Result<ColumnInfo, ColumnInfoError>{
		let mut ci = ColumnInfo::new();

		match sf.node.kind {
			StructFieldKind::NamedField(ref ident, vis) => {
				match vis {
					Visibility::Public => {
						let iname = String::from(ident.name.as_str());
						ci.name = iname.clone();
						ci.db_name = iname.clone();

						// what if it's not path?
						if let Ty_::TyPath(_, ref path) = sf.node.ty.node {
							ci.ty = format!("{}", path);
						}

					},
					Visibility::Inherited => return Err(ColumnInfoError::ErrorPrivateField)
				}
			},
			_ => return Err(ColumnInfoError::ErrorPrivateField)
		}

		// set defaults for given field
		// for this I should create macro that expands type
//		Column::set_defaults<i32>(&mut ci);

		return Ok(ci)
	}

	pub fn get_impl(&self) -> String {
		let implitem = {
			format!(r#"treasure::models::columns::options::ColumnInfo{{
					name: "{0}".to_string(),
					db_name: "{1}".to_string(),
					// We need to get type, maybe from CodeMap.span_to_string???
					ty: "{2}".to_string(),
					// @TODO: add generation of options!!!
					options: vec![],
				}}
		"#, self.name, self.db_name, self.ty)
        };
		implitem
	}

	// returns list of ColumnInfo from structdef
	pub fn new_vec_from_struct_def(sd:&StructDef) -> Result<Vec<ColumnInfo>, ColumnInfoError> {
		let mut cis = vec![];

		for field in sd.fields.iter() {
			let result = Self::new_from_struct_field(field);
			match result {
				Ok(v) => cis.push(v),
				Err(e) => println!("this is error {:?}", e),
			}
		};

		Ok(cis)
	}


}