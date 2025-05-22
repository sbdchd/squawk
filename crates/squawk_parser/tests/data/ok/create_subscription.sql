-- simple
create subscription s
  connection ''
  publication a;

-- full
create subscription s
  connection 'host=10.0.0.1 port=5432 user=u dbname=d'
  publication a, b, c
  with (
    connect = false,
    create_slot = true,
    enabled = false,
    slot_name = 'bar',
    binary = true,
    copy_data = true,
    synchronous_commit = off,
    two_phase = false,
    disable_on_error = true,
    password_required = false,
    run_as_owner = true,
    origin = 'foo',
    failover = false
  );

