---
id: require-table-schema
title: require-table-schema
---

## problem

Without an explicit schema, Postgres uses the `search_path` when looking up tables and similar objects. This can result in ambiguity as to what specific table you're acting on.

```sql
-- bad
create table posts(id bigint);

alter table posts add column name text;

drop table posts;

create table posts as select 1;
```

## solution

Specify the schema (i.e., `public`) alongside the table names in your DDL.

```sql
-- good
create table public.posts(id bigint);

alter table public.posts add column name text;

drop table public.posts;

create table public.posts as select 1;
```

## links

- https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH
