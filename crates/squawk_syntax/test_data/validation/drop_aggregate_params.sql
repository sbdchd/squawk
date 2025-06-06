
-- full
drop aggregate 
  if exists 
    a(*), 
    foo.bar(*), 
    foo.bar(
        in foo integer,
        out bar integer,
        text
    ), 
    c(*)
  cascade;

-- aggregate
drop aggregate a(
  integer,
  text,
  numeric
  order by
    in a timestamp,
    out b numeric,
    text
) restrict;

drop aggregate foo.bar(
  order by
    in a timestamp,
    out b numeric,
    text
);

