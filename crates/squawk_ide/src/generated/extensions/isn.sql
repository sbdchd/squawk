-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- International European Article Number (EAN13)
-- size: 8, align: 8
create type ean13;

-- International Standard Book Number (ISBN)
-- size: 8, align: 8
create type isbn;

-- International Standard Book Number 13 (ISBN13)
-- size: 8, align: 8
create type isbn13;

-- International Standard Music Number (ISMN)
-- size: 8, align: 8
create type ismn;

-- International Standard Music Number 13 (ISMN13)
-- size: 8, align: 8
create type ismn13;

-- International Standard Serial Number (ISSN)
-- size: 8, align: 8
create type issn;

-- International Standard Serial Number 13 (ISSN13)
-- size: 8, align: 8
create type issn13;

-- Universal Product Code (UPC)
-- size: 8, align: 8
create type upc;

create function btean13cmp(ean13, ean13) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, isbn) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, isbn13) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, ismn) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, ismn13) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, issn) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, issn13) returns integer
  language internal immutable strict parallel safe;

create function btean13cmp(ean13, upc) returns integer
  language internal immutable strict parallel safe;

create function btisbn13cmp(isbn13, ean13) returns integer
  language internal immutable strict parallel safe;

create function btisbn13cmp(isbn13, isbn) returns integer
  language internal immutable strict parallel safe;

create function btisbn13cmp(isbn13, isbn13) returns integer
  language internal immutable strict parallel safe;

create function btisbncmp(isbn, ean13) returns integer
  language internal immutable strict parallel safe;

create function btisbncmp(isbn, isbn) returns integer
  language internal immutable strict parallel safe;

create function btisbncmp(isbn, isbn13) returns integer
  language internal immutable strict parallel safe;

create function btismn13cmp(ismn13, ean13) returns integer
  language internal immutable strict parallel safe;

create function btismn13cmp(ismn13, ismn) returns integer
  language internal immutable strict parallel safe;

create function btismn13cmp(ismn13, ismn13) returns integer
  language internal immutable strict parallel safe;

create function btismncmp(ismn, ean13) returns integer
  language internal immutable strict parallel safe;

create function btismncmp(ismn, ismn) returns integer
  language internal immutable strict parallel safe;

create function btismncmp(ismn, ismn13) returns integer
  language internal immutable strict parallel safe;

create function btissn13cmp(issn13, ean13) returns integer
  language internal immutable strict parallel safe;

create function btissn13cmp(issn13, issn) returns integer
  language internal immutable strict parallel safe;

create function btissn13cmp(issn13, issn13) returns integer
  language internal immutable strict parallel safe;

create function btissncmp(issn, ean13) returns integer
  language internal immutable strict parallel safe;

create function btissncmp(issn, issn) returns integer
  language internal immutable strict parallel safe;

create function btissncmp(issn, issn13) returns integer
  language internal immutable strict parallel safe;

create function btupccmp(upc, ean13) returns integer
  language internal immutable strict parallel safe;

create function btupccmp(upc, upc) returns integer
  language internal immutable strict parallel safe;

create function ean13_in(cstring) returns ean13
  language c immutable strict parallel safe;

create function ean13_out(ean13) returns cstring
  language c immutable strict parallel safe;

create function ean13_out(isbn13) returns cstring
  language c immutable strict parallel safe;

create function ean13_out(ismn13) returns cstring
  language c immutable strict parallel safe;

create function ean13_out(issn13) returns cstring
  language c immutable strict parallel safe;

create function hashean13(ean13) returns integer
  language internal immutable strict parallel safe;

create function hashisbn(isbn) returns integer
  language internal immutable strict parallel safe;

create function hashisbn13(isbn13) returns integer
  language internal immutable strict parallel safe;

create function hashismn(ismn) returns integer
  language internal immutable strict parallel safe;

create function hashismn13(ismn13) returns integer
  language internal immutable strict parallel safe;

create function hashissn(issn) returns integer
  language internal immutable strict parallel safe;

create function hashissn13(issn13) returns integer
  language internal immutable strict parallel safe;

create function hashupc(upc) returns integer
  language internal immutable strict parallel safe;

create function is_valid(ean13) returns boolean
  language c immutable strict parallel safe;

create function is_valid(isbn) returns boolean
  language c immutable strict parallel safe;

create function is_valid(isbn13) returns boolean
  language c immutable strict parallel safe;

create function is_valid(ismn) returns boolean
  language c immutable strict parallel safe;

create function is_valid(ismn13) returns boolean
  language c immutable strict parallel safe;

create function is_valid(issn) returns boolean
  language c immutable strict parallel safe;

create function is_valid(issn13) returns boolean
  language c immutable strict parallel safe;

create function is_valid(upc) returns boolean
  language c immutable strict parallel safe;

create function isbn(ean13) returns isbn
  language c immutable strict parallel safe;

create function isbn13(ean13) returns isbn13
  language c immutable strict parallel safe;

create function isbn13_in(cstring) returns isbn13
  language c immutable strict parallel safe;

create function isbn_in(cstring) returns isbn
  language c immutable strict parallel safe;

create function ismn(ean13) returns ismn
  language c immutable strict parallel safe;

create function ismn13(ean13) returns ismn13
  language c immutable strict parallel safe;

create function ismn13_in(cstring) returns ismn13
  language c immutable strict parallel safe;

create function ismn_in(cstring) returns ismn
  language c immutable strict parallel safe;

create function isn_out(isbn) returns cstring
  language c immutable strict parallel safe;

create function isn_out(ismn) returns cstring
  language c immutable strict parallel safe;

create function isn_out(issn) returns cstring
  language c immutable strict parallel safe;

create function isn_out(upc) returns cstring
  language c immutable strict parallel safe;

create function isn_weak() returns boolean
  language c stable strict parallel safe;

create function isn_weak(boolean) returns boolean
  language c strict;

create function isneq(ean13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ean13, upc) returns boolean
  language internal immutable strict parallel safe;

create function isneq(isbn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(isbn, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(isbn, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(isbn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(isbn13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(isbn13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ismn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ismn, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ismn, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ismn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ismn13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(ismn13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(issn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(issn, issn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(issn, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(issn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(issn13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isneq(issn13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(upc, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isneq(upc, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ean13, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnge(isbn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(isbn, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(isbn, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(isbn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(isbn13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(isbn13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ismn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ismn, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ismn, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ismn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ismn13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(ismn13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(issn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(issn, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(issn, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(issn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(issn13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnge(issn13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(upc, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnge(upc, upc) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ean13, upc) returns boolean
  language internal immutable strict parallel safe;

create function isngt(isbn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(isbn, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(isbn, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(isbn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(isbn13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(isbn13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ismn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ismn, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ismn, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ismn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ismn13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(ismn13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(issn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(issn, issn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(issn, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(issn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(issn13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isngt(issn13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(upc, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isngt(upc, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ean13, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnle(isbn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(isbn, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(isbn, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(isbn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(isbn13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(isbn13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ismn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ismn, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ismn, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ismn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ismn13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(ismn13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(issn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(issn, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(issn, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(issn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(issn13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnle(issn13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(upc, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnle(upc, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ean13, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(isbn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(isbn, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(isbn, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(isbn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(isbn13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(isbn13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ismn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ismn, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ismn, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ismn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ismn13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(ismn13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(issn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(issn, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(issn, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(issn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(issn13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(issn13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(upc, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnlt(upc, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ean13, upc) returns boolean
  language internal immutable strict parallel safe;

create function isnne(isbn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(isbn, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(isbn, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(isbn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(isbn13, isbn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(isbn13, isbn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ismn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ismn, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ismn, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ismn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ismn13, ismn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(ismn13, ismn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(issn, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(issn, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(issn, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(issn13, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(issn13, issn) returns boolean
  language internal immutable strict parallel safe;

create function isnne(issn13, issn13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(upc, ean13) returns boolean
  language internal immutable strict parallel safe;

create function isnne(upc, upc) returns boolean
  language internal immutable strict parallel safe;

create function issn(ean13) returns issn
  language c immutable strict parallel safe;

create function issn13(ean13) returns issn13
  language c immutable strict parallel safe;

create function issn13_in(cstring) returns issn13
  language c immutable strict parallel safe;

create function issn_in(cstring) returns issn
  language c immutable strict parallel safe;

create function make_valid(ean13) returns ean13
  language c immutable strict parallel safe;

create function make_valid(isbn) returns isbn
  language c immutable strict parallel safe;

create function make_valid(isbn13) returns isbn13
  language c immutable strict parallel safe;

create function make_valid(ismn) returns ismn
  language c immutable strict parallel safe;

create function make_valid(ismn13) returns ismn13
  language c immutable strict parallel safe;

create function make_valid(issn) returns issn
  language c immutable strict parallel safe;

create function make_valid(issn13) returns issn13
  language c immutable strict parallel safe;

create function make_valid(upc) returns upc
  language c immutable strict parallel safe;

create function upc(ean13) returns upc
  language c immutable strict parallel safe;

create function upc_in(cstring) returns upc
  language c immutable strict parallel safe;

create type ean13 (input = ean13_in, output = ean13_out);

create type isbn (input = isbn_in, output = isn_out);

create type isbn13 (input = isbn13_in, output = ean13_out);

create type ismn (input = ismn_in, output = isn_out);

create type ismn13 (input = ismn13_in, output = ean13_out);

create type issn (input = issn_in, output = isn_out);

create type issn13 (input = issn13_in, output = ean13_out);

create type upc (input = upc_in, output = isn_out);

create operator < (
  leftarg = ean13,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = isbn,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = isbn13,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = ismn,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = ismn13,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = issn,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = issn13,
  function = isnlt
);

create operator < (
  leftarg = ean13,
  rightarg = upc,
  function = isnlt
);

create operator < (
  leftarg = isbn,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = isbn,
  rightarg = isbn,
  function = isnlt
);

create operator < (
  leftarg = isbn,
  rightarg = isbn13,
  function = isnlt
);

create operator < (
  leftarg = isbn13,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = isbn13,
  rightarg = isbn,
  function = isnlt
);

create operator < (
  leftarg = isbn13,
  rightarg = isbn13,
  function = isnlt
);

create operator < (
  leftarg = ismn,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = ismn,
  rightarg = ismn,
  function = isnlt
);

create operator < (
  leftarg = ismn,
  rightarg = ismn13,
  function = isnlt
);

create operator < (
  leftarg = ismn13,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = ismn13,
  rightarg = ismn,
  function = isnlt
);

create operator < (
  leftarg = ismn13,
  rightarg = ismn13,
  function = isnlt
);

create operator < (
  leftarg = issn,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = issn,
  rightarg = issn,
  function = isnlt
);

create operator < (
  leftarg = issn,
  rightarg = issn13,
  function = isnlt
);

create operator < (
  leftarg = issn13,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = issn13,
  rightarg = issn,
  function = isnlt
);

create operator < (
  leftarg = issn13,
  rightarg = issn13,
  function = isnlt
);

create operator < (
  leftarg = upc,
  rightarg = ean13,
  function = isnlt
);

create operator < (
  leftarg = upc,
  rightarg = upc,
  function = isnlt
);

create operator <= (
  leftarg = ean13,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = isbn,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = isbn13,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = ismn,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = ismn13,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = issn,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = issn13,
  function = isnle
);

create operator <= (
  leftarg = ean13,
  rightarg = upc,
  function = isnle
);

create operator <= (
  leftarg = isbn,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = isbn,
  rightarg = isbn,
  function = isnle
);

create operator <= (
  leftarg = isbn,
  rightarg = isbn13,
  function = isnle
);

create operator <= (
  leftarg = isbn13,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = isbn13,
  rightarg = isbn,
  function = isnle
);

create operator <= (
  leftarg = isbn13,
  rightarg = isbn13,
  function = isnle
);

create operator <= (
  leftarg = ismn,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = ismn,
  rightarg = ismn,
  function = isnle
);

create operator <= (
  leftarg = ismn,
  rightarg = ismn13,
  function = isnle
);

create operator <= (
  leftarg = ismn13,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = ismn13,
  rightarg = ismn,
  function = isnle
);

create operator <= (
  leftarg = ismn13,
  rightarg = ismn13,
  function = isnle
);

create operator <= (
  leftarg = issn,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = issn,
  rightarg = issn,
  function = isnle
);

create operator <= (
  leftarg = issn,
  rightarg = issn13,
  function = isnle
);

create operator <= (
  leftarg = issn13,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = issn13,
  rightarg = issn,
  function = isnle
);

create operator <= (
  leftarg = issn13,
  rightarg = issn13,
  function = isnle
);

create operator <= (
  leftarg = upc,
  rightarg = ean13,
  function = isnle
);

create operator <= (
  leftarg = upc,
  rightarg = upc,
  function = isnle
);

create operator <> (
  leftarg = ean13,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = isbn,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = isbn13,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = ismn,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = ismn13,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = issn,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = issn13,
  function = isnne
);

create operator <> (
  leftarg = ean13,
  rightarg = upc,
  function = isnne
);

create operator <> (
  leftarg = isbn,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = isbn,
  rightarg = isbn,
  function = isnne
);

create operator <> (
  leftarg = isbn,
  rightarg = isbn13,
  function = isnne
);

create operator <> (
  leftarg = isbn13,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = isbn13,
  rightarg = isbn,
  function = isnne
);

create operator <> (
  leftarg = isbn13,
  rightarg = isbn13,
  function = isnne
);

create operator <> (
  leftarg = ismn,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = ismn,
  rightarg = ismn,
  function = isnne
);

create operator <> (
  leftarg = ismn,
  rightarg = ismn13,
  function = isnne
);

create operator <> (
  leftarg = ismn13,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = ismn13,
  rightarg = ismn,
  function = isnne
);

create operator <> (
  leftarg = ismn13,
  rightarg = ismn13,
  function = isnne
);

create operator <> (
  leftarg = issn,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = issn,
  rightarg = issn,
  function = isnne
);

create operator <> (
  leftarg = issn,
  rightarg = issn13,
  function = isnne
);

create operator <> (
  leftarg = issn13,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = issn13,
  rightarg = issn,
  function = isnne
);

create operator <> (
  leftarg = issn13,
  rightarg = issn13,
  function = isnne
);

create operator <> (
  leftarg = upc,
  rightarg = ean13,
  function = isnne
);

create operator <> (
  leftarg = upc,
  rightarg = upc,
  function = isnne
);

create operator = (
  leftarg = ean13,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = isbn,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = isbn13,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = ismn,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = ismn13,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = issn,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = issn13,
  function = isneq
);

create operator = (
  leftarg = ean13,
  rightarg = upc,
  function = isneq
);

create operator = (
  leftarg = isbn,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = isbn,
  rightarg = isbn,
  function = isneq
);

create operator = (
  leftarg = isbn,
  rightarg = isbn13,
  function = isneq
);

create operator = (
  leftarg = isbn13,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = isbn13,
  rightarg = isbn,
  function = isneq
);

create operator = (
  leftarg = isbn13,
  rightarg = isbn13,
  function = isneq
);

create operator = (
  leftarg = ismn,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = ismn,
  rightarg = ismn,
  function = isneq
);

create operator = (
  leftarg = ismn,
  rightarg = ismn13,
  function = isneq
);

create operator = (
  leftarg = ismn13,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = ismn13,
  rightarg = ismn,
  function = isneq
);

create operator = (
  leftarg = ismn13,
  rightarg = ismn13,
  function = isneq
);

create operator = (
  leftarg = issn,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = issn,
  rightarg = issn,
  function = isneq
);

create operator = (
  leftarg = issn,
  rightarg = issn13,
  function = isneq
);

create operator = (
  leftarg = issn13,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = issn13,
  rightarg = issn,
  function = isneq
);

create operator = (
  leftarg = issn13,
  rightarg = issn13,
  function = isneq
);

create operator = (
  leftarg = upc,
  rightarg = ean13,
  function = isneq
);

create operator = (
  leftarg = upc,
  rightarg = upc,
  function = isneq
);

create operator > (
  leftarg = ean13,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = isbn,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = isbn13,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = ismn,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = ismn13,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = issn,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = issn13,
  function = isngt
);

create operator > (
  leftarg = ean13,
  rightarg = upc,
  function = isngt
);

create operator > (
  leftarg = isbn,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = isbn,
  rightarg = isbn,
  function = isngt
);

create operator > (
  leftarg = isbn,
  rightarg = isbn13,
  function = isngt
);

create operator > (
  leftarg = isbn13,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = isbn13,
  rightarg = isbn,
  function = isngt
);

create operator > (
  leftarg = isbn13,
  rightarg = isbn13,
  function = isngt
);

create operator > (
  leftarg = ismn,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = ismn,
  rightarg = ismn,
  function = isngt
);

create operator > (
  leftarg = ismn,
  rightarg = ismn13,
  function = isngt
);

create operator > (
  leftarg = ismn13,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = ismn13,
  rightarg = ismn,
  function = isngt
);

create operator > (
  leftarg = ismn13,
  rightarg = ismn13,
  function = isngt
);

create operator > (
  leftarg = issn,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = issn,
  rightarg = issn,
  function = isngt
);

create operator > (
  leftarg = issn,
  rightarg = issn13,
  function = isngt
);

create operator > (
  leftarg = issn13,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = issn13,
  rightarg = issn,
  function = isngt
);

create operator > (
  leftarg = issn13,
  rightarg = issn13,
  function = isngt
);

create operator > (
  leftarg = upc,
  rightarg = ean13,
  function = isngt
);

create operator > (
  leftarg = upc,
  rightarg = upc,
  function = isngt
);

create operator >= (
  leftarg = ean13,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = isbn,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = isbn13,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = ismn,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = ismn13,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = issn,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = issn13,
  function = isnge
);

create operator >= (
  leftarg = ean13,
  rightarg = upc,
  function = isnge
);

create operator >= (
  leftarg = isbn,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = isbn,
  rightarg = isbn,
  function = isnge
);

create operator >= (
  leftarg = isbn,
  rightarg = isbn13,
  function = isnge
);

create operator >= (
  leftarg = isbn13,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = isbn13,
  rightarg = isbn,
  function = isnge
);

create operator >= (
  leftarg = isbn13,
  rightarg = isbn13,
  function = isnge
);

create operator >= (
  leftarg = ismn,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = ismn,
  rightarg = ismn,
  function = isnge
);

create operator >= (
  leftarg = ismn,
  rightarg = ismn13,
  function = isnge
);

create operator >= (
  leftarg = ismn13,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = ismn13,
  rightarg = ismn,
  function = isnge
);

create operator >= (
  leftarg = ismn13,
  rightarg = ismn13,
  function = isnge
);

create operator >= (
  leftarg = issn,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = issn,
  rightarg = issn,
  function = isnge
);

create operator >= (
  leftarg = issn,
  rightarg = issn13,
  function = isnge
);

create operator >= (
  leftarg = issn13,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = issn13,
  rightarg = issn,
  function = isnge
);

create operator >= (
  leftarg = issn13,
  rightarg = issn13,
  function = isnge
);

create operator >= (
  leftarg = upc,
  rightarg = ean13,
  function = isnge
);

create operator >= (
  leftarg = upc,
  rightarg = upc,
  function = isnge
);

create operator family isn_ops using btree;

create operator family isn_ops using hash;

create operator class ean13_ops default for type ean13 using btree family isn_ops as storage ean13;

create operator class ean13_ops default for type ean13 using hash family isn_ops as storage ean13;

create operator class isbn13_ops default for type isbn13 using btree family isn_ops as storage isbn13;

create operator class isbn13_ops default for type isbn13 using hash family isn_ops as storage isbn13;

create operator class isbn_ops default for type isbn using btree family isn_ops as storage isbn;

create operator class isbn_ops default for type isbn using hash family isn_ops as storage isbn;

create operator class ismn13_ops default for type ismn13 using btree family isn_ops as storage ismn13;

create operator class ismn13_ops default for type ismn13 using hash family isn_ops as storage ismn13;

create operator class ismn_ops default for type ismn using btree family isn_ops as storage ismn;

create operator class ismn_ops default for type ismn using hash family isn_ops as storage ismn;

create operator class issn13_ops default for type issn13 using btree family isn_ops as storage issn13;

create operator class issn13_ops default for type issn13 using hash family isn_ops as storage issn13;

create operator class issn_ops default for type issn using btree family isn_ops as storage issn;

create operator class issn_ops default for type issn using hash family isn_ops as storage issn;

create operator class upc_ops default for type upc using btree family isn_ops as storage upc;

create operator class upc_ops default for type upc using hash family isn_ops as storage upc;

create cast (ean13 as isbn) with function isbn(ean13);

create cast (ean13 as isbn13) with function isbn13(ean13);

create cast (ean13 as ismn) with function ismn(ean13);

create cast (ean13 as ismn13) with function ismn13(ean13);

create cast (ean13 as issn) with function issn(ean13);

create cast (ean13 as issn13) with function issn13(ean13);

create cast (ean13 as upc) with function upc(ean13);

create cast (isbn as ean13) without function as assignment;

create cast (isbn as isbn13) without function as assignment;

create cast (isbn13 as ean13) without function as assignment;

create cast (isbn13 as isbn) without function as assignment;

create cast (ismn as ean13) without function as assignment;

create cast (ismn as ismn13) without function as assignment;

create cast (ismn13 as ean13) without function as assignment;

create cast (ismn13 as ismn) without function as assignment;

create cast (issn as ean13) without function as assignment;

create cast (issn as issn13) without function as assignment;

create cast (issn13 as ean13) without function as assignment;

create cast (issn13 as issn) without function as assignment;

create cast (upc as ean13) without function as assignment;

