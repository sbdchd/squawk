-- ok
select x'1F';
select X'deadBEEF';
select x'';

-- errors
select x'1FZ';
select x'1G';
select x'1G2H';
select X'GHIJ';
