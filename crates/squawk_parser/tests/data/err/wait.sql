-- missing lsn
wait for lsn;

-- missing lsn keyword
wait for '0/306EE20';

-- missing for
wait lsn '0/306EE20';

-- missing option list
wait for lsn '0/306EE20' with;
