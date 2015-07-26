use syntax::ast::{MetaItem_,LitStr,Item_};
use syntax::ext::base::{Annotatable};
use super::meta_item::{MetaItemWalker,MetaItemError,walk_meta_items};
use super::super::utils::string::camel_to_snake;
use super::super::utils::attrs::{Attrs,AttrError};
use super::columns::options::ColumnInfo;


/*
ModelOptionsError
	error
 */
#[derive(Debug)]
pub enum ModelOptionsError {
	NotStruct,
	MetaItemError,
	NoColumns,
	ColumnInfoError,
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
	pub columns: Vec<ColumnInfo>,
	pub managed: bool,
}

impl <'a> ModelOptions <'a> {

	// Returns new ModelOptions from Annotatable object
	pub fn from_annotatable(ann:&Annotatable) -> Result<ModelOptions, ModelOptionsError> {
		let mut model_opts = ModelOptions{
			name: "",
			db_name: "".to_string(),
			primary_key: "",
			managed: true,
			columns: vec![],
		};
		match ann {
			&Annotatable::Item(ref item) => {
				model_opts.name = item.ident.name.as_str();
				model_opts.db_name = camel_to_snake(model_opts.name);

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

				if let Item_::ItemStruct(ref sd, _) = item.node {
					let result = ColumnInfo::new_vec_from_struct_def(sd);
					match result {
						Err(_) => {
							return Err(ModelOptionsError::ColumnInfoError)
						}
						Ok(f) => {
							if f.len() == 0 {
								return Err(ModelOptionsError::NoColumns)
							}
							model_opts.columns = f
						}
					}
				}

			},
			_ => return Err(ModelOptionsError::NotStruct)
		}


		Ok(model_opts)
	}

	// Returns iplementation source
	pub fn get_impl(&self) -> String {

		let mut cinfos:Vec<String> = vec![];

		for i in self.columns.iter() {
			cinfos.push(i.get_impl())
		}

		// generate column initials
		let mut column_inits = vec![];
		for i in self.columns.iter() {
			column_inits.push(format!(r#"
			{0}: treasure::models::columns::column::init_column(
				&{1}		// for runtime version use => &Self::column_info("{0}".to_string()).unwrap())
			)"#, i.name, i.get_impl()))
		}

		let implitem = {
			format!(r#"
impl treasure::models::model::Model for {0} {{

	// option implementation for model {0}
	// should we inline options? or at least provide model annotation for this?
	fn options(&self) -> treasure::models::options::ModelOptions {{
		treasure::models::options::ModelOptions{{
			name: "{0}",
			db_name: "{1}".to_string(),
			primary_key: "{2}",
			managed: false,
			columns: vec![{3}],
		}}
	}}

	// Returns ColumnInfo for given column
	// Will be needed for querying
	// TODO: implement this method to return info about option
	fn column_info(_c: String) -> Option<treasure::models::columns::options::ColumnInfo> {{
		None
	}}

	// Constructor function for given model (query will use this)
	fn init_new() -> {0} {{
		{0} {{{4}
		}}
	}}
}}

"#, self.name, self.db_name, self.primary_key, cinfos.join(", "), column_inits.join(", "))
        };

		println!("This is generated implementation of model:\n{}", implitem);

		implitem
	}

}

