#![feature(plugin_registrar, rustc_private, trace_macros)]
#![feature(convert)]
#![feature(plugin)]
#![allow(unused_imports)]
#![allow(dead_code)]


extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ext::base::{SyntaxExtension};
use syntax::parse::token;


pub use models::model::Model;
pub use models::columns;
pub use models::expand_model;
pub use models::columns::column::*;

pub mod models;
pub mod utils;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let nm = token::intern("model");
    let ext = SyntaxExtension::MultiDecorator(Box::new(expand_model));
    reg.register_syntax_extension(nm, ext);
}

