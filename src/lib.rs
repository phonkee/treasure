#![feature(plugin_registrar, rustc_private, trace_macros, convert, plugin)]
#![allow(unused_imports)]
#![allow(dead_code)]


extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ext::base::{SyntaxExtension};
use syntax::parse::token;


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let nm = token::intern("model");
    let ext = SyntaxExtension::MultiDecorator(Box::new(expand_model));
    reg.register_syntax_extension(nm, ext);
	reg.register_macro("default_attrs", expand_default_attrs);
}


//pub use models::model::Model;
//pub use models::columns;
pub use models::columns::options::ColumnOptions;
pub use models::{expand_model,expand_default_attrs,get_model_options};
pub use query::*;
pub use models::model::Model;
pub use models::options::ModelOptions;
pub use utils::attrs::{Attr,Attrs};
pub mod models;
pub mod query;
pub mod utils;
