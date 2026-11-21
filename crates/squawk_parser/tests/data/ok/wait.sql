-- wait for lsn
wait for lsn '0/306EE20';

-- options
wait for lsn '0/306EE20' with (timeout '1d');
wait for lsn '0/306EE20' with (mode 'standby_write', timeout '1d');
wait for lsn '0/306EE20' with (mode 'primary_flush');
wait for lsn '0/306EE20' with (timeout '0.1s', no_throw);
wait for lsn '0/306EE20' with (timeout 100);
