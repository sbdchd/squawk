create database app;

create database app_copy with owner = app_owner template = template0 encoding = 'UTF8' locale = 'en_US.UTF-8' tablespace = app_space connection limit = 100;

create database extraordinarily_long_database_name with owner = extraordinarily_long_database_owner template = extraordinarily_long_template_database encoding = 'UTF8' strategy = wal_log locale_provider = icu icu_locale = 'en-US-u-va-posix' collation_version = '153.120' tablespace = extraordinarily_long_tablespace_name allow_connections = true connection limit = 250 is_template = false oid = 16384;

create /* database keyword */ database /* database name */ commented_database /* with keyword */ with /* owner option */ owner /* equals */ = /* owner value */ commented_owner /* template option */ template = /* template value */ template0 /* encoding option */ encoding = /* encoding value */ 'UTF8' /* generic option */ locale /* generic equals */ = /* generic value */ 'en_US.UTF-8' /* connection option */ connection /* limit keyword */ limit = /* limit value */ 42 /* tablespace option */ tablespace = /* tablespace value */ commented_space /* semicolon */;
