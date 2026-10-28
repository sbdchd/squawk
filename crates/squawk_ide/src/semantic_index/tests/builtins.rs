use insta::assert_snapshot;
use squawk_syntax::SourceFile;

use super::check;
use crate::extensions::BundledExtension;
use crate::file::{FileId, InFile};
use crate::semantic_index_builder::SemanticIndexBuilder;

#[test]
fn builtin_functions() {
    assert_snapshot!(check("
create table t (a int, b text, c timestamptz);
select
  now(), lower(b), upper(b), length(b), left(b, 1), right(b, 1),
  coalesce(a, 1), nullif(a, 1), greatest(a, 1), least(a, 1),
  count(*), count(a), sum(a), avg(a), max(c), array_agg(a order by a),
  string_agg(b, ',' order by b), jsonb_build_object(), jsonb_build_object('a', a),
  concat(b, b), format('%s', b), to_char(c, 'YYYY'), date_trunc('day', c),
  make_interval(days => 1), pg_catalog.lower(b),
  row_number() over (order by a), rank() over (order by a),
  percentile_cont(0.5) within group (order by a),
  extract(year from c), substring(b from 1 for 2), trim(b), position('a' in b)
from t
group by grouping sets ((a), ());
"), @"no diagnostics");
}

#[test]
fn builtin_tables() {
    assert_snapshot!(check("
select relname, relkind, nope from pg_class;
select table_name from information_schema.tables;
select table_name from tables;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    2 │ select relname, relkind, nope from pg_class;
      ╰╴                         ━━━━
    error: relation `tables` does not exist
      ╭▸ 
    4 │ select table_name from tables;
      │                        ━━━━━━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    4 │ select table_name from pg_tables;
      ╰╴                       +++
    ");
}

#[test]
fn builtin_set_returning_functions() {
    assert_snapshot!(check("
select g, x from generate_series(1, 3) g, unnest(array[1, 2]) as u(x);
select word, catcode, nope from pg_get_keywords();
"), @"
    error: column `nope` does not exist
      ╭▸ 
    3 │ select word, catcode, nope from pg_get_keywords();
      ╰╴                      ━━━━
    ");
}

#[test]
fn builtins_have_no_diagnostics() {
    let parse = SourceFile::parse(crate::builtins::BUILTINS_SQL);
    assert_eq!(parse.errors(), vec![]);
    assert_eq!(
        crate::semantic_index_builder::builtins().diagnostics(),
        vec![]
    );
}

#[test]
fn extensions_have_no_diagnostics() {
    for extension in BundledExtension::ALL {
        let parse = extension.parse();
        assert_eq!(parse.errors(), vec![], "{}", extension.name());
        let mut builder = SemanticIndexBuilder::new();
        builder.add_file(InFile::new(extension.file_id(), &parse.tree()));
        let index = builder.build();
        assert_eq!(index.diagnostics(), vec![], "{}", extension.name());
    }
}

#[test]
fn extensions_table_matches_generated_files() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generated/extensions");
    let mut files = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            path.file_stem().unwrap().to_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>();
    files.sort();
    let names = BundledExtension::ALL
        .iter()
        .map(|extension| extension.name())
        .collect::<Vec<_>>();
    assert_eq!(names, files);
}

#[test]
fn extension_file_lookup() {
    for extension in BundledExtension::ALL {
        assert_eq!(
            BundledExtension::from_name(extension.name()),
            Some(extension)
        );
        assert_eq!(
            BundledExtension::from_file(extension.file_id()),
            Some(extension)
        );
    }
    assert_eq!(BundledExtension::from_name("not_an_extension"), None);
    assert_eq!(BundledExtension::from_file(FileId::Builtins), None);
    let db = crate::db::Database::default();
    let file = crate::db::File::new(&db, "".into());
    assert_eq!(BundledExtension::from_file(file.into()), None);
}
