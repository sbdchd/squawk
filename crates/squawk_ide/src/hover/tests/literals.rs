use insta::assert_snapshot;

use super::hover;

#[test]
fn string() {
    assert_snapshot!(hover(r"
select 'foo$0';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select 'foo';
      │        ───── hover
      │
      ╰ note: value of literal: ` foo `
    ");
}

#[test]
fn string_with_doubled_quote() {
    assert_snapshot!(hover(r"
select 'it''$0s';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select 'it''s';
      │        ─────── hover
      │
      ╰ note: value of literal: ` it's `
    ");
}

#[test]
fn string_only_quote() {
    assert_snapshot!(hover(r"
select '''$0';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select '''';
      │        ──── hover
      │
      ╰ note: value of literal: ` ' `
    ");
}

#[test]
fn string_with_backtick() {
    assert_snapshot!(hover(r"
select 'a`$0b';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select 'a`b';
      │        ───── hover
      │
      ╰ note: value of literal: `` a`b ``
    ");
}

#[test]
fn string_with_leading_backtick() {
    assert_snapshot!(hover(r"
select '`$0hello';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select '`hello';
      │        ──────── hover
      │
      ╰ note: value of literal: `` `hello ``
    ");
}

#[test]
fn string_with_consecutive_backticks() {
    assert_snapshot!(hover(r"
select 'a``$0b';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select 'a``b';
      │        ────── hover
      │
      ╰ note: value of literal: ``` a``b ```
    ");
}

#[test]
fn esc_string_with_newline() {
    assert_snapshot!(hover(r"
select e'fo$0o\nbar';
"), @r"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select e'foo\nbar';
      │        ─────────── hover
      │
      ╰ note: value of literal (truncated up to newline): ` foo `
    ");
}

#[test]
fn esc_string_with_cr() {
    assert_snapshot!(hover(r"
select e'fo$0o\rbar';
"), @r"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select e'foo\rbar';
      │        ─────────── hover
      │
      ╰ note: value of literal (truncated up to newline): ` foo `
    ");
}

#[test]
fn esc_string_with_tab() {
    assert_snapshot!(hover(r"
select e'a\tb$0';
"), @r"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select e'a\tb';
      │        ─────── hover
      │
      ╰ note: value of literal: ` a	b `
    ");
}

#[test]
fn esc_string_utf8_bytes() {
    assert_snapshot!(hover(r"
select e'\xC3\xA$09';
"), @r"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select e'\xC3\xA9';
      │        ─────────── hover
      │
      ╰ note: value of literal: ` é `
    ");
}

#[test]
fn string_continuation() {
    assert_snapshot!(hover(r"
select e'foo$0'
'\nbar';
"), @r"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │   select e'foo'
      │ ┌────────┘
    3 │ │ '\nbar';
      │ └───────┘ hover
      │
      ╰ note: value of literal (truncated up to newline): ` foo `
    ");
}

#[test]
fn national_string() {
    assert_snapshot!(hover(r"
select N'fo$0o';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select N'foo';
      │        ────── hover
      │
      ╰ note: value of literal: ` foo `
    ");
}

#[test]
fn unicode_esc_string() {
    assert_snapshot!(hover(r"
select U&'d\0061t\+00006$01';
"), @r"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select U&'d\0061t\+000061';
      │        ─────────────────── hover
      │
      ╰ note: value of literal: ` data `
    ");
}

#[test]
fn unicode_esc_string_uescape() {
    assert_snapshot!(hover(r"
select U&'!0061!0062$0' uescape '!';
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select U&'!0061!0062' uescape '!';
      │        ────────────────────────── hover
      │
      ╰ note: value of literal: ` ab `
    ");
}

#[test]
fn dollar_quoted_string() {
    assert_snapshot!(hover(r"
select $$he$0llo$$;
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select $$hello$$;
      │        ───────── hover
      │
      ╰ note: value of literal: ` hello `
    ");
}

#[test]
fn bit_string() {
    assert_snapshot!(hover(r"
select b'10$010';
"), @"
    hover: bit
      ╭▸ file0.sql:2:8
      │
    2 │ select b'1010';
      │        ─────── hover
      │
      ╰ note: value of literal: ` x'A'|b'1010' `
    ");
}

#[test]
fn bit_string_keeps_width() {
    assert_snapshot!(hover(r"
select b'0$00';
"), @"
    hover: bit
      ╭▸ file0.sql:2:8
      │
    2 │ select b'00';
      │        ───── hover
      │
      ╰ note: value of literal: ` x'0'|b'00' `
    ");
}

#[test]
fn bit_string_empty() {
    assert_snapshot!(hover(r"
select b'$0';
"), @"
    hover: bit
      ╭▸ file0.sql:2:8
      │
    2 │ select b'';
      │        ─── hover
      │
      ╰ note: value of literal: ` x''|b'' `
    ");
}

#[test]
fn byte_string() {
    assert_snapshot!(hover(r"
select x'1A$03F';
"), @"
    hover: bit
      ╭▸ file0.sql:2:8
      │
    2 │ select x'1A3F';
      │        ─────── hover
      │
      ╰ note: value of literal: ` x'1A3F'|b'0001101000111111' `
    ");
}

#[test]
fn byte_string_keeps_width() {
    assert_snapshot!(hover(r"
select x'0F$0F';
"), @"
    hover: bit
      ╭▸ file0.sql:2:8
      │
    2 │ select x'0FF';
      │        ────── hover
      │
      ╰ note: value of literal: ` x'0FF'|b'000011111111' `
    ");
}

#[test]
fn byte_string_empty() {
    assert_snapshot!(hover(r"
select x'$0';
"), @"
    hover: bit
      ╭▸ file0.sql:2:8
      │
    2 │ select x'';
      │        ─── hover
      │
      ╰ note: value of literal: ` x''|b'' `
    ");
}

#[test]
fn cursor_at_end() {
    assert_snapshot!(hover(r"
select 'foo'$0;
"), @"
    hover: text
      ╭▸ file0.sql:2:8
      │
    2 │ select 'foo';
      │        ───── hover
      │
      ╰ note: value of literal: ` foo `
    ");
}

#[test]
fn integer_literal() {
    assert_snapshot!(hover(r"
select 1$0;
"), @"no hover");
}

#[test]
fn null_literal() {
    assert_snapshot!(hover(r"
select null$0;
"), @"no hover");
}

#[test]
fn enum_label_uses_semantic_hover() {
    assert_snapshot!(hover(r"
create type mood as enum ('happy', 'sad');
create table t (m mood);
select * from t where m = 'sad$0';
"), @"
    hover: enum label 'sad' of public.mood
      ╭▸ file0.sql:4:27
      │
    4 │ select * from t where m = 'sad';
      ╰╴                          ───── hover
    ");
}

#[test]
fn regclass_string_uses_semantic_hover() {
    assert_snapshot!(hover(r"
create sequence s;
select nextval('s$0');
"), @"
    hover: sequence public.s
      ╭▸ file0.sql:3:17
      │
    3 │ select nextval('s');
      ╰╴                ─ hover
    ");
}

#[test]
fn string_in_function_body() {
    assert_snapshot!(hover(r"
create function f() returns text
  language sql
  return 'he$0llo';
"), @"
    hover: text
      ╭▸ file0.sql:4:10
      │
    4 │   return 'hello';
      │          ─────── hover
      │
      ╰ note: value of literal: ` hello `
    ");
}

#[test]
fn string_in_dollar_quoted_function_body() {
    assert_snapshot!(hover(r"
create function f() returns text
  language sql
  as $$ select 'he$0llo' $$;
"), @"
    hover: text
      ╭▸ file0.sql:4:16
      │
    4 │   as $$ select 'hello' $$;
      │                ─────── hover
      │
      ╰ note: value of literal: ` hello `
    ");
}

#[test]
fn string_in_quoted_function_body() {
    assert_snapshot!(hover(r"
create function f() returns text
  language sql
  as 'select ''he$0llo''';
"), @"
    hover: text
      ╭▸ file0.sql:4:14
      │
    4 │   as 'select ''hello''';
      │              ───────── hover
      │
      ╰ note: value of literal: ` hello `
    ");
}

#[test]
fn whitespace_after_string() {
    assert_snapshot!(hover(r"
select 'foo' $0;
"), @"no hover");
}
