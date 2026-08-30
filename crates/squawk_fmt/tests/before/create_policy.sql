create policy account_access on public.accounts;

create policy extraordinarily_long_account_access_policy_name on extraordinarily_long_schema_name.extraordinarily_long_accounts_table_name as restrictive for select to extraordinarily_long_account_administrator_role, extraordinarily_long_account_auditor_role using (account_owner_identifier = current_user and account_identifier > 1000000) with check (account_is_active and account_owner_identifier = current_user);

-- comments in every position
create /* policy */ policy /* name */ account_access /* on */ on /* schema */ public /* dot */ . /* table */ accounts /* as */ as /* policy type */ restrictive /* for */ for /* command */ select /* to */ to /* first role */ account_admin /* comma */, /* second role */ account_auditor /* using */ using /* left paren */ (/* expression */ owner_name = current_user /* right paren */) /* with */ with /* check */ check /* check left paren */ (/* check expression */ active /* check right paren */) /* end */;
