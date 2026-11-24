-- if exists only allowed with set statistics
alter statistics if exists s owner to u;
alter statistics if exists s rename to n;
alter statistics if exists s set schema n;
