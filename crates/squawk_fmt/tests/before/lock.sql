LOCK t;

lock table t, only b, c *;

lock t in access share mode;

lock t in row share mode;

lock t in row exclusive mode;

lock t in share update exclusive mode;

lock t in share mode;

lock t in share row exclusive mode;

lock t in exclusive mode;

lock t in access exclusive mode;

lock table t, a *, only c in row exclusive mode nowait;

lock table an_intentionally_long_schema_name.an_intentionally_long_table_name, another_intentionally_long_schema_name.another_intentionally_long_table_name in access exclusive mode nowait;

/* before lock */ LOCK /* before table */ TABLE /* before first relation */ ONLY /* before first name */ public /* before dot */ . /* after dot */ records /* before comma */, /* after comma */ archived_records /* before in */ IN /* before access */ ACCESS /* before exclusive */ EXCLUSIVE /* before mode */ MODE /* before nowait */ NOWAIT /* before semicolon */;
