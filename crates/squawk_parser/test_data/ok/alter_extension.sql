-- update_to
alter extension e update;
alter extension e update to json;
alter extension e update to 'foo';

-- schema
alter extension e set schema s;

-- add_drop_access_method
alter extension e add access method o;
alter extension e drop access method o;

-- add_drop_aggregate
alter extension e add aggregate a ( in a text order by b, c );
alter extension e drop aggregate a (*);

-- add_drop_cast
alter extension e add cast (bigint as text);
alter extension e drop cast (varchar(100) as smallint);

-- add_drop_collation
alter extension e add collation o;
alter extension e add collation s.o;
alter extension e drop collation o;
alter extension e drop collation s.o;

-- add_drop_conversion
alter extension e add conversion o;
alter extension e add conversion s.o;
alter extension e drop conversion o;
alter extension e drop conversion s.o;

-- add_drop_domain
alter extension e add domain o;
alter extension e drop domain o;

-- add_drop_event_trigger
alter extension e add event trigger o;
alter extension e drop event trigger o;

-- add_drop_foreign_data_wrapper
alter extension e add foreign data wrapper o;
alter extension e drop foreign data wrapper o;

-- add_drop_foreign_table
alter extension e add foreign table o;
alter extension e add foreign table s.o;
alter extension e drop foreign table o;
alter extension e drop foreign table s.o;

-- add_drop_function
alter extension e add function f;
alter extension e add function s.f(in varchar(100), bigint);
alter extension e drop function f;
alter extension e drop function s.f(in a text, out b varchar(100));

-- add_drop_materialized_view
alter extension e add materialized view o;
alter extension e add materialized view s.o;
alter extension e drop materialized view o;
alter extension e drop materialized view s.o;

-- add_drop_operator
alter extension e add operator << (t, varchar(100));
alter extension e drop operator >> (u, varchar(100));

-- add_drop_operator_class
alter extension e add operator class o using i;
alter extension e drop operator class o using i;

-- add_drop_operator_family
alter extension e add operator family o using i;
alter extension e drop operator family o using i;

-- add_drop_language
alter extension e add language o;
alter extension e add procedural language o;
alter extension e drop language o;
alter extension e drop procedural language o;

-- add_drop_procedure
alter extension e add procedure f;
alter extension e add procedure s.f(in varchar(100), bigint);
alter extension e drop procedure f;
alter extension e drop procedure s.f(in a text, out b varchar(100));

-- add_drop_routine
alter extension e add routine f;
alter extension e add routine s.f(in varchar(100), bigint);
alter extension e drop routine f;
alter extension e drop routine s.f(in a text, out b varchar(100));

-- add_drop_sequence
alter extension e add sequence o;
alter extension e add sequence s.o;
alter extension e drop sequence o;
alter extension e drop sequence s.o;

-- add_drop_table
alter extension e add table o;
alter extension e drop table o;

-- add_drop_text_search_configuration
alter extension e add text search configuration o;
alter extension e add text search configuration s.o;
alter extension e drop text search configuration o;
alter extension e drop text search configuration s.o;

-- add_drop_text_search_dictionary
alter extension e add text search dictionary o;
alter extension e add text search dictionary s.o;
alter extension e drop text search dictionary o;
alter extension e drop text search dictionary s.o;

-- add_drop_text_search_parser
alter extension e add text search parser o;
alter extension e add text search parser s.o;
alter extension e drop text search parser o;
alter extension e drop text search parser s.o;

-- add_drop_text_search_template
alter extension e add text search template o;
alter extension e add text search template s.o;
alter extension e drop text search template o;
alter extension e drop text search template s.o;

-- add_drop_transform
alter extension e add transform for type_name language l;
alter extension e drop transform for type_name language l;

-- add_drop_type
alter extension e add type o;
alter extension e drop type o;

-- add_drop_view
alter extension e add view o;
alter extension e add view s.o;
alter extension e drop view o;
alter extension e drop view s.o;

-- add_drop_extension
alter extension e add extension o;
alter extension e drop extension o;

-- add_drop_publication
alter extension e add publication o;
alter extension e drop publication o;

-- add_drop_schema
alter extension e add schema o;
alter extension e drop schema o;

-- add_drop_server
alter extension e add server o;
alter extension e drop server o;

-- add_drop_database
alter extension e add database o;
alter extension e drop database o;

-- add_drop_index
alter extension e add index o;
alter extension e add index s.o;
alter extension e drop index o;
alter extension e drop index s.o;

-- add_drop_statistics
alter extension e add statistics o;
alter extension e add statistics s.o;
alter extension e drop statistics o;
alter extension e drop statistics s.o;

-- add_drop_role
alter extension e add role o;
alter extension e drop role o;

-- add_drop_subscription
alter extension e add subscription o;
alter extension e drop subscription o;

-- add_drop_tablespace
alter extension e add tablespace o;
alter extension e drop tablespace o;

