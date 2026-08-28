ALTER /* operator */ OPERATOR /* signature */ public.+ /* left paren */ (/* left type */ integer /* comma */, /* right type */ integer /* right paren */) /* owner */ OWNER /* to */ TO /* role */ application_owner /* end */;

ALTER OPERATOR public.## (NONE, integer) SET (RESTRICT = schema_a.restrict_function_with_a_very_long_name, JOIN = schema_a.join_function_with_a_very_long_name, HASHES, MERGES);

ALTER OPERATOR public.+ (integer, integer) /* set */ SET /* schema */ SCHEMA /* name */ archive;
