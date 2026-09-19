do $$
declare
  assert int := 1;
begin
  assert true;
  assert 1 = 1, 'boom';
  assert (select count(*) from onecol) > 0, 'empty: ' || 'x';
  assert := 2;
end
$$;
