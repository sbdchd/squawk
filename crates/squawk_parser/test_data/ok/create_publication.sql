-- simple
create publication p;

-- all_tables
create publication p
  for all tables
  with (foo = bar, bar);

-- table
create publication p for 
  table only foo.bar, 
  table foo.bar.buzz *, 
  table foo, 
  table foo(a, b, c) where (x > 10 or a != b),
  table only (foo)
  with (foo = bar, bar);

-- table_in_schema
create publication p
  for tables in schema a, b, c, current_schema;

-- multiple tables
create publication pub for table chats, users;
