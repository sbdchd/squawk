do $$
#option dumpp
begin end $$;

do $$
#foo bar
begin end $$;

do $$
#variable_conflict
begin end $$;

do $$
#variable_conflict use_default
begin end $$;

-- a number is a syntax error
do $$
#print_strict_params 1
begin end $$;

-- must be on/off
do $$
#print_strict_params banana
begin end $$;

-- a quoted value keeps its case, so this one doesn't match `on`
do $$
#print_strict_params "ON"
begin end $$;

-- `loop` is PL/pgSQL-reserved
do $$
#print_strict_params loop
begin end $$;

-- options only come before the block
do $$
declare x int;
#option dump
begin end $$;
