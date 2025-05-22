-- insert
PREPARE fooplan (int, text, bool, numeric) AS
    INSERT INTO foo VALUES($1, $2, $3, $4);



-- select
PREPARE usrrptplan (int) AS
    SELECT * FROM users u, logs l WHERE u.usrid=$1 AND u.usrid=l.usrid
    AND l.date = $2;

PREPARE foo AS
    with t as (select 1)
    select * from t;

-- update
prepare foo as
  update foo set x = 1 where x > 10;

-- delete
prepare foo as
  delete from foo where x = 1;

-- merge
prepare foo as
  merge into t1
  using t2 on t2.id = t1.id
  when matched then do nothing
  when not matched by source then do nothing
  when not matched then do nothing;

-- values
prepare foo as
  values (1, 'one'), (2, 'two');

