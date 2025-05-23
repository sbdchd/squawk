-- simple
create statistics on (a) from t;

-- full
create statistics if not exists foo.s(mcv, ndistinct, dependencies)
on (foo), bar, buzz, (a + b)
from foo.t;

-- docs_1
CREATE STATISTICS s1 (dependencies) ON a, b FROM t1;

-- docs_2
CREATE STATISTICS s2 (mcv) ON a, b FROM t2;

-- docs_3
CREATE STATISTICS s3 (ndistinct) ON date_trunc('month', a), date_trunc('day', a) FROM t3;

