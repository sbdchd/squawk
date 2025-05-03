-- begin_commit_abort_rollback
-- https://www.postgresql.org/docs/17/sql-commit.html
commit;
commit work;
commit transaction;
commit and chain;
commit and no chain;
commit prepared 'bar';

-- https://www.postgresql.org/docs/17/sql-begin.html
begin;
begin transaction;
begin work;
begin 
  isolation level read committed
  read only
  read write
  deferrable
  not deferrable;

begin
  isolation level read committed,
  isolation level read uncommitted,
  isolation level repeatable read,
  isolation level serializable,
  read only,
  read write,
  deferrable,
  not deferrable;

start transaction
  isolation level read committed
  read only
  read write
  deferrable
  not deferrable;

start transaction
  isolation level read committed,
  isolation level read uncommitted,
  isolation level repeatable read,
  isolation level serializable,
  read only,
  read write,
  deferrable,
  not deferrable;

prepare transaction 'f';

savepoint foo;

release savepoint foo;
release foo;

rollback to savepoint foo;
rollback work to savepoint foo;
rollback transaction to savepoint foo;
rollback to foo;
rollback work to foo;
rollback transaction to foo;

end;
end work;
end transaction;
end and chain;
end and no chain;

abort;
abort work;
abort transaction;
abort and chain;
abort and no chain;

rollback;
rollback work;
rollback transaction;
rollback and chain;
rollback and no chain;
rollback prepared 'foo';

