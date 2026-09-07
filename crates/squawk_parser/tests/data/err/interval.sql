select 1::interval(6) day to second(3);
select interval(6) '1 day' day to second;

-- SECOND precision requires an unsigned integer
select '1'::interval second(1.5);
select '1'::interval second(-1);
select '1'::interval day to second(foo);
select '1'::interval second();
