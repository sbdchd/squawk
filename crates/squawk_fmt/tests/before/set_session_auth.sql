set session authorization app_user;

set local session authorization default;

set session session authorization current_role;

set session authorization 'literal role';

set session authorization an_intentionally_long_role_name_that_makes_this_statement_longer_than_eighty_characters;

/* before set */ SET /* before scope */ LOCAL /* before session */ SESSION /* before authorization */ AUTHORIZATION /* before target */ GROUP /* before role name */ legacy_user /* before semicolon */;
