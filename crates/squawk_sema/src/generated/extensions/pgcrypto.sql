-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function armor(bytea) returns text
  language c
  immutable
  strict
  parallel safe;

create function armor(bytea, text[], text[]) returns text
  language c
  immutable
  strict
  parallel safe;

create function crypt(text, text) returns text
  language c
  immutable
  strict
  parallel safe;

create function dearmor(text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function decrypt(bytea, bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function decrypt_iv(bytea, bytea, bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function digest(bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function digest(text, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function encrypt(bytea, bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function encrypt_iv(bytea, bytea, bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function fips_mode() returns boolean
  language c
  strict
  parallel safe;

create function gen_random_bytes(integer) returns bytea
  language c
  strict
  parallel safe;

create function gen_random_uuid() returns uuid
  language c
  parallel safe;

create function gen_salt(text) returns text
  language c
  strict
  parallel safe;

create function gen_salt(text, integer) returns text
  language c
  strict
  parallel safe;

create function hmac(bytea, bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function hmac(text, text, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function pgp_armor_headers(
  text,
  out key text,
  out value text
) returns setof record
  language c
  immutable
  strict
  parallel safe;

create function pgp_key_id(bytea) returns text
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_decrypt(bytea, bytea) returns text
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_decrypt(bytea, bytea, text) returns text
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_decrypt(bytea, bytea, text, text) returns text
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_decrypt_bytea(bytea, bytea) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_decrypt_bytea(bytea, bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_decrypt_bytea(bytea, bytea, text, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function pgp_pub_encrypt(text, bytea) returns bytea
  language c
  strict
  parallel safe;

create function pgp_pub_encrypt(text, bytea, text) returns bytea
  language c
  strict
  parallel safe;

create function pgp_pub_encrypt_bytea(bytea, bytea) returns bytea
  language c
  strict
  parallel safe;

create function pgp_pub_encrypt_bytea(bytea, bytea, text) returns bytea
  language c
  strict
  parallel safe;

create function pgp_sym_decrypt(bytea, text) returns text
  language c
  immutable
  strict
  parallel safe;

create function pgp_sym_decrypt(bytea, text, text) returns text
  language c
  immutable
  strict
  parallel safe;

create function pgp_sym_decrypt_bytea(bytea, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function pgp_sym_decrypt_bytea(bytea, text, text) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function pgp_sym_encrypt(text, text) returns bytea
  language c
  strict
  parallel safe;

create function pgp_sym_encrypt(text, text, text) returns bytea
  language c
  strict
  parallel safe;

create function pgp_sym_encrypt_bytea(bytea, text) returns bytea
  language c
  strict
  parallel safe;

create function pgp_sym_encrypt_bytea(bytea, text, text) returns bytea
  language c
  strict
  parallel safe;
