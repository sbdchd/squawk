show all;

show work_mem;

show custom.an_intentionally_long_config_group.an_intentionally_long_parameter_name_that_exceeds_eighty_characters;

show time zone;

show transaction isolation level;

show session authorization;

/* before show */ SHOW /* before parameter */ custom /* before dot */ . /* after dot */ parameter /* before semicolon */;

SHOW /* before transaction */ TRANSACTION /* before isolation */ ISOLATION /* before level */ LEVEL /* before semicolon */;
