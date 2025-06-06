-- Check that stack depth detection mechanism works and
-- max_stack_depth is not set too high.

create function infinite_recurse() returns int as
'select infinite_recurse()' language sql;

-- Unfortunately, up till mid 2020 the Linux kernel had a bug in PPC64
-- signal handling that would cause this test to crash if it happened
-- to receive an sinval catchup interrupt while the stack is deep:
-- https://bugzilla.kernel.org/show_bug.cgi?id=205183
-- It is likely to be many years before that bug disappears from all
-- production kernels, so disable this test on such platforms.
-- (We still create the function, so as not to have a cross-platform
-- difference in the end state of the regression database.)

SELECT version() ~ 'powerpc64[^,]*-linux-gnu'
       AS skip_test ;

-- The full error report is not very stable, so we show only SQLSTATE
-- and primary error message.


select infinite_recurse();

