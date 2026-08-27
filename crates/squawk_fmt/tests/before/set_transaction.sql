set transaction isolation level serializable;

set transaction isolation level repeatable read, read write, not deferrable;

set session characteristics as transaction isolation level read committed, read only, deferrable;

set transaction snapshot '00000003-0000001B-1';

set session characteristics as transaction isolation level serializable, read write, not deferrable;

/* before set */ SET /* before transaction */ TRANSACTION /* before first mode */ ISOLATION /* before level */ LEVEL /* before serializable */ SERIALIZABLE /* before comma */, /* before read */ READ /* before only */ ONLY /* before second comma */, /* before not */ NOT /* before deferrable */ DEFERRABLE /* before semicolon */;

SET /* before session */ SESSION /* before characteristics */ CHARACTERISTICS /* before as */ AS /* before transaction */ TRANSACTION /* before mode */ READ /* before write */ WRITE /* before semicolon */;

SET /* before transaction */ TRANSACTION /* before snapshot */ SNAPSHOT /* before literal */ '00000003-0000001B-1' /* before semicolon */;
