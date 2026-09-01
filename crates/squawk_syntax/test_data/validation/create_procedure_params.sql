create procedure invalid_variadic(variadic items int[], following int)
language sql
as '';

create procedure invalid_variadic_out(variadic items int[], out result int)
language sql
as '';
