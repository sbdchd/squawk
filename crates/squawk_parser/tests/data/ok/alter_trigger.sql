-- rename
alter trigger t on x rename to u;

-- depends_on
alter trigger t on x depends on extension e;

-- full
alter trigger t on s.t no depends on extension e;

