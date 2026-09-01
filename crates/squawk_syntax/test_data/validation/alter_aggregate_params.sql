-- can't have out params in aggregates
alter aggregate a (in t, out u, inout v, in out w)
    set schema s;

alter aggregate a(value integer default 1) rename to a2;
