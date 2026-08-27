DEALLOCATE ALL;

DEALLOCATE PREPARE ALL;

deallocate statement_name;

deallocate prepare statement_name;

deallocate "Case-Sensitive Statement";

deallocate prepare prepared_statement_with_an_intentionally_long_name_that_makes_the_statement_longer_than_eighty_characters;

/* before deallocate */ DEALLOCATE /* before prepare */ PREPARE /* before target */ statement_name /* before semicolon */;

DEALLOCATE /* before all */ ALL /* before all semicolon */;
