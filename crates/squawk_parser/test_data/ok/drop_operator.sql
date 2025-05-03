-- simple
drop operator ^ (integer, integer);

-- schema
drop operator foo.foo.^ (f.smallint, bar.smallint);

-- unary
drop operator ~ (none, bit);

-- multiple
drop operator ~ (none, bit), ^ (integer, integer);

-- full
drop operator if exists # (NONE, bigint), @ (text, text), ## (none, integer) cascade;
drop operator if exists ! (int, int) restrict;

-- non_default_operator
drop operator if exists !!!!!!!!!!!!!!!!!!! (int, int) restrict;

drop operator bar.bar.@@@@@@@@@@@@@@@@@ (int, int) restrict;

