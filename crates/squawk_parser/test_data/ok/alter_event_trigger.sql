-- disable
alter event trigger t disable;

-- enable
alter event trigger t enable;

-- enable_replica
alter event trigger t enable replica;

-- enable_always
alter event trigger t enable always;

-- owner
alter event trigger t owner to u;

-- owner_current_role
alter event trigger t owner to current_role;

-- rename
alter event trigger t rename to u;

