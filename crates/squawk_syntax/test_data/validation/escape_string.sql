-- ok
select e'a\U00000062';
select e'\\u00';
select e'no escapes here';

-- errors
select E'\u00';
select E'\UFFFF';
select E'hello \UFGFF world';
