use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_syntax::ast;

use crate::{
    db::{parse, semantic_index},
    file::{FilePosition, FileRange, InFile, IrFileRange},
    ir::{Item, Ty},
    offsets::item_at_position,
    semantic_index::SemanticIndex,
};

pub fn goto_type_definition(db: &dyn Db, position: InFile<TextSize>) -> Option<InFile<TextRange>> {
    let file = position.file_id;
    goto_type_definition_with_index(semantic_index(db, file), &parse(db, file).tree(), position)
}

pub(crate) fn goto_type_definition_with_index(
    index: &SemanticIndex,
    file: &ast::SourceFile,
    position: FilePosition,
) -> Option<FileRange> {
    let item = item_at_position(index, file, position)?.value.item;
    let ty = item_type(index, item)?;
    Some(index.range(ty_range(index, ty)))
}

fn item_type(index: &SemanticIndex, item: Item) -> Option<Ty> {
    match item {
        Item::Reference(reference) => {
            item_type(index, index.references[reference].resolution.item()?)
        }
        Item::Column(column) => index.columns[column].ty,
        Item::Source(source) => index.source_type(source),
        Item::Table(table) => Some(Ty::Table(table)),
        Item::Cte(cte) => Some(Ty::Cte(cte)),
        Item::Type(ty) => Some(Ty::Type(ty)),
        Item::Param(param) => index.params[param].ty,
        Item::Function(_)
        | Item::Window(_)
        | Item::JsonPath(_)
        | Item::Index(_)
        | Item::Constraint(_)
        | Item::Trigger(_)
        | Item::Rule(_)
        | Item::Role(_)
        | Item::Policy(_)
        | Item::AccessMethod(_)
        | Item::EventTrigger(_)
        | Item::Language(_)
        | Item::ForeignDataWrapper(_)
        | Item::Server(_)
        | Item::Conversion(_)
        | Item::OperatorFamily(_)
        | Item::OperatorClass(_)
        | Item::Statistics(_)
        | Item::TextSearch(_)
        | Item::Publication(_)
        | Item::Subscription(_)
        | Item::GraphElement(_)
        | Item::GraphLabel(_)
        | Item::EnumLabel(_)
        | Item::GraphProperty(_)
        | Item::Collation(_)
        | Item::Schema(_)
        | Item::Tablespace(_)
        | Item::Database(_)
        | Item::Extension(_)
        | Item::Operator(_)
        | Item::Savepoint(_)
        | Item::Cursor(_)
        | Item::PreparedTransaction(_)
        | Item::PreparedStatement(_)
        | Item::Channel(_)
        | Item::TransactionBegin(_)
        | Item::TransactionEnd(_)
        | Item::CaseKeyword(_)
        | Item::Star(_) => None,
    }
}

fn ty_range(index: &SemanticIndex, ty: Ty) -> IrFileRange {
    match ty {
        Ty::Type(ty) => index.types[ty].name_ptr.file_range(),
        Ty::Table(table) => index.tables[table].name_ptr.file_range(),
        Ty::Cte(cte) => index.ctes[cte].name_ptr.file_range(),
        Ty::Array(element) => ty_range(index, element.into()),
    }
}

#[cfg(test)]
mod tests;
