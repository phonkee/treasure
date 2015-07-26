use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::quote::rt::{ExtParseUtils};
use syntax::ast::{MetaItem,MetaList};
use syntax::codemap::{Span};

trace_macros!(true);

use super::options::ModelOptions;

#[allow(unused_variables)]
pub fn expand_model(excx: &mut ExtCtxt,
					sp: Span,
					mi: &MetaItem,
					item: &Annotatable,
					push: &mut FnMut(Annotatable)) {

	// get model options
	let result = ModelOptions::from_annotatable(item);

	match result {
		Err(e) => excx.span_err(sp, format!(r#"Error {:?}"#, e).as_str()),
		Ok(r) => {
			let mo_impl = r.get_impl();
			let _a = Annotatable::Item(excx.parse_item(mo_impl));
			push(_a)

			// @TODO: additional generated implementations.
		},
	}

	excx.span_note(sp, "Please don't forged Treasure is for now proof of concept, which will hopefully end as succesfull ORM!
There are still unanswered questions, and I hope I will found right answers.
However any help with design and/or programming is really appreciated, since write ORM is not easy task.

Right now it's very rough still nice concept, which we can make together to green badge on arewewebyet.com

Thank you,
	(phonkee)
	");
}

#[test]
fn it_works() {
}
