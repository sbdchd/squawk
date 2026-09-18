do $$
declare
  call int := 1;
begin
  call transaction_test1();
  call p1(1, 2);
  call p2(a => 1, b := 2);
  call := 2;
end
$$;
