-- OVERLAPS is only defined over row expressions, not scalars
SELECT 1 OVERLAPS 2;

-- wrong number of parameters on left side of OVERLAPS expression
SELECT (1, 2, 3) OVERLAPS (1, 2);

-- wrong number of parameters on right side of OVERLAPS expression
SELECT (1, 2) OVERLAPS (1, 2, 3);
