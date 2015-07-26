Treasure ORM
============

**!!!! Treasure ORM is in phase of experimenting !!!**

ORM library for rust (or rather proof of concept, with following heavy development).

This is still experiment, and I started probably from the other side of ORM that should be started by its development, 
but I think that ease of definition of models and its columns is the "sale argument" of every ORM.
Right after that it's query language.

Treasure ORM will be developed first on just Postgres, but with mind of multiple available dialects.

All ORMs use some kind of reflection to have information about all columns, which is easy to implement in dynamic languages,
but in statically compiled languages it should be done another way.
Rust has for this its healthy macro system from which Treasure uses syntax expressions to generate needed code.
I know you say "that's not idiomatic rust", but it helps to avoid a lot of things.
Let me show my idea how should model definition look like. I will omit for now ForeignKey, ManyToMany, OneToOne which
I am still doing design decisions for now.

```rust
#![feature(custom_attribute,plugin)]
#![plugin(treasure)]

extern crate treasure;

use treasure::models::model::Model;

#[model(db_name="custom_user",primary_key="id",unique(email,test),unique(some,other),index(some,other)]
struct User {

    #[column(db_name="ID",primary_key)]
    pub id: i32,

	#[column(unique)]
	pub username: String,

    #[column]
    pub password: Option<String>,

	#[column]
	pub email: String,

	#[column]
	pub some: String,

	#[column(db_name="custom_other")]
	pub other: String,
}
```

Treasure will generate impl methods such as options() which returns informations about inspected Model.

you can try to run provided example:
    cargo run --example foo --verbose
    
currently for debugging purposes I also print implementation that Treasure generates.
    


Design
------

Every struct that will be persistable should implement trait Model (call that struct model).
This struct must be annotated with "model" attribute that tells Treasure to inspect this model and generate
needed Model trait method. 
Every field in model that is annotated with "field" attribute will be accepted as database column. All other not
annotated will be in options list but marked as "unused".

Model attrs
-----------

For now model has following possible annotations
* db_name - name of the database table, if not given snake case of struct name will be used
* primary_key - name of the field that is primary_key, if incorrect compiler must raise sane error.
    primary key can be also set as column attribute "primary_key"
* unique - list of fields that are unique. Multiple occurences can happen. This will not be used much, but will be used
    in database migrations (which we will support in the future)
* index - list of fields that should belong to index. Multiple occurences can happen also. (For migrations)
* managed - whether Treasure should handle creation of model in db (in future)
Main goal is to write exhaustive compiler errors in case of error. We must have a lot of validations! We can do more!

Decision needed: add annotation inline_options that inlines options() method


Column attrs
------------

Every struct field that you want to persiste to database table must be annotated with "column"
For column there are following possigle annotations:
* db_name - database table column name, if not given struct field name will be used.
* primary_key - information that column is primary_key
* unique - treat column as unique (will be used in db migrations)
* index - attach to column index (in db migration)

These are implemented and added to ColumnInfo which holds all informations about column.
You can see that we have trait Column which all POD types must implement. Also future Option<T> (which stands for nullable column),
ForeignKey, ManyToMany, OneToOne wil implement.
This gives us interesting way of defining new fields in the future (postgres array??)

Column attributes will be extendable, so every Column implementor can have its own set of additional annotation attributes,
e.g. for number types we can implement min, max, default.. The sky is the limit.

Column validations
------------------

Under design decisions!
Every model will have its own possibility to provide validation_fn in model annotations that will be called.
Also Column trait will have validate method that treasure will call with arguments: columnvalue, ColumnOptions instance
for given column.

Code generation
---------------

Treasure is doing quite a lot of code generation to be easily usable without code repetition. Also for future query language
we will need inspected information about model and its fields. That's why Treasure generates option() method for every model,
that gives all these information.
Treasure also generates init_new function for every model where it calls Column::init with ColumnOptions parameter so it
can return appropriate value (default?)


Query
-----

Under design decisions!!

Dialects: postgres...

maybe something like this? (some ideas):

```rust
<<<<<<< HEAD
let users = query::select!(User, query::and(id__gt=13, id__lte=100), active=true).collect()

session::query!(query::select!(User, where(query::and(id__gt=13, id__lte=100), active=true))

let user = User::init_new();
query::insert!(conn, user)
query::update!(user)
query::update!(user, "name", "username")
query::delete!(user)

// @TODO: Raw queries
```

TODO: how to do pluggable modifiers such as "__gt", "__gte" etc...
Ideally every modifier should be passed to Column trait method that will return 
how to do query.
=======
let session = Session::new(conn)
session.begin()

session.select(User()).where("id__lt").collect()
let user = User::init_new()
session.insert(user)
session.commit()
```

Or this?

```rust
let users = query::select!(User, query::and(id__gt=13, id__lte=100), active=true).collect()
```

Q: how to do pluggable modifiers such as "__gt", "__gte" etc...
>>>>>>> d68a6d4918c586172fe13a71f61ba681051d2582


Signals
-------

add support for signals that will be probably in annotation, e.g.:

```rust
#[model(pre_insert="pre_insert")]
struct User {

}
```

This will generate more code ... I am looking forward to all of this.
    
    
Author: 
-------
Peter Vrba (phonkee)

Contributions:
--------------
If you want to contribute with ideas and/or code, I will be very happy!


