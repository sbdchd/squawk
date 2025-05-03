-- table
SECURITY LABEL FOR selinux ON TABLE mytable IS 'system_u:object_r:sepgsql_table_t:s0';

SECURITY LABEL FOR selinux ON TABLE mytable IS NULL;

-- column
security label on column foo.bar is null;

-- on_aggregate_star
security label on aggregate foo.bar(*) is '';

-- on_aggregate_args
security label on aggregate foo.bar(
  in foo integer,
  bar integer,
  text
) is '';

-- on_aggregate_args_with_order_by
security label on aggregate foo.bar(
  integer,
  text,
  numeric
  order by
    in a timestamp,
    b numeric,
    text
) is '';

security label on aggregate foo.bar(
  order by
    in a timestamp,
    b numeric,
    text
) is '';

-- database
security label on database foo is null;

-- domain
security label on domain bar.foo is null;

security label on domain foo is null;

-- event_trigger
security label on event trigger foo is null;

-- foreign_table
security label on foreign table bar is null;

-- function
security label on function foo.bar.buzz is null;

security label on function foo () is null;

security label on function foo (
  in a text,
  out b numeric,
  bigint
) is null;

-- large_object
security label on large object 1234 is null;

-- materialized_view
security label on materialized view foo.bar is null;

-- language
security label on procedural language bar is null;

security label on language bar is null;

-- procedure
security label on procedure foo.bar is null;

security label on procedure foo.bar() is null;

security label on procedure bar(
  in foo text, 
  numeric
) is null;

-- publication
security label on publication bar is null;

-- role
security label on role bar is null;

-- routine
security label on routine foo.bar is null;

security label on routine foo.bar() is null;

security label on routine foo.bar(
  in foo text, 
  numeric
) is null;

-- schema
security label on schema bar is null;

-- sequence
security label on sequence foo.bar is null;

-- subscription
security label on subscription bar is null;

-- tablespace
security label on tablespace bar is null;

-- type_
security label on type foo.bar is null;

-- view
security label on view foo.bar is null;

