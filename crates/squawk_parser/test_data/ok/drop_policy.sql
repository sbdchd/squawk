-- simple
drop policy s on t;

-- full
drop policy if exists r on foo.t_name cascade;
drop policy if exists r on a restrict;

