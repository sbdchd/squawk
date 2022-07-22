---
id: prefer-timestamptz
title: prefer-timestamptz
---

## problem

When Postgres stores a datetime in a `timestamp` field, Postgres drops the UTC offset. This means `2019-10-11 21:11:24+02` and `2019-10-11 21:11:24-06` will both be stored as `2019-10-11 21:11:24` in the database, even though they are eight hours apart in time.

## solution

Use a `timestamptz` field to ensure Postgres returns your timestamp with the correct UTC offset.

### create table

Instead of:

```sql
create table app.users
(
    created_ts   timestamp,
    modified_ts  timestamp without time zone
)
```

Use:

```sql
create table app.users
(
    created_ts   timestamptz,
    modified_ts  timestamptz
)
```

### alter table

Instead of:

```sql
alter table app.users
    alter column created_ts type timestamp,
    alter column modified_ts type timestamp without time zone
```

Use:

```sql
alter table app.users
    alter column created_ts type timestamptz,
    alter column modified_ts type timestamptz
```

## links

- ["Don't use timestamp (without time zone) to store UTC times"](https://wiki.postgresql.org/wiki/Don%27t_Do_This#Don.27t_use_timestamp_.28without_time_zone.29_to_store_UTC_times)
- http://www.creativedeletion.com/2015/03/19/persisting_future_datetimes.html
- https://codeblog.jonskeet.uk/2019/03/27/storing-utc-is-not-a-silver-bullet/
