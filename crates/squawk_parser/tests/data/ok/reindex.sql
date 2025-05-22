-- pg_docs
REINDEX INDEX my_index;

REINDEX TABLE my_table;

REINDEX TABLE CONCURRENTLY my_broken_table;

-- complete_syntax
reindex (concurrently true, tablespace fooo, verbose false) database concurrently foo;

reindex system foo;

reindex index foo;
reindex table foo;
reindex schema foo;

