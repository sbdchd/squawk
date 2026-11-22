use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

use squawk_syntax::SourceFile;

use crate::file::{FileId, FileIndex, InFile};
use crate::semantic_index_builder::SemanticIndexBuilder;

#[test]
fn parser_fixtures_dont_panic() {
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("../squawk_parser/tests/data");
    let mut panicked = vec![];
    let mut checked = 0;
    for dir in ["ok", "err"] {
        let mut paths: Vec<_> = std::fs::read_dir(data.join(dir))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
            .collect();
        paths.sort();
        for path in paths {
            let sql = std::fs::read_to_string(&path).unwrap();
            let result = catch_unwind(AssertUnwindSafe(|| {
                let parse = SourceFile::parse(&sql);
                let mut builder = SemanticIndexBuilder::new();
                builder.add_file(InFile::new(
                    FileId::File(FileIndex::from_raw(0)),
                    &parse.tree(),
                ));
                builder.build()
            }));
            checked += 1;
            if result.is_err() {
                panicked.push(format!("{dir}/{}", path.file_name().unwrap().display()));
            }
        }
    }
    assert!(checked > 0);
    assert!(panicked.is_empty(), "sema panicked on: {panicked:#?}");
}
