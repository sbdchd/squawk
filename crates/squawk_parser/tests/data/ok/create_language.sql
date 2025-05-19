-- simple
create language l;

-- full
create or replace trusted procedural language l
  handler foo.bar
  inline f.b
  validator x.y;

