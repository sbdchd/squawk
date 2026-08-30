drop statistics public.user_activity_stats;

drop statistics if exists extraordinarily_long_schema_name.extraordinarily_long_statistics_name_for_customer_activity, another_extraordinarily_long_schema_name.another_extraordinarily_long_statistics_name cascade;

-- comments in every position
drop /* statistics */ statistics /* if */ if /* exists */ exists /* first statistics */ public /* dot */ . user_stats /* before comma */, /* second statistics */ reporting.activity_stats /* behavior */ restrict /* end */;
