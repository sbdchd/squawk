use insta::assert_snapshot;
use squawk_syntax::SourceFile;

use crate::file::{FileId, FileIndex, InFile};
use crate::{ir::ItemRange, semantic_index_builder::SemanticIndexBuilder};

#[test]
fn item_at_range() {
    let sql = "with t as (select 1 a) select a, b from t;";
    let parse = SourceFile::parse(sql);
    let mut builder = SemanticIndexBuilder::new();
    let file = FileId::File(FileIndex::from_raw(0));
    builder.add_file(InFile::new(file, &parse.tree()));
    let index = builder.build();
    let items = index
        .items()
        .filter(|item| item.file_id == file)
        .map(|item| {
            let ItemRange { range, item } = item.value;
            format!("{range:?} {:?} {item:?}", &sql[range])
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_snapshot!(items, @r#"
    5..6 "t" Cte(Idx::<Cte>(0))
    20..21 "a" Column(Idx::<Column>(5727))
    30..31 "a" Reference(Idx::<Reference>(28393))
    33..34 "b" Reference(Idx::<Reference>(28394))
    40..41 "t" Reference(Idx::<Reference>(28392))
    "#);

    let token = InFile::new(file, rowan::TextRange::new(33.into(), 34.into()));
    let ItemRange { range, item } = index.item_at_range(token).unwrap().value;
    assert_eq!(range, rowan::TextRange::new(33.into(), 34.into()));
    let crate::ir::Item::Reference(reference) = item else {
        panic!("expected a reference");
    };
    assert_eq!(
        index.references[reference].resolution,
        crate::ir::Resolution::Unresolved
    );
}
