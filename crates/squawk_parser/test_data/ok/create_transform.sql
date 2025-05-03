-- simple
create transform for t language l (
  from sql with function t,
  to sql with function g
);

-- full
create or replace transform for foo.t(10231) language l (
  from sql with function bar.foo.f(a text),
  to sql with function g
);

