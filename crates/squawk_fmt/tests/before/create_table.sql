create table u();

create table t(a int,b text);

-- users table
create table users (id int);

-- columns that have various quoting requirements
create table cols ("left" int, "select" text, "data" int, "Mixed" text, UPPER int, U&"c!006fl" uescape '!' int);

-- table names
create table PUBLIC.Accounts (id int);
create table "Public"."Users" (id int);
create table "foo"."quoted_names" ("data" int, "value" text);
create table "left" (id int);
create table "table" (id int);
create table U&"d\0061t\+000061" (id int);
create table U&"d!0061tum" uescape '!' (id int);
create table /* foo */ foo /* bar  */ . /* buzz */ bar (id int);

-- comments inside a name node
create table U&"d!0061tum" /* mid */ uescape '!' (id int);
create table t (U&"c!006fl" /* c */ uescape '!' int);
create table t (U&"c!006fl" uescape /* c */ '!' int);

-- comments inside a path
create table foo/*a*//*b*/.bar (id int);
create table foo -- a line comment
. bar (id int);
