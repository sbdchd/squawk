-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function blhandler(internal) returns index_am_handler
  language c;

-- bloom index access method
create access method bloom
  type index
  handler blhandler;

create operator family int4_ops
  using bloom;

create operator family text_ops
  using bloom;

create operator class int4_ops
  default for type integer
  using bloom
  family int4_ops as
  storage integer;

create operator class text_ops
  default for type text
  using bloom
  family text_ops as
  storage text;
