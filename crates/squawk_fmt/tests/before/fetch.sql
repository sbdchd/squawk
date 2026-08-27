FETCH NEXT FROM cursor_name;

fetch prior in cursor_name;

fetch first from cursor_name;

fetch last from cursor_name;

fetch absolute 10 from cursor_name;

fetch relative -3 from cursor_name;

fetch 10 from cursor_name;

fetch all from cursor_name;

fetch forward from cursor_name;

fetch forward 10 in cursor_name;

fetch forward all from cursor_name;

fetch backward from cursor_name;

fetch backward 10 from cursor_name;

fetch backward all from cursor_name;

fetch prior cursor_name;

fetch next from cursor_with_an_intentionally_long_name_that_makes_this_fetch_statement_longer_than_eighty_characters;

/* before fetch */ FETCH /* before action */ FORWARD /* before all */ ALL /* before from */ FROM /* before cursor */ cursor_name /* before semicolon */;

FETCH /* before absolute */ ABSOLUTE /* before count */ 10 /* before in */ IN /* before second cursor */ cursor_name /* before second semicolon */;
