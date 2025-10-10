with t as (
  select 1
), -- <--- extra comma!
select * from t;

-- search depth missing item
with t as (select 1)
search depth first by a, , c set ordercol
select * from t order by ordercol;

-- search depth missing comma
with t as (select 1)
search depth first by a, b c set ordercol
select * from t order by ordercol;

with 
   a as (
      select 1
   ) -- <-- missing a comma
   b as (
      select 3
   )
select 2;

-- table name isn't an plain ident
with 
   a as (
      select 1
   ) -- <-- missing a comma
   row as (select 1)
select 1;


-- extra comma with values (we didn't support values before)
with 
   a as (
      select 1
   ), -- <-- extra comma
values (2);
