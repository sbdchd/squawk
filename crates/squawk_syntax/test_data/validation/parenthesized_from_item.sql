select * from (values (0,9998)) v(id,x), lateral (select 1) ss;
select * from (foo join bar on foo.id = bar.id);

select * from (values (0,9998)) v(id,x), (lateral (select 1)) ss;
select * from (foo);
select * from (foo *);
select * from (generate_series(1, 2));
select * from (lateral generate_series(1, 2));
select * from (rows from (generate_series(1, 2)));
select * from (only (foo));
