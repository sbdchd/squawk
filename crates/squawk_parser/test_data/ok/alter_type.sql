-- owner
alter type t owner to u;
alter type t owner to current_user;

-- rename
alter type t rename to u;

-- schema
alter type t set schema s;

-- rename_attribute
alter type t rename attribute a to b;

-- rename_attribute_cascade
alter type s.t rename attribute a to b cascade;
alter type s.t rename attribute a to b restrict;

-- add_value
alter type t add value 'v';

-- add_value_full
alter type s.t add value if not exists 'v' before 'w';
alter type s.t add value if not exists 'v' after 'w';

-- rename_value
alter type t rename value 'v' to 'w';

-- set_property
alter type t set (p = 'v');
alter type t set (p = 'v', q = 'w');

-- add_attribute
alter type t add attribute a b;

-- add_attribute_collate
alter type t add attribute a b collate c;

-- add_attribute_cascade
alter type t add attribute a text cascade;
alter type t add attribute a varchar(100) restrict;

-- drop_attribute
alter type t drop attribute a;
alter type t drop attribute if exists a cascade;
alter type t drop attribute if exists a restrict;

-- alter_attribute
alter type t alter attribute a type b;

-- alter_attribute_set_data
alter type s.t alter attribute a set data type varchar(100) collate c cascade;
alter type t alter attribute a type t restrict;

-- multiple_actions
alter type t add attribute a b, drop attribute c;

