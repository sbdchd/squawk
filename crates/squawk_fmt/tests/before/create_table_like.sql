create table t (LIKE foo);
create table t (like foo, id int);
create table t (id int, like foo, like bar);

-- like options
create table t (like foo INCLUDING ALL);
create table t (like foo EXCLUDING INDEXES);
create table t (like foo INCLUDING DEFAULTS EXCLUDING CONSTRAINTS INCLUDING IDENTITY);
create table t (like foo including comments including compression including generated including statistics including storage);

-- table names
create table t (like PUBLIC.Accounts);
create table t (like "Public"."Users" including all);
create table t (like "foo"."bar");
create table t (like "select");
create table t (like U&"d!0061tum" uescape '!' including all);

-- comments
create table t (like /* a */ foo /* b */ including /* c */ all);
create table t (like foo -- a line comment
including all);

create table a_very_long_destination_table_name (like a_very_long_source_schema_name.a_very_long_source_table_name including comments including compression including constraints including defaults including generated including identity including indexes including statistics including storage);
