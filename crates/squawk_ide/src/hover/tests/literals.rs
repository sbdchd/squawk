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
"), @"
    hover: int4
      ╭▸ file0.sql:2:8
      │
    2 │ select 1;
      │        ─ hover
      │
      ╰ note: value of literal: ` 1 (0x1|0b1) `
    ");
}

#[test]
fn bigint_literal() {
    assert_snapshot!(hover(r"
select 3000000000$0;
"), @"
    hover: int8
      ╭▸ file0.sql:2:8
      │
    2 │ select 3000000000;
      │        ────────── hover
      │
      ╰ note: value of literal: ` 3000000000 (0xB2D05E00|0b10110010110100000101111000000000) `
    ");
}

#[test]
fn integer_literal_too_big_for_bigint() {
    assert_snapshot!(hover(r"
select 10000000000000000000$0;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 10000000000000000000;
      │        ──────────────────── hover
      │
      ╰ note: value of literal: ` 10000000000000000000 (0x8AC7230489E80000|0b1000101011000111001000110000010010001001111010000000000000000000) `
    ");
}

#[test]
fn integer_literal_too_long_for_value() {
    let sql = format!("select {}$0;", "9".repeat(1001));
    assert_snapshot!(hover(&sql), @"
    hover: numeric
      ╭▸ file0.sql:1:8
      │
    1 │ …lect 999999999999999999999999999999999999999999999…999999999999999999999999999999999999999999999;
      ╰╴      ─────────────────────────────────────────────…───────────────────────────────────────────── hover
    ");
}

#[test]
fn hex_integer_literal() {
    assert_snapshot!(hover(r"
select 0x1$0F;
"), @"
    hover: int4
      ╭▸ file0.sql:2:8
      │
    2 │ select 0x1F;
      │        ──── hover
      │
      ╰ note: value of literal: ` 31 (0x1F|0b11111) `
    ");
}

#[test]
fn numeric_literal() {
    assert_snapshot!(hover(r"
select 1.5$0;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1.5;
      │        ─── hover
      │
      ╰ note: value of literal: ` 1.5 `
    ");
}

#[test]
fn numeric_literal_with_exponent() {
    assert_snapshot!(hover(r"
select 1.5e$03;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1.5e3;
      │        ───── hover
      │
      ╰ note: value of literal: ` 1500 `
    ");
}

#[test]
fn numeric_literal_with_negative_exponent() {
    assert_snapshot!(hover(r"
select 1e-$03;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1e-3;
      │        ──── hover
      │
      ╰ note: value of literal: ` 0.001 `
    ");
}

#[test]
fn numeric_literal_with_positive_exponent() {
    assert_snapshot!(hover(r"
select 1e+$03;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1e+3;
      │        ──── hover
      │
      ╰ note: value of literal: ` 1000 `
    ");
}

#[test]
fn numeric_literal_leading_dot() {
    assert_snapshot!(hover(r"
select .$05;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select .5;
      │        ── hover
      │
      ╰ note: value of literal: ` 0.5 `
    ");
}

#[test]
fn numeric_literal_trailing_dot() {
    assert_snapshot!(hover(r"
select 5$0.;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 5.;
      │        ── hover
      │
      ╰ note: value of literal: ` 5 `
    ");
}

#[test]
fn numeric_literal_keeps_scale() {
    assert_snapshot!(hover(r"
select 1.5$00;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1.50;
      │        ──── hover
      │
      ╰ note: value of literal: ` 1.50 `
    ");
}

#[test]
fn numeric_literal_exponent_keeps_scale() {
    assert_snapshot!(hover(r"
select 1.50$0e1;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1.50e1;
      │        ────── hover
      │
      ╰ note: value of literal: ` 15.0 `
    ");
}

#[test]
fn numeric_literal_with_underscores() {
    assert_snapshot!(hover(r"
select 1_00$00.5;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1_000.5;
      │        ─────── hover
      │
      ╰ note: value of literal: ` 1000.5 `
    ");
}

#[test]
fn numeric_literal_zero_with_exponent() {
    assert_snapshot!(hover(r"
select 0$0e5;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 0e5;
      │        ─── hover
      │
      ╰ note: value of literal: ` 0 `
    ");
}

#[test]
fn numeric_literal_small_with_exponent() {
    assert_snapshot!(hover(r"
select 0.05$0e-1;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 0.05e-1;
      │        ─────── hover
      │
      ╰ note: value of literal: ` 0.005 `
    ");
}

#[test]
fn numeric_literal_huge_exponent() {
    assert_snapshot!(hover(r"
select 1$0e2000000000;
"), @"
    hover: numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select 1e2000000000;
      ╰╴       ──────────── hover
    ");
}

#[test]
fn integer_literal_with_underscores() {
    assert_snapshot!(hover(r"
select 1_00$00;
"), @"
    hover: int4
      ╭▸ file0.sql:2:8
      │
    2 │ select 1_000;
      │        ───── hover
      │
      ╰ note: value of literal: ` 1000 (0x3E8|0b1111101000) `
    ");
}

#[test]
fn true_literal() {
    assert_snapshot!(hover(r"
select true$0;
"), @"
    hover: bool
      ╭▸ file0.sql:2:8
      │
    2 │ select true;
      ╰╴       ──── hover
    ");
}

#[test]
fn false_literal() {
    assert_snapshot!(hover(r"
select false$0;
"), @"
    hover: bool
      ╭▸ file0.sql:2:8
      │
    2 │ select false;
      ╰╴       ───── hover
    ");
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
