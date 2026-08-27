refresh materialized view public.summary;

refresh materialized view concurrently public.an_intentionally_long_materialized_view_name_that_makes_this_statement_longer_than_eighty_characters with no data;

/* before refresh */ REFRESH /* before materialized */ MATERIALIZED /* before view */ VIEW /* before concurrently */ CONCURRENTLY /* before name */ public /* before dot */ . /* after dot */ summary /* before with */ WITH /* before no */ NO /* before data */ DATA /* before semicolon */;
