use std::collections;
use std::fmt;
use syntax::ast;

use super::column::Column;
use super::super::super::utils::attrs;

// errors
#[derive(Debug)]
pub enum OptionsError{
	ErrorPrivateField,
}

/*
Information about columns
 */
#[derive(Debug)]
pub struct Options {
	pub name: String,
	pub db_name: String,
	pub attrs: attrs::Attrs,
	pub ty: String,
}

#[derive(Debug)]
pub struct ColumnOptions(pub collections::HashMap<String, Options>);

impl ColumnOptions {
	pub fn new() -> ColumnOptions {
		ColumnOptions(collections::HashMap::new())
	}
}


/*
Options - information about column
 */
impl Options {

	pub fn new() -> Options {
		Options {
			name: "".to_string(),
			db_name: "".to_string(),
			ty: "".to_string(),
			attrs: attrs::Attrs::new(),
		}
	}

	// returns new Options from struct field
	pub fn new_from_struct_field(sf:&ast::StructField) -> Result<Options, OptionsError>{
		let mut ci = Options::new();

		match sf.node.kind {
			ast::StructFieldKind::NamedField(ref ident, vis) => {
				match vis {
					ast::Visibility::Public => {
						let iname = String::from(ident.name.as_str());
						ci.name = iname.clone();
						ci.db_name = iname.clone();

						// what if it's not path?
						if let ast::Ty_::TyPath(_, ref path) = sf.node.ty.node {
							ci.ty = format!("{}", path);
						}

					},
					ast::Visibility::Inherited => return Err(OptionsError::ErrorPrivateField)
				}
			},
			_ => return Err(OptionsError::ErrorPrivateField)
		}

		// set defaults for given field
		// for this I should create macro that expands type
//		Column::set_defaults<i32>(&mut ci);

		return Ok(ci)
	}

	pub fn get_impl(&self) -> String {
		let implitem = {
			format!(r#"treasure::models::columns::options::Options{{
					name: "{0}".to_string(),
					db_name: "{1}".to_string(),
					ty: "{2}".to_string(),
					// @TODO: add generation of attrs!!!
					attrs: treasure::utils::attrs::Attrs::new(),
				}}
		"#, self.name, self.db_name, self.ty)
        };
		implitem
	}

	// returns list of Options from structdef
	pub fn new_hm_from_struct_def(sd:&ast::StructDef) -> Result<ColumnOptions, OptionsError> {

		let mut fr = ColumnOptions::new();

		for field in sd.fields.iter() {
			let result = Self::new_from_struct_field(field);
			match result {
				Ok(v) => {
					fr.0.insert(v.name.clone(), v);
				},
				Err(e) => println!("this is error {:?}", e),
			}
		};

		Ok(fr)
	}
}

impl fmt::Display for Options {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, r#"treasure::models::columns::options::Options{{
						name: "{name}".to_string(),
						db_name: "{db_name}".to_string(),
						ty: "{ty}".to_string(),
						// @TODO: add generation of attrs!!!
						attrs: treasure::utils::attrs::Attrs::new(),
					}}
			"#,
			name=self.name,
			db_name=self.db_name,
			ty=self.ty)
	}

}