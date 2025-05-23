-- add
alter group g add user u;

-- add_multiple
alter group g add user u, v, w;

-- drop
alter group g drop user u;

-- drop_multiple
alter group g drop user u, v, w;

-- rename
alter group g rename to n;

-- current_role
alter group current_role add user u;

-- current_user
alter group current_user drop user u;

-- session_user
alter group session_user add user u;

