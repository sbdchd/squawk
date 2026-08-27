notify events;

notify events, 'payload';

notify an_intentionally_long_channel_name_that_makes_this_statement_longer_than_eighty_characters, 'an intentionally long payload that also wraps';

/* before notify */ NOTIFY /* before channel */ events /* before comma */, /* before payload */ 'payload' /* before semicolon */;
