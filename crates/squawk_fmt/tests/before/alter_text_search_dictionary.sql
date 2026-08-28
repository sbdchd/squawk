ALTER /* text */ TEXT /* search */ SEARCH /* dictionary */ DICTIONARY /* name */ public.english /* left paren */ (/* first option */ stopwords /* equals */ = /* first value */ english /* comma */, /* second option */ language /* second equals */ = /* second value */ 'english' /* right paren */) /* end */;

ALTER TEXT SEARCH DICTIONARY public.english (stopwords = english, accept = false, long_dictionary_configuration_option_name = 'a very long dictionary configuration value for wrapping');

ALTER TEXT SEARCH DICTIONARY public.english OWNER /* to */ TO /* owner */ application_owner;

ALTER TEXT SEARCH DICTIONARY public.english SET /* schema */ SCHEMA /* schema */ archive;

ALTER TEXT SEARCH DICTIONARY public.english RENAME /* to */ TO /* name */ english_dictionary_for_archived_documents_and_historical_search_results;
