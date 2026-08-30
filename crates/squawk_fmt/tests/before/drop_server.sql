drop server reporting_server;

drop server if exists extraordinarily_long_foreign_server_name_for_customer_reporting_database, another_extraordinarily_long_foreign_server_name_for_historical_analytics cascade;

-- comments in every position
drop /* server */ server /* if */ if /* exists */ exists /* first server */ reporting_server /* comma */, /* second server */ archive_server /* behavior */ restrict /* end */;
