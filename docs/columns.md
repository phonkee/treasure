Model columns
=============

Rulues:
-------

Column must not have two following underscores in itself "__", because that's how we do spanned relationship queries.

For columns we will provide separate trait called Column (unexpected name).
Every column must implement following trait methods:

    init - init value from column_options (such as default, etc..)
    set_attribute - if given annotation attribute is valid, or make some post proceessing (e.g. in case of "max" convert to integer
    valid_lookups - returns list of valid lookups for given column (eq, gt, gte, ....)

And next it must provide most neede two generic methods:
    to_sql - dump to sql native format
    from_sql - parse sql native format

@TODO: what about to implement also limit?

Crucial is to make implementation so, that additional dialects can extend these, so
that columns will support their database system.


