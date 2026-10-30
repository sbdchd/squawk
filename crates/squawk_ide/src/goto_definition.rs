use rowan::{TextRange, TextSize};
use smallvec::{SmallVec, smallvec};
use squawk_sema::file::{FilePosition, FileRange, InFile, IrFileRange};
use squawk_sema::ir::{ColumnId, FunctionId, Item, ParamId, SourceId, SourceKind, TypeId};
use squawk_sema::semantic_index::{AsOf, SemanticIndex};
use squawk_syntax::ast;

use crate::db::{Db, parse, semantic_index};
use crate::offsets::item_at_position;

pub fn goto_definition(
    db: &dyn Db,
    position: InFile<TextSize>,
) -> SmallVec<[InFile<TextRange>; 1]> {
    let file = position.file_id;
    goto_definition_with_index(semantic_index(db, file), &parse(db, file).tree(), position)
}

pub(crate) fn goto_definition_with_index(
    index: &SemanticIndex,
    file: &ast::SourceFile,
    position: FilePosition,
) -> SmallVec<[FileRange; 1]> {
    let Some(item) = item_at_position(index, file, position) else {
        return smallvec![];
    };
    let as_of = if let Item::Reference(reference) = item.value.item {
        AsOf::Ref(reference)
    } else {
        AsOf::Def(item.map(|item| item.range))
    };
    item_definitions(index, item.value.item, as_of)
}

pub(crate) fn item_definitions(
    index: &SemanticIndex,
    item: Item,
    as_of: AsOf,
) -> SmallVec<[FileRange; 1]> {
    definition_ranges(index, item, as_of)
        .into_iter()
        .map(|range| index.range(range))
        .collect()
}

fn definition_ranges(index: &SemanticIndex, item: Item, as_of: AsOf) -> SmallVec<[IrFileRange; 1]> {
    match item {
        Item::Reference(reference) => match index.references[reference].resolution.item() {
            Some(item) => definition_ranges(index, item, as_of),
            None => smallvec![],
        },
        Item::Cte(cte) => smallvec![index.ctes[cte].name_ptr.file_range()],
        Item::Column(column) => column_definitions(index, column),
        Item::Source(source) => source_definition(index, source, as_of)
            .into_iter()
            .collect(),
        Item::Table(table) => {
            smallvec![index.name_at(item, index.tables[table].name_ptr.file_range(), as_of)]
        }
        Item::Function(function) => smallvec![function_definition(index, function)],
        Item::Param(param) => smallvec![param_definition(index, param)],
        Item::Type(ty) => type_definitions(index, ty, as_of),
        Item::Window(window) => smallvec![index.windows[window].name_ptr.file_range()],
        Item::JsonPath(path) => smallvec![index.json_paths[path].name_ptr.file_range()],
        Item::Savepoint(savepoint) => smallvec![index.savepoints[savepoint].name_ptr.file_range()],
        Item::Cursor(cursor) => smallvec![index.cursors[cursor].name_ptr.file_range()],
        Item::PreparedTransaction(prepared) => {
            smallvec![index.prepared_transactions[prepared].name_ptr.file_range()]
        }
        Item::PreparedStatement(prepared) => {
            smallvec![index.prepared_statements[prepared].name_ptr.file_range()]
        }
        Item::Channel(channel) => smallvec![index.channels[channel].name_ptr.file_range()],
        Item::TransactionBegin(transaction) => index.transactions[transaction]
            .end
            .iter()
            .map(|end| end.file_range())
            .collect(),
        Item::TransactionEnd(transaction) => {
            smallvec![index.transactions[transaction].begin.file_range()]
        }
        Item::CaseKeyword(case) => smallvec![index.cases[case].case_range],
        Item::Star(_) => smallvec![],
        Item::Index(idx) => {
            smallvec![index.name_at(item, index.indexes[idx].name_ptr.file_range(), as_of)]
        }
        Item::Constraint(constraint) => {
            smallvec![index.constraints[constraint].name_ptr.file_range()]
        }
        Item::Trigger(trigger) => smallvec![index.triggers[trigger].name_ptr.file_range()],
        Item::Rule(rule) => smallvec![index.rules[rule].name_ptr.file_range()],
        Item::Role(role) => smallvec![index.roles[role].name_ptr.file_range()],
        Item::Policy(policy) => smallvec![index.policies[policy].name_ptr.file_range()],
        Item::AccessMethod(access_method) => {
            smallvec![index.access_methods[access_method].name_ptr.file_range()]
        }
        Item::EventTrigger(event_trigger) => {
            smallvec![index.event_triggers[event_trigger].name_ptr.file_range()]
        }
        Item::Language(language) => smallvec![index.languages[language].name_ptr.file_range()],
        Item::ForeignDataWrapper(wrapper) => {
            smallvec![index.foreign_data_wrappers[wrapper].name_ptr.file_range()]
        }
        Item::Server(server) => smallvec![index.servers[server].name_ptr.file_range()],
        Item::Conversion(conversion) => {
            smallvec![index.conversions[conversion].name_ptr.file_range()]
        }
        Item::OperatorFamily(family) => {
            smallvec![index.operator_families[family].name_ptr.file_range()]
        }
        Item::OperatorClass(class) => {
            smallvec![index.operator_classes[class].name_ptr.file_range()]
        }
        Item::Statistics(statistics) => {
            smallvec![index.statistics[statistics].name_ptr.file_range()]
        }
        Item::TextSearch(text_search) => {
            smallvec![index.text_search[text_search].name_ptr.file_range()]
        }
        Item::Publication(publication) => {
            smallvec![index.publications[publication].name_ptr.file_range()]
        }
        Item::Subscription(subscription) => {
            smallvec![index.subscriptions[subscription].name_ptr.file_range()]
        }
        Item::GraphElement(element) => {
            smallvec![index.graph_elements[element].name_ptr.file_range()]
        }
        Item::GraphLabel(label) => smallvec![index.graph_labels[label].name_ptr.file_range()],
        Item::EnumLabel(label) => {
            smallvec![index.name_at(item, index.enum_labels[label].name_ptr.file_range(), as_of)]
        }
        Item::GraphProperty(property) => {
            column_definitions(index, index.graph_properties[property].column)
        }
        Item::Collation(collation) => smallvec![index.collations[collation].name_ptr.file_range()],
        Item::Schema(schema) => smallvec![index.schemas[schema].name_ptr.file_range()],
        Item::Tablespace(tablespace) => {
            smallvec![index.tablespaces[tablespace].name_ptr.file_range()]
        }
        Item::Database(database) => smallvec![index.databases[database].name_ptr.file_range()],
        Item::Extension(extension) => {
            smallvec![index.extensions[extension].name_ptr.file_range()]
        }
        Item::Operator(operator) => smallvec![index.operators[operator].name_range],
    }
}

fn column_definitions(index: &SemanticIndex, column: ColumnId) -> SmallVec<[IrFileRange; 1]> {
    index
        .merged_columns(column)
        .iter()
        .map(|column| index.columns[column].ptr.file_range())
        .collect()
}

fn function_definition(index: &SemanticIndex, function: FunctionId) -> IrFileRange {
    index.functions[function].name_ptr.file_range()
}

fn type_definitions(index: &SemanticIndex, id: TypeId, as_of: AsOf) -> SmallVec<[IrFileRange; 1]> {
    let ty = &index.types[id];
    ty.shell_name_ptr
        .iter()
        .map(|name_ptr| name_ptr.file_range())
        .chain([index.name_at(Item::Type(id), ty.name_ptr.file_range(), as_of)])
        .collect()
}

fn param_definition(index: &SemanticIndex, param: ParamId) -> IrFileRange {
    let param = &index.params[param];
    param
        .name_ptr
        .map_or(param.ptr.file_range(), |name_ptr| name_ptr.file_range())
}

fn source_definition(index: &SemanticIndex, source: SourceId, as_of: AsOf) -> Option<IrFileRange> {
    let source = &index.sources[source];
    if let Some(alias) = source.alias_ptr {
        return Some(alias.file_range());
    }
    match source.kind {
        SourceKind::Cte(cte) => Some(index.ctes[cte].name_ptr.file_range()),
        SourceKind::Table(table) => Some(index.name_at(
            Item::Table(table),
            index.tables[table].name_ptr.file_range(),
            as_of,
        )),
        SourceKind::Function(function) => Some(function_definition(index, function)),
        SourceKind::Returning(target)
        | SourceKind::Excluded(target)
        | SourceKind::RuleRelation(target) => source_definition(index, target, as_of),
        SourceKind::ElementVariable => Some(source.ptr.file_range()),
        SourceKind::Subquery(_)
        | SourceKind::External
        | SourceKind::Join
        | SourceKind::UsingAlias
        | SourceKind::Other => None,
    }
}

#[cfg(test)]
mod tests;
