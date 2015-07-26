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

/*
ModelOptions
	inspected options about model
 */
#[derive(Debug)]
pub struct ModelOptions <'a>{
	pub name: &'a str,
	pub db_name: String,
	pub primary_key: &'a str,
	pub managed: bool,
	pub columns: Vec<String>,
}

fn print_hm(name:String, m:&collections::HashMap<String, options::Options>) -> String {
	let mut lines = vec![];
	for (k, v) in m {
		let s = format!("{}.insert(\"{}\".to_string(), {});", name, k, v);
		lines.push(s);
	}
	String::from(format!(r#"{}"#, lines.join("\n")))
}



impl <'a> ModelOptions <'a> {

	/*
	Returns model options for given annotatable
	 */
	fn get_column_options(ann:&base::Annotatable) -> Result<options::ColumnOptions, ModelOptionsError> {
		match ann {
			&base::Annotatable::Item(ref item) => {
				if let ast::Item_::ItemStruct(ref sd, _) = item.node {

					let result = options::Options::new_hm_from_struct_def(sd);
					match result {
						Err(_) => {
							return Err(ModelOptionsError::OptionsError)
						}
						Ok(f) => {
							if f.0.len() == 0 {
								return Err(ModelOptionsError::NoColumns)
							}
							return Ok(f)
						}
					};
				}
			},
			_ => return Err(ModelOptionsError::NotStruct)
		}


		/*
		let mut attrs = Attrs::new();
		let _result = attrs.from_attribute_vec(&item.attrs, "model", |name, r| {
			// @TODO: get annotations to model_opts (implement
			match name.as_str() {
				"db_name" => (),
				"primary_key" => (),
				_ => (),
			};
			r
		});
=				*/


		Err(ModelOptionsError::NotStruct)
	}


	// Returns new ModelOptions from Annotatable object
	pub fn from_annotatable(ann:&base::Annotatable) -> Result<ModelOptions, ModelOptionsError> {
		let mut model_opts = ModelOptions{
			name: "",
			db_name: "".to_string(),
			primary_key: "",
			managed: true,
			columns: vec![],
		};
		match ann {
			&base::Annotatable::Item(ref item) => {
				model_opts.name = item.ident.name.as_str();
				model_opts.db_name = string::camel_to_snake(model_opts.name);

				// add column names
				if let Ok(co) = Self::get_column_options(ann) {
					for (i, _) in &co.0 {
						model_opts.columns.push((*i).clone());
					}
				}
			},
			_ => return Err(ModelOptionsError::NotStruct)
		}


		Ok(model_opts)
	}

	// Returns iplementation source
	pub fn get_impl(&self, ann:&base::Annotatable) -> String {

		let mut cinfos:Vec<String> = vec![];

		match Self::get_column_options(ann) {
			Ok(ref f) => {
				for (_, v) in &f.0 {
					cinfos.push(format!("{}", v))
				}
				let implitem = format!(r#"
					impl treasure::models::model::Model for {name} {{
						{model_options}
						{column_options}
						{init_new}
					}}"#,
					name=self.name,
					model_options=self.generate_model_options(f),
					column_options=self.generate_column_options(f),
					init_new=self.generate_init_new(f)
				);

				println!("This is generated implementation of model:\n{}", implitem);

				return implitem
			},
			Err(_) => return "".to_string(),
		}
	}

	/*
	generate_model_options generates model_options method
	 */
	pub fn generate_model_options(&self, _co:&options::ColumnOptions) -> String {

		let cols:Vec<_> = self.columns.iter().map(|x| format!("\"{}\".to_string()", x)).collect();

		format!(r#"
			// option implementation for model {name}
			// should we inline options? or at least provide model annotation for this?
			fn model_options(&self) -> treasure::models::options::ModelOptions {{
				treasure::models::options::ModelOptions{{
					name: "{name}",
					db_name: "{db_name}".to_string(),
					primary_key: "{primary_key}",
					managed: false,
					columns: vec![{columns}],
				}}
			}}
		"#,
		name=self.name,
		db_name=self.db_name,
		primary_key=self.primary_key,
		columns=cols.join(", ")
		)
	}

	pub fn generate_column_options(&self, co:&options::ColumnOptions) -> String {

		let mut lines = vec![];

		for (k, v) in &co.0 {
			lines.push(format!("		\"{}\" => {{ Some({}) }}", k, v.get_impl()))
		}

		lines.push("_ => None".to_string());

		format!(r#"
			// Returns Options for given column
			// Will be needed for querying
			// TODO: implement this method to return info about option
			fn column_options(_c:String) -> Option<treasure::models::columns::options::Options> {{
				match _c.as_ref() {{
					{matches}
				}}
			}}
		"#, matches=lines.join(",\n")
		)

	}

	// Generate init_new function for given model
	pub fn generate_init_new(&self, co:&options::ColumnOptions) -> String {

		// generate column initials
		let mut column_inits = vec![];

		for (_, v) in &co.0 {
			column_inits.push(format!(r#"
					{name}: treasure::models::columns::column::init_column(
						&{model}::column_options("{name}".to_string()).unwrap()
					)"#, name=v.name,model=self.name))
		}

		format!(r#"
			// Constructor function for given model (query will use this)
			fn init_new() -> {name} {{
				{name} {{
					{column_inits}
				}}
			}}
		"#,
		name=self.name,
		column_inits=column_inits.join(", ")
		)
	}
}
