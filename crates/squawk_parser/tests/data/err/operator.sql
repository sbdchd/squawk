-- all_Op is symbolic only, keyword operators aren't operator names
select 1 operator(and) 2;
select 1 operator(or) 2;
select 1 operator(in) 2;
create operator and (leftarg = int, rightarg = int, function = int4pl);
drop operator and (int, int);

-- neither are the tokens Postgres reserves
select 1 operator(::) 2;
select 1 operator(:=) 2;
select 1 operator(=>) 2;
create operator => (rightarg = int8, function = factorial);
create operator class c for type int using btree as operator 1 ::;
create operator === (leftarg = int, rightarg = int, commutator = ::);

-- qualifiers are ColId, so type function name keywords aren't allowed
select 1 operator(binary.+) 2;
select 1 operator(left.+) 2;
select 1 operator(collation.+) 2;
select 1 operator(a.binary.+) 2;
create operator binary.+ (leftarg = int, rightarg = int, function = int4pl);

-- order by using takes qual_all_Op, a qualified operator needs operator(...)
select 1 from t order by a using and;
select 1 from t order by a using pg_catalog.<;

-- same for the operator of an exclusion constraint
create table t (c int, exclude using gist (c with and));
create table t (c int, exclude using gist (c with binary.=));
