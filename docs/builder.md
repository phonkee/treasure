Query language
--------------

Main component for query-ing is Builder. Builder must support all types of queries and also 
other things.

Aside of builder treasure will have implemented set of macros that will make querying a breeze.
Since macro language supports very broad range of things, we can do querying really interesting.

Following statements are just a proposol for query language, I am still not sure if all these syntactic
sugar things are possible (I hope so). First will be to check how macros in macros work.
Have a look at this:

```rust
selecting data from database

let qb = select!(many:User[
    filter[
        ["name" => 10]
        ["username" => "hello"]
    ]
]);

let qb = select!(many:User[
    filter[
        ["name__in" => ["Peter", "Vrba"]
    ]
    limit[1]
]);

let qb = select!(many:User[
    filter[
        and [
            ["name__icontains" => "Peter"]
            ["age" => 30]
        ]
    ] 
    limit[1, 10]
]);
    
let qb = select!(many:User[
    filter[
        or[
            ["pk" => 22], 
            ["name" => "Peter"]
        ]
    ]
]);

let qb = select!(one:User[
    filter["id" => 2]
]);

// filter expressions will know how to span relationships (when ForeignKey will be implemented
let qb = select!(many:BlogPost[
    filter[
        ["author__name__exact" => ["Peter", "Vrba"]]
    ]
    limit[1, 10]
]);

// additional filtering you can reuse already written macros
let query = select!(many:BlogPost[
    filter[
        or[ 
            ["author__pk" => 22] 
            ["author__pk" => 10]
        ]
    ]
]);
let qb  = filter!(qb, 
    and[
        ["pk" => 2]
    ]
);

// updating either model instance or bulk update
let qb = update!(blog[
    columns[
        "name",
        "username"
    ]
]);

let qb = update!(model:BlogPost[ 
    set[
        ["name" = "hello"]
    ]
    filter[
        ["something__isnull" => true]
    ]
]);

// delete either model instance or bulk deletion
let qb = delete!(blog);
let qb = delete!(model:BlogPost[
    filter[
        ["rating__gt" => 10]
    ]
]);

// inserting new instance
let qb = insert!(blog);

```


Builder
=======

struct that will hold any type of sql query witch all informations.
Currently we will support just simple queries (no subqueries), although in the future we can think about that


**Warning**

This api is in its beginning phase and can be still subject to change. Please accept this as early proposal.
Some parts are still not implemented!

Example api calls to query buidler:

```rust
// select query buidler
builder::select()
	.columns(["t1.some", "t1.other"])
	.column("t1.column1")
	.column("t1.column2")
	.table("table")
	.filter(
		or(
			not(
				and(
					-q("some", in, ["asdf", "asdf"])
				).and(
					q("other", eq, "aaa")
				)
			)
		).or(
			-and(
                -q(c("something"), eq, "asdf")
			).and(
				q("other", contains, "eq")
			)
		)
	)
	.left_join(
		table("table").alias("t2"),
		"t1.id",
		"t2.id"
	)
	.right_join(
		table("table").alias("t3"),
		"t2.id",
		"t3.id"
	)
	.limit(10)
	.offset(10)
	.suffix("something")
	

// update query builder
builder::update()
	.table("table")
	.alias("t1")
	.set("column", "something")
	.set(
	    c("column2"), 
	    func("wohooo")
	        .arg(
	            col("other")
	        ).arg(
	            col("some")
	        )
	.set(c("column3"), "tramtarara")
	.set(c("column4"), "bumtsss!")
	.filter(
		or(
            !and(
                q(c("t1.some"), in, ["asdf", "asdf"])
            ).and(
                !q(c("other"), eq, "aaa")
            )
		).or(
		    -and(
				not(
					q(c("something"), eq, "asdf")
				)
			).and(
				-q(c("other"), contains, "eq")
			)
		)
	)
	.join(t("table").alias("t2"), c("t1.id"), c("t2.something"))
	.limit(10)
	.offset(10)

// delete query builder
builder::delete()
	.table("table")
	.filter(
		or(
            !and(
                q("some", in, ["asdf", "asdf"])
            ).and(
                q("other", eq, "aaa")
            )
		).or(
			and(
                !q("something", eq, "asdf")
			).and(
				q("other", contains, "eq")
			)
		)
	)
	.limit(10)

// insert query builder
builder::insert()
	.table("table")
	.value("asdf", "asdf")
	.value("asdf", "asdf")
	.suffix("some")
```
