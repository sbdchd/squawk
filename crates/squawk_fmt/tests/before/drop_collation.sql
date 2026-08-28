drop collation public.english;

drop collation if exists extraordinarily_long_schema_name.extraordinarily_long_collation_name, another_extraordinarily_long_schema_name.another_extraordinarily_long_collation_name cascade;

-- comments in every position
drop /* collation */ collation /* if */ if /* exists */ exists /* first collation */ public.english /* before comma */, /* second collation */ public.french /* behavior */ restrict /* end */;
