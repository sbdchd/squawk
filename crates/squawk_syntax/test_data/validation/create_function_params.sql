create function invalid_variadic(variadic items int[], following int)
returns void
language sql
as '';

-- Functions can have OUT parameters after a VARIADIC parameter.
create function valid_variadic(variadic items int[], out result int)
returns void
language sql
as '';
