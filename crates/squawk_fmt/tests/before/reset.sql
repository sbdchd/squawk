RESET ALL;

reset some_config_param;

reset foo.bar.buzz;

reset time zone;

reset transaction isolation level;

reset an_intentionally_long_config_namespace.an_intentionally_long_config_group.an_intentionally_long_config_parameter_name;

/* before reset */ RESET /* before parameter */ custom /* before first dot */ . /* after first dot */ group_name /* before second dot */ . /* after second dot */ parameter_name /* before semicolon */;

RESET /* before transaction */ TRANSACTION /* before isolation */ ISOLATION /* before level */ LEVEL /* before transaction semicolon */;

RESET /* before time */ TIME /* before zone */ ZONE /* before time semicolon */;
