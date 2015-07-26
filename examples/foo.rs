#![feature(custom_attribute,plugin)]
#![allow(dead_code)]
#![allow(plugin_as_library)]
#![allow(unused_attributes)]
#![plugin(treasure)]

extern crate treasure;

use treasure::models::model::Model;

#[model(db_name="custom_user",primary_key="id",unique(email,test),unique(some,other))]
struct User {

	#[field(db_name="ID",primary_key)]
	pub id: i32,

	#[field(db_name="username",unique)]
	pub username: String,

	#[field(db_name="password",null)]
	pub password: Option<String>,

	#[field(db_name="email",null)]
	pub email: String,

	#[field(db_name="some",null)]
	pub some: String,

	#[field(db_name="other",null)]
	pub other: String,
}

fn main() {
	let _u = User::init_new();
}
