drop user mapping for app_user server app_server;

drop user mapping for /* special user target */ user /* server */ server app_server;

drop user mapping if exists for extraordinarily_long_application_reporting_user_name server extraordinarily_long_foreign_data_server_name;

-- comments in every position
drop /* user */ user /* mapping */ mapping /* if */ if /* exists */ exists /* for */ for /* role */ current_user /* server */ server /* server name */ remote_server /* end */;
