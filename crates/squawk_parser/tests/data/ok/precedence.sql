-- see: https://github.com/postgres/postgres/blob/028b4b21df26fee67b3ce75c6f14fcfd3c7cf2ee/src/backend/parser/gram.y#L12699
SELECT (((SELECT 2)) + 3);
SELECT (((SELECT 2)) UNION SELECT 2);


SELECT foo UNION SELECT bar ORDER BY baz;
-- equal to:
(SELECT foo UNION SELECT bar) ORDER BY baz;

-- binds tighter than between
select a + 1 between 1 and 2;
select a::date between x and y;
select - a between 1 and 2;
select a between b is distinct from c and d;
select a between b and not c;
select a between 1 and 2 + 3;
select a collate "C" between b and c;

-- is null, isnull, and notnull bind looser than arithmetic and prefix ops
select a + 1 isnull;
select - a isnull;
select a = b isnull;
select a isnull isnull;
select a notnull notnull;
select a collate "C" isnull;
select not a = b isnull;

-- postfix after is
select a is null::text;
select a is null collate "C";
select a is null isnull;
select a is true::text;

-- operators after in
select x in (1, 2)::text;
select x in (1, 2) collate "C";
select x in (1, 2) || 'z';

-- prefix operators are left associative with binary operators
select ~ a ~ b;
select ~ a + b;

-- operators after any/all apply to the comparison
select 1 = any(array[1])::text;
select a = any(c) collate "C" from t;
select a = any(c) || 'x' from t;
select a < all(select 1)::int;
