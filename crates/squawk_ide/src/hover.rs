use squawk_sema::arena::IdxRange;
use squawk_sema::file::{FilePosition, FileRange, InFile};
use squawk_sema::interner::Name;
use squawk_sema::ir::{
    Column, ColumnId, ColumnOwner, ConstraintOwner, FunctionId, FunctionKind, GraphElementKind,
    Item, Operand, ParamId, ParamMode, SourceId, SourceKind, StarId, TableId, TableKind, TypeId,
    TypeKind,
};
use squawk_sema::semantic_index::SemanticIndex;
use std::fmt::{self, Write as _};

use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};

use crate::db::{Db, parse, semantic_index};
use crate::hover_literal::{LiteralHover, hover_literal};
use crate::offsets::{TokenAt, item_at_token};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hover {
    pub snippet: String,
    pub comment: Option<String>,
}

impl Hover {
    pub fn markdown(&self) -> String {
        let snippet = &self.snippet;
        let mut out = format!(
            "
```sql
{snippet}
```
"
        );

        if let Some(comment) = &self.comment {
            writeln!(out, "---\n{comment}").unwrap();
        }

        out
    }
}

pub fn hover(db: &dyn Db, position: InFile<TextSize>) -> Option<Hover> {
    let file = position.file_id;
    let hover = hover_with_index(semantic_index(db, file), &parse(db, file).tree(), position)?;
    Some(Hover {
        snippet: hover.snippet,
        comment: hover.comment,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverInfo {
    pub snippet: String,
    pub comment: Option<String>,
    pub range: FileRange,
}

pub(crate) fn hover_with_index(
    index: &SemanticIndex,
    file: &ast::SourceFile,
    position: FilePosition,
) -> Option<HoverInfo> {
    let token = TokenAt::new(file, position.value)?;
    if let Some(item) = item_at_token(index, position, &token)
        && let Some(snippet) = item_hover(index, item.value.item)
    {
        return Some(HoverInfo {
            snippet,
            comment: item_comment(index, item.value.item).map(str::to_string),
            range: item.map(|item| item.range),
        });
    }
    let literal = token.token.parent().and_then(ast::Literal::cast)?;
    let LiteralHover { snippet, comment } = hover_literal(&literal)?;
    Some(HoverInfo {
        snippet: snippet.to_string(),
        comment,
        range: position.with_value(token.original_range(literal.syntax().text_range())),
    })
}

pub(crate) fn function_signature(index: &SemanticIndex, function: FunctionId) -> String {
    Hovers { index }.function_signature(function)
}

fn item_comment(index: &SemanticIndex, item: Item) -> Option<&str> {
    match item {
        Item::Reference(reference) => {
            item_comment(index, index.references[reference].resolution.item()?)
        }
        Item::Source(source) => match index.sources[source].kind {
            SourceKind::Table(table) => index.comment(Item::Table(table)),
            SourceKind::Function(function) => index.comment(Item::Function(function)),
            SourceKind::Returning(target)
            | SourceKind::Excluded(target)
            | SourceKind::RuleRelation(target) => item_comment(index, Item::Source(target)),
            SourceKind::Cte(_)
            | SourceKind::External
            | SourceKind::Subquery(_)
            | SourceKind::Join
            | SourceKind::UsingAlias
            | SourceKind::ElementVariable
            | SourceKind::Other => None,
        },
        Item::Column(column) if index.merged_columns(column).len() > 1 => None,
        Item::Column(_)
        | Item::Cte(_)
        | Item::Table(_)
        | Item::Function(_)
        | Item::Param(_)
        | Item::Type(_)
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
        | Item::CaseKeyword(_) => index.comment(item),
        Item::Star(_) => None,
    }
}

fn item_hover(index: &SemanticIndex, item: Item) -> Option<String> {
    let hover = Hovers { index };
    Some(match item {
        Item::Reference(reference) => {
            return item_hover(index, index.references[reference].resolution.item()?);
        }
        Item::Column(column) => hover.columns(column),
        Item::Star(star) => return hover.star(star),
        Item::Source(source) => return hover.source(source),
        Item::Cte(cte) => {
            let cte = &index.ctes[cte];
            format!(
                "with {}{}",
                hover.name(cte.name),
                hover.column_list(cte.columns.range)
            )
        }
        Item::Table(table) => hover.table(table),
        Item::Function(function) => hover.function(function),
        Item::Param(param) => hover.param(param)?,
        Item::Type(ty) => hover.ty(ty),
        Item::Window(window) => format!("window {}", hover.name(index.windows[window].name)),
        Item::JsonPath(path) => format!("json path {}", hover.name(index.json_paths[path].name)),
        Item::Index(idx) => {
            let idx = &index.indexes[idx];
            let mut snippet = format!("index {}", hover.qualified(idx.schema, idx.name));
            if let Some(table) = idx.table {
                write!(snippet, " on {}", hover.table_name(table)).unwrap();
            }
            snippet
        }
        Item::Constraint(constraint) => {
            let constraint = &index.constraints[constraint];
            let name = hover.name(constraint.name);
            match constraint.owner {
                ConstraintOwner::Table(table) => {
                    format!("constraint {name} on {}", hover.table_name(table))
                }
                ConstraintOwner::Domain(ty) => {
                    let ty = &index.types[ty];
                    format!(
                        "constraint {name} on domain {}",
                        hover.qualified(ty.schema, ty.name)
                    )
                }
            }
        }
        Item::Trigger(trigger) => {
            let trigger = &index.triggers[trigger];
            let kind = if trigger.constraint {
                "constraint trigger"
            } else {
                "trigger"
            };
            format!(
                "{kind} {} on {}",
                hover.name(trigger.name),
                hover.table_name(trigger.table)
            )
        }
        Item::Rule(rule) => {
            let rule = &index.rules[rule];
            format!(
                "rule {} on {}",
                hover.name(rule.name),
                hover.table_name(rule.table)
            )
        }
        Item::Role(role) => format!("role {}", hover.name(index.roles[role].name)),
        Item::Policy(policy) => {
            let policy = &index.policies[policy];
            format!(
                "policy {} on {}",
                hover.name(policy.name),
                hover.table_name(policy.table)
            )
        }
        Item::AccessMethod(access_method) => format!(
            "access method {}",
            hover.name(index.access_methods[access_method].name)
        ),
        Item::EventTrigger(event_trigger) => format!(
            "event trigger {}",
            hover.name(index.event_triggers[event_trigger].name)
        ),
        Item::Language(language) => {
            format!("language {}", hover.name(index.languages[language].name))
        }
        Item::ForeignDataWrapper(wrapper) => format!(
            "foreign data wrapper {}",
            hover.name(index.foreign_data_wrappers[wrapper].name)
        ),
        Item::Server(server) => format!("server {}", hover.name(index.servers[server].name)),
        Item::Conversion(conversion) => {
            let conversion = &index.conversions[conversion];
            format!(
                "conversion {}",
                hover.qualified(conversion.schema, conversion.name)
            )
        }
        Item::OperatorFamily(family) => {
            let family = &index.operator_families[family];
            let mut snippet = format!(
                "operator family {}",
                hover.qualified(family.schema, family.name)
            );
            if let Some(access_method) = family.access_method {
                snippet.push_str(" using ");
                snippet.push_str(hover.name(index.access_methods[access_method].name));
            }
            snippet
        }
        Item::OperatorClass(class) => {
            let class = &index.operator_classes[class];
            let mut snippet = format!(
                "operator class {}",
                hover.qualified(class.schema, class.name)
            );
            if let Some(access_method) = class.access_method {
                snippet.push_str(" using ");
                snippet.push_str(hover.name(index.access_methods[access_method].name));
            }
            snippet
        }
        Item::Statistics(statistics) => {
            let statistics = &index.statistics[statistics];
            let mut snippet = format!(
                "statistics {}",
                hover.qualified(statistics.schema, statistics.name)
            );
            if let Some(table) = statistics.table {
                write!(snippet, " on {}", hover.table_name(table)).unwrap();
            }
            snippet
        }
        Item::TextSearch(text_search) => {
            let text_search = &index.text_search[text_search];
            format!(
                "{} {}",
                text_search.kind.label(),
                hover.qualified(text_search.schema, text_search.name)
            )
        }
        Item::Publication(publication) => format!(
            "publication {}",
            hover.name(index.publications[publication].name)
        ),
        Item::Subscription(subscription) => format!(
            "subscription {}",
            hover.name(index.subscriptions[subscription].name)
        ),
        Item::GraphElement(element) => {
            let element = &index.graph_elements[element];
            let kind = match element.kind {
                GraphElementKind::Vertex => "vertex",
                GraphElementKind::Edge => "edge",
            };
            format!(
                "{kind} table {}.{}",
                hover.table_name(element.graph),
                hover.name(element.alias)
            )
        }
        Item::GraphLabel(label) => format!("label {}", hover.name(index.graph_labels[label].name)),
        Item::EnumLabel(label) => {
            let label = &index.enum_labels[label];
            let ty = &index.types[label.ty];
            format!(
                "enum label '{}' of {}",
                hover.name(label.name),
                hover.qualified(ty.schema, ty.name)
            )
        }
        Item::GraphProperty(property) => {
            let property = &index.graph_properties[property];
            let mut snippet = format!("property {}", hover.name(property.name));
            if let Some(ty) = index.columns[property.column].ty {
                write!(snippet, " {}", index.type_name(ty)).unwrap();
            }
            snippet
        }
        Item::Collation(collation) => {
            let collation = &index.collations[collation];
            format!(
                "collation {}",
                hover.qualified(collation.schema, collation.name)
            )
        }
        Item::Schema(schema) => format!("schema {}", hover.name(index.schemas[schema].name)),
        Item::Tablespace(tablespace) => format!(
            "tablespace {}",
            hover.name(index.tablespaces[tablespace].name)
        ),
        Item::Database(database) => {
            format!("database {}", hover.name(index.databases[database].name))
        }
        Item::Extension(extension) => {
            format!("extension {}", hover.name(index.extensions[extension].name))
        }
        Item::Operator(operator) => {
            let operator = &index.operators[operator];
            let mut snippet = format!(
                "operator {}({}, {})",
                hover.qualified(operator.schema, operator.name),
                hover.operand(operator.left),
                hover.operand(operator.right)
            );
            if let Some(ret) = operator
                .function
                .and_then(|function| index.functions[function].ret)
            {
                write!(snippet, " returns {}", index.type_name(ret)).unwrap();
            }
            snippet
        }
        Item::Savepoint(savepoint) => {
            format!("savepoint {}", hover.name(index.savepoints[savepoint].name))
        }
        Item::Cursor(cursor) => {
            let cursor = &index.cursors[cursor];
            let hold = if cursor.hold { " with hold" } else { "" };
            format!("cursor {}{hold}", hover.name(cursor.name))
        }
        Item::PreparedTransaction(prepared) => format!(
            "prepared transaction '{}'",
            hover.name(index.prepared_transactions[prepared].name)
        ),
        Item::PreparedStatement(prepared) => {
            let prepared = &index.prepared_statements[prepared];
            format!(
                "prepare {}{}",
                hover.name(prepared.name),
                hover.column_list(prepared.columns.range)
            )
        }
        Item::Channel(channel) => format!("channel {}", hover.name(index.channels[channel].name)),
        Item::TransactionBegin(_) | Item::TransactionEnd(_) | Item::CaseKeyword(_) => return None,
    })
}

#[derive(Clone, Copy)]
struct Hovers<'a> {
    index: &'a SemanticIndex,
}

impl<'a> Hovers<'a> {
    fn name(self, name: Name) -> &'a str {
        self.index.names.resolve(name)
    }

    fn qualified(self, schema: Name, name: Name) -> QualifiedName<'a> {
        QualifiedName {
            index: self.index,
            schema,
            name,
        }
    }

    fn table_name(self, table: TableId) -> QualifiedName<'a> {
        let table = &self.index.tables[table];
        self.qualified(table.schema, table.name)
    }

    fn column_list(self, columns: IdxRange<Column>) -> ColumnList<'a> {
        ColumnList {
            index: self.index,
            columns,
        }
    }

    fn operand(self, operand: Operand) -> OperandName<'a> {
        OperandName {
            index: self.index,
            operand,
        }
    }

    fn table(self, table: TableId) -> String {
        let data = &self.index.tables[table];
        let kind = data.kind.label();
        let name = self.table_name(table);
        match data.kind {
            TableKind::Table
            | TableKind::ForeignTable
            | TableKind::View
            | TableKind::MaterializedView => {
                format!("{kind} {name}{}", self.column_list(data.columns.range))
            }
            TableKind::Sequence | TableKind::PropertyGraph => format!("{kind} {name}"),
        }
    }

    fn function(self, function: FunctionId) -> String {
        let data = &self.index.functions[function];
        let kind = match data.kind {
            FunctionKind::Function => "function",
            FunctionKind::Aggregate => "aggregate",
            FunctionKind::Procedure => "procedure",
        };
        format!("{kind} {}", self.function_signature(function))
    }

    fn function_signature(self, function: FunctionId) -> String {
        let data = &self.index.functions[function];
        let mut snippet = format!(
            "{}{}",
            self.qualified(data.schema, data.name),
            ParamList {
                index: self.index,
                function,
            }
        );
        let has_out_params = data.params.iter().any(|param| {
            matches!(
                self.index.params[param].mode,
                ParamMode::Out | ParamMode::InOut
            )
        });
        match (data.ret, data.columns) {
            (_, Some(columns)) if !has_out_params => {
                write!(snippet, " returns table{}", self.column_list(columns.range)).unwrap();
            }
            (Some(ret), _) => {
                write!(snippet, " returns {}", self.index.type_name(ret)).unwrap();
            }
            (None, _) => {}
        }
        snippet
    }

    fn param(self, param: ParamId) -> Option<String> {
        let function = &self.index.functions[self.index.param_function(param)?];
        let position = param.into_raw().into_u32() - function.params.start().into_raw().into_u32();
        let data = &self.index.params[param];
        let mut snippet = format!(
            "parameter {}.",
            self.qualified(function.schema, function.name)
        );
        match data.name {
            Some(name) => snippet.push_str(self.name(name)),
            None => write!(snippet, "${}", position + 1).unwrap(),
        }
        if let Some(ty) = data.ty {
            write!(snippet, " {}", self.index.type_name(ty)).unwrap();
        }
        Some(snippet)
    }

    fn ty(self, ty: TypeId) -> String {
        let data = &self.index.types[ty];
        let name = self.qualified(data.schema, data.name);
        match data.kind {
            TypeKind::Shell => format!("shell type {name}"),
            TypeKind::Type => match data.columns {
                Some(columns) => format!("type {name} as {}", self.column_list(columns.range)),
                None => format!("type {name}"),
            },
            TypeKind::Domain => match data.base {
                Some(base) => format!("domain {name} as {}", self.index.type_name(base)),
                None => format!("domain {name}"),
            },
            TypeKind::Enum => {
                let mut snippet = format!("type {name} as enum (");
                for (i, label) in self.index.enum_labels(ty).enumerate() {
                    if i > 0 {
                        snippet.push_str(", ");
                    }
                    snippet.push('\'');
                    snippet.push_str(self.name(self.index.enum_labels[label].name));
                    snippet.push('\'');
                }
                snippet.push(')');
                snippet
            }
            TypeKind::Range => match data.subtype {
                Some(subtype) => format!(
                    "type {name} as range (subtype = {})",
                    self.index.type_name(subtype)
                ),
                None => format!("type {name} as range"),
            },
            TypeKind::Multirange => format!("multirange type {name}"),
        }
    }

    fn source(self, source: SourceId) -> Option<String> {
        let data = &self.index.sources[source];
        match data.kind {
            SourceKind::Table(table) => Some(self.table(table)),
            SourceKind::Cte(cte) => item_hover(self.index, Item::Cte(cte)),
            SourceKind::Function(function) => Some(self.function(function)),
            SourceKind::Returning(target)
            | SourceKind::Excluded(target)
            | SourceKind::RuleRelation(target) => self.source(target),
            SourceKind::Subquery(_) => Some(match data.name {
                Some(name) => format!(
                    "subquery {}{}",
                    self.name(name),
                    self.column_list(data.columns.range)
                ),
                None => format!("subquery{}", self.column_list(data.columns.range)),
            }),
            SourceKind::External
            | SourceKind::Join
            | SourceKind::UsingAlias
            | SourceKind::ElementVariable
            | SourceKind::Other => None,
        }
    }

    fn columns(self, column: ColumnId) -> String {
        let mut snippet = String::new();
        for (i, column) in self.index.merged_columns(column).iter().enumerate() {
            if i > 0 {
                snippet.push('\n');
            }
            self.write_column(&mut snippet, column);
        }
        snippet
    }

    fn star(self, star: StarId) -> Option<String> {
        let columns = self.index.stars[star].columns;
        if columns.is_empty() {
            return None;
        }
        let mut snippet = String::new();
        for column in columns {
            for column in self.index.merged_columns(self.index.star_columns[column]) {
                if !snippet.is_empty() {
                    snippet.push('\n');
                }
                self.write_column(&mut snippet, column);
            }
        }
        Some(snippet)
    }

    fn write_column(self, out: &mut String, column: ColumnId) {
        let index = self.index;
        match index.column_owner(column) {
            ColumnOwner::Table(table) => write!(out, "column {}.", self.table_name(table)),
            ColumnOwner::Type(ty) => {
                let ty = &index.types[ty];
                write!(out, "field {}.", self.qualified(ty.schema, ty.name))
            }
            ColumnOwner::Function(function) => {
                let function = &index.functions[function];
                write!(
                    out,
                    "column {}.",
                    self.qualified(function.schema, function.name)
                )
            }
            ColumnOwner::Cte(cte) => write!(out, "column {}.", self.name(index.ctes[cte].name)),
            ColumnOwner::PreparedStatement(prepared) => write!(
                out,
                "column {}.",
                self.name(index.prepared_statements[prepared].name)
            ),
            ColumnOwner::Anonymous => write!(out, "column "),
        }
        .unwrap();
        let data = &index.columns[column];
        out.push_str(data.name.map_or("?column?", |name| self.name(name)));
        if let Some(ty) = data.ty {
            write!(out, " {}", index.type_name(ty)).unwrap();
        }
    }
}

struct QualifiedName<'a> {
    index: &'a SemanticIndex,
    schema: Name,
    name: Name,
}

impl fmt::Display for QualifiedName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = &self.index.names;
        write!(
            f,
            "{}.{}",
            names.resolve(self.schema),
            names.resolve(self.name)
        )
    }
}

struct OperandName<'a> {
    index: &'a SemanticIndex,
    operand: Operand,
}

impl fmt::Display for OperandName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operand {
            Operand::None => f.write_str("none"),
            Operand::Unknown => f.write_str("unknown"),
            Operand::Ty(ty) => write!(f, "{}", self.index.type_name(ty)),
        }
    }
}

struct ColumnList<'a> {
    index: &'a SemanticIndex,
    columns: IdxRange<Column>,
}

impl fmt::Display for ColumnList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            let column = &self.index.columns[column];
            let name = column
                .name
                .map_or("?column?", |name| self.index.names.resolve(name));
            f.write_str(name)?;
            if let Some(ty) = column.ty {
                write!(f, " {}", self.index.type_name(ty))?;
            }
        }
        f.write_str(")")
    }
}

struct ParamList<'a> {
    index: &'a SemanticIndex,
    function: FunctionId,
}

impl fmt::Display for ParamList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index = self.index;
        let function = &index.functions[self.function];
        f.write_str("(")?;
        for (i, param) in function.params.iter().enumerate() {
            match (i, function.direct_args == Some(i)) {
                (0, true) => f.write_str("order by ")?,
                (_, true) => f.write_str(" order by ")?,
                (0, false) => {}
                (_, false) => f.write_str(", ")?,
            }
            let param = &index.params[param];
            let mut parts = 0;
            let mode = match param.mode {
                ParamMode::In => None,
                ParamMode::Out => Some("out"),
                ParamMode::InOut => Some("inout"),
                ParamMode::Variadic => Some("variadic"),
            };
            for part in [mode, param.name.map(|name| index.names.resolve(name))]
                .into_iter()
                .flatten()
            {
                if parts > 0 {
                    f.write_str(" ")?;
                }
                f.write_str(part)?;
                parts += 1;
            }
            if let Some(ty) = param.ty {
                if parts > 0 {
                    f.write_str(" ")?;
                }
                write!(f, "{}", index.type_name(ty))?;
            }
            if param.has_default {
                f.write_str(" default")?;
            }
        }
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests;
