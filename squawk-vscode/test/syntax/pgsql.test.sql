-- SYNTAX TEST "source.pgsql" "basic PostgreSQL highlighting"

select 1;
-- <------ keyword.other.pgsql

count(1);
-- <----- entity.name.function.pgsql

'hello';
-- <------- string.quoted.single.pgsql

42;
-- <-- constant.numeric.pgsql

::;
-- <-- keyword.operator.cast.pgsql

:=;
-- <-- keyword.operator.pgsql

-- comment
-- <---------- comment.line.double-dash.pgsql

/* comment */
-- <------------- comment.block.c

create function x
-- <------ keyword.other.create.pgsql
--     ^^^^^^^^ keyword.other.pgsql
--              ^ entity.name.function.pgsql
