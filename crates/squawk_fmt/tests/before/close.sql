CLOSE ALL;

close active_cursor;

close "Case-Sensitive Cursor";

close cursor_with_an_intentionally_long_name_that_makes_this_close_statement_longer_than_eighty_characters;

/* before */ CLOSE /* after close */ ALL /* before semicolon */;

CLOSE /* before cursor */ cursor_name /* before cursor semicolon */;
