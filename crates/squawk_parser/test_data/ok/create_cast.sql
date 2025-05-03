-- simple
create cast (t as u)
  without function;

-- inout
create cast (t as u)
  with inout
  as implicit;

-- full
create cast (foo.t as bar.u)
  with function foo.bar(in a text, out b bigint, text)
  as assignment;

