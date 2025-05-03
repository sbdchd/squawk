-- simple
create sequence s;

-- full
create temporary sequence if not exists s
 as varchar(100)
 increment by 10
 minvalue 1
 no minvalue
 maxvalue 100
 no maxvalue
 start with 10
 cache 10
 no cycle
 owned by foo.bar;

-- unlogged
create unlogged sequence s;

