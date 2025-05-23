-- pg_grammar
truncate table only t
restart identity
cascade;

-- multiple_tables
TRUNCATE only a, b *, c;

-- rest
truncate t;
truncate a continue identity;
truncate a continue identity restrict;

