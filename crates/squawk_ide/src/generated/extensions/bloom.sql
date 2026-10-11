-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function public.blhandler(internal) returns index_am_handler
  language c;

-- bloom index access method
create access method bloom type index handler public.blhandler;

create operator family public.int4_ops using bloom;

create operator family public.text_ops using bloom;

create operator class public.int4_ops default for type integer using bloom family public.int4_ops as storage integer;

create operator class public.text_ops default for type text using bloom family public.text_ops as storage text;

