-- GROUP is only allowed as a grantee prefix, not when naming a role
create group group g;
create role group r;
create user group u;
create schema authorization group g;
alter role foo rename to group bar;
