-- can't have out params in aggregates
alter aggregate a (in t, out u) 
    set schema s;

