-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

create function public.blhandler(internal) returns index_am_handler
  language c;

