Treasure ORM
============

**!!!! Treasure ORM is in phase of experimenting !!!**

ORM library for rust (or rather proof of concept, with following heavy development) inspired by awesome django framework.

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

Treasure will generate Model trait impl methods such as:
```rust
fn model_options(&self) -> ModelOptions;
```
which returns inspect information about model (store as static? in the future for speed improvements)

```rust
fn init_new() -> Self;
```
which is constructor method to create new model instance.


Right now you can try and see what currently Treasure orm does by runnin:
```bash
    cargo run --example simple --verbose
```

For debugging/implementation purposes Treasure dumps generated implementations to stdout.


Design
------

Every struct that will be persistable should implement trait Model (call that struct model).
This struct must be annotated with "model" attribute that tells Treasure to inspect this model and generate
needed Model trait method. 
Every field in model that is annotated with "column" attribute will be accepted as database column. All other not
annotated will be in options list but marked as "unused".

Model attrs
-----------

For now model has following possible annotations
* db_name - name of the database table, if not given snake case of struct name will be used
* primary_key - name of the column that is primary_key, if incorrect compiler must raise sane error.
    primary key can be also set as column attribute "primary_key"
* unique - list of columns that are unique. Multiple occurences can happen. This will not be used much, but will be used
    in database migrations (which we will support in the future)
* index - list of fields that should belong to index. Multiple occurences can happen also. (For migrations)
* managed - whether Treasure should handle creation of model in db (in future)
Main goal is to write exhaustive compiler errors in case of error. We must have a lot of validations! We can do more!

Decision needed: add annotation inline_options that inlines model_options() method


Column attrs
------------

Every struct field that you want to persiste to database table must be annotated with "column"
For column there are following possigle annotations:
* db_name - database table column name, if not given struct field name will be used.
* primary_key - information that column is primary_key
* unique - treat column as unique (will be used in db migrations)
* index - attach to column index (in db migration)
* not_persist - do not persist this field to database

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
we will need inspected information about model and its fields. That's why Treasure generates supporting methods for every model,
that gives all model information.
Treasure also generates init_new function for every model where it calls Column::init with ColumnOptions parameter so it
can return appropriate value (default?)


Query
-----

Treasure ORM provides set of macros to make querying of models easier. 
This part is still in the making, some small part of select macros are already written, however now they need
to be connected to real QueryBuilder. Treasure will provide two builders:
QueryBuilder - this builder works upon tables, columns
ModelQueryBuilder - this builder will be tightly coupled with models and will have method to return QueryBuilder that
                    will be populated from data from given model.

QueryBuilder will also have ability to "map" results to object, probably it will be function that accepts closure with 
argument rows (in single mode row). This rows will not be direct rows from database engine, but abstraction over them
because we have also possibility to have "aliases" for columns defined in model (db_name).

*Query macros:*

Query macros have their names by sql counterparts.
* select - macro to perform select queries
* update - macro to perform update queries (single instances or multiple rows)
* delete - macro to perform delete queries (single instances or multiple rows)
* insert - macro to insert model instances to database

Every macro has first argument sort of identification followed by "[" where are all parts of query are specified, ending with "]". 
select query has 2 possibilities:
* many:<struct> - this is for selecting multiple objects

```rust 
select![many:User[<query_parts>]]
```

* one:<struct> - this selects just one object from database. (TODO: exceptions DoesNotExist, MultipleObjectsReturned)
```rust 
select![one:User[<query_parts>]]
```        

The <query_parts> part shown in example is where all modifiers are set. these modifiers are defined following way:
<name of modifier>[<modifier options>].
You can see that modifiers are not separated by "," it's from the nature of macros, rather their values are
surrounded by [] which makes them quite readable. In following example you can see that.

Example:
```rust
select![many:User[
    filter[
        ["age__lt" => 10]    
    ]
    limit[1, 10]
]]
```

In next parts I will try to explain every query_part of queries

filter:
-------

Filter applies to following queries: select, update(mass), delete(mass)

In filter you can specify separate clauses such as:
```rust
["username" => "phonkee"]
```

First is name of model column following by => and value. Column name can have field lookups (such as in django)
Lookups take the form ```["field__lookuptype" => value]```. If lookup type is not specified "__exact" is used.
The plan is to have support for following lookup types;
* exact
* iexact
* contains
* icontains
* in
* gt
* gte
* lt
* lte
* startswith
* istartswith
* endswith
* iendswith
* range
* year
* month
* day
* week_day
* isnull
* search
* regex
* iregex


and probably other...

@TODO: add not[....] modifier.

Filtering supports also AND and OR conditions. They both have this format and [...], or [...].
You can stack them anyway you want. If you don't provide single "and" or "or" in filter, 
they will be default wrapped in AND clause.

```rust
select!(many:User[
    filter[
        ["name__icontains" => "Peter"]
        ["age__gte" => 30]
    ]
])
```

will be automatically wrapped to and clause and will equal to this:
```rust
select!(many:User[
    filter[
        and [
            ["name__icontains" => "Peter"]
            ["age__gte" => 30]
        ]
    ]
])
```

AND and OR clauses can be stacked, so you can create really complex clauses:

```rust
select!(many:User[
    filter[
        or [
            ["something__icontains" => "ehm"]            
        ]
        and [
            ["name__icontains" => "Peter"]
            ["age__gte" => 30]
        ]
        ["one__in" => ["one", "two", "three"] 
    ]
])
```

will be equal as:
```rust
select!(many:User[
    filter[
        and [
            or [
                ["something__icontains" => "ehm"]            
            ]
            and [
                ["name__icontains" => "Peter"]
                ["age__gte" => 30]
            ]
            ["one__in" => ["one", "two", "three"] 
        ]
    ]
])
```

nice example is also [Example](examples/simple.rs#L38-62)

Isn't that pretty?
I hope you like this query language as I like, more updates will come later, there are a ton of things more to implement,
and yes I mean a ton just in these querying macros. Like update already instantiated query builder with additional filters, limits...

TODO: 
define other macros

select
------

select macro instantiates select query builder and prepopulates it with model options.

```rust
// selecting data from database
select!(many:User[
    filter[
        ["username" => "phonkee"]
        ["age__gte" => 30]
    ]
]).collect(db)
```

update
------

update macro has two possibilities to call:
* update!(model_instance[...]) - this updates model instance
* update!(many:User[...]) - this makes bulk update to database (TODO: not implemented)

examples:

```rust
// example of update of single model instance
let _user = User::init_new();
let _qb = update!(user[
    columns[
        "count_logins",
        "last_logged"
    ]
]);
```

```rust
// example of update of single model instance
// @TODO: implement, find out how to do additions...
let _qb = update!(many:User[
    set[
        ["quote" => "something"]
    ]
]);
```


Signals
-------

add support for signals that will be probably in annotation, e.g.:

```rust
#[model(pre_insert="pre_insert")]
struct User {
}
```

Signals to support:
* post_load - after object has been load from db
* pre_insert - before insert to db
* post_insert - after insert to db
* pre_update - before update of model instance to db
* post_update - after update to database

This will generate more code ....


Database connection
-------------------

Design decision needed!
Write wrapper around results (all dialects can implement).
    
Contributions:
--------------
If you want to contribute with ideas and/or code, I will be very happy!

Author: Peter Vrba (phonkee)
License: MIT
