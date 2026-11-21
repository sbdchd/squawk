WAIT FOR LSN '0/306EE20';

wait   for   lsn   '0/306EE20'   with   ( timeout '1d' );

WAIT FOR LSN '0/306EE20' WITH (MODE 'standby_write', TIMEOUT '1d');

wait for lsn '0/306EE20' with (timeout '0.1s', no_throw);

wait for lsn '0/306EE20' with (timeout 100);

wait for lsn 'FFFFFFFF/FFFFFFFF' with (mode 'standby_write', timeout '10min', no_throw);

/* before wait */ WAIT /* before for */ FOR /* before lsn */ LSN /* before literal */ '0/306EE20' /* before with */ WITH /* before list */ (/* before option */ timeout /* before value */ '1d' /* after value */) /* before semicolon */;
