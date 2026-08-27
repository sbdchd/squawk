EXECUTE statement_name;

execute statement_name(1, true, some_value);

execute "Case-Sensitive Statement";

execute statement_name(an_intentionally_long_argument_name, another_intentionally_long_argument_name, a_third_intentionally_long_argument_name);

/* before execute */ EXECUTE /* before statement */ statement_name /* before left paren */ (/* after left paren */ 1 /* before comma */, /* after comma */ TRUE /* before right paren */) /* before semicolon */;
