set role app_user;

set local role none;

set session role current_user;

set role group legacy_user;

set role 'literal role';

set session role an_intentionally_long_role_name_that_makes_this_statement_longer_than_eighty_characters;

/* before set */ SET /* before scope */ LOCAL /* before role */ ROLE /* before target */ GROUP /* before role name */ legacy_user /* before semicolon */;
