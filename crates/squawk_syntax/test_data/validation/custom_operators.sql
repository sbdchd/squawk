-- disallowed prefix operators
select *c;
select /d;
select <e;
select >f;
select =g;
select %l;
select ^m;
-- operators may not exceed NAMEDATALEN-1 (63) characters
select |||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||| 1;
select 1 |||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||| 2;
-- 63 chars is still allowed
select 1 ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||| 2;
