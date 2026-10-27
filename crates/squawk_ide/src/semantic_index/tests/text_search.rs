use insta::assert_snapshot;

use super::check;

#[test]
fn create_text_search_objects() {
    assert_snapshot!(check(r#"
create function ps(internal, int) returns internal language c as 'f';
create function pt(internal, internal, internal) returns internal language c as 'f';
create function pe(internal) returns void language c as 'f';
create function pl(internal) returns internal language c as 'f';
create text search parser p (start = ps, gettoken = pt, end = pe, lextypes = pl, headline = missing_fn);
create text search template t (init = dsimple_init, lexize = dsimple_lexize);
create text search template t2 (lexize = missing_lexize);
create text search dictionary d (template = t, stopwords = 'english');
create text search dictionary d2 (template = missing_template);
create text search dictionary d3 (template = pg_catalog.snowball, language = 'english');
create text search configuration c (parser = p);
create text search configuration c2 (copy = english);
create text search configuration c3 (parser = missing_parser);
create text search configuration c4 (copy = missing_config);
create text search configuration c5 (parser = "default");
create text search parser p (start = ps, gettoken = pt, end = pe, lextypes = pl);
create text search configuration english (parser = p);
create text search configuration pg_catalog.english (parser = p);
"#), @"
    error: function `missing_fn` does not exist
      ╭▸ 
    6 │ create text search parser p (start = ps, gettoken = pt, end = pe, lextypes = pl, headline = missing_fn);
      ╰╴                                                                                            ━━━━━━━━━━
    error: function `missing_lexize` does not exist
      ╭▸ 
    8 │ create text search template t2 (lexize = missing_lexize);
      ╰╴                                         ━━━━━━━━━━━━━━
    error: text search template `missing_template` does not exist
       ╭▸ 
    10 │ create text search dictionary d2 (template = missing_template);
       ╰╴                                             ━━━━━━━━━━━━━━━━
    error: text search parser `missing_parser` does not exist
       ╭▸ 
    14 │ create text search configuration c3 (parser = missing_parser);
       ╰╴                                              ━━━━━━━━━━━━━━
    error: text search configuration `missing_config` does not exist
       ╭▸ 
    15 │ create text search configuration c4 (copy = missing_config);
       ╰╴                                            ━━━━━━━━━━━━━━
    error: text search parser `p` already exists
       ╭▸ 
    17 │ create text search parser p (start = ps, gettoken = pt, end = pe, lextypes = pl);
       ╰╴                          ━
    error: text search configuration `english` already exists
       ╭▸ 
    19 │ create text search configuration pg_catalog.english (parser = p);
       ╰╴                                            ━━━━━━━
    ");
}

#[test]
fn alter_text_search_configuration() {
    assert_snapshot!(check("
create role r;
create text search configuration c (copy = english);
alter text search configuration c add mapping for asciiword with english_stem, simple;
alter text search configuration c add mapping for word with missing_dict;
alter text search configuration c alter mapping for asciiword with simple;
alter text search configuration c alter mapping replace english_stem with missing_dict2;
alter text search configuration c alter mapping for word replace simple with english_stem;
alter text search configuration c drop mapping if exists for asciiword;
alter text search configuration c owner to r;
alter text search configuration missing owner to r;
"), @"
    error: text search dictionary `missing_dict` does not exist
      ╭▸ 
    5 │ alter text search configuration c add mapping for word with missing_dict;
      ╰╴                                                            ━━━━━━━━━━━━
    error: text search dictionary `missing_dict2` does not exist
      ╭▸ 
    7 │ alter text search configuration c alter mapping replace english_stem with missing_dict2;
      ╰╴                                                                          ━━━━━━━━━━━━━
    error: text search configuration `missing` does not exist
       ╭▸ 
    11 │ alter text search configuration missing owner to r;
       ╰╴                                ━━━━━━━
    ");
}

#[test]
fn alter_text_search_objects() {
    assert_snapshot!(check("
create role r;
create schema s;
create text search dictionary d (template = simple);
create text search dictionary d2 (template = simple);
alter text search dictionary d (stopwords = 'english');
alter text search dictionary d owner to r;
alter text search dictionary d rename to d2;
alter text search dictionary d rename to d3;
alter text search dictionary d3 set schema s;
alter text search dictionary s.d3 owner to r;
alter text search dictionary d3 owner to r;
alter text search template simple rename to simple2;
alter text search template simple2 set schema s;
alter text search template s.simple2 rename to simple3;
alter text search parser missing rename to p;
create text search dictionary d4 (template = simple);
"), @"
    error: text search dictionary `d2` already exists
      ╭▸ 
    8 │ alter text search dictionary d rename to d2;
      ╰╴                                         ━━
    error: text search dictionary `d3` does not exist
       ╭▸ 
    12 │ alter text search dictionary d3 owner to r;
       ╰╴                             ━━
    error: text search parser `missing` does not exist
       ╭▸ 
    16 │ alter text search parser missing rename to p;
       ╰╴                         ━━━━━━━
    error: text search template `simple` does not exist
       ╭▸ 
    17 │ create text search dictionary d4 (template = simple);
       ╰╴                                             ━━━━━━
    ");
}

#[test]
fn drop_text_search_objects() {
    assert_snapshot!(check(r#"
create text search dictionary d (template = simple);
drop text search dictionary d;
drop text search dictionary d;
drop text search dictionary if exists d;
drop text search configuration english;
drop text search configuration english;
drop text search template if exists missing;
drop text search parser "default";
create text search configuration c (parser = "default");
drop text search dictionary simple;
drop text search template simple;
create text search dictionary d (template = simple);
"#), @r#"
    error: text search dictionary `d` does not exist
      ╭▸ 
    4 │ drop text search dictionary d;
      ╰╴                            ━
    error: text search configuration `english` does not exist
      ╭▸ 
    7 │ drop text search configuration english;
      ╰╴                               ━━━━━━━
    error: text search parser `default` does not exist
       ╭▸ 
    10 │ create text search configuration c (parser = "default");
       ╰╴                                             ━━━━━━━━━
    error: text search template `simple` does not exist
       ╭▸ 
    13 │ create text search dictionary d (template = simple);
       ╰╴                                            ━━━━━━
    "#);
}

#[test]
fn text_search_rollback() {
    assert_snapshot!(check("
create text search dictionary d (template = simple);
begin;
drop text search dictionary d;
create text search dictionary d2 (template = simple);
rollback;
drop text search dictionary d;
drop text search dictionary d2;
"), @"
    error: text search dictionary `d2` does not exist
      ╭▸ 
    8 │ drop text search dictionary d2;
      ╰╴                            ━━
    ");
}
