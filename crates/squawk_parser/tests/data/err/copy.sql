-- missing a couple commas
copy x (i y) from '/tmp/input.file' (on_error ignore  log_verbosity verbose);

-- legacy force option requires a target
copy t from 's' with force;
copy t from 's' with force null;
copy t from 's' with force zzz;
