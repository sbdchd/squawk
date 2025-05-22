-- access_method
comment on access method m is '';

-- aggregate_star
comment on aggregate my_agg (*) is '';

-- aggregate
comment on aggregate a (*) is '';

-- aggregate_with_params
comment on aggregate a (in a text, text) is '';

-- aggregate_order_by
comment on aggregate a (int, text order by timestamp) is '';

-- cast
comment on cast (s as t) is '';

-- collation
comment on collation c is '';

-- column
comment on column a.b is '';

-- constraint_on_table
comment on constraint c on t is '';

-- constraint_on_domain
comment on constraint c on domain d is '';

-- conversion
comment on conversion c is '';

-- database
comment on database d is '';

-- domain
comment on domain d is '';

-- extension
comment on extension e is '';

-- event_trigger
comment on event trigger e is '';

-- foreign_data_wrapper
comment on foreign data wrapper f is '';

-- foreign_table
comment on foreign table f is '';

-- function
comment on function f() is '';

-- function_with_args
comment on function f(in a int, b int, text) is '';

-- index
comment on index idx is '';

-- large_object
comment on large object 1 is '';

-- materialized_view
comment on materialized view v is '';

-- operator
comment on operator @>(jsonb, jsonb) is '';

comment on operator @>(varchar(100), varchar(200)) is '';

-- operator_class
comment on operator class c using i is '';

-- operator_family
comment on operator family f using i is '';

-- policy
comment on policy p on t is '';

-- language
comment on language l is '';

-- procedural_language
comment on procedural language l is '';

-- procedure
comment on procedure p() is '';

-- procedure_with_args
comment on procedure p(in a date, b date, bigint) is '';

-- publication
comment on publication p is '';

-- role
comment on role r is '';

-- routine
comment on routine r() is '';

-- routine_with_args
comment on routine r(in a text, out b json, bigint) is '';

-- rule
comment on rule r on t is '';

-- schema
comment on schema s is '';

-- sequence
comment on sequence s is '';

-- server
comment on server s is '';

-- statistics
comment on statistics s is '';

-- subscription
comment on subscription s is '';

-- table
comment on table t is '';

-- table_null
comment on table t is null;

-- tablespace
comment on tablespace t is '';

-- text_search_configuration
comment on text search configuration t is '';

-- text_search_dictionary
comment on text search dictionary t is '';

-- text_search_parser
comment on text search parser t is '';

-- text_search_template
comment on text search template t is '';

-- transform
comment on transform for t language l is '';

-- trigger
comment on trigger t on u is '';

-- type_
comment on type t is '';

-- view
comment on view v is '';

