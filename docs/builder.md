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
		!(
			(
				!q("some", in, ["asdf", "asdf"]) 
				& 
				q("other", eq, "aaa")
			) | (
				!q(c("something"), eq, "asdf")
				&
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
		!(
			(
				!q("some", in, ["asdf", "asdf"])
				&
				q("other", eq, "aaa")
			) | (
				!q(c("something"), eq, "asdf")
				&
				q("other", contains, "eq")
			)
		)
	)
	.join(t("table").alias("t2"), c("t1.id"), c("t2.something"))
	.limit(10)
	.offset(10)

	.filter(
		q("t1.some", eq, "asdf") | q("t2.some", eq, "something")
	)


// delete query builder
builder::delete()
	.table("table")
	.filter(
		!(
			(
				!q("some", in, ["asdf", "asdf"])
				&
				q("other", eq, "aaa")
			) | (
				!q(c("something"), eq, "asdf")
				&
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
