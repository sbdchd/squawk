# parser

> Adapted from [rust-analyzer](https://github.com/rust-lang/rust-analyzer)

via: https://www.postgresql.org/docs/17/sql-commands.html

| cmd                              | supported? | description                                                                            |
| -------------------------------- | ---------- | -------------------------------------------------------------------------------------- |
| ABORT                            | y          | abort the current transaction                                                          |
| ALTER AGGREGATE                  | y          | change the definition of an aggregate function                                         |
| ALTER COLLATION                  | y          | change the definition of a collation                                                   |
| ALTER CONVERSION                 | y          | change the definition of a conversion                                                  |
| ALTER DATABASE                   | y          | change a database                                                                      |
| ALTER DEFAULT PRIVILEGES         | y          | define default access privileges                                                       |
| ALTER DOMAIN                     | y          | change the definition of a domain                                                      |
| ALTER EVENT TRIGGER              | y          | change the definition of an event trigger                                              |
| ALTER EXTENSION                  | y          | change the definition of an extension                                                  |
| ALTER FOREIGN DATA WRAPPER       | y          | change the definition of a foreign-data wrapper                                        |
| ALTER FOREIGN TABLE              | y          | change the definition of a foreign table                                               |
| ALTER FUNCTION                   | y          | change the definition of a function                                                    |
| ALTER GROUP                      | y          | change role name or membership                                                         |
| ALTER INDEX                      | y          | change the definition of an index                                                      |
| ALTER LANGUAGE                   | y          | change the definition of a procedural language                                         |
| ALTER LARGE OBJECT               | y          | change the definition of a large object                                                |
| ALTER MATERIALIZED VIEW          | y          | change the definition of a materialized view                                           |
| ALTER OPERATOR                   | y          | change the definition of an operator                                                   |
| ALTER OPERATOR CLASS             | y          | change the definition of an operator class                                             |
| ALTER OPERATOR FAMILY            | y          | change the definition of an operator family                                            |
| ALTER POLICY                     | y          | change the definition of a row-level security policy                                   |
| ALTER PROCEDURE                  | y          | change the definition of a procedure                                                   |
| ALTER PUBLICATION                | y          | change the definition of a publication                                                 |
| ALTER ROLE                       | y          | change a database role                                                                 |
| ALTER ROUTINE                    | y          | change the definition of a routine                                                     |
| ALTER RULE                       | y          | change the definition of a rule                                                        |
| ALTER SCHEMA                     | y          | change the definition of a schema                                                      |
| ALTER SEQUENCE                   | y          | change the definition of a sequence generator                                          |
| ALTER SERVER                     | y          | change the definition of a foreign server                                              |
| ALTER STATISTICS                 | y          | change the definition of an extended statistics object                                 |
| ALTER SUBSCRIPTION               | y          | change the definition of a subscription                                                |
| ALTER SYSTEM                     | y          | change a server configuration parameter                                                |
| ALTER TABLE                      | y          | change the definition of a table                                                       |
| ALTER TABLESPACE                 | y          | change the definition of a tablespace                                                  |
| ALTER TEXT SEARCH CONFIGURATION  | y          | change the definition of a text search configuration                                   |
| ALTER TEXT SEARCH DICTIONARY     | y          | change the definition of a text search dictionary                                      |
| ALTER TEXT SEARCH PARSER         | y          | change the definition of a text search parser                                          |
| ALTER TEXT SEARCH TEMPLATE       | y          | change the definition of a text search template                                        |
| ALTER TRIGGER                    | y          | change the definition of a trigger                                                     |
| ALTER TYPE                       | y          | change the definition of a type                                                        |
| ALTER USER                       | y          | change a database role                                                                 |
| ALTER USER MAPPING               | y          | change the definition of a user mapping                                                |
| ALTER VIEW                       | y          | change the definition of a view                                                        |
| ANALYZE                          | y          | collect statistics about a database                                                    |
| BEGIN                            | y          | start a transaction block                                                              |
| CALL                             | y          | invoke a procedure                                                                     |
| CHECKPOINT                       | y          | force a write-ahead log checkpoint                                                     |
| CLOSE                            | y          | close a cursor                                                                         |
| CLUSTER                          | y          | cluster a table according to an index                                                  |
| COMMENT                          | y          | define or change the comment of an object                                              |
| COMMIT                           | y          | commit the current transaction                                                         |
| COMMIT PREPARED                  | y          | commit a transaction that was earlier prepared for two-phase commit                    |
| COPY                             | y          | copy data between a file and a table                                                   |
| CREATE ACCESS METHOD             | y          | define a new access method                                                             |
| CREATE AGGREGATE                 | y          | define a new aggregate function                                                        |
| CREATE CAST                      | y          | define a new cast                                                                      |
| CREATE COLLATION                 | y          | define a new collation                                                                 |
| CREATE CONVERSION                | y          | define a new encoding conversion                                                       |
| CREATE DATABASE                  | y          | create a new database                                                                  |
| CREATE DOMAIN                    | y          | define a new domain                                                                    |
| CREATE EVENT TRIGGER             | y          | define a new event trigger                                                             |
| CREATE EXTENSION                 | y          | install an extension                                                                   |
| CREATE FOREIGN DATA WRAPPER      | y          | define a new foreign-data wrapper                                                      |
| CREATE FOREIGN TABLE             | y          | define a new foreign table                                                             |
| CREATE FUNCTION                  | y          | define a new function                                                                  |
| CREATE GROUP                     | y          | define a new database role                                                             |
| CREATE INDEX                     | y          | define a new index                                                                     |
| CREATE LANGUAGE                  | y          | define a new procedural language                                                       |
| CREATE MATERIALIZED VIEW         | y          | define a new materialized view                                                         |
| CREATE OPERATOR                  | y          | define a new operator                                                                  |
| CREATE OPERATOR CLASS            | y          | define a new operator class                                                            |
| CREATE OPERATOR FAMILY           | y          | define a new operator family                                                           |
| CREATE POLICY                    | y          | define a new row-level security policy for a table                                     |
| CREATE PROCEDURE                 | y          | define a new procedure                                                                 |
| CREATE PUBLICATION               | y          | define a new publication                                                               |
| CREATE ROLE                      | y          | define a new database role                                                             |
| CREATE RULE                      | y          | define a new rewrite rule                                                              |
| CREATE SCHEMA                    | y          | define a new schema                                                                    |
| CREATE SEQUENCE                  | y          | define a new sequence generator                                                        |
| CREATE SERVER                    | y          | define a new foreign server                                                            |
| CREATE STATISTICS                | y          | define extended statistics                                                             |
| CREATE SUBSCRIPTION              | y          | define a new subscription                                                              |
| CREATE TABLE                     | y          | define a new table                                                                     |
| CREATE TABLE AS                  | y          | define a new table from the results of a query                                         |
| CREATE TABLESPACE                | y          | define a new tablespace                                                                |
| CREATE TEXT SEARCH CONFIGURATION | y          | define a new text search configuration                                                 |
| CREATE TEXT SEARCH DICTIONARY    | y          | define a new text search dictionary                                                    |
| CREATE TEXT SEARCH PARSER        | y          | define a new text search parser                                                        |
| CREATE TEXT SEARCH TEMPLATE      | y          | define a new text search template                                                      |
| CREATE TRANSFORM                 | y          | define a new transform                                                                 |
| CREATE TRIGGER                   | y          | define a new trigger                                                                   |
| CREATE TYPE                      | y          | define a new data type                                                                 |
| CREATE USER                      | y          | define a new database role                                                             |
| CREATE USER MAPPING              | y          | define a new mapping of a user to a foreign server                                     |
| CREATE VIEW                      | y          | define a new view                                                                      |
| DEALLOCATE                       | y          | deallocate a prepared statement                                                        |
| DECLARE                          | y          | define a cursor                                                                        |
| DELETE                           | y          | delete rows of a table                                                                 |
| DISCARD                          | y          | discard session state                                                                  |
| DO                               | y          | execute an anonymous code block                                                        |
| DROP ACCESS METHOD               | y          | remove an access method                                                                |
| DROP AGGREGATE                   | y          | remove an aggregate function                                                           |
| DROP CAST                        | y          | remove a cast                                                                          |
| DROP COLLATION                   | y          | remove a collation                                                                     |
| DROP CONVERSION                  | y          | remove a conversion                                                                    |
| DROP DATABASE                    | y          | remove a database                                                                      |
| DROP DOMAIN                      | y          | remove a domain                                                                        |
| DROP EVENT TRIGGER               | y          | remove an event trigger                                                                |
| DROP EXTENSION                   | y          | remove an extension                                                                    |
| DROP FOREIGN DATA WRAPPER        | y          | remove a foreign-data wrapper                                                          |
| DROP FOREIGN TABLE               | y          | remove a foreign table                                                                 |
| DROP FUNCTION                    | y          | remove a function                                                                      |
| DROP GROUP                       | y          | remove a database role                                                                 |
| DROP INDEX                       | y          | remove an index                                                                        |
| DROP LANGUAGE                    | y          | remove a procedural language                                                           |
| DROP MATERIALIZED VIEW           | y          | remove a materialized view                                                             |
| DROP OPERATOR                    | y          | remove an operator                                                                     |
| DROP OPERATOR CLASS              | y          | remove an operator class                                                               |
| DROP OPERATOR FAMILY             | y          | remove an operator family                                                              |
| DROP OWNED                       | y          | remove database objects owned by a database role                                       |
| DROP POLICY                      | y          | remove a row-level security policy from a table                                        |
| DROP PROCEDURE                   | y          | remove a procedure                                                                     |
| DROP PUBLICATION                 | y          | remove a publication                                                                   |
| DROP ROLE                        | y          | remove a database role                                                                 |
| DROP ROUTINE                     | y          | remove a routine                                                                       |
| DROP RULE                        | y          | remove a rewrite rule                                                                  |
| DROP SCHEMA                      | y          | remove a schema                                                                        |
| DROP SEQUENCE                    | y          | remove a sequence                                                                      |
| DROP SERVER                      | y          | remove a foreign server descriptor                                                     |
| DROP STATISTICS                  | y          | remove extended statistics                                                             |
| DROP SUBSCRIPTION                | y          | remove a subscription                                                                  |
| DROP TABLE                       | y          | remove a table                                                                         |
| DROP TABLESPACE                  | y          | remove a tablespace                                                                    |
| DROP TEXT SEARCH CONFIGURATION   | y          | remove a text search configuration                                                     |
| DROP TEXT SEARCH DICTIONARY      | y          | remove a text search dictionary                                                        |
| DROP TEXT SEARCH PARSER          | y          | remove a text search parser                                                            |
| DROP TEXT SEARCH TEMPLATE        | y          | remove a text search template                                                          |
| DROP TRANSFORM                   | y          | remove a transform                                                                     |
| DROP TRIGGER                     | y          | remove a trigger                                                                       |
| DROP TYPE                        | y          | remove a data type                                                                     |
| DROP USER                        | y          | remove a database role                                                                 |
| DROP USER MAPPING                | y          | remove a user mapping for a foreign server                                             |
| DROP VIEW                        | y          | remove a view                                                                          |
| END                              | y          | commit the current transaction                                                         |
| EXECUTE                          | y          | execute a prepared statement                                                           |
| EXPLAIN                          | y          | show the execution plan of a statement                                                 |
| FETCH                            | y          | retrieve rows from a query using a cursor                                              |
| GRANT                            | y          | define access privileges                                                               |
| IMPORT FOREIGN SCHEMA            | y          | import table definitions from a foreign server                                         |
| INSERT                           | y          | create new rows in a table                                                             |
| LISTEN                           | y          | listen for a notification                                                              |
| LOAD                             | y          | load a shared library file                                                             |
| LOCK                             | y          | lock a table                                                                           |
| MERGE                            | y          | conditionally insert, update, or delete rows of a table                                |
| MOVE                             | y          | position a cursor                                                                      |
| NOTIFY                           | y          | generate a notification                                                                |
| PREPARE                          | y          | prepare a statement for execution                                                      |
| PREPARE TRANSACTION              | y          | prepare the current transaction for two-phase commit                                   |
| REASSIGN OWNED                   | y          | change the ownership of database objects owned by a database role                      |
| REFRESH MATERIALIZED VIEW        | y          | replace the contents of a materialized view                                            |
| REINDEX                          | y          | rebuild indexes                                                                        |
| RELEASE SAVEPOINT                | y          | release a previously defined savepoint                                                 |
| RESET                            | y          | restore the value of a run-time parameter to the default value                         |
| REVOKE                           | y          | remove access privileges                                                               |
| ROLLBACK                         | y          | abort the current transaction                                                          |
| ROLLBACK PREPARED                | y          | cancel a transaction that was earlier prepared for two-phase commit                    |
| ROLLBACK TO SAVEPOINT            | y          | roll back to a savepoint                                                               |
| SAVEPOINT                        | y          | define a new savepoint within the current transaction                                  |
| SECURITY LABEL                   | y          | define or change a security label applied to an object                                 |
| SELECT                           | y          | retrieve rows from a table or view                                                     |
| SELECT INTO                      | y          | define a new table from the results of a query                                         |
| SET                              | y          | change a run-time parameter                                                            |
| SET CONSTRAINTS                  | y          | set constraint check timing for the current transaction                                |
| SET ROLE                         | y          | set the current user identifier of the current session                                 |
| SET SESSION AUTHORIZATION        | y          | set the session user identifier and the current user identifier of the current session |
| SET TRANSACTION                  | y          | set the characteristics of the current transaction                                     |
| SHOW                             | y          | show the value of a run-time parameter                                                 |
| START TRANSACTION                | y          | start a transaction block                                                              |
| TRUNCATE                         | y          | empty a table or set of tables                                                         |
| UNLISTEN                         | y          | stop listening for a notification                                                      |
| UPDATE                           | y          | update rows of a table                                                                 |
| VACUUM                           | y          | garbage-collect and optionally analyze a database                                      |
| VALUES                           | y          | compute a set of rows                                                                  |
