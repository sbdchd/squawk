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

-- from only
create transform for only_from language l (
  from sql with function t
);

-- to only
create transform for only_to language l (
  to sql with function g
);

-- reverse order
create transform for reverse_order language l (
  to sql with function g,
  from sql with function t
);

