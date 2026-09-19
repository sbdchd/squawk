do $$
declare
  a int;
  b constant text := 'x';
  c numeric(10, 2) default 0;
  d text collate "C" not null = 'y';
  e some_table.some_col%type;
begin
  null;
end
$$;
