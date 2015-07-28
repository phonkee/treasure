use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::quote::rt::{ExtParseUtils};
use syntax::ast::{MetaItem,MetaList};
use syntax::codemap::{Span};


use super::options;

use super::super::utils::attrs;


#[allow(unused_variables)]
pub fn expand_model(excx: &mut ExtCtxt,
					sp: Span,
					mi: &MetaItem,
					item: &Annotatable,
					push: &mut FnMut(Annotatable)) {

	// get model options
	match options::generate_model_options_impls(item){
		Err(e) => excx.span_err(sp, format!(r#"Error {:?}"#, e).as_str()),
		Ok(impls) => {
			for mo_impl in impls.iter() {
				println!("impl {}", mo_impl);
				let _a = Annotatable::Item(excx.parse_item(mo_impl.clone()));
				push(_a);
			}
		},
	}

	excx.span_note(sp, "Please don't forget Treasure is for now proof of concept, which will hopefully end as succesfull ORM!
Design decisions are still to be done, but mapper (inspection) will be soon done.
Any help with design and/or implementation is really appreciated, since write ORM is not easy task.
Right now it's very rough, still nice concept, which we can make together to green badge on arewewebyet.com in category ORM.

Thank you,
	(phonkee)
	");
}

