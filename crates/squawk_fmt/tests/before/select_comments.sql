-- intentional new line follows, we should keep that

/* bar */
select/*a*/1/*b*/,/*c*/2/*d*/;

select/*z*/;

select/*a*/*;

select/*a*/all/*b*/1;

select/*a*/distinct/*b*/1;

select 1 /*a*/ /*b*/ group by 1;

select 1 /* before as */ as /* before alias */ number;

select 1 /* before bare alias */ number;

select 1 -- a line comment
, 2;

-- line comments before the semicolon
select 1 -- a
;
select 1 -- b
-- c
;
select 1, 2 -- d
;
select 1 /*e*/ -- f
;
