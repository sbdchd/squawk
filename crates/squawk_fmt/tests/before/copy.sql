COPY foo FROM '/tmp/foo.csv';

copy foo (id, name) to stdout with (format csv, header true, delimiter ',', null '', encoding 'UTF8');

copy foo to stdout ("select" 'x');

copy (select id, a_very_long_column_name, another_very_long_column_name from a_very_long_schema_name.a_very_long_table_name) to program 'gzip > /tmp/a_very_long_output_file_name.csv' with (format csv, header on);

copy binary foo from stdin binary freeze csv header json delimiter as ',' null as '' quote as '"' escape as '\\' encoding 'UTF8' force not null id,name force quote * force null description where id > 0;

/* before */ COPY /* after copy */ BINARY /* after binary */ public /* before dot */ . /* after dot */ records /* before columns */ (/* after left paren */ id /* before comma */, /* after comma */ description /* before right paren */) /* before from */ FROM /* after from */ PROGRAM /* after program */ 'cat /tmp/records' /* before with */ WITH /* before options */ (/* after options left paren */ FORMAT /* before format value */ CSV /* before option comma */, /* after option comma */ HEADER /* before header value */ ON /* before second option comma */, /* after second option comma */ FORCE_NULL /* before nested options */ (/* after nested left paren */ id /* before nested comma */, /* after nested comma */ description /* before nested right paren */) /* before options right paren */) /* before where */ WHERE /* after where */ id > 0 /* before semicolon */;

copy (/* after query left paren */ select /* after select */ id from records /* before query right paren */) to /* before stdout */ stdout;
