ALTER /* operator */ OPERATOR /* family */ FAMILY /* name */ public.integer_family /* using */ USING /* method */ btree /* add */ ADD /* operator option */ OPERATOR /* strategy */ 1 /* op */ < /* left paren */ (/* lhs */ integer /* comma */, /* rhs */ integer /* right paren */) /* for */ FOR /* order */ ORDER /* by */ BY /* family */ public.sort_family /* option comma */, /* function option */ FUNCTION /* support */ 1 /* argument types */ (integer, integer) /* function */ public.compare_integer_values(integer, integer) /* option comma */, /* storage */ STORAGE /* type */ bigint /* end */;

ALTER OPERATOR FAMILY public.integer_family USING btree /* drop */ DROP /* operator */ OPERATOR /* strategy */ 1 /* params */ (integer, integer) /* comma */, /* function */ FUNCTION /* support */ 1 /* params */ (integer, integer);

ALTER OPERATOR FAMILY public.integer_family USING btree RENAME TO integer_family_for_archived_measurements;

ALTER OPERATOR FAMILY public.integer_family USING btree OWNER TO application_owner;

ALTER OPERATOR FAMILY public.integer_family USING btree SET SCHEMA archive;
