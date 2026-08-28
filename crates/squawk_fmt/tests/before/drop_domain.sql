drop domain public.email_address;

drop domain if exists extraordinarily_long_schema_name.extraordinarily_long_domain_name, another_extraordinarily_long_schema_name.another_extraordinarily_long_domain_name cascade;

-- comments in every position
drop /* domain */ domain /* if */ if /* exists */ exists /* first domain */ public.email_address /* before comma */, /* second domain */ public.postal_code /* behavior */ restrict /* end */;
