ALTER /* policy */ POLICY /* name */ account_access_policy /* on */ ON /* table */ public.accounts /* to */ TO /* first role */ account_administrator, /* second role */ account_auditor /* using */ USING /* left paren */ (/* expression */ owner_identifier = current_user /* right paren */) /* with */ WITH /* check */ CHECK /* left paren */ (/* expression */ account_is_active AND account_identifier > 1000000 /* right paren */) /* end */;

ALTER POLICY account_access_policy ON public.accounts USING (account_identifier = a_very_long_session_account_identifier_function(current_user, current_role));

ALTER POLICY account_access_policy ON public.accounts /* rename */ RENAME /* to */ TO /* new name */ archived_account_access_policy;
