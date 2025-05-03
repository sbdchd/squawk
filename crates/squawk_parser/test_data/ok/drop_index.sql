-- drop_index
drop index i;

drop index foo.i;

drop index concurrently if exists a, b, c cascade;

drop index if exists d restrict;

drop index if exists "field_name_idx";

drop index concurrently if exists "field_name_idx";

