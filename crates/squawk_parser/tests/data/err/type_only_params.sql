-- type-only parameter lists should recover instead of getting stuck
prepare p (variadic) as select 1;
prepare p (in) as select 1;
create operator class c for type int using btree as function 1 (variadic) f(int);
alter operator family f using btree add function 1 (variadic) f(int);
alter operator family f using btree drop function 1 (variadic);
