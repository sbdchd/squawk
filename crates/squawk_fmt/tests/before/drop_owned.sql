drop owned by alice;

drop owned by extraordinarily_long_database_role_name, another_extraordinarily_long_database_role_name, current_user cascade;

-- comments in every position
drop /* owned */ owned /* by */ by /* first role */ alice /* first comma */, /* second role */ current_user /* second comma */, /* group keyword */ group /* group name */ managers /* behavior */ restrict /* end */;
