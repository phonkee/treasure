				impl treasure::Model for User
				{
					fn model_options(&self) -> treasure::ModelOptions{
						struct MO;
						impl treasure::ModelOptions for MO {
							pub fn name() -> String {
								"User".to_string()
							}
							pub fn db_name() -> String {
								"user".to_string()
							}
							pub fn primary_key() -> String {
								"".to_string()
							}
							pub fn columns() -> Vec<String> {
								return vec!["email".to_string(),"id".to_string(),"other".to_string(),"password".to_string(),"some".to_string(),"username".to_string()];
							}
							pub fn column_options (column:String) -> &'static treasure::ColumnOptions {
								let mut i = match column.as_ref() {

						"email" => {

	struct CO_email;
	impl treasure::ColumnOptions for CO_email {
		pub fn name() -> String {
			"email".to_string()
		}
		pub fn db_name() -> String {
			"email".to_string()
		}
		pub fn ty() -> String {
			"String".to_string()
		}
		pub fn attrs() -> treasure::Attrs {
			{ let i = treasure::Attrs::new();i.insert(&"db_name".to_string(), treasure::Attr::NamedAttr("db_name".to_string(), Box::new(treasure::Attr::StringAttr("email".to_string()))));*&i.clone() }
		}
	}
	CO_email


						},
						"id" => {

	struct CO_id;
	impl treasure::ColumnOptions for CO_id {
		pub fn name() -> String {
			"id".to_string()
		}
		pub fn db_name() -> String {
			"id".to_string()
		}
		pub fn ty() -> String {
			"i32".to_string()
		}
		pub fn attrs() -> treasure::Attrs {
			{ let i = treasure::Attrs::new();i.insert(&"db_name".to_string(), treasure::Attr::NamedAttr("db_name".to_string(), Box::new(treasure::Attr::StringAttr("ID".to_string()))));i.insert(&"primary_key".to_string(), treasure::Attr::StringAttr("primary_key".to_string()));i.insert(&"other".to_string(), treasure::Attr::ListAttr("other".to_string(), vec![treasure::Attr::ListAttr("other".to_string(), vec![treasure::Attr::StringAttr("other".to_string())])]));*&i.clone() }
		}
	}
	CO_id


						},
						"other" => {

	struct CO_other;
	impl treasure::ColumnOptions for CO_other {
		pub fn name() -> String {
			"other".to_string()
		}
		pub fn db_name() -> String {
			"other".to_string()
		}
		pub fn ty() -> String {
			"String".to_string()
		}
		pub fn attrs() -> treasure::Attrs {
			{ let i = treasure::Attrs::new();i.insert(&"db_name".to_string(), treasure::Attr::NamedAttr("db_name".to_string(), Box::new(treasure::Attr::StringAttr("other".to_string()))));*&i.clone() }
		}
	}
	CO_other


						},
						"password" => {

	struct CO_password;
	impl treasure::ColumnOptions for CO_password {
		pub fn name() -> String {
			"password".to_string()
		}
		pub fn db_name() -> String {
			"password".to_string()
		}
		pub fn ty() -> String {
			"Option<String>".to_string()
		}
		pub fn attrs() -> treasure::Attrs {
			{ let i = treasure::Attrs::new();i.insert(&"db_name".to_string(), treasure::Attr::NamedAttr("db_name".to_string(), Box::new(treasure::Attr::StringAttr("password".to_string()))));*&i.clone() }
		}
	}
	CO_password


						},
						"some" => {

	struct CO_some;
	impl treasure::ColumnOptions for CO_some {
		pub fn name() -> String {
			"some".to_string()
		}
		pub fn db_name() -> String {
			"some".to_string()
		}
		pub fn ty() -> String {
			"String".to_string()
		}
		pub fn attrs() -> treasure::Attrs {
			{ let i = treasure::Attrs::new();i.insert(&"db_name".to_string(), treasure::Attr::NamedAttr("db_name".to_string(), Box::new(treasure::Attr::StringAttr("some".to_string()))));*&i.clone() }
		}
	}
	CO_some


						},
						"username" => {

	struct CO_username;
	impl treasure::ColumnOptions for CO_username {
		pub fn name() -> String {
			"username".to_string()
		}
		pub fn db_name() -> String {
			"username".to_string()
		}
		pub fn ty() -> String {
			"String".to_string()
		}
		pub fn attrs() -> treasure::Attrs {
			{ let i = treasure::Attrs::new();i.insert(&"some".to_string(), treasure::Attr::ListAttr("some".to_string(), vec![treasure::Attr::StringAttr("one".to_string()), treasure::Attr::StringAttr("two".to_string()), treasure::Attr::StringAttr("three".to_string())]));i.insert(&"unique".to_string(), treasure::Attr::StringAttr("unique".to_string()));i.insert(&"db_name".to_string(), treasure::Attr::NamedAttr("db_name".to_string(), Box::new(treasure::Attr::StringAttr("username".to_string()))));*&i.clone() }
		}
	}
	CO_username


						}
								};
								i
							}
						}
						let i = MO;
						i
					}
				}