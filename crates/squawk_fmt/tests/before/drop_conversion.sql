drop conversion public.my_conversion;

drop conversion if exists extraordinarily_long_schema_name.extraordinarily_long_character_set_conversion_name cascade;

-- comments in every position
drop /* conversion */ conversion /* if */ if /* exists */ exists /* conversion name */ public.commented_conversion /* behavior */ restrict /* end */;
