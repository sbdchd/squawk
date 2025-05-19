-- star
alter aggregate a (*) rename to b;

-- simple_args
alter aggregate a (t) set schema s;

-- complex_args
alter aggregate a (in x text, y numeric) 
    owner to current_user;

-- with_order_by
alter aggregate a (x order by y) 
    rename to b;

-- qualified_names
alter aggregate foo.bar (in t, u) 
    set schema new_schema;

