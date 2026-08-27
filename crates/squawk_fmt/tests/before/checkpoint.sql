CHECKPOINT;

checkpoint (mode fast);

checkpoint (mode spread, flush_unlogged true);

checkpoint (flush_unlogged false);

checkpoint (flush_unlogged);

checkpoint (an_intentionally_long_checkpoint_option_name an_intentionally_long_value_name, another_intentionally_long_checkpoint_option_name another_intentionally_long_value_name);

/* before checkpoint */ CHECKPOINT /* before left paren */ (/* after left paren */ MODE /* before value */ FAST /* before comma */, /* after comma */ FLUSH_UNLOGGED /* before second value */ FALSE /* before right paren */) /* before semicolon */;
