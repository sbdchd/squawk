-- owner
alter operator p.+ (int4, int4) 
  owner to u;

-- schema
alter operator + (none, text) 
  set schema s;

-- options
alter operator & (bool, bool) 
  set (
    restrict = r,
    join = j,
    commutator = c,
    negator = n,
    hashes = enabled,
    merges = enabled
  );

-- none_options
alter operator % (int, int) 
  set (restrict = none, join = none);

