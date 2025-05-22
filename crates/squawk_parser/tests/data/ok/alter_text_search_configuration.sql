-- add_mapping
alter text search configuration n
    add mapping for t with d;

alter text search configuration foo.n
    add mapping for t, u with foo.d, foo.e;

-- alter_mapping
alter text search configuration n
    alter mapping for t with d;

alter text search configuration f.n
    alter mapping for t with f.d;

-- alter_mapping_replace
alter text search configuration n
    alter mapping replace o with n;

alter text search configuration f.n
    alter mapping replace f.o with f.n;

-- alter_mapping_for_replace
alter text search configuration n
    alter mapping for t replace o with n;

alter text search configuration f.n
    alter mapping for t replace f.o with f.n;

-- drop_mapping
alter text search configuration n
    drop mapping for t;

alter text search configuration f.n
    drop mapping if exists for t, b;

-- rename
alter text search configuration n rename to m;

alter text search configuration f.n rename to m;

-- owner_to
alter text search configuration n owner to u;

alter text search configuration f.n owner to current_user;

-- set_schema
alter text search configuration n set schema s;

alter text search configuration f.n set schema s;

