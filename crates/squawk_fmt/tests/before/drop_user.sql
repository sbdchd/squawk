drop user alice;

drop user if exists extraordinarily_long_application_reporting_user_name, extraordinarily_long_customer_support_user_name, current_user;

-- comments in every position
drop /* user */ user /* if */ if /* exists */ exists /* first */ alice /* comma */, /* second */ current_user /* end */;
