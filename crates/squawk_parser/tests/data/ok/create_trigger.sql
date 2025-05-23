-- create_trigger
CREATE TRIGGER update_foo_column
BEFORE INSERT ON core_recipe
FOR EACH ROW
EXECUTE PROCEDURE foo_update_trigger();

-- with_most_fields
create or replace trigger buzz instead of insert or delete 
on foo.bar.buzz
referencing 
  old table as foo
  new table as bar
for each statement 
when (x > 10 and b is not null)
execute function x.y.z(1,2,'3');

-- constraint
create constraint trigger t after insert or delete 
on f
for each row
execute function f();

-- with_most_fields_part2
create trigger bar after update of a, b, c
on foo
referencing 
  new table bar
  old table foo
for row 
execute procedure foo('bar');

-- simple
create trigger bar before truncate or delete or insert
on buzz
execute function a();
