-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

-- International European Article Number (EAN13)
-- size: 8, align: 8
create type public.ean13;

-- International Standard Book Number (ISBN)
-- size: 8, align: 8
create type public.isbn;

-- International Standard Book Number 13 (ISBN13)
-- size: 8, align: 8
create type public.isbn13;

-- International Standard Music Number (ISMN)
-- size: 8, align: 8
create type public.ismn;

-- International Standard Music Number 13 (ISMN13)
-- size: 8, align: 8
create type public.ismn13;

-- International Standard Serial Number (ISSN)
-- size: 8, align: 8
create type public.issn;

-- International Standard Serial Number 13 (ISSN13)
-- size: 8, align: 8
create type public.issn13;

-- Universal Product Code (UPC)
-- size: 8, align: 8
create type public.upc;

create function public.btean13cmp(ean13, ean13) returns integer
  language internal;

create function public.btean13cmp(ean13, isbn) returns integer
  language internal;

create function public.btean13cmp(ean13, isbn13) returns integer
  language internal;

create function public.btean13cmp(ean13, ismn) returns integer
  language internal;

create function public.btean13cmp(ean13, ismn13) returns integer
  language internal;

create function public.btean13cmp(ean13, issn) returns integer
  language internal;

create function public.btean13cmp(ean13, issn13) returns integer
  language internal;

create function public.btean13cmp(ean13, upc) returns integer
  language internal;

create function public.btisbn13cmp(isbn13, ean13) returns integer
  language internal;

create function public.btisbn13cmp(isbn13, isbn) returns integer
  language internal;

create function public.btisbn13cmp(isbn13, isbn13) returns integer
  language internal;

create function public.btisbncmp(isbn, ean13) returns integer
  language internal;

create function public.btisbncmp(isbn, isbn) returns integer
  language internal;

create function public.btisbncmp(isbn, isbn13) returns integer
  language internal;

create function public.btismn13cmp(ismn13, ean13) returns integer
  language internal;

create function public.btismn13cmp(ismn13, ismn) returns integer
  language internal;

create function public.btismn13cmp(ismn13, ismn13) returns integer
  language internal;

create function public.btismncmp(ismn, ean13) returns integer
  language internal;

create function public.btismncmp(ismn, ismn) returns integer
  language internal;

create function public.btismncmp(ismn, ismn13) returns integer
  language internal;

create function public.btissn13cmp(issn13, ean13) returns integer
  language internal;

create function public.btissn13cmp(issn13, issn) returns integer
  language internal;

create function public.btissn13cmp(issn13, issn13) returns integer
  language internal;

create function public.btissncmp(issn, ean13) returns integer
  language internal;

create function public.btissncmp(issn, issn) returns integer
  language internal;

create function public.btissncmp(issn, issn13) returns integer
  language internal;

create function public.btupccmp(upc, ean13) returns integer
  language internal;

create function public.btupccmp(upc, upc) returns integer
  language internal;

create function public.ean13_in(cstring) returns ean13
  language c;

create function public.ean13_out(ean13) returns cstring
  language c;

create function public.ean13_out(isbn13) returns cstring
  language c;

create function public.ean13_out(ismn13) returns cstring
  language c;

create function public.ean13_out(issn13) returns cstring
  language c;

create function public.hashean13(ean13) returns integer
  language internal;

create function public.hashisbn(isbn) returns integer
  language internal;

create function public.hashisbn13(isbn13) returns integer
  language internal;

create function public.hashismn(ismn) returns integer
  language internal;

create function public.hashismn13(ismn13) returns integer
  language internal;

create function public.hashissn(issn) returns integer
  language internal;

create function public.hashissn13(issn13) returns integer
  language internal;

create function public.hashupc(upc) returns integer
  language internal;

create function public.is_valid(ean13) returns boolean
  language c;

create function public.is_valid(isbn) returns boolean
  language c;

create function public.is_valid(isbn13) returns boolean
  language c;

create function public.is_valid(ismn) returns boolean
  language c;

create function public.is_valid(ismn13) returns boolean
  language c;

create function public.is_valid(issn) returns boolean
  language c;

create function public.is_valid(issn13) returns boolean
  language c;

create function public.is_valid(upc) returns boolean
  language c;

create function public.isbn(ean13) returns isbn
  language c;

create function public.isbn13(ean13) returns isbn13
  language c;

create function public.isbn13_in(cstring) returns isbn13
  language c;

create function public.isbn_in(cstring) returns isbn
  language c;

create function public.ismn(ean13) returns ismn
  language c;

create function public.ismn13(ean13) returns ismn13
  language c;

create function public.ismn13_in(cstring) returns ismn13
  language c;

create function public.ismn_in(cstring) returns ismn
  language c;

create function public.isn_out(isbn) returns cstring
  language c;

create function public.isn_out(ismn) returns cstring
  language c;

create function public.isn_out(issn) returns cstring
  language c;

create function public.isn_out(upc) returns cstring
  language c;

create function public.isn_weak() returns boolean
  language c;

create function public.isn_weak(boolean) returns boolean
  language c;

create function public.isneq(ean13, ean13) returns boolean
  language internal;

create function public.isneq(ean13, isbn) returns boolean
  language internal;

create function public.isneq(ean13, isbn13) returns boolean
  language internal;

create function public.isneq(ean13, ismn) returns boolean
  language internal;

create function public.isneq(ean13, ismn13) returns boolean
  language internal;

create function public.isneq(ean13, issn) returns boolean
  language internal;

create function public.isneq(ean13, issn13) returns boolean
  language internal;

create function public.isneq(ean13, upc) returns boolean
  language internal;

create function public.isneq(isbn, ean13) returns boolean
  language internal;

create function public.isneq(isbn, isbn) returns boolean
  language internal;

create function public.isneq(isbn, isbn13) returns boolean
  language internal;

create function public.isneq(isbn13, ean13) returns boolean
  language internal;

create function public.isneq(isbn13, isbn) returns boolean
  language internal;

create function public.isneq(isbn13, isbn13) returns boolean
  language internal;

create function public.isneq(ismn, ean13) returns boolean
  language internal;

create function public.isneq(ismn, ismn) returns boolean
  language internal;

create function public.isneq(ismn, ismn13) returns boolean
  language internal;

create function public.isneq(ismn13, ean13) returns boolean
  language internal;

create function public.isneq(ismn13, ismn) returns boolean
  language internal;

create function public.isneq(ismn13, ismn13) returns boolean
  language internal;

create function public.isneq(issn, ean13) returns boolean
  language internal;

create function public.isneq(issn, issn) returns boolean
  language internal;

create function public.isneq(issn, issn13) returns boolean
  language internal;

create function public.isneq(issn13, ean13) returns boolean
  language internal;

create function public.isneq(issn13, issn) returns boolean
  language internal;

create function public.isneq(issn13, issn13) returns boolean
  language internal;

create function public.isneq(upc, ean13) returns boolean
  language internal;

create function public.isneq(upc, upc) returns boolean
  language internal;

create function public.isnge(ean13, ean13) returns boolean
  language internal;

create function public.isnge(ean13, isbn) returns boolean
  language internal;

create function public.isnge(ean13, isbn13) returns boolean
  language internal;

create function public.isnge(ean13, ismn) returns boolean
  language internal;

create function public.isnge(ean13, ismn13) returns boolean
  language internal;

create function public.isnge(ean13, issn) returns boolean
  language internal;

create function public.isnge(ean13, issn13) returns boolean
  language internal;

create function public.isnge(ean13, upc) returns boolean
  language internal;

create function public.isnge(isbn, ean13) returns boolean
  language internal;

create function public.isnge(isbn, isbn) returns boolean
  language internal;

create function public.isnge(isbn, isbn13) returns boolean
  language internal;

create function public.isnge(isbn13, ean13) returns boolean
  language internal;

create function public.isnge(isbn13, isbn) returns boolean
  language internal;

create function public.isnge(isbn13, isbn13) returns boolean
  language internal;

create function public.isnge(ismn, ean13) returns boolean
  language internal;

create function public.isnge(ismn, ismn) returns boolean
  language internal;

create function public.isnge(ismn, ismn13) returns boolean
  language internal;

create function public.isnge(ismn13, ean13) returns boolean
  language internal;

create function public.isnge(ismn13, ismn) returns boolean
  language internal;

create function public.isnge(ismn13, ismn13) returns boolean
  language internal;

create function public.isnge(issn, ean13) returns boolean
  language internal;

create function public.isnge(issn, issn) returns boolean
  language internal;

create function public.isnge(issn, issn13) returns boolean
  language internal;

create function public.isnge(issn13, ean13) returns boolean
  language internal;

create function public.isnge(issn13, issn) returns boolean
  language internal;

create function public.isnge(issn13, issn13) returns boolean
  language internal;

create function public.isnge(upc, ean13) returns boolean
  language internal;

create function public.isnge(upc, upc) returns boolean
  language internal;

create function public.isngt(ean13, ean13) returns boolean
  language internal;

create function public.isngt(ean13, isbn) returns boolean
  language internal;

create function public.isngt(ean13, isbn13) returns boolean
  language internal;

create function public.isngt(ean13, ismn) returns boolean
  language internal;

create function public.isngt(ean13, ismn13) returns boolean
  language internal;

create function public.isngt(ean13, issn) returns boolean
  language internal;

create function public.isngt(ean13, issn13) returns boolean
  language internal;

create function public.isngt(ean13, upc) returns boolean
  language internal;

create function public.isngt(isbn, ean13) returns boolean
  language internal;

create function public.isngt(isbn, isbn) returns boolean
  language internal;

create function public.isngt(isbn, isbn13) returns boolean
  language internal;

create function public.isngt(isbn13, ean13) returns boolean
  language internal;

create function public.isngt(isbn13, isbn) returns boolean
  language internal;

create function public.isngt(isbn13, isbn13) returns boolean
  language internal;

create function public.isngt(ismn, ean13) returns boolean
  language internal;

create function public.isngt(ismn, ismn) returns boolean
  language internal;

create function public.isngt(ismn, ismn13) returns boolean
  language internal;

create function public.isngt(ismn13, ean13) returns boolean
  language internal;

create function public.isngt(ismn13, ismn) returns boolean
  language internal;

create function public.isngt(ismn13, ismn13) returns boolean
  language internal;

create function public.isngt(issn, ean13) returns boolean
  language internal;

create function public.isngt(issn, issn) returns boolean
  language internal;

create function public.isngt(issn, issn13) returns boolean
  language internal;

create function public.isngt(issn13, ean13) returns boolean
  language internal;

create function public.isngt(issn13, issn) returns boolean
  language internal;

create function public.isngt(issn13, issn13) returns boolean
  language internal;

create function public.isngt(upc, ean13) returns boolean
  language internal;

create function public.isngt(upc, upc) returns boolean
  language internal;

create function public.isnle(ean13, ean13) returns boolean
  language internal;

create function public.isnle(ean13, isbn) returns boolean
  language internal;

create function public.isnle(ean13, isbn13) returns boolean
  language internal;

create function public.isnle(ean13, ismn) returns boolean
  language internal;

create function public.isnle(ean13, ismn13) returns boolean
  language internal;

create function public.isnle(ean13, issn) returns boolean
  language internal;

create function public.isnle(ean13, issn13) returns boolean
  language internal;

create function public.isnle(ean13, upc) returns boolean
  language internal;

create function public.isnle(isbn, ean13) returns boolean
  language internal;

create function public.isnle(isbn, isbn) returns boolean
  language internal;

create function public.isnle(isbn, isbn13) returns boolean
  language internal;

create function public.isnle(isbn13, ean13) returns boolean
  language internal;

create function public.isnle(isbn13, isbn) returns boolean
  language internal;

create function public.isnle(isbn13, isbn13) returns boolean
  language internal;

create function public.isnle(ismn, ean13) returns boolean
  language internal;

create function public.isnle(ismn, ismn) returns boolean
  language internal;

create function public.isnle(ismn, ismn13) returns boolean
  language internal;

create function public.isnle(ismn13, ean13) returns boolean
  language internal;

create function public.isnle(ismn13, ismn) returns boolean
  language internal;

create function public.isnle(ismn13, ismn13) returns boolean
  language internal;

create function public.isnle(issn, ean13) returns boolean
  language internal;

create function public.isnle(issn, issn) returns boolean
  language internal;

create function public.isnle(issn, issn13) returns boolean
  language internal;

create function public.isnle(issn13, ean13) returns boolean
  language internal;

create function public.isnle(issn13, issn) returns boolean
  language internal;

create function public.isnle(issn13, issn13) returns boolean
  language internal;

create function public.isnle(upc, ean13) returns boolean
  language internal;

create function public.isnle(upc, upc) returns boolean
  language internal;

create function public.isnlt(ean13, ean13) returns boolean
  language internal;

create function public.isnlt(ean13, isbn) returns boolean
  language internal;

create function public.isnlt(ean13, isbn13) returns boolean
  language internal;

create function public.isnlt(ean13, ismn) returns boolean
  language internal;

create function public.isnlt(ean13, ismn13) returns boolean
  language internal;

create function public.isnlt(ean13, issn) returns boolean
  language internal;

create function public.isnlt(ean13, issn13) returns boolean
  language internal;

create function public.isnlt(ean13, upc) returns boolean
  language internal;

create function public.isnlt(isbn, ean13) returns boolean
  language internal;

create function public.isnlt(isbn, isbn) returns boolean
  language internal;

create function public.isnlt(isbn, isbn13) returns boolean
  language internal;

create function public.isnlt(isbn13, ean13) returns boolean
  language internal;

create function public.isnlt(isbn13, isbn) returns boolean
  language internal;

create function public.isnlt(isbn13, isbn13) returns boolean
  language internal;

create function public.isnlt(ismn, ean13) returns boolean
  language internal;

create function public.isnlt(ismn, ismn) returns boolean
  language internal;

create function public.isnlt(ismn, ismn13) returns boolean
  language internal;

create function public.isnlt(ismn13, ean13) returns boolean
  language internal;

create function public.isnlt(ismn13, ismn) returns boolean
  language internal;

create function public.isnlt(ismn13, ismn13) returns boolean
  language internal;

create function public.isnlt(issn, ean13) returns boolean
  language internal;

create function public.isnlt(issn, issn) returns boolean
  language internal;

create function public.isnlt(issn, issn13) returns boolean
  language internal;

create function public.isnlt(issn13, ean13) returns boolean
  language internal;

create function public.isnlt(issn13, issn) returns boolean
  language internal;

create function public.isnlt(issn13, issn13) returns boolean
  language internal;

create function public.isnlt(upc, ean13) returns boolean
  language internal;

create function public.isnlt(upc, upc) returns boolean
  language internal;

create function public.isnne(ean13, ean13) returns boolean
  language internal;

create function public.isnne(ean13, isbn) returns boolean
  language internal;

create function public.isnne(ean13, isbn13) returns boolean
  language internal;

create function public.isnne(ean13, ismn) returns boolean
  language internal;

create function public.isnne(ean13, ismn13) returns boolean
  language internal;

create function public.isnne(ean13, issn) returns boolean
  language internal;

create function public.isnne(ean13, issn13) returns boolean
  language internal;

create function public.isnne(ean13, upc) returns boolean
  language internal;

create function public.isnne(isbn, ean13) returns boolean
  language internal;

create function public.isnne(isbn, isbn) returns boolean
  language internal;

create function public.isnne(isbn, isbn13) returns boolean
  language internal;

create function public.isnne(isbn13, ean13) returns boolean
  language internal;

create function public.isnne(isbn13, isbn) returns boolean
  language internal;

create function public.isnne(isbn13, isbn13) returns boolean
  language internal;

create function public.isnne(ismn, ean13) returns boolean
  language internal;

create function public.isnne(ismn, ismn) returns boolean
  language internal;

create function public.isnne(ismn, ismn13) returns boolean
  language internal;

create function public.isnne(ismn13, ean13) returns boolean
  language internal;

create function public.isnne(ismn13, ismn) returns boolean
  language internal;

create function public.isnne(ismn13, ismn13) returns boolean
  language internal;

create function public.isnne(issn, ean13) returns boolean
  language internal;

create function public.isnne(issn, issn) returns boolean
  language internal;

create function public.isnne(issn, issn13) returns boolean
  language internal;

create function public.isnne(issn13, ean13) returns boolean
  language internal;

create function public.isnne(issn13, issn) returns boolean
  language internal;

create function public.isnne(issn13, issn13) returns boolean
  language internal;

create function public.isnne(upc, ean13) returns boolean
  language internal;

create function public.isnne(upc, upc) returns boolean
  language internal;

create function public.issn(ean13) returns issn
  language c;

create function public.issn13(ean13) returns issn13
  language c;

create function public.issn13_in(cstring) returns issn13
  language c;

create function public.issn_in(cstring) returns issn
  language c;

create function public.make_valid(ean13) returns ean13
  language c;

create function public.make_valid(isbn) returns isbn
  language c;

create function public.make_valid(isbn13) returns isbn13
  language c;

create function public.make_valid(ismn) returns ismn
  language c;

create function public.make_valid(ismn13) returns ismn13
  language c;

create function public.make_valid(issn) returns issn
  language c;

create function public.make_valid(issn13) returns issn13
  language c;

create function public.make_valid(upc) returns upc
  language c;

create function public.upc(ean13) returns upc
  language c;

create function public.upc_in(cstring) returns upc
  language c;

create operator public.< (
  leftarg = ean13,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = isbn,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = isbn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = ismn,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = ismn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = issn,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = issn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ean13,
  rightarg = upc,
  function = public.isnlt
);

create operator public.< (
  leftarg = isbn,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = isbn,
  rightarg = isbn,
  function = public.isnlt
);

create operator public.< (
  leftarg = isbn,
  rightarg = isbn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = isbn13,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = isbn13,
  rightarg = isbn,
  function = public.isnlt
);

create operator public.< (
  leftarg = isbn13,
  rightarg = isbn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ismn,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ismn,
  rightarg = ismn,
  function = public.isnlt
);

create operator public.< (
  leftarg = ismn,
  rightarg = ismn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ismn13,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = ismn13,
  rightarg = ismn,
  function = public.isnlt
);

create operator public.< (
  leftarg = ismn13,
  rightarg = ismn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = issn,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = issn,
  rightarg = issn,
  function = public.isnlt
);

create operator public.< (
  leftarg = issn,
  rightarg = issn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = issn13,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = issn13,
  rightarg = issn,
  function = public.isnlt
);

create operator public.< (
  leftarg = issn13,
  rightarg = issn13,
  function = public.isnlt
);

create operator public.< (
  leftarg = upc,
  rightarg = ean13,
  function = public.isnlt
);

create operator public.< (
  leftarg = upc,
  rightarg = upc,
  function = public.isnlt
);

create operator public.<= (
  leftarg = ean13,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = isbn,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = isbn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = ismn,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = ismn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = issn,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = issn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ean13,
  rightarg = upc,
  function = public.isnle
);

create operator public.<= (
  leftarg = isbn,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = isbn,
  rightarg = isbn,
  function = public.isnle
);

create operator public.<= (
  leftarg = isbn,
  rightarg = isbn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = isbn13,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = isbn13,
  rightarg = isbn,
  function = public.isnle
);

create operator public.<= (
  leftarg = isbn13,
  rightarg = isbn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ismn,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ismn,
  rightarg = ismn,
  function = public.isnle
);

create operator public.<= (
  leftarg = ismn,
  rightarg = ismn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ismn13,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = ismn13,
  rightarg = ismn,
  function = public.isnle
);

create operator public.<= (
  leftarg = ismn13,
  rightarg = ismn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = issn,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = issn,
  rightarg = issn,
  function = public.isnle
);

create operator public.<= (
  leftarg = issn,
  rightarg = issn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = issn13,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = issn13,
  rightarg = issn,
  function = public.isnle
);

create operator public.<= (
  leftarg = issn13,
  rightarg = issn13,
  function = public.isnle
);

create operator public.<= (
  leftarg = upc,
  rightarg = ean13,
  function = public.isnle
);

create operator public.<= (
  leftarg = upc,
  rightarg = upc,
  function = public.isnle
);

create operator public.<> (
  leftarg = ean13,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = isbn,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = isbn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = ismn,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = ismn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = issn,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = issn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ean13,
  rightarg = upc,
  function = public.isnne
);

create operator public.<> (
  leftarg = isbn,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = isbn,
  rightarg = isbn,
  function = public.isnne
);

create operator public.<> (
  leftarg = isbn,
  rightarg = isbn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = isbn13,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = isbn13,
  rightarg = isbn,
  function = public.isnne
);

create operator public.<> (
  leftarg = isbn13,
  rightarg = isbn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ismn,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ismn,
  rightarg = ismn,
  function = public.isnne
);

create operator public.<> (
  leftarg = ismn,
  rightarg = ismn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ismn13,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = ismn13,
  rightarg = ismn,
  function = public.isnne
);

create operator public.<> (
  leftarg = ismn13,
  rightarg = ismn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = issn,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = issn,
  rightarg = issn,
  function = public.isnne
);

create operator public.<> (
  leftarg = issn,
  rightarg = issn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = issn13,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = issn13,
  rightarg = issn,
  function = public.isnne
);

create operator public.<> (
  leftarg = issn13,
  rightarg = issn13,
  function = public.isnne
);

create operator public.<> (
  leftarg = upc,
  rightarg = ean13,
  function = public.isnne
);

create operator public.<> (
  leftarg = upc,
  rightarg = upc,
  function = public.isnne
);

create operator public.= (
  leftarg = ean13,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = isbn,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = isbn13,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = ismn,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = ismn13,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = issn,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = issn13,
  function = public.isneq
);

create operator public.= (
  leftarg = ean13,
  rightarg = upc,
  function = public.isneq
);

create operator public.= (
  leftarg = isbn,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = isbn,
  rightarg = isbn,
  function = public.isneq
);

create operator public.= (
  leftarg = isbn,
  rightarg = isbn13,
  function = public.isneq
);

create operator public.= (
  leftarg = isbn13,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = isbn13,
  rightarg = isbn,
  function = public.isneq
);

create operator public.= (
  leftarg = isbn13,
  rightarg = isbn13,
  function = public.isneq
);

create operator public.= (
  leftarg = ismn,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = ismn,
  rightarg = ismn,
  function = public.isneq
);

create operator public.= (
  leftarg = ismn,
  rightarg = ismn13,
  function = public.isneq
);

create operator public.= (
  leftarg = ismn13,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = ismn13,
  rightarg = ismn,
  function = public.isneq
);

create operator public.= (
  leftarg = ismn13,
  rightarg = ismn13,
  function = public.isneq
);

create operator public.= (
  leftarg = issn,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = issn,
  rightarg = issn,
  function = public.isneq
);

create operator public.= (
  leftarg = issn,
  rightarg = issn13,
  function = public.isneq
);

create operator public.= (
  leftarg = issn13,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = issn13,
  rightarg = issn,
  function = public.isneq
);

create operator public.= (
  leftarg = issn13,
  rightarg = issn13,
  function = public.isneq
);

create operator public.= (
  leftarg = upc,
  rightarg = ean13,
  function = public.isneq
);

create operator public.= (
  leftarg = upc,
  rightarg = upc,
  function = public.isneq
);

create operator public.> (
  leftarg = ean13,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = isbn,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = isbn13,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = ismn,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = ismn13,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = issn,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = issn13,
  function = public.isngt
);

create operator public.> (
  leftarg = ean13,
  rightarg = upc,
  function = public.isngt
);

create operator public.> (
  leftarg = isbn,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = isbn,
  rightarg = isbn,
  function = public.isngt
);

create operator public.> (
  leftarg = isbn,
  rightarg = isbn13,
  function = public.isngt
);

create operator public.> (
  leftarg = isbn13,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = isbn13,
  rightarg = isbn,
  function = public.isngt
);

create operator public.> (
  leftarg = isbn13,
  rightarg = isbn13,
  function = public.isngt
);

create operator public.> (
  leftarg = ismn,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = ismn,
  rightarg = ismn,
  function = public.isngt
);

create operator public.> (
  leftarg = ismn,
  rightarg = ismn13,
  function = public.isngt
);

create operator public.> (
  leftarg = ismn13,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = ismn13,
  rightarg = ismn,
  function = public.isngt
);

create operator public.> (
  leftarg = ismn13,
  rightarg = ismn13,
  function = public.isngt
);

create operator public.> (
  leftarg = issn,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = issn,
  rightarg = issn,
  function = public.isngt
);

create operator public.> (
  leftarg = issn,
  rightarg = issn13,
  function = public.isngt
);

create operator public.> (
  leftarg = issn13,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = issn13,
  rightarg = issn,
  function = public.isngt
);

create operator public.> (
  leftarg = issn13,
  rightarg = issn13,
  function = public.isngt
);

create operator public.> (
  leftarg = upc,
  rightarg = ean13,
  function = public.isngt
);

create operator public.> (
  leftarg = upc,
  rightarg = upc,
  function = public.isngt
);

create operator public.>= (
  leftarg = ean13,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = isbn,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = isbn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = ismn,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = ismn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = issn,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = issn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ean13,
  rightarg = upc,
  function = public.isnge
);

create operator public.>= (
  leftarg = isbn,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = isbn,
  rightarg = isbn,
  function = public.isnge
);

create operator public.>= (
  leftarg = isbn,
  rightarg = isbn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = isbn13,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = isbn13,
  rightarg = isbn,
  function = public.isnge
);

create operator public.>= (
  leftarg = isbn13,
  rightarg = isbn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ismn,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ismn,
  rightarg = ismn,
  function = public.isnge
);

create operator public.>= (
  leftarg = ismn,
  rightarg = ismn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ismn13,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = ismn13,
  rightarg = ismn,
  function = public.isnge
);

create operator public.>= (
  leftarg = ismn13,
  rightarg = ismn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = issn,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = issn,
  rightarg = issn,
  function = public.isnge
);

create operator public.>= (
  leftarg = issn,
  rightarg = issn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = issn13,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = issn13,
  rightarg = issn,
  function = public.isnge
);

create operator public.>= (
  leftarg = issn13,
  rightarg = issn13,
  function = public.isnge
);

create operator public.>= (
  leftarg = upc,
  rightarg = ean13,
  function = public.isnge
);

create operator public.>= (
  leftarg = upc,
  rightarg = upc,
  function = public.isnge
);

