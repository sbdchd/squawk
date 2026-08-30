SET search_path TO myschema, public;

set session search_path = public;

set local work_mem to '64MB';

set foo from current;

set foo = default;

set foo to off;

SET search_path = myschema, public;

SET /* before list parameter */ search_path /* before list equals */ = /* before first value */ myschema /* before list comma */, /* before second value */ public /* before list semicolon */;

set foo to a, 10.0, 1, 'foo', true, false;

set schema 'my_schema';

set catalog 'my_database';

set xml option document;

set xml option content;

set time zone 'America/Los_Angeles';

set time zone default;

set time zone local;

SET LOCAL TIME ZONE -8;

SET TIME ZONE INTERVAL '-08:00' HOUR TO MINUTE;

SET TIME ZONE INTERVAL(2) '-08:00';

SET extra_float_digits = -1;

SET /* before negative config parameter */ extra_float_digits /* before negative config equals */ = /* before config minus */ - /* before config number */ 1 /* before negative config semicolon */;

SET /* before negative scope */ LOCAL /* before negative time */ TIME /* before negative zone */ ZONE /* before minus */ - /* before timezone number */ 8 /* before negative timezone semicolon */;

set an_intentionally_long_config_namespace.an_intentionally_long_config_group.an_intentionally_long_parameter_name to an_intentionally_long_value_name, another_intentionally_long_value_name;

/* before set */ SET /* before scope */ LOCAL /* before parameter */ custom /* before dot */ . /* after dot */ parameter /* before equals */ = /* before first value */ first_value /* before comma */, /* after comma */ 'second value' /* before semicolon */;

SET /* before time */ TIME /* before zone */ ZONE /* before timezone value */ DEFAULT /* before timezone semicolon */;

SET /* before xml */ XML /* before option */ OPTION /* before document */ DOCUMENT /* before xml semicolon */;
