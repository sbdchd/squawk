-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function public.armor(bytea) returns text
  language c;

create function public.armor(bytea, text[], text[]) returns text
  language c;

create function public.crypt(text, text) returns text
  language c;

create function public.dearmor(text) returns bytea
  language c;

create function public.decrypt(bytea, bytea, text) returns bytea
  language c;

create function public.decrypt_iv(bytea, bytea, bytea, text) returns bytea
  language c;

create function public.digest(bytea, text) returns bytea
  language c;

create function public.digest(text, text) returns bytea
  language c;

create function public.encrypt(bytea, bytea, text) returns bytea
  language c;

create function public.encrypt_iv(bytea, bytea, bytea, text) returns bytea
  language c;

create function public.fips_mode() returns boolean
  language c;

create function public.gen_random_bytes(integer) returns bytea
  language c;

create function public.gen_random_uuid() returns uuid
  language c;

create function public.gen_salt(text) returns text
  language c;

create function public.gen_salt(text, integer) returns text
  language c;

create function public.hmac(bytea, bytea, text) returns bytea
  language c;

create function public.hmac(text, text, text) returns bytea
  language c;

create function public.pgp_armor_headers(text, OUT key text, OUT value text) returns SETOF record
  language c;

create function public.pgp_key_id(bytea) returns text
  language c;

create function public.pgp_pub_decrypt(bytea, bytea) returns text
  language c;

create function public.pgp_pub_decrypt(bytea, bytea, text) returns text
  language c;

create function public.pgp_pub_decrypt(bytea, bytea, text, text) returns text
  language c;

create function public.pgp_pub_decrypt_bytea(bytea, bytea) returns bytea
  language c;

create function public.pgp_pub_decrypt_bytea(bytea, bytea, text) returns bytea
  language c;

create function public.pgp_pub_decrypt_bytea(bytea, bytea, text, text) returns bytea
  language c;

create function public.pgp_pub_encrypt(text, bytea) returns bytea
  language c;

create function public.pgp_pub_encrypt(text, bytea, text) returns bytea
  language c;

create function public.pgp_pub_encrypt_bytea(bytea, bytea) returns bytea
  language c;

create function public.pgp_pub_encrypt_bytea(bytea, bytea, text) returns bytea
  language c;

create function public.pgp_sym_decrypt(bytea, text) returns text
  language c;

create function public.pgp_sym_decrypt(bytea, text, text) returns text
  language c;

create function public.pgp_sym_decrypt_bytea(bytea, text) returns bytea
  language c;

create function public.pgp_sym_decrypt_bytea(bytea, text, text) returns bytea
  language c;

create function public.pgp_sym_encrypt(text, text) returns bytea
  language c;

create function public.pgp_sym_encrypt(text, text, text) returns bytea
  language c;

create function public.pgp_sym_encrypt_bytea(bytea, text) returns bytea
  language c;

create function public.pgp_sym_encrypt_bytea(bytea, text, text) returns bytea
  language c;

