grant select, update (payload) on table public.records, archived_records to app_user;

grant select on public.records to app_user;

grant all privileges on all tables in schema public, audit to app_user with grant option granted by current_user;

grant app_reader, app_writer to app_user with admin option, inherit true granted by current_user;

grant app_reader to app_user with /* before admin */ admin /* before option */ option /* before comma */, /* before inherit */ inherit /* before true */ true;

grant app_reader to app_user with -- before admin
admin option;

grant usage on sequence public.an_intentionally_long_sequence_name_that_makes_this_statement_exceed_eighty_characters to an_intentionally_long_role_name;

grant execute on function /* before first function */ f(integer), /* before second function */ public.g(text) to app_user;

grant execute on procedure /* before procedure */ p(integer) to app_user;

grant execute on routine /* before routine */ r(integer) to app_user;

/* before grant */ GRANT /* before select */ SELECT /* before columns */ (/* before column */ payload /* before close */) /* before on */ ON /* before table */ TABLE /* before object */ public /* before dot */ . /* after dot */ records /* before to */ TO /* before role */ app_user /* before with */ WITH /* before grant option */ GRANT /* before option */ OPTION /* before granted */ GRANTED /* before by */ BY /* before grantor */ current_user /* before semicolon */;
