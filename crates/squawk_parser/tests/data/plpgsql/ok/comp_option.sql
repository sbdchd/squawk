do $$
#option dump
begin end $$;

do $$
#variable_conflict error
begin end $$;

do $$
#variable_conflict use_variable
begin end $$;

do $$
#variable_conflict use_column
begin end $$;

do $$
#print_strict_params on
begin end $$;

do $$
#print_strict_params off
begin end $$;

-- unquoted values are down-cased, so these are the same option
do $$
#print_strict_params ON
begin end $$;

do $$
#print_strict_params "on"
begin end $$;

-- whitespace after the `#`, and more than one option, are both fine
do $$ # option dump
#variable_conflict use_variable
#print_strict_params on
#variable_conflict use_column
begin end $$;

-- no newline required before the block
do $$ #option dump begin end $$;
