ALTER /* text */ TEXT /* search */ SEARCH /* parser */ PARSER /* name */ public.default_parser /* rename */ RENAME /* to */ TO /* new name */ parser_for_archived_documents_and_historical_search_results /* end */;

ALTER TEXT SEARCH PARSER public.default_parser SET /* schema */ SCHEMA /* schema name */ archive;
