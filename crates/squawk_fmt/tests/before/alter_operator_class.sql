ALTER /* operator */ OPERATOR /* class */ CLASS /* name */ public.integer_operations /* using */ USING /* method */ btree /* rename */ RENAME /* to */ TO /* new name */ integer_operations_for_archived_measurements /* end */;

ALTER OPERATOR CLASS public.integer_operations USING btree OWNER TO application_owner_with_a_very_long_descriptive_name;

ALTER OPERATOR CLASS public.integer_operations USING btree /* set */ SET /* schema */ SCHEMA /* schema name */ archive;
