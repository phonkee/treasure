#![feature(custom_attribute,plugin)]
#![allow(dead_code)]
#![allow(plugin_as_library)]
#![allow(unused_attributes)]
#![plugin(treasure)]


#[macro_use]
extern crate treasure;




#[model(db_name="custom_user",primary_key="id",unique(email,test),unique(some,other))]
struct User {

	#[field(db_name="ID",primary_key,other(other(other)))]
	pub id: i32,

	#[field(db_name="username",unique,some(one,two,three))]
	pub username: String,

	#[field(db_name="password")]
	pub password: Option<String>,

	#[field(db_name="email")]
	pub email: String,

	#[field(db_name="some")]
	pub some: String,

	#[field(db_name="other")]
	pub other: String,
}




fn main() {

	let _q = Select!(model:User);
	println!("this is select {:?}", _q)
}