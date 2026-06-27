-- PostgreSQL splits trailing + or - from multi-character operators unless
-- the operator contains one of: ~ ! @ # % ^ & | ` ?
select 2*-3;
select 2/-3;
select 2+-3;
select 2<=-3;
select 2=-3;
select 2*+3;
select 2<=+3;
select 2++3;
select 2@-3;
select 2<@-3;
select 2!=-3;
