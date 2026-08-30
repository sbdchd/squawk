create type shell_type;

create type inventory_item as (name text, supplier_id integer, extraordinarily_long_product_description character varying(255) collate "C", price numeric);

create type mood as enum ('sad', 'ok', 'happy', 'extraordinarily delighted and enthusiastic');

create type float8_range as range (subtype = float8, subtype_diff = float8mi);

create type complex_base (input = complex_in, output = complex_out, internallength = 16);

create type null_default_base (default = null);

create type commented_null_default_base (default /* equals */ = /* null value */ null /* close */);

-- comments in every composite type position
create /* type */ type /* name */ commented_composite /* as */ as /* open */ (/* field */ first_field /* type */ text /* collate */ collate /* collation */ "C" /* comma */, /* next field */ second_field /* next type */ integer /* close */) /* end */;

-- comments in every enum type position
create type /* enum name */ commented_enum /* as */ as /* enum */ enum /* open */ (/* first */ 'one' /* comma */, /* second */ 'two' /* close */) /* end */;

-- comments in every range/base type position
create type /* range name */ commented_range /* as */ as /* range */ range /* attributes */ (/* option */ subtype /* equals */ = /* value */ numeric /* comma */, /* option two */ multirange_type_name = commented_multirange /* close */) /* end */;

create type /* base name */ commented_base /* attributes */ (/* input */ input /* equals */ = /* value */ commented_in /* comma */, /* output */ output = commented_out /* close */) /* end */;
