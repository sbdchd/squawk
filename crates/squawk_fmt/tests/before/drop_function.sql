drop function public.calculate_total(integer, numeric);

drop function if exists extraordinarily_long_schema_name.extraordinarily_long_function_name(extraordinarily_long_schema_name.extraordinarily_long_argument_type_name, double precision), public.another_extraordinarily_long_function_name(integer) cascade;

-- comments in every position
drop /* function */ function /* if */ if /* exists */ exists /* first function */ public /* dot */ . calculate_total /* open */ (/* first argument */ integer /* argument comma */, /* second argument */ numeric /* close */) /* function comma */, /* second function */ reporting.refresh_cache(/* second function argument */ text) /* behavior */ restrict /* end */;
