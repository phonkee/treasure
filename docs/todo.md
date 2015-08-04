TODO tasks
==========

General:
--------
* turn off all unused_... and have clean compilation without warnings. (low prio)

Model:
------
* validations - think about possibilities
* custom init_new method (impl must precede model struct)
* managed attribute

Signals
-------
* think about possibilities - maybe annotation post_save="post_save" should be enough
    but to have in mind that implementation must precede model struct

Columns
-------
* default column_info
* validate attributes (on first run, namely when column_options are created)
* to_sql - related tightly to dialects
* from_sql - tightly related to dialects
* trait OrderColumn - instance of this trait can perform query limit[] clause
* ignored column - just init_new will be fill this columns with valid values
* ignored columns use for _saved attribute.
* how to do autoincrement?
* do we need to have something like django does "contribute_to_struct" (yeah not class, but struct) which
    would to pass also ExtCtxt or item so field can add additional code to struct (pretty dark side huh?)?
    if yes think about how to add additional code to init_new.
* lookups that django supports, think about to also implement them
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


Dialects
--------
* Think about architecture of dialects (related to columns)
* Provide single abstraction

Builder
------------
* well all of it...
* closure (call it map maybe) - to provide mapping function that can fetch data from
    result
* in macros provide imlpementation of map closure - generate code from inspected model
    to read all the data
* tables_count - count of tables (joined) so we can incrementaly add aliases such as t1, t2, t3
* add possibility to have functions in expressions
* name and implement macro that creates map closure for given query builder. This macro must be called at the end of the chain,
    so it means at the end of the query.

Query macros
------------
* provide "not [...]"
* succesfull valid parsing with debug writes
* error checking - work with depth of macro invocation.
* rethink if model_options is needed as macro argument or we can use topmost instance
* rethink if filter is needed as macro argument or we can use top most instance
* add debug to macros - find out bes solution.
* update many:Model - how to handle updates that involve column e.g.: column = column + 1;