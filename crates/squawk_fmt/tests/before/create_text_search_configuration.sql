create text search configuration public.english (parser = public.default_parser);

create text search configuration extraordinarily_long_schema_name.extraordinarily_long_configuration_name (copy = public.extraordinarily_long_source_configuration_name);

create /* text keyword */ text /* search keyword */ search /* configuration keyword */ configuration /* name */ public /* dot */ . /* name segment */ commented_configuration /* left parenthesis */ (/* first option */ parser /* equals */ = /* value */ public.default_parser /* comma */, /* second option */ copy = /* second value */ public.english /* right parenthesis */) /* semicolon */;
