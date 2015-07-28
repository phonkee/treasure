pub mod builder;

#[macro_export]
macro_rules! Select {
	(model : $e:ident) => {
		{
			let mo = $e::model_options_static();
			let mut q = $crate::builder::Query::select();
			for column in mo.columns.iter() {
				q.add_column(column.clone());
				let column_option = mo.column_options(column);
			}
			q
		}
	}
}
