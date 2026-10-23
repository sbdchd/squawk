use rowan::TextRange;
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxNode,
    ast::{self, AstNode, NameLike},
};

use crate::arena::IdxRange;
use crate::db::{File, parse, semantic_index};
use crate::file::{FileId, InFile, IrFileId};
use crate::interner::Name;
use crate::ir::{Column, FunctionKind, Item, MemberKind, MemberOwner, TableKind, TypeId, TypeKind};
use crate::semantic_index::SemanticIndex;

#[salsa::tracked]
pub fn document_symbols(db: &dyn Db, file: File) -> Vec<DocumentSymbol> {
    document_symbols_with_index(
        semantic_index(db, file.into()),
        crate::file::InFile::new(file, &parse(db, file).tree()),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentSymbolKind {
    Schema,
    Table,
    View,
    MaterializedView,
    Function,
    Aggregate,
    Procedure,
    EventTrigger,
    Role,
    Rule,
    Policy,
    PropertyGraph,
    Type,
    Enum,
    Index,
    Domain,
    Sequence,
    Statistics,
    Trigger,
    Tablespace,
    Database,
    Server,
    Extension,
    OperatorClass,
    OperatorFamily,
    Operator,
    Publication,
    Subscription,
    Column,
    Variant,
    Cursor,
    PreparedStatement,
    Channel,
    Savepoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: DocumentSymbolKind,
    pub full_range: TextRange,
    pub focus_range: TextRange,
    pub children: Vec<DocumentSymbol>,
}

pub(crate) fn document_symbols_with_index(
    index: &SemanticIndex,
    file: InFile<&ast::SourceFile>,
) -> Vec<DocumentSymbol> {
    let symbols = Symbols {
        index,
        file_id: file.file_id,
        root: file.value.syntax(),
    };
    let mut result = vec![];
    for stmt in file.value.stmts() {
        symbols.stmt(&stmt, &mut result);
    }
    result
}

struct Symbols<'a> {
    index: &'a SemanticIndex,
    file_id: FileId,
    root: &'a SyntaxNode,
}

impl Symbols<'_> {
    fn stmt(&self, stmt: &ast::Stmt, result: &mut Vec<DocumentSymbol>) {
        let symbol = match stmt {
            ast::Stmt::CreateSchema(create_schema) => {
                let mut symbol = create_schema
                    .schema_name()
                    .and_then(|name| self.symbol(stmt.syntax(), &name));
                if let Some(symbol) = &mut symbol {
                    for element in create_schema.schema_elements() {
                        self.stmt(&element.into(), &mut symbol.children);
                    }
                }
                symbol
            }
            ast::Stmt::CreateOperator(create_operator) => create_operator.op().and_then(|op| {
                self.symbol_at_range(stmt.syntax().text_range(), op.operator_range())
            }),
            ast::Stmt::CreatePublication(create_publication) => {
                self.node_symbol(stmt, create_publication.publication())
            }
            ast::Stmt::CreateSubscription(create_subscription) => {
                self.node_symbol(stmt, create_subscription.subscription())
            }
            ast::Stmt::CreateTable(create_table) => {
                self.path_symbol(stmt, create_table.table_name().and_then(|name| name.path()))
            }
            ast::Stmt::CreateForeignTable(create_table) => {
                self.path_symbol(stmt, create_table.table_name().and_then(|name| name.path()))
            }
            ast::Stmt::CreateTableAs(create_table_as) => self.path_symbol(
                stmt,
                create_table_as.table_name().and_then(|name| name.path()),
            ),
            ast::Stmt::CreateView(create_view) => {
                self.path_symbol(stmt, create_view.view().and_then(|view| view.path()))
            }
            ast::Stmt::CreateMaterializedView(create_view) => {
                self.path_symbol(stmt, create_view.view().and_then(|view| view.path()))
            }
            ast::Stmt::CreateFunction(create_function) => {
                self.path_symbol(stmt, create_function.name().and_then(|name| name.path()))
            }
            ast::Stmt::CreateAggregate(create_aggregate) => self.path_symbol(
                stmt,
                create_aggregate
                    .aggregate_name()
                    .and_then(|name| name.path()),
            ),
            ast::Stmt::CreateProcedure(create_procedure) => {
                self.path_symbol(stmt, create_procedure.name().and_then(|name| name.path()))
            }
            ast::Stmt::CreateOperatorClass(create_class) => self.path_symbol(
                stmt,
                create_class.op_class_name().and_then(|name| name.path()),
            ),
            ast::Stmt::CreateOperatorFamily(create_family) => self.path_symbol(
                stmt,
                create_family.op_family_name().and_then(|name| name.path()),
            ),
            ast::Stmt::CreateIndex(create_index) => {
                self.path_symbol(stmt, create_index.index().and_then(|name| name.path()))
            }
            ast::Stmt::CreateDomain(create_domain) => {
                self.path_symbol(stmt, create_domain.domain().and_then(|name| name.path()))
            }
            ast::Stmt::CreateSequence(create_sequence) => self.path_symbol(
                stmt,
                create_sequence.sequence().and_then(|name| name.path()),
            ),
            ast::Stmt::CreateStatistics(create_statistics) => self.path_symbol(
                stmt,
                create_statistics.statistics().and_then(|name| name.path()),
            ),
            ast::Stmt::CreatePropertyGraph(create_property_graph) => self.path_symbol(
                stmt,
                create_property_graph
                    .property_graph()
                    .and_then(|name| name.path()),
            ),
            ast::Stmt::CreateType(create_type) => {
                self.path_symbol(stmt, create_type.type_name().and_then(|name| name.path()))
            }
            ast::Stmt::CreateTrigger(create_trigger) => {
                self.node_symbol(stmt, create_trigger.trigger())
            }
            ast::Stmt::CreateEventTrigger(create_event_trigger) => {
                self.node_symbol(stmt, create_event_trigger.event_trigger())
            }
            ast::Stmt::CreateTablespace(create_tablespace) => {
                self.node_symbol(stmt, create_tablespace.tablespace())
            }
            ast::Stmt::CreateDatabase(create_database) => {
                self.node_symbol(stmt, create_database.database())
            }
            ast::Stmt::CreateServer(create_server) => {
                self.node_symbol(stmt, create_server.server())
            }
            ast::Stmt::CreateExtension(create_extension) => {
                self.node_symbol(stmt, create_extension.extension())
            }
            ast::Stmt::CreateRole(create_role) => self.node_symbol(stmt, create_role.role()),
            ast::Stmt::CreateUser(create_user) => self.node_symbol(stmt, create_user.role()),
            ast::Stmt::CreateGroup(create_group) => self.node_symbol(stmt, create_group.role()),
            ast::Stmt::CreateRule(create_rule) => self.node_symbol(stmt, create_rule.rule()),
            ast::Stmt::CreatePolicy(create_policy) => {
                self.node_symbol(stmt, create_policy.policy())
            }
            ast::Stmt::Declare(declare) => self.node_symbol(stmt, declare.cursor()),
            ast::Stmt::Prepare(prepare) => self.node_symbol(stmt, prepare.name()),
            ast::Stmt::SavepointCreate(savepoint) => self.node_symbol(stmt, savepoint.savepoint()),
            ast::Stmt::Listen(listen) => listen
                .channel()
                .map(|channel| channel_symbol(stmt, &channel, "listen")),
            ast::Stmt::Notify(notify) => notify
                .channel_ref()
                .map(|channel| channel_symbol(stmt, &channel, "notify")),
            ast::Stmt::Unlisten(unlisten) => unlisten
                .channel_ref()
                .map(|channel| channel_symbol(stmt, &channel, "unlisten")),
            ast::Stmt::Select(select) => {
                self.ctes(select, result);
                None
            }
            ast::Stmt::SelectInto(select_into) => {
                self.ctes(select_into, result);
                self.path_symbol(
                    stmt,
                    select_into
                        .into_clause()
                        .and_then(|into| into.table_name())
                        .and_then(|name| name.path()),
                )
            }
            ast::Stmt::Insert(insert) => {
                self.ctes(insert, result);
                None
            }
            ast::Stmt::Update(update) => {
                self.ctes(update, result);
                None
            }
            ast::Stmt::Delete(delete) => {
                self.ctes(delete, result);
                None
            }
            ast::Stmt::AlterAggregate(_)
            | ast::Stmt::AlterCollation(_)
            | ast::Stmt::AlterConversion(_)
            | ast::Stmt::AlterDatabase(_)
            | ast::Stmt::AlterDefaultPrivileges(_)
            | ast::Stmt::AlterDomain(_)
            | ast::Stmt::AlterEventTrigger(_)
            | ast::Stmt::AlterExtension(_)
            | ast::Stmt::AlterForeignDataWrapper(_)
            | ast::Stmt::AlterForeignTable(_)
            | ast::Stmt::AlterFunction(_)
            | ast::Stmt::AlterGroup(_)
            | ast::Stmt::AlterIndex(_)
            | ast::Stmt::AlterLanguage(_)
            | ast::Stmt::AlterLargeObject(_)
            | ast::Stmt::AlterMaterializedView(_)
            | ast::Stmt::AlterOperator(_)
            | ast::Stmt::AlterOperatorClass(_)
            | ast::Stmt::AlterOperatorFamily(_)
            | ast::Stmt::AlterPolicy(_)
            | ast::Stmt::AlterProcedure(_)
            | ast::Stmt::AlterPropertyGraph(_)
            | ast::Stmt::AlterPublication(_)
            | ast::Stmt::AlterRole(_)
            | ast::Stmt::AlterRoutine(_)
            | ast::Stmt::AlterRule(_)
            | ast::Stmt::AlterSchema(_)
            | ast::Stmt::AlterSequence(_)
            | ast::Stmt::AlterServer(_)
            | ast::Stmt::AlterStatistics(_)
            | ast::Stmt::AlterSubscription(_)
            | ast::Stmt::AlterSystem(_)
            | ast::Stmt::AlterTable(_)
            | ast::Stmt::AlterTablespace(_)
            | ast::Stmt::AlterTextSearchConfiguration(_)
            | ast::Stmt::AlterTextSearchDictionary(_)
            | ast::Stmt::AlterTextSearchParser(_)
            | ast::Stmt::AlterTextSearchTemplate(_)
            | ast::Stmt::AlterTrigger(_)
            | ast::Stmt::AlterType(_)
            | ast::Stmt::AlterUser(_)
            | ast::Stmt::AlterUserMapping(_)
            | ast::Stmt::AlterView(_)
            | ast::Stmt::Analyze(_)
            | ast::Stmt::Begin(_)
            | ast::Stmt::Call(_)
            | ast::Stmt::Checkpoint(_)
            | ast::Stmt::Close(_)
            | ast::Stmt::Cluster(_)
            | ast::Stmt::CommentOn(_)
            | ast::Stmt::CompoundSelect(_)
            | ast::Stmt::Copy(_)
            | ast::Stmt::CreateAccessMethod(_)
            | ast::Stmt::CreateCast(_)
            | ast::Stmt::CreateCollation(_)
            | ast::Stmt::CreateConversion(_)
            | ast::Stmt::CreateForeignDataWrapper(_)
            | ast::Stmt::CreateLanguage(_)
            | ast::Stmt::CreateTextSearchConfiguration(_)
            | ast::Stmt::CreateTextSearchDictionary(_)
            | ast::Stmt::CreateTextSearchParser(_)
            | ast::Stmt::CreateTextSearchTemplate(_)
            | ast::Stmt::CreateTransform(_)
            | ast::Stmt::CreateUserMapping(_)
            | ast::Stmt::Deallocate(_)
            | ast::Stmt::Discard(_)
            | ast::Stmt::Do(_)
            | ast::Stmt::DropAccessMethod(_)
            | ast::Stmt::DropAggregate(_)
            | ast::Stmt::DropCast(_)
            | ast::Stmt::DropCollation(_)
            | ast::Stmt::DropConversion(_)
            | ast::Stmt::DropDatabase(_)
            | ast::Stmt::DropDomain(_)
            | ast::Stmt::DropEventTrigger(_)
            | ast::Stmt::DropExtension(_)
            | ast::Stmt::DropForeignDataWrapper(_)
            | ast::Stmt::DropForeignTable(_)
            | ast::Stmt::DropFunction(_)
            | ast::Stmt::DropGroup(_)
            | ast::Stmt::DropIndex(_)
            | ast::Stmt::DropLanguage(_)
            | ast::Stmt::DropMaterializedView(_)
            | ast::Stmt::DropOperator(_)
            | ast::Stmt::DropOperatorClass(_)
            | ast::Stmt::DropOperatorFamily(_)
            | ast::Stmt::DropOwned(_)
            | ast::Stmt::DropPolicy(_)
            | ast::Stmt::DropProcedure(_)
            | ast::Stmt::DropPropertyGraph(_)
            | ast::Stmt::DropPublication(_)
            | ast::Stmt::DropRole(_)
            | ast::Stmt::DropRoutine(_)
            | ast::Stmt::DropRule(_)
            | ast::Stmt::DropSchema(_)
            | ast::Stmt::DropSequence(_)
            | ast::Stmt::DropServer(_)
            | ast::Stmt::DropStatistics(_)
            | ast::Stmt::DropSubscription(_)
            | ast::Stmt::DropTable(_)
            | ast::Stmt::DropTablespace(_)
            | ast::Stmt::DropTextSearchConfig(_)
            | ast::Stmt::DropTextSearchDict(_)
            | ast::Stmt::DropTextSearchParser(_)
            | ast::Stmt::DropTextSearchTemplate(_)
            | ast::Stmt::DropTransform(_)
            | ast::Stmt::DropTrigger(_)
            | ast::Stmt::DropType(_)
            | ast::Stmt::DropUser(_)
            | ast::Stmt::DropUserMapping(_)
            | ast::Stmt::DropView(_)
            | ast::Stmt::EmptyStmt(_)
            | ast::Stmt::Execute(_)
            | ast::Stmt::Explain(_)
            | ast::Stmt::Fetch(_)
            | ast::Stmt::Grant(_)
            | ast::Stmt::ImportForeignSchema(_)
            | ast::Stmt::Load(_)
            | ast::Stmt::Lock(_)
            | ast::Stmt::Merge(_)
            | ast::Stmt::Move(_)
            | ast::Stmt::ParenSelect(_)
            | ast::Stmt::PrepareTransaction(_)
            | ast::Stmt::Reassign(_)
            | ast::Stmt::Refresh(_)
            | ast::Stmt::Reindex(_)
            | ast::Stmt::ReleaseSavepoint(_)
            | ast::Stmt::Repack(_)
            | ast::Stmt::Reset(_)
            | ast::Stmt::ResetRole(_)
            | ast::Stmt::ResetSessionAuth(_)
            | ast::Stmt::Revoke(_)
            | ast::Stmt::SecurityLabel(_)
            | ast::Stmt::Set(_)
            | ast::Stmt::SetConstraints(_)
            | ast::Stmt::SetRole(_)
            | ast::Stmt::SetSessionAuth(_)
            | ast::Stmt::SetTransaction(_)
            | ast::Stmt::Show(_)
            | ast::Stmt::Table(_)
            | ast::Stmt::Truncate(_)
            | ast::Stmt::Vacuum(_)
            | ast::Stmt::Values(_)
            | ast::Stmt::Commit(_)
            | ast::Stmt::Rollback(_) => None,
        };
        result.extend(symbol);
    }

    fn ctes(&self, stmt: &impl ast::HasWithClause, result: &mut Vec<DocumentSymbol>) {
        let Some(with_clause) = stmt.with_clause() else {
            return;
        };
        for with_table in with_clause.with_tables() {
            if let Some(name) = with_table.name() {
                result.extend(self.symbol(with_table.syntax(), name.syntax()));
            }
        }
    }

    fn path_symbol(&self, stmt: &ast::Stmt, path: Option<ast::Path>) -> Option<DocumentSymbol> {
        let segment = path?.segment()?;
        self.symbol(stmt.syntax(), segment.syntax())
    }

    fn node_symbol(&self, stmt: &ast::Stmt, name: Option<impl AstNode>) -> Option<DocumentSymbol> {
        self.symbol(stmt.syntax(), name?.syntax())
    }

    fn symbol(&self, node: &SyntaxNode, name: &SyntaxNode) -> Option<DocumentSymbol> {
        self.symbol_at_range(node.text_range(), name.text_range())
    }

    fn symbol_at_range(
        &self,
        full_range: TextRange,
        focus_range: TextRange,
    ) -> Option<DocumentSymbol> {
        let Described {
            name,
            kind,
            children,
        } = self
            .index
            .items_at_range(InFile::new(self.file_id, focus_range))
            .find_map(|item| self.describe(item, full_range))?;
        Some(DocumentSymbol {
            name,
            detail: None,
            kind,
            full_range,
            focus_range,
            children,
        })
    }

    fn describe(&self, item: Item, full_range: TextRange) -> Option<Described> {
        let index = self.index;
        Some(match item {
            Item::Table(table) => {
                let table = &index.tables[table];
                let kind = match table.kind {
                    TableKind::Table | TableKind::ForeignTable => DocumentSymbolKind::Table,
                    TableKind::View => DocumentSymbolKind::View,
                    TableKind::MaterializedView => DocumentSymbolKind::MaterializedView,
                    TableKind::Sequence => DocumentSymbolKind::Sequence,
                    TableKind::PropertyGraph => DocumentSymbolKind::PropertyGraph,
                };
                let children = match table.kind {
                    TableKind::Table
                    | TableKind::ForeignTable
                    | TableKind::View
                    | TableKind::MaterializedView => self.columns(table.columns.range, full_range),
                    TableKind::Sequence | TableKind::PropertyGraph => vec![],
                };
                Described {
                    name: self.qualified(table.schema, table.name),
                    kind,
                    children,
                }
            }
            Item::Cte(cte) => {
                let cte = &index.ctes[cte];
                Described {
                    name: self.name(cte.name).to_string(),
                    kind: DocumentSymbolKind::Table,
                    children: self.columns(cte.columns.range, full_range),
                }
            }
            Item::Function(function) => {
                let function = &index.functions[function];
                let kind = match function.kind {
                    FunctionKind::Function => DocumentSymbolKind::Function,
                    FunctionKind::Aggregate => DocumentSymbolKind::Aggregate,
                    FunctionKind::Procedure => DocumentSymbolKind::Procedure,
                };
                Described::leaf(self.qualified(function.schema, function.name), kind)
            }
            Item::Type(id) => {
                let ty = &index.types[id];
                let (kind, children) = match ty.kind {
                    TypeKind::Enum => (DocumentSymbolKind::Enum, self.variants(id, full_range)),
                    TypeKind::Domain => (DocumentSymbolKind::Domain, vec![]),
                    TypeKind::Shell | TypeKind::Type | TypeKind::Range | TypeKind::Multirange => (
                        DocumentSymbolKind::Type,
                        ty.columns
                            .map(|columns| self.columns(columns.range, full_range))
                            .unwrap_or_default(),
                    ),
                };
                Described {
                    name: self.qualified(ty.schema, ty.name),
                    kind,
                    children,
                }
            }
            Item::Index(idx) => {
                let idx = &index.indexes[idx];
                Described::leaf(
                    self.qualified(idx.schema, idx.name),
                    DocumentSymbolKind::Index,
                )
            }
            Item::Statistics(statistics) => {
                let statistics = &index.statistics[statistics];
                Described::leaf(
                    self.qualified(statistics.schema, statistics.name),
                    DocumentSymbolKind::Statistics,
                )
            }
            Item::Schema(schema) => {
                self.leaf(index.schemas[schema].name, DocumentSymbolKind::Schema)
            }
            Item::Trigger(trigger) => {
                self.leaf(index.triggers[trigger].name, DocumentSymbolKind::Trigger)
            }
            Item::EventTrigger(event_trigger) => self.leaf(
                index.event_triggers[event_trigger].name,
                DocumentSymbolKind::EventTrigger,
            ),
            Item::Rule(rule) => self.leaf(index.rules[rule].name, DocumentSymbolKind::Rule),
            Item::Role(role) => self.leaf(index.roles[role].name, DocumentSymbolKind::Role),
            Item::Policy(policy) => {
                self.leaf(index.policies[policy].name, DocumentSymbolKind::Policy)
            }
            Item::Tablespace(tablespace) => self.leaf(
                index.tablespaces[tablespace].name,
                DocumentSymbolKind::Tablespace,
            ),
            Item::Database(database) => {
                self.leaf(index.databases[database].name, DocumentSymbolKind::Database)
            }
            Item::Server(server) => {
                self.leaf(index.servers[server].name, DocumentSymbolKind::Server)
            }
            Item::Extension(extension) => self.leaf(
                index.extensions[extension].name,
                DocumentSymbolKind::Extension,
            ),
            Item::OperatorClass(class) => {
                let class_data = &index.operator_classes[class];
                Described {
                    name: self.qualified(class_data.schema, class_data.name),
                    kind: DocumentSymbolKind::OperatorClass,
                    children: self.members(MemberOwner::Class(class), full_range),
                }
            }
            Item::OperatorFamily(family) => {
                let family_data = &index.operator_families[family];
                Described {
                    name: self.qualified(family_data.schema, family_data.name),
                    kind: DocumentSymbolKind::OperatorFamily,
                    children: self.members(MemberOwner::Family(family), full_range),
                }
            }
            Item::Cursor(cursor) => {
                self.leaf(index.cursors[cursor].name, DocumentSymbolKind::Cursor)
            }
            Item::PreparedStatement(prepared) => self.leaf(
                index.prepared_statements[prepared].name,
                DocumentSymbolKind::PreparedStatement,
            ),
            Item::Savepoint(savepoint) => self.leaf(
                index.savepoints[savepoint].name,
                DocumentSymbolKind::Savepoint,
            ),
            Item::Operator(operator) => {
                let operator = &index.operators[operator];
                Described::leaf(
                    self.qualified(operator.schema, operator.name),
                    DocumentSymbolKind::Operator,
                )
            }
            Item::Publication(publication) => self.leaf(
                index.publications[publication].name,
                DocumentSymbolKind::Publication,
            ),
            Item::Subscription(subscription) => self.leaf(
                index.subscriptions[subscription].name,
                DocumentSymbolKind::Subscription,
            ),
            Item::Reference(_)
            | Item::Column(_)
            | Item::Source(_)
            | Item::Param(_)
            | Item::Window(_)
            | Item::JsonPath(_)
            | Item::Constraint(_)
            | Item::AccessMethod(_)
            | Item::Language(_)
            | Item::ForeignDataWrapper(_)
            | Item::Conversion(_)
            | Item::TextSearch(_)
            | Item::GraphElement(_)
            | Item::GraphLabel(_)
            | Item::EnumLabel(_)
            | Item::GraphProperty(_)
            | Item::Collation(_)
            | Item::PreparedTransaction(_)
            | Item::Channel(_)
            | Item::TransactionBegin(_)
            | Item::TransactionEnd(_)
            | Item::CaseKeyword(_)
            | Item::Star(_) => return None,
        })
    }

    fn columns(&self, columns: IdxRange<Column>, full_range: TextRange) -> Vec<DocumentSymbol> {
        columns
            .into_iter()
            .filter_map(|id| {
                let column = &self.index.columns[id];
                let name = self.name(column.name?).to_string();
                let Some(focus_range) = self.local_range(
                    column.ptr.file_id,
                    column.ptr.value.text_range(),
                    full_range,
                ) else {
                    let star_range = self.star_range(column, full_range)?;
                    return Some(DocumentSymbol {
                        name,
                        detail: column.ty.map(|ty| self.index.type_name(ty).to_string()),
                        kind: DocumentSymbolKind::Column,
                        full_range: star_range,
                        focus_range: star_range,
                        children: vec![],
                    });
                };
                let parent = column.ptr.value.to_node(self.root).parent();
                let declared = parent.as_ref().and_then(|parent| {
                    if let Some(column) = ast::Column::cast(parent.clone()) {
                        Some((parent.text_range(), column.ty()))
                    } else {
                        ast::CompositeFieldDef::cast(parent.clone())
                            .map(|field| (parent.text_range(), field.ty()))
                    }
                });
                let (full_range, detail) = match declared {
                    Some((range, ty)) => (range, ty.map(|ty| ty.syntax().text().to_string())),
                    None => (
                        focus_range,
                        column.ty.map(|ty| self.index.type_name(ty).to_string()),
                    ),
                };
                Some(DocumentSymbol {
                    name,
                    detail,
                    kind: DocumentSymbolKind::Column,
                    full_range,
                    focus_range,
                    children: vec![],
                })
            })
            .collect()
    }

    fn star_range(&self, column: &Column, full_range: TextRange) -> Option<TextRange> {
        let items = self.index.file_items(self.file_id);
        let start = items.partition_point(|item| item.value.range.start() < full_range.start());
        items[start..]
            .iter()
            .take_while(|item| item.value.range.start() < full_range.end())
            .find_map(|item| {
                let Item::Star(star) = item.value.item else {
                    return None;
                };
                self.index.stars[star]
                    .columns
                    .into_iter()
                    .any(|id| self.index.columns[self.index.star_columns[id]].ptr == column.ptr)
                    .then_some(item.value.range)
            })
    }

    fn variants(&self, ty: TypeId, full_range: TextRange) -> Vec<DocumentSymbol> {
        self.index
            .enum_labels(ty)
            .filter_map(|label| {
                let label = &self.index.enum_labels[label];
                let range = self.local_range(
                    label.name_ptr.file_id,
                    label.name_ptr.value.text_range(),
                    full_range,
                )?;
                Some(DocumentSymbol {
                    name: self.name(label.name).to_string(),
                    detail: None,
                    kind: DocumentSymbolKind::Variant,
                    full_range: range,
                    focus_range: range,
                    children: vec![],
                })
            })
            .collect()
    }

    fn members(&self, owner: MemberOwner, full_range: TextRange) -> Vec<DocumentSymbol> {
        self.index
            .operator_family_members
            .get(owner)
            .rev()
            .filter_map(|member| {
                let member = &self.index.operator_family_members[member];
                let range = self.local_range(
                    member.ptr.file_id,
                    member.ptr.value.text_range(),
                    full_range,
                )?;
                let option = ast::OpClassOption::cast(member.ptr.value.to_node(self.root))?;
                let (name, literal) = match option {
                    ast::OpClassOption::OpClassOptionOperator(operator) => {
                        (operator.op()?.syntax().clone(), operator.literal())
                    }
                    ast::OpClassOption::OpClassOptionFunction(function) => (
                        function
                            .function_sig()?
                            .function_name_ref()?
                            .syntax()
                            .clone(),
                        function.literal(),
                    ),
                    ast::OpClassOption::OpClassOptionStorage(_) => return None,
                };
                let (kind, number) = match member.kind {
                    MemberKind::Operator => (DocumentSymbolKind::Operator, "strategy"),
                    MemberKind::Function => (DocumentSymbolKind::Function, "support"),
                };
                Some(DocumentSymbol {
                    name: name.text().to_string(),
                    detail: literal.map(|literal| format!("{number} {}", literal.syntax().text())),
                    kind,
                    full_range: range,
                    focus_range: name.text_range(),
                    children: vec![],
                })
            })
            .collect()
    }

    fn local_range(
        &self,
        file_id: IrFileId,
        range: TextRange,
        parent: TextRange,
    ) -> Option<TextRange> {
        (file_id == self.file_id.into() && parent.contains_range(range)).then_some(range)
    }

    fn leaf(&self, name: Name, kind: DocumentSymbolKind) -> Described {
        Described::leaf(self.name(name).to_string(), kind)
    }

    fn name(&self, name: Name) -> &str {
        self.index.names.resolve(name)
    }

    fn qualified(&self, schema: Name, name: Name) -> String {
        format!("{}.{}", self.name(schema), self.name(name))
    }
}

fn channel_symbol(stmt: &ast::Stmt, channel: &impl NameLike, detail: &str) -> DocumentSymbol {
    DocumentSymbol {
        name: channel.text(),
        detail: Some(detail.to_string()),
        kind: DocumentSymbolKind::Channel,
        full_range: stmt.syntax().text_range(),
        focus_range: channel.syntax().text_range(),
        children: vec![],
    }
}

struct Described {
    name: String,
    kind: DocumentSymbolKind,
    children: Vec<DocumentSymbol>,
}

impl Described {
    fn leaf(name: String, kind: DocumentSymbolKind) -> Described {
        Described {
            name,
            kind,
            children: vec![],
        }
    }
}

#[cfg(test)]
mod tests;
