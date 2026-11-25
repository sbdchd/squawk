-- owner
alter statistics s owner to u;
alter statistics foo.s owner to current_user;

-- rename
alter statistics s rename to n;

-- schema
alter statistics s set schema n;

-- statistics_value
alter statistics s set statistics 100;
alter statistics s set statistics default;
alter statistics if exists s set statistics 100;
alter statistics if exists foo.s set statistics default;

