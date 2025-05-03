-- see: https://github.com/postgres/postgres/blob/028b4b21df26fee67b3ce75c6f14fcfd3c7cf2ee/src/backend/parser/gram.y#L12699
SELECT (((SELECT 2)) + 3);
SELECT (((SELECT 2)) UNION SELECT 2);


-- TODO!
SELECT foo UNION SELECT bar ORDER BY baz;
-- equal to:
(SELECT foo UNION SELECT bar) ORDER BY baz;
