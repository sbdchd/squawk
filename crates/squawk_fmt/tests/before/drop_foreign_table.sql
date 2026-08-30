drop foreign table public.events;

drop foreign table if exists extraordinarily_long_schema_name.extraordinarily_long_foreign_table_name, another_extraordinarily_long_schema_name.another_extraordinarily_long_foreign_table_name cascade;

-- comments in every position
drop /* foreign */ foreign /* table */ table /* if */ if /* exists */ exists /* first table */ public /* dot */ . events /* before comma */, /* second table */ reporting.archive /* behavior */ restrict /* end */;
