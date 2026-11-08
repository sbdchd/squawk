use insta::assert_snapshot;

use super::check;

#[test]
fn invalid_numeric_cast() {
    assert_snapshot!(check("
select '1.2.3'::numeric, '1_.5'::decimal, 'NaNx'::numeric, '-NaN'::numeric;
"), @"
    error: invalid input value for type `numeric`: `1.2.3`
      ╭▸ 
    2 │ select '1.2.3'::numeric, '1_.5'::decimal, 'NaNx'::numeric, '-NaN'::numeric;
      ╰╴       ━━━━━━━
    error: invalid input value for type `numeric`: `1_.5`
      ╭▸ 
    2 │ select '1.2.3'::numeric, '1_.5'::decimal, 'NaNx'::numeric, '-NaN'::numeric;
      ╰╴                         ━━━━━━
    error: invalid input value for type `numeric`: `NaNx`
      ╭▸ 
    2 │ select '1.2.3'::numeric, '1_.5'::decimal, 'NaNx'::numeric, '-NaN'::numeric;
      ╰╴                                          ━━━━━━
    error: invalid input value for type `numeric`: `-NaN`
      ╭▸ 
    2 │ select '1.2.3'::numeric, '1_.5'::decimal, 'NaNx'::numeric, '-NaN'::numeric;
      ╰╴                                                           ━━━━━━
    ");
}

#[test]
fn numeric_format_overflow() {
    assert_snapshot!(check("
select '1e131072'::numeric, '1e-16384'::numeric;
"), @"
    error: value `1e131072` is out of range for type `numeric`
      ╭▸ 
    2 │ select '1e131072'::numeric, '1e-16384'::numeric;
      ╰╴       ━━━━━━━━━━
    error: value `1e-16384` is out of range for type `numeric`
      ╭▸ 
    2 │ select '1e131072'::numeric, '1e-16384'::numeric;
      ╰╴                            ━━━━━━━━━━
    ");
}

#[test]
fn numeric_typmod_overflow() {
    assert_snapshot!(check("
select
  '9.96'::numeric(2, 1),
  '0.995'::decimal(2, 2),
  '99950'::numeric(3, -2),
  'Infinity'::numeric(3, 1),
  '1000'::numeric(3);
"), @"
    error: value `9.96` is out of range for type `numeric(2, 1)`
      ╭▸ 
    3 │   '9.96'::numeric(2, 1),
      │   ━━━━━━
      │
      ╰ note: a field with precision 2 and scale 1 must round to an absolute value less than `10^1`
    error: value `0.995` is out of range for type `numeric(2, 2)`
      ╭▸ 
    4 │   '0.995'::decimal(2, 2),
      │   ━━━━━━━
      │
      ╰ note: a field with precision 2 and scale 2 must round to an absolute value less than `1`
    error: value `99950` is out of range for type `numeric(3, -2)`
      ╭▸ 
    5 │   '99950'::numeric(3, -2),
      │   ━━━━━━━
      │
      ╰ note: a field with precision 3 and scale -2 must round to an absolute value less than `10^5`
    error: value `Infinity` is out of range for type `numeric(3, 1)`
      ╭▸ 
    6 │   'Infinity'::numeric(3, 1),
      │   ━━━━━━━━━━
      │
      ╰ note: a field with precision 3 and scale 1 cannot hold an infinite value
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    7 │   '1000'::numeric(3);
      │   ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    ");
}

#[test]
fn numeric_comparison() {
    assert_snapshot!(check("
create table t (a numeric(3, 1));
select * from t where a = 'x' or a = '1000';
"), @"
    error: invalid input value for type `numeric`: `x`
      ╭▸ 
    3 │ select * from t where a = 'x' or a = '1000';
      ╰╴                          ━━━
    ");
}

#[test]
fn numeric_domain_typmod() {
    assert_snapshot!(check("
create domain price as numeric(3, 0);
create domain price2 as price;
create table t (a price);
select '1000'::price, '1000'::price2;
select * from t where a = '1000';
"), @"
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    5 │ select '1000'::price, '1000'::price2;
      │        ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    5 │ select '1000'::price, '1000'::price2;
      │                       ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    ");
}

#[test]
fn numeric_assignment_typmod() {
    assert_snapshot!(check("
create table t (
  a numeric(3, 0) default '1000',
  b int
);
insert into t (a) values ('1000');
update t set a = '1000';
alter table t add column c numeric(2, 1) default '9.96';
alter table t alter column b type numeric(3, 0);
insert into t (b) values ('1000');
"), @"
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    3 │   a numeric(3, 0) default '1000',
      │                           ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    6 │ insert into t (a) values ('1000');
      │                           ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    7 │ update t set a = '1000';
      │                  ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    error: value `9.96` is out of range for type `numeric(2, 1)`
      ╭▸ 
    8 │ alter table t add column c numeric(2, 1) default '9.96';
      │                                                  ━━━━━━
      │
      ╰ note: a field with precision 2 and scale 1 must round to an absolute value less than `10^1`
    error: value `1000` is out of range for type `numeric(3, 0)`
       ╭▸ 
    10 │ insert into t (b) values ('1000');
       │                           ━━━━━━
       │
       ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    ");
}

#[test]
fn numeric_string_typmod() {
    assert_snapshot!(check("
select '1000'::numeric('3', '0');
"), @"
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    2 │ select '1000'::numeric('3', '0');
      │        ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    ");
}
