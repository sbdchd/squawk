revoke select, update (payload) on table public.records from app_user;

revoke grant option for all privileges on all tables in schema public, audit from app_user granted by current_user cascade;

revoke admin option for app_reader, app_writer from an_intentionally_long_role_name_that_makes_this_statement_longer_than_eighty_characters restrict;

/* before revoke */ REVOKE /* before grant */ GRANT /* before option */ OPTION /* before for */ FOR /* before select */ SELECT /* before on */ ON /* before table */ TABLE /* before object */ public /* before dot */ . /* after dot */ records /* before from */ FROM /* before role */ app_user /* before granted */ GRANTED /* before by */ BY /* before grantor */ current_user /* before cascade */ CASCADE /* before semicolon */;
