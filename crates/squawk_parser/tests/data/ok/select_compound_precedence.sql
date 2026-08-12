-- INTERSECT has stronger precedence than UNION
select 1 intersect select 2 union select 3;
-- so this is:
-- (select 1 intersect select 2) union select 3;

select 1 union select 2 intersect select 3;
-- and this is:
-- select 1 union (select 2 intersect select 3);

-- UNION and EXCEPT are left associative
select 1 except select 2 union select 3;
select 1 union select 2 except select 3;

-- parens override precedence and associativity
select 1 intersect (select 2 union select 3);
select 1 union (select 2 union select 3);

-- trailing clauses belong to the outermost compound select
select 1 union select 2 intersect select 3 union select 4 order by 1 limit 2;
