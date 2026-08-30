ALTER /* text */ TEXT /* search */ SEARCH /* template */ TEMPLATE /* name */ public.default_template /* rename */ RENAME /* to */ TO /* new name */ template_for_archived_documents_and_historical_search_results /* end */;

ALTER TEXT SEARCH TEMPLATE public.default_template SET /* schema */ SCHEMA /* schema name */ archive;
