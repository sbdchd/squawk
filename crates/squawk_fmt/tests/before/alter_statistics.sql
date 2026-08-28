ALTER STATISTICS reporting.user_activity_statistics OWNER TO analytics_administrator;

ALTER STATISTICS reporting.user_activity_statistics RENAME TO user_activity_statistics_for_archived_reporting;

ALTER STATISTICS reporting.user_activity_statistics SET SCHEMA reporting_archive;

ALTER STATISTICS reporting.user_activity_statistics SET STATISTICS 1000;

ALTER STATISTICS reporting.user_activity_statistics SET STATISTICS DEFAULT;

ALTER /* statistics */ STATISTICS /* name */ reporting.commented_statistics /* action */ RENAME /* to */ TO /* new name */ reporting.renamed_commented_statistics /* semicolon */;

ALTER STATISTICS reporting.statistics_with_a_very_long_and_descriptive_name SET STATISTICS /* expression */ 10000;
