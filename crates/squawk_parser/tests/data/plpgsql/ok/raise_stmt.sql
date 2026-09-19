do $$
declare
  raise int := 1;
begin
  raise;
  raise notice 'plain';
  raise notice 'fmt % %', 1, 2;
  raise 'no level';
  raise exception 'boom' using errcode := '22012';
  raise warning using message = 'm', detail = 'd', hint = 'h';
  raise exception using column = 'c', constraint = 'k', datatype = 'd',
    table = 't', schema = 's';
  raise division_by_zero;
  raise sqlstate '22012';
  raise debug 'dbg';
  raise log 'lg';
  raise info 'nf';
  raise notice E'esc';
  raise using errcode = '22012';
  raise := 2;
end
$$;
