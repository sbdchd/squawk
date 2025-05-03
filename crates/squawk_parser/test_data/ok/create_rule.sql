-- simple
create rule r as on select
  to t
  do nothing;

-- full
create or replace rule r as on select
  to foo.t
  where new.foo = old.foo or old.id != new.id
  do (
    select 1;
    insert into t values (1, 2);
    delete from t;
    values (1, 2);
    update t set foo = 1;
    notify f;
  );

-- doc_1
CREATE RULE "_RETURN" AS
    ON SELECT TO t1
    DO INSTEAD
        SELECT * FROM t2;

CREATE RULE "_RETURN" AS
    ON SELECT TO t2
    DO INSTEAD
        SELECT * FROM t1;

SELECT * FROM t1;

-- doc_2
CREATE RULE notify_me AS ON UPDATE TO mytable DO ALSO NOTIFY mytable;

UPDATE mytable SET name = 'foo' WHERE id = 42;

