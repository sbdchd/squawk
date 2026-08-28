ALTER /* text */ TEXT /* search */ SEARCH /* configuration */ CONFIGURATION /* name */ public.english /* add */ ADD /* mapping */ MAPPING /* for */ FOR /* kind one */ asciiword /* kind comma */, /* kind two */ asciihword /* with */ WITH /* dictionary one */ public.simple /* dictionary comma */, /* dictionary two */ public.english_stem /* end */;

ALTER TEXT SEARCH CONFIGURATION public.english ALTER /* mapping */ MAPPING /* for */ FOR /* kind */ word /* with */ WITH /* dictionary */ public.simple;

ALTER TEXT SEARCH CONFIGURATION public.english ALTER MAPPING FOR asciiword, word REPLACE /* old dictionary */ public.old_dictionary /* with */ WITH /* new dictionary */ archive.new_dictionary;

ALTER TEXT SEARCH CONFIGURATION public.english ALTER MAPPING /* replace */ REPLACE /* old */ public.old_dictionary /* with */ WITH /* new */ public.new_dictionary;

ALTER TEXT SEARCH CONFIGURATION public.english DROP /* mapping */ MAPPING /* if */ IF /* exists */ EXISTS /* for */ FOR /* kind */ url /* comma */, /* second kind */ host;

ALTER TEXT SEARCH CONFIGURATION public.english OWNER /* to */ TO /* role */ application_owner;

ALTER TEXT SEARCH CONFIGURATION public.english SET /* schema */ SCHEMA /* schema name */ archive;

ALTER TEXT SEARCH CONFIGURATION public.english RENAME /* to */ TO /* new name */ english_configuration_for_archived_documents_and_historical_search_results;
