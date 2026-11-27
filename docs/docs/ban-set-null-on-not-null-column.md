---
id: ban-set-null-on-not-null-column
title: ban-set-null-on-not-null-column
---

## problem

A foreign key with `ON DELETE SET NULL` (or `ON UPDATE SET NULL`) on a `NOT NULL` column can never succeed.

The migration runs fine, but deleting a referenced row will fail at runtime:

```sql
create table conversations (
  id uuid primary key
);

create table messages (
  id uuid primary key,
  conversation_id uuid not null,
  constraint fk_messages_conversation_id
    foreign key (conversation_id)
    references conversations (id) on delete set null
);

delete from conversations where id = '...';
-- ERROR: null value in column "conversation_id" of relation "messages" violates not-null constraint
```

Columns in a primary key are implicitly `NOT NULL`, so they are also checked.

## solution

Pick a referential action that doesn't write `NULL`:

```sql
create table messages (
  id uuid primary key,
  conversation_id uuid not null,
  constraint fk_messages_conversation_id
    foreign key (conversation_id)
    references conversations (id) on delete cascade
);
```

Or allow the column to be `NULL`:

```sql
create table messages (
  id uuid primary key,
  conversation_id uuid,
  constraint fk_messages_conversation_id
    foreign key (conversation_id)
    references conversations (id) on delete set null
);
```

With a multi-column foreign key, you can also limit which columns are set to `NULL` (Postgres 15+):

```sql
foreign key (tenant_id, conversation_id)
  references conversations (tenant_id, id) on delete set null (conversation_id)
```

## limitations

Squawk only knows about `NOT NULL` constraints that are in the same file as the foreign key, from `create table`, `alter table ... add column`, `alter table ... alter column ... set not null`, and primary keys.

## links

- https://www.postgresql.org/docs/current/ddl-constraints.html#DDL-CONSTRAINTS-FK
- https://github.com/sbdchd/squawk/issues/1136
