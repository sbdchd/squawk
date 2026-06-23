-- NATURAL with no following join type
merge into t using u natural;
select * from a natural;
select * from a join natural;
