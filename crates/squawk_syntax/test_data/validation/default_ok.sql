UPDATE t SET col = DEFAULT;

UPDATE t SET (a,b) = (DEFAULT, 1);

INSERT into t VALUES (DEFAULT);

MERGE INTO target t
  USING source s
     ON t.id = s.id
  WHEN NOT MATCHED THEN
      INSERT (id, name, qty) VALUES (s.id, DEFAULT, DEFAULT);

MERGE INTO target t
  USING source s
     ON t.id = s.id
  WHEN MATCHED THEN
      UPDATE SET qty2 = DEFAULT;

UPDATE t SET col = (DEFAULT);

UPDATE t SET col = (((DEFAULT)));

UPDATE t SET (a, b) = ((DEFAULT), 1);

INSERT INTO t VALUES ((DEFAULT));
