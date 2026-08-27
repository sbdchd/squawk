create trigger update_foo_column before insert on core_recipe for each row execute procedure foo_update_trigger();

create or replace trigger buzz instead of insert or delete on foo.bar.buzz referencing old table as foo new table as bar for each statement when (x > 10 and b is not null) execute function x.y.z(1,2,'3');

create constraint trigger t after insert or delete on f from other_f deferrable initially deferred for each row execute function f();

create trigger bar after update of a, b, c on foo referencing new table bar old table foo for row execute procedure foo('bar');

create trigger a_trigger_with_a_very_long_name before update of a_column_with_a_very_long_name or insert or delete on a_schema_with_a_very_long_name.a_table_with_a_very_long_name for each statement execute function a_schema_with_a_very_long_name.a_function_with_a_very_long_name('a long argument value');

-- comments in every position
create /*a*/ or /*b*/ replace /*c*/ constraint /*d*/ trigger /*e*/ commented_trigger /*f*/ instead /*g*/ of /*h*/ update /*i*/ of /*j*/ first_column /*k*/, /*l*/ second_column /*m*/ or /*n*/ delete /*o*/ on /*p*/ app /*q*/. /*r*/ records /*s*/ from /*t*/ app /*u*/. /*v*/ source_records /*w*/ deferrable /*x*/ initially /*y*/ deferred /*z*/ referencing /*aa*/ old /*ab*/ table /*ac*/ as /*ad*/ old_rows /*ae*/ new /*af*/ table /*ag*/ new_rows /*ah*/ for /*ai*/ each /*aj*/ row /*ak*/ when /*al*/ (/*am*/ old_rows.first_column /*an*/ > /*ao*/ 1 /*ap*/) /*aq*/ execute /*ar*/ function /*as*/ app /*at*/. /*au*/ handle_records(/*av*/ 1 /*aw*/, /*ax*/ 'two' /*ay*/) /*az*/;
