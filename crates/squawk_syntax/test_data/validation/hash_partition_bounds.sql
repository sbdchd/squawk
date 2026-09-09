create table p1 partition of p for values with (modulus 3, remainder 2);
create table p1 partition of p for values with (MODULUS 3, REMAINDER 2);
create table p1 partition of p for values with ("modulus" 3, "remainder" 2);
create table p1 partition of p for values with ( REMAINDER 2, modulus 3);

create table p1 partition of p for values with (modulus 3);
create table p1 partition of p for values with (remainder 2);
create table p1 partition of p for values with (modulus 3, remainder 2, modulus 3, remainder 2);
create table p1 partition of p for values with (MODULUSMODULUS 3, REMAINDER 2);
create table p1 partition of p for values with (modulus 3, remaining 2);
create table p1 partition of p for values with (year 3, remainder 2);
create table p1 partition of p for values with (modulus 3, collation 2);
create table p partition of e for values with (modulus 1, remainder 2, foo 3);
