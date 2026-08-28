ALTER TABLESPACE application_data OWNER TO storage_administrator;

ALTER TABLESPACE application_data RENAME TO application_data_for_archived_customer_records;

ALTER TABLESPACE application_data SET (seq_page_cost = 1.0, random_page_cost = 1.1, effective_io_concurrency = 200);

ALTER TABLESPACE application_data RESET (seq_page_cost, random_page_cost, effective_io_concurrency);

ALTER /* tablespace */ TABLESPACE /* name */ commented_application_data /* action */ OWNER /* to */ TO /* owner */ storage_administrator_with_a_very_long_name /* semicolon */;

ALTER /* tablespace */ TABLESPACE /* name */ commented_options /* action */ SET /* left paren */ (/* option */ seq_page_cost /* equals */ = /* value */ 1.0, /* second option */ random_page_cost /* second equals */ = /* second value */ 1.1 /* right paren */) /* semicolon */;

ALTER /* tablespace */ TABLESPACE /* name */ commented_reset /* action */ RESET /* left paren */ (/* option */ seq_page_cost, /* second option */ random_page_cost /* right paren */) /* semicolon */;

ALTER /* tablespace */ TABLESPACE /* old name */ commented_old_name /* rename */ RENAME /* to */ TO /* new name */ commented_new_name /* semicolon */;
