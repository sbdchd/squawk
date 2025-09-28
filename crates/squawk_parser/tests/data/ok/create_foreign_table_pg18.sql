-- like clause is >=pg18
create foreign table u (
  like t
) server s;

-- like clause is >=pg18
create foreign table remote_users (
  like local_users
    including defaults
    including constraints
    excluding generated
    excluding statistics
    excluding all
) server remote_server;

