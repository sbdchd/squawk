-- ok strings with new lines
select 'foo' 
'bar';
select e'foo'
'bar';
select u&'foo'
'bar';
select b'01'
'10';
select x'1F'
'10';

-- error string
select 'foo' 'bar';
select 'foo' /* comment */ 'bar';
select 'hello' /* comment */ 'bar' /* another comment */ ' world';
select 'hello' -- comment
'world';
select 'foo' 'bar' 'buzz';

-- error escape string
select e'foo' 'bar';
select e'foo' /* comment */ 'bar';
select e'hello' /* comment */ 'bar' /* another comment */ ' world';
select e'hello' -- comment
'world';
select e'foo' 'bar' 'buzz';

-- error unicode escape string
select u'foo' 'bar';
select u'foo' /* comment */ 'bar';
select u'hello' /* comment */ 'bar' /* another comment */ ' world';
select u'hello' -- comment
'world';
select u'foo' 'bar' 'buzz';

-- error bit string
select b'01' '10';
select b'01' /* comment */ '11';
select b'01' /* comment */ '11' /* another comment */ '10';
select b'111' -- comment
'10';
select b'10' '10' '11';

-- error byte string
select x'0F' '10';
select x'01' /* comment */ '1F';
select x'01' /* comment */ '1F' /* another comment */ '10';
select x'1F1' -- comment
'10';
select x'1F' 'F0' '11';
