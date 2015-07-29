Model signals
=============

signals are functions that are called during the lifetime of model instance, when some event occurs.

Signals will be:
    * post_load
    * pre_insert
    * post_insert
    * pre_update
    * post_update
    * pre_delete
    * post_delete

The question is how to implement them so that the user can have some context (e.g. for cache invalidation).

Stay tuned...