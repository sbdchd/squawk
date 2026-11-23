ALTER /* foreign */ FOREIGN /* table */ TABLE /* if */ IF /* not */ EXISTS /* table name */ public.very_long_foreign_table_name ADD /* column */ COLUMN /* if */ IF /* not */ NOT /* exists */ EXISTS /* column name */ column_with_a_very_long_descriptive_name /* type */ varchar(255) /* options */ OPTIONS (/* key */ remote_name /* value */ 'remote_column'), /* drop */ DROP /* column */ COLUMN /* if */ IF /* exists */ EXISTS /* name */ obsolete_column /* behavior */ CASCADE;

ALTER FOREIGN TABLE public.items ALTER COLUMN quantity SET DEFAULT 100, ALTER COLUMN description TYPE text USING description::text;

ALTER FOREIGN TABLE public.items ADD CONSTRAINT positive_quantity CHECK (quantity > 0), RENAME COLUMN old_name TO new_name;

ALTER FOREIGN TABLE IF EXISTS public.items OWNER TO role_with_a_very_long_descriptive_name;

ALTER FOREIGN TABLE ft1 /* options */ OPTIONS /* open */ (/* drop */ DROP /* name */ delimiter /* comma */, /* set */ SET /* name */ quote /* value */ '~' /* comma */, /* add */ ADD /* name */ escape /* value */ '@' /* close */);

alter foreign table ft1 alter c options (add p1 'v1');

alter foreign table ft1 alter column c8 options (set p2 'V2', drop p1, add a_long_option_name 'a long option value');
