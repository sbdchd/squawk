select 'foo' 'bar';
select 'foo' /* comment */ 'bar';
select 'hello' /* comment */ 'bar' /* another comment */ ' world';

select 'hello' -- comment
'world';

select 'foo' 
'bar';
-- ^ has new line so okay

select 'foo' 'bar' 'buzz';
