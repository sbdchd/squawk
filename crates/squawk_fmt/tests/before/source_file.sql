

  /* leading */ SELECT 1;SELECT 2; SELECT 3;

;SELECT 4;; SELECT 5;

SELECT 6; /* between */ SELECT 7;
SELECT 8; -- trailing
SELECT 9;


/* own line */

SELECT 'a string that is intentionally long enough to make this statement wrap across multiple lines';

\echo before
SELECT 10;
\echo after
;

/* trailing */
