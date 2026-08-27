DECLARE cursor_name CURSOR FOR SELECT * FROM t;

declare c binary insensitive no scroll cursor without hold for select 1;

declare c binary asensitive scroll cursor with hold for select 2;

declare c cursor for (values (1) union values (2));

declare cursor_with_an_intentionally_long_name binary insensitive no scroll cursor without hold for select an_intentionally_long_column_name, another_intentionally_long_column_name from an_intentionally_long_table_name;

/* before declare */ DECLARE /* before cursor name */ c /* before binary */ BINARY /* before sensitivity */ INSENSITIVE /* before no */ NO /* before scroll */ SCROLL /* before cursor keyword */ CURSOR /* before without */ WITHOUT /* before hold */ HOLD /* before for */ FOR /* before query */ SELECT /* before value */ 1 /* before semicolon */;
