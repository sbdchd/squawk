---
id: ban-truncate-cascade
title: ban-truncate-cascade
---

## problem

Using `TRUNCATE`'s `CASCADE` option will truncate any tables that are also foreign-keyed to the specified tables.

So if you had tables with foreign-keys like:

```
a <- b <- c
```

and ran:

```sql
truncate a cascade;
```

You'd end up with `a`, `b`, & `c` all being truncated!

### runnable example

Setup:

```sql
create table a (
  a_id int primary key
);
create table b (
  b_id int primary key,
  a_id int,
  foreign key (a_id) references a(a_id)
);
create table c (
  c_id int primary key,
  b_id int,
  foreign key (b_id) references b(b_id)
);
insert into a (a_id) values (1), (2), (3);
insert into b (b_id, a_id) values (101, 1), (102, 2), (103, 3);
insert into c (c_id, b_id) values (1001, 101), (1002, 102), (1003, 103);
```

Then you run:

```sql
truncate a cascade;
```

Which outputs:

```text
NOTICE:  truncate cascades to table "b"
NOTICE:  truncate cascades to table "c"

Query 1 OK: TRUNCATE TABLE
```

And now tables `a`, `b`, & `c` are empty!

## solution

Don't use the `CASCADE` option, instead manually specify the tables you want.

So if you just wanted tables `a` and `b` from the example above:

```sql
truncate a, b;
```

## links

- https://www.postgresql.org/docs/current/sql-truncate.html
- `CASCADE`'s recursive nature [caused Linear's 2024-01-24 incident](https://linear.app/blog/linear-incident-on-jan-24th-2024).
