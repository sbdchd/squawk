create user mapping for app_user server app_server;

create user mapping if not exists for extraordinarily_long_application_reporting_user server extraordinarily_long_foreign_data_server options (user 'extraordinarily_long_remote_user_name', password 'an-extraordinarily-long-secret-value');

-- comments in every position
create /* user */ user /* mapping */ mapping /* if */ if /* not */ not /* exists */ exists /* for */ for /* role */ current_user /* server */ server /* server name */ remote_server /* options */ options /* open */ (/* option */ user /* value */ 'remote_user' /* comma */, /* next option */ password /* next value */ 'secret' /* close */) /* end */;
