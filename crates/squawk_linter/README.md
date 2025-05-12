# linter

## Error message style guide

### Lock messages

Each lock name should be:

- accompanied by an explanation of its meaning in simple terms
- tables that are affected

Instead of:

```
Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock.
```

include a lock description & table name:

```
Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock, which prevents reads and writes to `users`.
```

### Help

A help message should be short an actionable.

Instead of:

```
You can remove the `CASCADE` keyword and then specify exactly which tables you want to truncate directly.
```

tell the user what to change (add, update, remove):

```
Remove the `CASCADE` and specify exactly which tables you want to truncate.
```
