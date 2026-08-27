LOAD 'foo';

load '$libdir/extension';

load 'an/intentionally/long/path/to/a/postgresql/shared/library/that/makes/this/load/statement/longer/than/eighty/characters';

/* before load */ LOAD /* before filename */ 'filename' /* before semicolon */;
