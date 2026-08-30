create tablespace fastspace location '/data/fastspace';

create tablespace extraordinarily_long_tablespace_name_for_historical_analytics owner extraordinarily_long_database_owner_role location '/srv/postgresql/extraordinarily/long/location/for/historical/analytics/tablespace' with (random_page_cost = 1.1, seq_page_cost = 0.9, effective_io_concurrency = 200);

-- comments in every position
create /* tablespace */ tablespace /* name */ reporting_space /* owner */ owner /* role */ reporting_owner /* location */ location /* path */ '/srv/reporting' /* with */ with /* open */ (/* first name */ random_page_cost /* equals */ = /* first value */ 1.2 /* comma */, /* second name */ seq_page_cost /* second equals */ = /* second value */ 1.0 /* close */) /* end */;
