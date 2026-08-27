set constraints all deferred;

set constraints first_constraint, public.second_constraint immediate;

set constraints an_intentionally_long_schema_name.an_intentionally_long_constraint_name, another_intentionally_long_constraint_name deferred;

/* before set */ SET /* before constraints */ CONSTRAINTS /* before first name */ first_constraint /* before comma */, /* before second name */ public /* before dot */ . /* after dot */ second_constraint /* before timing */ IMMEDIATE /* before semicolon */;

SET /* before constraints */ CONSTRAINTS /* before all */ ALL /* before deferred */ DEFERRED /* before semicolon */;
