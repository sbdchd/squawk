-- ok
select n'a';
select n'a'::varchar;
select cast(n'a' as varchar);
select b'01'::varchar;
select x'0a'::varchar;

-- errors
select varchar n'a';
select nchar n'a';
select national character n'a';
select varchar b'01';
select varchar x'0a';
