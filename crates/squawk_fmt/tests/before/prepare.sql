PREPARE statement_name AS SELECT 1;

prepare statement_name(int, text) as insert into t values ($1, $2);

prepare statement_name as update t set value = 1 where id = $1;

prepare statement_name as delete from t where id = $1;

prepare statement_name as values (1, 'one'), (2, 'two');

prepare prepared_statement_with_an_intentionally_long_name(integer, character varying, timestamp with time zone, double precision) as select an_intentionally_long_column_name from an_intentionally_long_table_name;

/* before prepare */ PREPARE /* before name */ statement_name /* before left paren */ (/* after left paren */ INT /* before comma */, /* after comma */ TEXT /* before right paren */) /* before as */ AS /* before statement */ SELECT /* before value */ $1 /* before semicolon */;

prepare p1 (int) as (select $1);
prepare p1 as (select 1);
prepare p1 as ((select 1));
prepare p1 as (table t);
prepare p1 as (values (1));
