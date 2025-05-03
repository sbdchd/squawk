-- simple
analyze;
analyse;

-- full
analyze verbose foo.bar, foo.bar(a, b, c), foo;

-- full_parens
analyze (verbose false, skip_locked, buffer_usage_limit 10) foo.bar(a, b, c);

