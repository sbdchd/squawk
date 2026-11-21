use squawk_rowan::{TextRange, TextSize};
use squawk_sema::{
    file::{FilePosition, FileRange, InFile, IrFileRange},
    ir::{Item, Ty},
    semantic_index::SemanticIndex,
};
use squawk_syntax::ast::{self, AstNode};

use squawk_ide_db::offsets::{TokenAt, item_at_token};
use squawk_ide_db::{Db, parse, semantic_index};

pub fn goto_type_definition(db: &dyn Db, position: InFile<TextSize>) -> Option<InFile<TextRange>> {
    let file = position.file_id;
    goto_type_definition_with_index(semantic_index(db, file), &parse(db, file).tree(), position)
}

pub(crate) fn goto_type_definition_with_index(
    index: &SemanticIndex,
    file: &ast::SourceFile,
    position: FilePosition,
) -> Option<FileRange> {
    let token = TokenAt::new(file, position.value)?;
    let ty = match item_at_token(index, position, &token) {
        Some(item) => item_type(index, item.value.item)?,
        None => {
            let literal = token.token.parent().and_then(ast::Literal::cast)?;
            let range = position.with_value(token.original_range(literal.syntax().text_range()));
            *index.literal_types.get(&range)?
        }
    };
    Some(index.range(ty_range(index, ty)))
}

fn item_type(index: &SemanticIndex, item: Item) -> Option<Ty> {
    match item {
        Item::Reference(reference) => match index.call_types.get(&reference) {
            Some(&ty) => ty,
            None => item_type(index, index.references[reference].resolution.item()?),
        },
        Item::Column(column) => index.columns[column].ty,
        Item::Source(source) => index.source_type(source),
        Item::Table(table) => Some(Ty::Table(table)),
        Item::Cte(cte) => Some(Ty::Cte(cte)),
        Item::Type(ty) => Some(Ty::Type(ty)),
        Item::Param(param) => index.params[param].ty,
        Item::EnumLabel(label) => Some(Ty::Type(index.enum_labels[label].ty)),
        Item::GraphProperty(property) => index.columns[index.graph_properties[property].column].ty,
        Item::Function(function) => index.functions[function].ret,
        Item::Operator(operator) => index.functions[index.operators[operator].function?].ret,
        Item::Window(_)
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
        | Item::Collation(_)
        | Item::Schema(_)
        | Item::Tablespace(_)
        | Item::Database(_)
        | Item::Extension(_)
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
