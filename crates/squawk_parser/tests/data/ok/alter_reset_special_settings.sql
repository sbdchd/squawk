alter database x reset time zone;
alter role x reset session authorization;
alter function f() reset time zone;
alter database x set names;
create function f() returns int set time zone interval '1 hour' language sql return 1;
