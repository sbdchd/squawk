-- ok
select b'01';
select B'1010';
select b'';

-- errors
select b'012';
select b'01A';
select b'0 1';
select B'2345';
