drop extension hstore;

drop extension if exists extraordinarily_long_extension_name_used_to_verify_statement_line_wrapping, another_extraordinarily_long_extension_name cascade;

-- comments in every position
drop /* extension */ extension /* if */ if /* exists */ exists /* first extension */ hstore /* before comma */, /* second extension */ pg_trgm /* behavior */ restrict /* end */;
