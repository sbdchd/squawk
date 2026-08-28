ALTER /* foreign */ FOREIGN /* data */ DATA /* wrapper */ WRAPPER /* name */ fdw /* options */ OPTIONS /* left */ (/* add */ ADD /* key */ host /* value */ 'localhost', /* set */ SET /* key */ port /* value */ '5432', /* drop */ DROP /* key */ obsolete /* right */);

ALTER FOREIGN DATA WRAPPER very_long_foreign_data_wrapper_name HANDLER public.very_long_handler_function_name;

ALTER FOREIGN DATA WRAPPER fdw NO VALIDATOR;

ALTER FOREIGN DATA WRAPPER fdw RENAME /* to */ TO /* name */ renamed_fdw;

ALTER FOREIGN DATA WRAPPER fdw OWNER /* to */ TO /* role */ current_user;
