-- connection
alter subscription s connection 'host=localhost port=5432';

-- set_publication
alter subscription s set publication p;
alter subscription s set publication p, q, r with (a, b = true);

-- add_publication
alter subscription s add publication p;
alter subscription s add publication a, b, c with (a, b = 1);

-- drop_publication
alter subscription s drop publication p;
alter subscription s drop publication a, b, c with (a, b = 1);

-- refresh
alter subscription s refresh publication with (copy_data = false);

-- enable
alter subscription s enable;

-- disable
alter subscription s disable;

-- set_parameters
alter subscription s set (slot_name = 'new_slot', synchronous_commit = 'off');

-- skip
alter subscription s skip (lsn = '0/12345678');

-- owner
alter subscription s owner to u;
alter subscription s owner to current_user;

-- rename
alter subscription s rename to t;

