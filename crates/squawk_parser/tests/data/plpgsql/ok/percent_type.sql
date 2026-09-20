do $$
declare
  v int;
  a v%type;
  b v%type[];
  c v%type[1][3];
  d v%type array;
  e v%type array[1];
  f foo%rowtype;
  g notice%rowtype;
  h foo.bar.baz%rowtype;
  i pg_catalog.pg_class%rowtype[];
  j table%rowtype;
  k select%type;
  l $1%type;
  m U&"int4" UESCAPE '!';
begin
  null;
end
$$;
