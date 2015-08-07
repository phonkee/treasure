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
