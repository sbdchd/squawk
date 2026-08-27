import foreign schema remote from server foreign_server into local;

import foreign schema remote limit to (records, users) from server foreign_server into local options (schema_name 'public');

import foreign schema remote except (ignored_records) from server foreign_server into an_intentionally_long_local_schema_name_that_makes_the_statement_exceed_eighty_characters;

/* before import */ IMPORT /* before foreign */ FOREIGN /* before schema */ SCHEMA /* before remote */ remote /* before limit */ LIMIT /* before to */ TO /* before open */ (/* before table */ records /* before comma */, /* before second */ users /* before close */) /* before from */ FROM /* before server */ SERVER /* before server name */ foreign_server /* before into */ INTO /* before local */ local /* before semicolon */;
