create domain public.email_address as varchar(255) collate "C" default 'unknown@example.com' constraint email_address_valid check (value like '%@%') not null;

create domain extraordinarily_long_schema_name.extraordinarily_long_domain_name as character varying(1024) collate extraordinarily_long_schema_name.extraordinarily_long_collation_name default 'an extraordinarily long default value that forces wrapping' check (length(value) > 3) not null;

/* before create */ create /* before domain */ domain /* before name */ public.commented_domain /* before as */ as /* before type */ text /* before collate */ collate /* before collation */ "C" /* before constraint */ constraint /* before constraint name */ valid_value /* before check */ check /* before left paren */ (/* before expression */ value <> '' /* before right paren */) /* before not */ not /* before null */ null /* before semicolon */;
