update t set (j k) = (1, 2);
--             ^ missing comma

update t set a = 1 b = 2;
--                ^ missing comma

-- trailing SET commas in nested statements must not consume the outer clause
copy (update t set v = 1,) to stdout;

with q as (update t set v = 1,) select * from q;

merge into t using s on t.id = s.id
when matched then update set v = s.v,
when not matched then insert values (s.id, s.v);
