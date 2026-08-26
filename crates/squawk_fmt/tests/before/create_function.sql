create function add(integer, integer) returns integer language sql as $$ select $1 + $2 $$;

create or replace function public.add(a integer, b integer default 1) returns integer language sql immutable strict parallel safe cost 10 rows 1 security definer set search_path to public as $$ select a + b $$;

create function get_users(p_active boolean) returns table (id bigint, name text) language plpgsql security definer set search_path = public, pg_temp as $body$ begin return query select id, name from users where active = p_active; end; $body$;

create function a_function_with_a_very_long_name(a_parameter_with_a_very_long_name numeric, another_parameter_with_a_very_long_name text default 'a long default value') returns table (a_column_with_a_very_long_name numeric, another_column_with_a_very_long_name text) language sql as $$ select $1, $2 $$;

create function option_examples(in first integer, out second text, inout third bigint, variadic rest text[]) returns text external security invoker called on null input returns null on null input not leakproof stable window support public.support_fn transform for type integer, for type text set work_mem from current reset all language 'sql' as $$ select null::text $$;

-- comments in every position
create /*a*/ or /*b*/ replace /*c*/ function /*d*/ app /*e*/. /*f*/ commented
(/*g*/ in /*h*/ value /*i*/ integer /*j*/ default /*k*/ 1 /*l*/, /*m*/ out /*n*/ result /*o*/ text /*p*/)
/*q*/ returns /*r*/ table /*s*/ (/*t*/ id /*u*/ bigint /*v*/, /*w*/ label /*x*/ text /*y*/)
/*z*/ language /*aa*/ sql
/*ab*/ immutable
/*ac*/ strict
/*ad*/ parallel /*ae*/ safe
/*af*/ cost /*ag*/ 10
/*ah*/ rows /*ai*/ 1
/*aj*/ security /*ak*/ definer
/*al*/ set /*am*/ search_path /*an*/ to /*ao*/ public /*ap*/, /*aq*/ pg_temp /*ar*/
/*as*/ reset /*at*/ all
/*au*/ support /*av*/ public /*aw*/. /*ax*/ support_fn
/*ay*/ as /*az*/ $$ select value::text $$ /*ba*/;

create function increment(value integer) returns integer language sql begin atomic return value + 1; end;

create function record_and_calculate(a_very_long_input_parameter_name integer, another_very_long_input_parameter_name integer) returns integer language sql begin atomic insert into function_audit_log (first_recorded_value, second_recorded_value) values (a_very_long_input_parameter_name, another_very_long_input_parameter_name); return a_very_long_input_parameter_name + another_very_long_input_parameter_name; end;

create function commented_body(value integer) returns integer language sql /*bb*/ begin /*bc*/ atomic /*bd*/ insert /*be*/ into function_log /*bf*/ (value) /*bg*/ values /*bh*/ (value) /*bi*/; /*bj*/ return /*bk*/ value + 1 /*bl*/; /*bm*/ end /*bn*/;

create function external_add(integer, integer) returns integer as '$libdir/example', 'external_add' language c;

create function commented_external() returns integer as /*bo*/ '$libdir/example' /*bp*/, /*bq*/ 'commented_external' /*br*/ language c;

create function function_with_a_very_long_external_definition() returns integer as '$libdir/a_very_long_object_file_name_that_does_not_fit_on_the_same_line', 'a_very_long_link_symbol_name_that_does_not_fit_on_the_same_line' language c;
