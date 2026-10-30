use rowan::{NodeOrToken, TextRange, TextSize, WalkEvent};
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{self, AstNode},
    match_ast,
};

use squawk_sema::file::{FileId, InFile, InIrFile, IrFileId};
use squawk_sema::ir::{Item, Resolution, TableId, TableKind};
use squawk_sema::lower::is_public_role;
use squawk_sema::semantic_index::SemanticIndex;

use crate::db::{Db, parse, semantic_index};

pub fn semantic_tokens(
    db: &dyn Db,
    file_id: FileId,
    range_to_highlight: Option<TextRange>,
) -> Vec<SemanticToken> {
    semantic_tokens_with_index(
        semantic_index(db, file_id),
        squawk_sema::file::InFile::new(file_id, &parse(db, file_id).tree()),
        range_to_highlight,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticToken {
    pub range: TextRange,
    pub token_type: SemanticTokenType,
    pub modifiers: Option<SemanticTokenModifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticTokenModifier {
    Definition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticTokenType {
    Keyword,
    String,
    Bool,
    Number,
    Function,
    Operator,
    Punctuation,
    Name,
    NameRef,
    Comment,
    Column,
    Type,
    Parameter,
    PositionalParam,
    PropertyGraph,
    Table,
    Schema,
    Window,
    JsonPath,
    Index,
    Constraint,
    Trigger,
    Rule,
    Role,
    Policy,
    AccessMethod,
    EventTrigger,
    Language,
    ForeignDataWrapper,
    Server,
    Conversion,
    OperatorFamily,
    OperatorClass,
    Statistics,
    TextSearch,
    Publication,
    Subscription,
    Collation,
    Tablespace,
    Database,
    Extension,
    Savepoint,
    Cursor,
    PreparedTransaction,
    PreparedStatement,
    Channel,
    EnumMember,
}

pub(crate) fn semantic_tokens_with_index(
    index: &SemanticIndex,
    file: InFile<&ast::SourceFile>,
    range: Option<TextRange>,
) -> Vec<SemanticToken> {
    let root = file.value.syntax();
    let mut highlighter = Highlighter {
        index,
        range: range.unwrap_or_else(|| root.text_range()),
        tokens: vec![],
    };
    highlighter.items(file.file_id);
    highlighter.syntax(file.file_id.into(), root);
    for (body_id, body) in index.bodies.iter_local() {
        if index.file(body.parent) != file.file_id {
            continue;
        }
        highlighter.syntax(IrFileId::Body(body_id), &body.sql.syntax());
    }
    highlighter.finish()
}

struct Highlighter<'a> {
    index: &'a SemanticIndex,
    range: TextRange,
    tokens: Vec<SemanticToken>,
}

impl Highlighter<'_> {
    fn finish(mut self) -> Vec<SemanticToken> {
        self.tokens
            .sort_by_key(|token| (token.range.start(), token.range.end()));
        let mut end = TextSize::new(0);
        self.tokens.retain(|token| {
            let keep = token.range.start() >= end;
            if keep {
                end = token.range.end();
            }
            keep
        });
        self.tokens
    }

    fn items(&mut self, file_id: FileId) {
        let index = self.index;
        for item in index.file_items(file_id) {
            let Some(token_type) = self.item_type(item.value.item) else {
                continue;
            };
            let modifiers = if let Item::Reference(_) = item.value.item {
                None
            } else {
                Some(SemanticTokenModifier::Definition)
            };
            self.push(file_id.into(), item.value.range, token_type, modifiers);
        }
    }

    fn item_type(&self, item: Item) -> Option<SemanticTokenType> {
        match item {
            Item::Reference(reference) => {
                self.resolution_type(self.index.references[reference].resolution)
            }
            Item::Cte(_) | Item::Source(_) | Item::GraphElement(_) => {
                Some(SemanticTokenType::Table)
            }
            Item::Column(_) => Some(SemanticTokenType::Column),
            Item::Table(table) => Some(self.table_type(table)),
            Item::Function(_) => Some(SemanticTokenType::Function),
            Item::Param(_) => Some(SemanticTokenType::Parameter),
            Item::Type(_) => Some(SemanticTokenType::Type),
            Item::Schema(_) => Some(SemanticTokenType::Schema),
            Item::GraphLabel(_) | Item::GraphProperty(_) => Some(SemanticTokenType::Name),
            Item::Window(_) => Some(SemanticTokenType::Window),
            Item::JsonPath(_) => Some(SemanticTokenType::JsonPath),
            Item::Index(_) => Some(SemanticTokenType::Index),
            Item::Constraint(_) => Some(SemanticTokenType::Constraint),
            Item::Trigger(_) => Some(SemanticTokenType::Trigger),
            Item::Rule(_) => Some(SemanticTokenType::Rule),
            Item::Role(_) => Some(SemanticTokenType::Role),
            Item::Policy(_) => Some(SemanticTokenType::Policy),
            Item::AccessMethod(_) => Some(SemanticTokenType::AccessMethod),
            Item::EventTrigger(_) => Some(SemanticTokenType::EventTrigger),
            Item::Language(_) => Some(SemanticTokenType::Language),
            Item::ForeignDataWrapper(_) => Some(SemanticTokenType::ForeignDataWrapper),
            Item::Server(_) => Some(SemanticTokenType::Server),
            Item::Conversion(_) => Some(SemanticTokenType::Conversion),
            Item::OperatorFamily(_) => Some(SemanticTokenType::OperatorFamily),
            Item::OperatorClass(_) => Some(SemanticTokenType::OperatorClass),
            Item::Statistics(_) => Some(SemanticTokenType::Statistics),
            Item::TextSearch(_) => Some(SemanticTokenType::TextSearch),
            Item::Publication(_) => Some(SemanticTokenType::Publication),
            Item::Subscription(_) => Some(SemanticTokenType::Subscription),
            Item::Collation(_) => Some(SemanticTokenType::Collation),
            Item::Tablespace(_) => Some(SemanticTokenType::Tablespace),
            Item::Database(_) => Some(SemanticTokenType::Database),
            Item::Extension(_) => Some(SemanticTokenType::Extension),
            Item::Savepoint(_) => Some(SemanticTokenType::Savepoint),
            Item::Cursor(_) => Some(SemanticTokenType::Cursor),
            Item::PreparedTransaction(_) => Some(SemanticTokenType::PreparedTransaction),
            Item::PreparedStatement(_) => Some(SemanticTokenType::PreparedStatement),
            Item::Channel(_) => Some(SemanticTokenType::Channel),
            Item::EnumLabel(_) => Some(SemanticTokenType::EnumMember),
            Item::Operator(_) => Some(SemanticTokenType::Operator),
            Item::TransactionBegin(_)
            | Item::TransactionEnd(_)
            | Item::CaseKeyword(_)
            | Item::Star(_) => None,
        }
    }

    fn resolution_type(&self, resolution: Resolution) -> Option<SemanticTokenType> {
        match resolution {
            Resolution::Source(_) | Resolution::Cte(_) | Resolution::GraphElement(_) => {
                Some(SemanticTokenType::Table)
            }
            Resolution::Column { .. } => Some(SemanticTokenType::Column),
            Resolution::Table(table) => Some(self.table_type(table)),
            Resolution::Function(_) => Some(SemanticTokenType::Function),
            Resolution::Param(_) => Some(SemanticTokenType::Parameter),
            Resolution::Type(_) => Some(SemanticTokenType::Type),
            Resolution::Schema(_) => Some(SemanticTokenType::Schema),
            Resolution::GraphLabel(_) | Resolution::GraphProperty(_) => {
                Some(SemanticTokenType::Name)
            }
            Resolution::Window(_) => Some(SemanticTokenType::Window),
            Resolution::JsonPath(_) => Some(SemanticTokenType::JsonPath),
            Resolution::Index(_) => Some(SemanticTokenType::Index),
            Resolution::Constraint(_) => Some(SemanticTokenType::Constraint),
            Resolution::Trigger(_) => Some(SemanticTokenType::Trigger),
            Resolution::Rule(_) => Some(SemanticTokenType::Rule),
            Resolution::Role(_) => Some(SemanticTokenType::Role),
            Resolution::Policy(_) => Some(SemanticTokenType::Policy),
            Resolution::AccessMethod(_) => Some(SemanticTokenType::AccessMethod),
            Resolution::EventTrigger(_) => Some(SemanticTokenType::EventTrigger),
            Resolution::Language(_) => Some(SemanticTokenType::Language),
            Resolution::ForeignDataWrapper(_) => Some(SemanticTokenType::ForeignDataWrapper),
            Resolution::Server(_) => Some(SemanticTokenType::Server),
            Resolution::Conversion(_) => Some(SemanticTokenType::Conversion),
            Resolution::OperatorFamily(_) => Some(SemanticTokenType::OperatorFamily),
            Resolution::OperatorClass(_) => Some(SemanticTokenType::OperatorClass),
            Resolution::Statistics(_) => Some(SemanticTokenType::Statistics),
            Resolution::TextSearch(_) => Some(SemanticTokenType::TextSearch),
            Resolution::Publication(_) => Some(SemanticTokenType::Publication),
            Resolution::Subscription(_) => Some(SemanticTokenType::Subscription),
            Resolution::Collation(_) => Some(SemanticTokenType::Collation),
            Resolution::Tablespace(_) => Some(SemanticTokenType::Tablespace),
            Resolution::Database(_) => Some(SemanticTokenType::Database),
            Resolution::Extension(_) => Some(SemanticTokenType::Extension),
            Resolution::Savepoint(_) => Some(SemanticTokenType::Savepoint),
            Resolution::Cursor(_) => Some(SemanticTokenType::Cursor),
            Resolution::PreparedTransaction(_) => Some(SemanticTokenType::PreparedTransaction),
            Resolution::PreparedStatement(_) => Some(SemanticTokenType::PreparedStatement),
            Resolution::Channel(_) => Some(SemanticTokenType::Channel),
            Resolution::EnumLabel(_) => Some(SemanticTokenType::EnumMember),
            Resolution::Operator(_) => Some(SemanticTokenType::Operator),
            Resolution::Unknown | Resolution::Unresolved | Resolution::Ambiguous => None,
        }
    }

    fn table_type(&self, table: TableId) -> SemanticTokenType {
        match self.index.tables[table].kind {
            TableKind::PropertyGraph => SemanticTokenType::PropertyGraph,
            TableKind::Table
            | TableKind::ForeignTable
            | TableKind::View
            | TableKind::MaterializedView
            | TableKind::Sequence => SemanticTokenType::Table,
        }
    }

    fn syntax(&mut self, file_id: IrFileId, root: &SyntaxNode) {
        let mut preorder = root.preorder_with_tokens();
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(element) = event else {
                continue;
            };
            if self
                .original_range(file_id, element.text_range())
                .intersect(self.range)
                .is_none()
            {
                preorder.skip_subtree();
                continue;
            }
            match element {
                NodeOrToken::Node(node) => self.node(file_id, &node),
                NodeOrToken::Token(token) => self.token(file_id, &token),
            }
        }
    }

    fn node(&mut self, file_id: IrFileId, node: &SyntaxNode) {
        match_ast! {
            match node {
                ast::Type(ty) => self.ty(file_id, ty),
                ast::ParamMode(mode) => self.param_mode(file_id, mode),
                ast::LikeClause(like_clause) => {
                    self.keyword(file_id, like_clause.like_token());
                },
                ast::NotNullConstraint(not_null_constraint) => {
                    self.keyword(file_id, not_null_constraint.not_token());
                },
                ast::PartitionForValuesIn(partition_for_values_in) => {
                    self.keyword(file_id, partition_for_values_in.in_token());
                },
                ast::RoleNameRef(name_ref) => {
                    if is_public_role(&name_ref) {
                        self.keyword(file_id, name_ref.ident_token());
                    }
                },
                _ => (),
            }
        }
    }

    fn token(&mut self, file_id: IrFileId, token: &SyntaxToken) {
        if token.kind() == SyntaxKind::POSITIONAL_PARAM {
            self.push(
                file_id,
                token.text_range(),
                SemanticTokenType::PositionalParam,
                None,
            );
            return;
        }
        let Some(token_type) = lexical_token_type(token.kind()) else {
            return;
        };
        let range = token.text_range();
        if let Some(body) = token
            .parent()
            .and_then(ast::Literal::cast)
            .and_then(|literal| literal.sql_body())
        {
            let content = body.source_range(TextRange::up_to(TextSize::of(body.text())));
            self.push(
                file_id,
                TextRange::new(range.start(), content.start()),
                token_type,
                None,
            );
            self.push(
                file_id,
                TextRange::new(content.end(), range.end()),
                token_type,
                None,
            );
            return;
        }
        self.push(file_id, range, token_type, None);
    }

    fn param_mode(&mut self, file_id: IrFileId, mode: ast::ParamMode) {
        match mode {
            ast::ParamMode::ParamIn(param_in) => self.keyword(file_id, param_in.in_token()),
            ast::ParamMode::ParamInOut(param_in_out) => {
                self.keyword(file_id, param_in_out.in_token());
                self.keyword(file_id, param_in_out.inout_token());
                self.keyword(file_id, param_in_out.out_token());
            }
            ast::ParamMode::ParamOut(param_out) => self.keyword(file_id, param_out.out_token()),
            ast::ParamMode::ParamVariadic(param_variadic) => {
                self.keyword(file_id, param_variadic.variadic_token());
            }
        }
    }

    fn ty(&mut self, file_id: IrFileId, ty: ast::Type) {
        match ty {
            ast::Type::ArrayType(_) => {}
            ast::Type::BitType(bit_type) => {
                self.type_keyword(file_id, bit_type.setof_token());
                self.type_keyword(file_id, bit_type.bit_token());
            }
            ast::Type::BitVaryingType(bit_varying_type) => {
                self.type_keyword(file_id, bit_varying_type.setof_token());
                self.type_keyword(file_id, bit_varying_type.bit_token());
                self.type_keyword(file_id, bit_varying_type.varying_token());
            }
            ast::Type::VarcharType(varchar_type) => {
                self.type_keyword(file_id, varchar_type.setof_token());
                self.type_keyword(file_id, varchar_type.national_token());
                self.type_keyword(
                    file_id,
                    varchar_type
                        .varchar_token()
                        .or_else(|| varchar_type.nchar_token())
                        .or_else(|| varchar_type.character_token())
                        .or_else(|| varchar_type.char_token()),
                );
                self.type_keyword(file_id, varchar_type.varying_token());
            }
            ast::Type::CharacterType(character_type) => {
                self.type_keyword(file_id, character_type.setof_token());
                self.type_keyword(file_id, character_type.national_token());
                self.type_keyword(
                    file_id,
                    character_type
                        .nchar_token()
                        .or_else(|| character_type.character_token())
                        .or_else(|| character_type.char_token()),
                );
            }
            ast::Type::DoubleType(double_type) => {
                self.type_keyword(file_id, double_type.setof_token());
                self.type_keyword(file_id, double_type.double_token());
                self.type_keyword(file_id, double_type.precision_token());
            }
            ast::Type::IntervalType(interval_type) => {
                self.type_keyword(file_id, interval_type.setof_token());
                self.type_keyword(file_id, interval_type.interval_token());
            }
            ast::Type::PathType(path_type) => {
                self.type_keyword(file_id, path_type.setof_token());
            }
            ast::Type::TimeType(time_type) => {
                self.type_keyword(file_id, time_type.setof_token());
                self.type_keyword(file_id, time_type.time_token());
                if let Some(timezone) = time_type.timezone() {
                    self.timezone(file_id, timezone);
                }
            }
            ast::Type::TimestampType(timestamp_type) => {
                self.type_keyword(file_id, timestamp_type.setof_token());
                self.type_keyword(file_id, timestamp_type.timestamp_token());
                if let Some(timezone) = timestamp_type.timezone() {
                    self.timezone(file_id, timezone);
                }
            }
        }
    }

    fn timezone(&mut self, file_id: IrFileId, timezone: ast::Timezone) {
        match timezone {
            ast::Timezone::WithTimezone(with_timezone) => {
                self.type_keyword(file_id, with_timezone.with_token());
                self.type_keyword(file_id, with_timezone.time_token());
                self.type_keyword(file_id, with_timezone.zone_token());
            }
            ast::Timezone::WithoutTimezone(without_timezone) => {
                self.type_keyword(file_id, without_timezone.without_token());
                self.type_keyword(file_id, without_timezone.time_token());
                self.type_keyword(file_id, without_timezone.zone_token());
            }
        }
    }

    fn keyword(&mut self, file_id: IrFileId, token: Option<SyntaxToken>) {
        if let Some(token) = token {
            self.push(
                file_id,
                token.text_range(),
                SemanticTokenType::Keyword,
                None,
            );
        }
    }

    fn type_keyword(&mut self, file_id: IrFileId, token: Option<SyntaxToken>) {
        if let Some(token) = token {
            self.push(file_id, token.text_range(), SemanticTokenType::Type, None);
        }
    }

    fn push(
        &mut self,
        file_id: IrFileId,
        range: TextRange,
        token_type: SemanticTokenType,
        modifiers: Option<SemanticTokenModifier>,
    ) {
        let range = self.original_range(file_id, range);
        if range.intersect(self.range).is_none() {
            return;
        }
        self.tokens.push(SemanticToken {
            range,
            token_type,
            modifiers,
        });
    }

    fn original_range(&self, file_id: IrFileId, range: TextRange) -> TextRange {
        self.index.range(InIrFile::new(file_id, range)).value
    }
}

fn lexical_token_type(kind: SyntaxKind) -> Option<SemanticTokenType> {
    match kind {
        SyntaxKind::TRUE_KW | SyntaxKind::FALSE_KW => Some(SemanticTokenType::Bool),
        SyntaxKind::BIT_STRING
        | SyntaxKind::BYTE_STRING
        | SyntaxKind::DOLLAR_QUOTED_STRING
        | SyntaxKind::ESC_STRING
        | SyntaxKind::NATIONAL_STRING
        | SyntaxKind::STRING
        | SyntaxKind::UNICODE_ESC_STRING => Some(SemanticTokenType::String),
        SyntaxKind::INT_NUMBER | SyntaxKind::NUMERIC_NUMBER => Some(SemanticTokenType::Number),
        SyntaxKind::COMMENT => Some(SemanticTokenType::Comment),
        SyntaxKind::IDENT => Some(SemanticTokenType::NameRef),
        _ if kind.is_keyword() => Some(SemanticTokenType::Keyword),
        _ if kind.is_punctuation() => Some(SemanticTokenType::Punctuation),
        _ if kind.is_operator() => Some(SemanticTokenType::Operator),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
