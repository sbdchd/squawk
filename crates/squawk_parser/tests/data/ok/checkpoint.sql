-- checkpoint
checkpoint;

-- checkpoint options
checkpoint (mode fast);
checkpoint (mode spread, flush_unlogged true);
checkpoint (flush_unlogged false);
checkpoint (flush_unlogged);

