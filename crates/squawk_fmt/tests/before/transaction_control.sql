begin;

begin work;

start transaction isolation level serializable, read write, deferrable;

begin transaction isolation level repeatable read, read only, not deferrable;

commit;

end work;

commit transaction and chain;

commit and no chain;

prepare transaction 'a very long prepared transaction identifier used to test transaction statement line length';

commit prepared 'prepared_transaction';

rollback;

abort work;

rollback transaction and chain;

rollback and no chain;

rollback to savepoint before_changes;

rollback work to savepoint before_changes;

rollback prepared 'prepared_transaction';

savepoint before_changes;

release savepoint before_changes;

-- comments in every position
begin /*a*/ transaction /*b*/ isolation /*c*/ level /*d*/ serializable /*e*/, /*f*/ read /*g*/ write /*h*/, /*i*/ not /*j*/ deferrable /*k*/;

commit /*a*/ transaction /*b*/ and /*c*/ no /*d*/ chain /*e*/;

prepare /*a*/ transaction /*b*/ 'prepared_transaction' /*c*/;

commit /*a*/ prepared /*b*/ 'prepared_transaction' /*c*/;

rollback /*a*/ work /*b*/ to /*c*/ savepoint /*d*/ before_changes /*e*/;

rollback /*a*/ prepared /*b*/ 'prepared_transaction' /*c*/;

savepoint /*a*/ before_changes /*b*/;

release /*a*/ savepoint /*b*/ before_changes /*c*/;
