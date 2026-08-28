ALTER /* foreign */ FOREIGN /* table */ TABLE /* if */ IF /* not */ EXISTS /* table name */ public.very_long_foreign_table_name ADD /* column */ COLUMN /* if */ IF /* not */ NOT /* exists */ EXISTS /* column name */ column_with_a_very_long_descriptive_name /* type */ varchar(255) /* options */ OPTIONS (/* key */ remote_name /* value */ 'remote_column'), /* drop */ DROP /* column */ COLUMN /* if */ IF /* exists */ EXISTS /* name */ obsolete_column /* behavior */ CASCADE;

ALTER FOREIGN TABLE public.items ALTER COLUMN quantity SET DEFAULT 100, ALTER COLUMN description TYPE text USING description::text;

ALTER FOREIGN TABLE public.items ADD CONSTRAINT positive_quantity CHECK (quantity > 0), RENAME COLUMN old_name TO new_name;

ALTER FOREIGN TABLE IF EXISTS public.items OWNER TO role_with_a_very_long_descriptive_name;
