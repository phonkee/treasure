#![feature(custom_attribute,plugin)]
#![allow(dead_code)]
#![allow(plugin_as_library)]
#![allow(unused_attributes)]
#![plugin(treasure)]


#[macro_use]
extern crate treasure;

use treasure::models::model::Model;


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

	// example select query (really hairy one)
	let _qb = select!(many:User[
		filter [
			["a__gt" => 2]
			["a__gt" => 3]
			and [
				["x__contains" => "something"]
			]
			or [
				or [
					and [
						["a" => 1]
						["a" => 2]
					]
					and[
						["c" => 4]
						["d" => 6]
					]
				]
				and [
					["a" => 2]
				]
			]
		]
		limit[1]
	]);

	// example of update query
	let _user = User::init_new();
	let _qb = update!(user[
		columns[
			["test"]
			["some"]
		]
	]);

}

