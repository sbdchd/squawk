-- add_table
alter publication p add table t;
alter publication p add table t1, table t2 where (a > b), table only t, table t (a, b);

-- add_tables_in_schema
alter publication p add tables in schema s;
alter publication p add tables in schema current_schema;

-- set_table
alter publication p set table t;
alter publication p set table t1, table t2 where (a > b), table only t, table t (a, b);

-- set_parameters
alter publication p set (param1, param2 = value);

-- drop_table
alter publication p drop table t;
alter publication p drop table t1, table t2 where (a > b), table only t, table t (a, b);

-- owner_to
alter publication p owner to u;
alter publication p owner to current_user;

-- rename
alter publication p rename to q;

