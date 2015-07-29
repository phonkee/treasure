#[feature(convert)]

use std::convert::AsRef;
use std::collections;
use std::fmt;
use syntax::ast;
use syntax::ext::base;
use super::super::utils::string;
//use super::super::utils::attrs::{Attrs,AttrError};
use super::columns::options;


/*
ModelOptionsError
	error
 */
#[derive(Debug)]
pub enum ModelOptionsError {
	NotStruct,
	MetaItemError,
	NoColumns,
	OptionsError,
}

#[derive(Debug)]
pub struct ModelOptions {
	pub name: &'static str,
	pub db_name: &'static str,
	pub primary_key: &'static str,
	pub columns: Vec<&'static str>,
	pub column_options: collections::BTreeMap<&'static str, options::ColumnOptions>,
}

// generates model options implementing ModelOptions trait
#[allow(unused_variables)]
pub fn generate_model_options_impls(ann:&base::Annotatable) -> Result<Vec<String>, ModelOptionsError> {
	let mut results:Vec<String> = vec![];
	let mut name = "".to_string();
	let mut db_name = "".to_string();
	let mut primary_key = "".to_string();
	let mut columns:Vec<String> = vec![];
	let mut column_options:Vec<String> = vec![];
	let mut column_inits:Vec<String> = vec![];

	if let &base::Annotatable::Item(ref item) = ann {
		name = item.ident.name.to_string();
		db_name = string::camel_to_snake(name.clone());
		primary_key = "".to_string();

		// add column names
		match get_column_options(ann) {
			Ok(items) => {
				if items.len() == 0 {
					return Err(ModelOptionsError::NoColumns);
				}
				for (ref key, ref value) in items {
					columns.push(format!(r#""{key}""#, key=key));
					column_options.push(format!(r#"column_options.insert("{key}", {value})"#, key=key, value=value));
					column_inits.push(format!(r#"{name}: treasure::models::columns::column::init_column(model_options.column_options.get("{name}").unwrap())"#,
						name=key));
				}

				let column_names = columns.iter().map(|x| format!(r#""{}""#, x)).collect::<Vec<_>>();
				results.push(format!(r#"
					impl treasure::Model for {name}
					{{
						fn model_options_static() -> treasure::ModelOptions {{
							use std::collections;
							let mut column_options:collections::BTreeMap<&'static str, treasure::ColumnOptions> = collections::BTreeMap::new();
							{column_options};
							treasure::ModelOptions{{
								name:"{name}",
								db_name: "{db_name}",
								primary_key: "{primary_key}",
								columns: vec![{columns}],
								column_options: column_options,
							}}
						}}
						fn model_options(&self) -> treasure::ModelOptions {{
							Self::model_options_static()
						}}

						// Constructor function for given model (query will use this)
						fn init_new() -> {name} {{
							let model_options = Self::model_options_static();
							{name} {{{column_inits}}}
						}}
					}}
					"#, name=name, db_name=db_name, primary_key=primary_key,
						columns=columns.join(", "),
						column_options=column_options.join(";"),
						column_inits=column_inits.join(",")
					).to_string()
				);
			},
			Err(e) => return Err(e),
		};
	};

	Ok(results)
}

fn get_column_options(ann:&base::Annotatable) -> Result<(Vec<(String, String)>), ModelOptionsError> {
	let mut result = vec![];
	let nserr = Err(ModelOptionsError::NotStruct);

	if let &base::Annotatable::Item(ref item) = ann {
		if let ast::Item_::ItemStruct(ref sd, _) = item.node {
			match options::get_columns(sd) {
				Ok(cols) => {
					for (k, v) in cols {
						result.push((k.clone(), format!(r#"{}"#, v)))
					}
				},
				Err(_) => return nserr,
			};
		} else {
			return nserr
		}
	}
	Ok(result)
}
