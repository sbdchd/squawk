use rowan::{TextRange, TextSize};
use squawk_sema::file::{FilePosition, FileRange, InFile};
use squawk_sema::ir::Item;
use squawk_sema::semantic_index::{AsOf, SemanticIndex};
use squawk_syntax::ast;

use crate::db::{Db, parse, semantic_index};
use crate::goto_definition::item_definitions;
use crate::offsets::item_at_position;

pub fn find_references(db: &dyn Db, position: InFile<TextSize>) -> Vec<InFile<TextRange>> {
    let file = position.file_id;
    find_references_with_index(semantic_index(db, file), &parse(db, file).tree(), position)
}

pub(crate) fn find_references_with_index(
    index: &SemanticIndex,
    file: &ast::SourceFile,
    position: FilePosition,
) -> Vec<FileRange> {
    let Some(item) = item_at_position(index, file, position) else {
        return vec![];
    };
    let definitions = item_definitions(index, item.value.item, AsOf::Latest);
    if definitions.is_empty() {
        return vec![];
    }
    let mut references: Vec<FileRange> = definitions.to_vec();
    for item in index.items() {
        if !matches!(item.value.item, Item::Reference(_)) {
            continue;
        }
        if item_definitions(index, item.value.item, AsOf::Latest)
            .iter()
            .any(|definition| definitions.contains(definition))
        {
            references.push(item.map(|item| item.range));
        }
    }
    references.sort_unstable_by_key(|range| {
        (
            range.file_id != position.file_id,
            range.file_id,
            range.value.start(),
            range.value.end(),
        )
    });
    references.dedup();
    references.shrink_to_fit();
    references
}

#[cfg(test)]
mod tests;
