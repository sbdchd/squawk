-- create_type_as
create type t as ();

create type t as (
  a int8 collate "fr_FR"
);

create type t as (
  a int8 collate "fr_FR",
  b int,
  c text
);

-- create_type_as_enum
create type t as enum ();
create type t as enum ('a');
create type t as enum ('a', 'b', 'c');

-- create_type_as_range
create type t as range (
  subtype = bigint
);



-- schema
create type public.t;
create type a.b.c.d;

-- create_type_name
create type t;

create type t (
  input = foo.bar.func_name,
  output = func_name
);

create type t (
  input = foo.bar.func_name,
  output = func_name,
  receive = receive_function,
  send = send_function,
  typmod_in = type_modifier_input_function,
  typmod_out = type_modifier_output_function,
  analyze = analyze_function,
  subscript = subscript_function,
  internallength = variable,
  passedbyvalue,
  -- The allowed values equate to alignment on 1, 2, 4, or 8 byte boundaries.
  alignment = 1,
  storage = plain,
  like = like_type,
  -- see: https://www.postgresql.org/docs/17/catalog-pg-type.html#CATALOG-TYPCATEGORY-TABLE
  category = 'U',
  preferred = false,
  default =  null,
  element = float4,
  delimiter = ',',
  collatable = true
);


