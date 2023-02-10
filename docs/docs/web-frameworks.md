---
id: web-frameworks
title: Web Frameworks
---

This page serves a reference guide for common tasks within web frameworks. Squawk doesn't have special support for Django or any other web ORM. This page is merely a guide.

## Django ORM

Django can auto generate schema changes, but many times the generated SQL won't pass Squawk's rules. These custom database backends for zero downtime migrations might work for your use case:

- [`yandex/zero-downtime-migrations`](https://github.com/yandex/zero-downtime-migrations)
- [`tbicr/django-pg-zero-downtime-migrations`](https://github.com/tbicr/django-pg-zero-downtime-migrations)

### making a field non-nullable

In the Django ORM, making a nullable field non-nullable requires care to make the change safe.

This example shows how to make a nullable boolean field a non-nullable field.

```python
# before
class Post(Model):
    published = models.BooleanField(null=True)
```

```python
# after
class Post(Model):
    published = models.BooleanField(null=False, default=False)
```


#### migration steps

We cannot use the auto generated SQL from Django's migration system. We must use [`django.db.migrations.operations.SeparateDatabaseAndState`](https://docs.djangoproject.com/en/3.2/ref/migration-operations/#django.db.migrations.operations.SeparateDatabaseAndState) to make changes to our Django models while using custom SQL.

1. Update the model with a default so newly created objects in Django get assigned a default value. This change must be deployed before making the SQL migrations.

    ```python
    class Post(Model):
        published = models.BooleanField(default=False)
    ```

2. Create trigger to set default value `false` for inserts `published` is `null`.

    ```sql
    CREATE FUNCTION null_published_trigger()
    RETURNS trigger AS '
    BEGIN
    IF NEW.published IS NULL THEN
        NEW.published := false;
    END IF;
    RETURN NEW;
    END' LANGUAGE 'plpgsql';

    CREATE TRIGGER published_default_trigger
    BEFORE INSERT ON post
    FOR EACH ROW
    EXECUTE PROCEDURE null_published_trigger();
    ```

3. Set a default value for existing objects.

    ```sql
    UPDATE "post" SET "published" = false WHERE "published" IS NULL;
    ```


4. Create a not null constraint.

    ```sql
    ALTER TABLE "post" ADD CONSTRAINT published_not_null
        CHECK ("published" IS NOT NULL) NOT VALID;
    ```

5. Validate constraint.

    ```sql
    ALTER TABLE "post" VALIDATE CONSTRAINT published_not_null;
    ```

6. Remove trigger.

    ```sql
    DROP TRIGGER IF EXISTS published_default_trigger on post;
    DROP FUNCTION IF EXISTS null_published_trigger;
    ```


## Alembic and SQLAlchemy

### usage

```shell
alembic upgrade head --sql | squawk
```
or you can choose revisions
```shell
alembic upgrade first_revision:last_revision --sql | squawk
```

### settings

Use `transaction_per_migration = True` 
in [configure](https://alembic.sqlalchemy.org/en/latest/api/runtime.html#alembic.runtime.environment.EnvironmentContext.configure.params.transaction_per_migration)

```python
# env.py
...
context.configure(
   ...
   transaction_per_migration=True
   ...
)
...
```

Set `lock_timeout` and `statement_timeout`

```python
# env.py
...
def run_migrations_online() -> None:
    """Run migrations in 'online' mode.
    In this scenario we need to create an Engine
    and associate a connection with the context.
    """
    connectable = engine_from_config(
        config.get_section(config.config_ini_section),
        prefix="sqlalchemy.",
        poolclass=pool.NullPool,
        connect_args={'options': '-c lock_timeout=4000 -c statement_timeout=5000'}
    )
...
```
