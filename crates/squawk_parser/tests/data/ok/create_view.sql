-- docs
CREATE VIEW vista AS SELECT 'Hello World';

CREATE VIEW vista AS SELECT text 'Hello World' AS hello;

CREATE VIEW comedies AS
    SELECT *
    FROM films
    WHERE kind = 'Comedy';

CREATE VIEW universal_comedies AS
    SELECT *
    FROM comedies
    WHERE classification = 'U'
    WITH LOCAL CHECK OPTION;

CREATE VIEW pg_comedies AS
    SELECT *
    FROM comedies
    WHERE classification = 'PG'
    WITH CASCADED CHECK OPTION;

CREATE VIEW comedies AS
    SELECT f.*,
           country_code_to_name(f.country_code) AS country,
           (SELECT avg(r.rating)
            FROM user_ratings r
            WHERE r.film_id = f.id) AS avg_rating
    FROM films f
    WHERE f.kind = 'Comedy';

CREATE RECURSIVE VIEW public.nums_1_100 (n) AS
    VALUES (1)
UNION ALL
    SELECT n+1 FROM nums_1_100 WHERE n < 100;

-- complete_syntax
create or replace temp recursive view foo (a, b, c)
with (foo = bar, buzz)
as select 1, 2, 3;

create temporary view foo
as select 1, 2, 3
with local check option;

-- regression test
create or replace view my_view as
select x from foo;

-- parenthesized / compound queries
create view v as (select 1);
create view v as (values (1));
create view v as (select 1 union select 2);
create view v as (values (1, 2) union values (3, 4));
create view v as (table t);
create view v as (select 1 order by 1);
create view v as ((select 1));
create view v as (with x as (select 1) select * from x);
