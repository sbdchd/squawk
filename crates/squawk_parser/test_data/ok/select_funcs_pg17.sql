
-- json_table
SELECT jt.* FROM
 my_films,
 JSON_TABLE (js, '$.favorites[*]' COLUMNS (
   id FOR ORDINALITY,
   kind text PATH '$.kind',
   title text PATH '$.films[*].title' WITH WRAPPER,
   director text PATH '$.films[*].director' WITH WRAPPER)) AS jt;

-- json_table_nested_path
SELECT jt.* FROM
 my_films,
 JSON_TABLE ( js, '$.favorites[*] ? (@.films[*].director == $filter)'
   PASSING 'Alfred Hitchcock' AS filter
   COLUMNS (
    id FOR ORDINALITY,
    kind text PATH '$.kind',
    NESTED PATH '$.films[*]' COLUMNS (
      title text FORMAT JSON PATH '$.title' OMIT QUOTES,
      director text PATH '$.director' KEEP QUOTES))) AS jt;

-- json_table_without_filter
SELECT jt.* FROM
 my_films,
 JSON_TABLE ( js, '$.favorites[*]'
   COLUMNS (
    id FOR ORDINALITY,
    kind text PATH '$.kind',
    NESTED PATH '$.films[*]' COLUMNS (
      title text FORMAT JSON PATH '$.title' OMIT QUOTES,
      director text PATH '$.director' KEEP QUOTES))) AS jt;

-- json_table_union
SELECT * FROM JSON_TABLE (
'{"favorites":
    {"movies":
      [{"name": "One", "director": "John Doe"},
       {"name": "Two", "director": "Don Joe"}],
     "books":
      [{"name": "Mystery", "authors": [{"name": "Brown Dan"}]},
       {"name": "Wonder", "authors": [{"name": "Jun Murakami"}, {"name":"Craig Doe"}]}]
}}'::json, '$.favorites[*]'
COLUMNS (
  user_id FOR ORDINALITY,
  NESTED '$.movies[*]'
    COLUMNS (
    movie_id FOR ORDINALITY,
    mname text PATH '$.name',
    director text),
  NESTED '$.books[*]'
    COLUMNS (
      book_id FOR ORDINALITY,
      bname text PATH '$.name',
      NESTED '$.authors[*]'
        COLUMNS (
          author_id FOR ORDINALITY,
          author_name text PATH '$.name'))));

-- json
select json(a format json);
select json(a format json with unique keys);
select json(a format json with unique);
select json(a format json without unique keys);
select json(a format json without unique);

-- json_serialize
select json_serialize(a);
select json_serialize(a format json);
select json_serialize(a returning foo);
select json_serialize(a format json returning foo format json);

-- json_query
select json_query(a, b with wrapper);
select json_query(
  a format json, 
  b passing c as foo, 
  d as bar 
  returning text 
  format json 
  with unconditional array wrapper
  keep quotes on scalar string
  unknown on empty
  empty array on error
);


-- json_exists
select json_exists(a, b);
select json_exists(
  a format json,
  b passing c as d
  default z on error
);

-- doc examples
select JSON_EXISTS(jsonb '{"key1": [1,2,3]}', 'strict $.key1[*] ? (@ > $x)' PASSING 2 AS x);
select JSON_EXISTS(jsonb '{"a": [1,2,3]}', 'lax $.a[5]' ERROR ON ERROR);
select JSON_EXISTS(jsonb '{"a": [1,2,3]}', 'strict $.a[5]' ERROR ON ERROR);

-- json_value
select json_value(a, b);
select json_value(
  a format json,
  b passing c as d
  returning foo format json
  unknown on empty
  default z on error
);

