-- rename
alter index i rename to j;
alter index if exists s.i rename to j;

-- set_tablespace
alter index i set tablespace t;
alter index if exists s.i set tablespace t;

-- set_params
alter index i set (a);
alter index if exists s.i set (a=1, b='v');

-- reset_params
alter index i reset (a);
alter index if exists s.i reset (a, b);

-- attach
alter index i attach partition p;
alter index s.i attach partition s.p;

-- depends
alter index i no depends on extension e;
alter index s.i depends on extension e;

-- alter_column
alter index i alter 1 set statistics 100;
alter index if exists s.i alter column 1 set statistics 100;

-- all_tablespace
alter index all in tablespace t 
  set tablespace n;
alter index all in tablespace t 
  owned by r, s
  set tablespace n
  nowait;

