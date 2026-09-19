do $$
begin
  insert into exs_t values (1, 2);
  update exs_t set a = 2 where b = 2;
  delete from exs_t where a = 2;
  truncate exs_t;
  create table exs_u (a int);
  drop table exs_u;
  values (1);
  with x as (select 1 as a) select * from x;
  -- the trailing `;` belongs to the statement here, not to `create function`
  create or replace function exs_f(int) returns int language sql as 'select $1 + 1';
  -- a `;` inside BEGIN ATOMIC doesn't end the statement
  create or replace function exs_g() returns int language sql begin atomic select 1; end;
  -- nor does one inside parens
  create rule exs_r as on insert to exs_t
    do also (insert into exs_t values (3, 3); insert into exs_t values (4, 4));
  -- `abort` is not a PL/pgSQL keyword, so it lands here and postgres refuses it
  -- when it compiles the statement rather than as a syntax error
  abort;
end
$$;
