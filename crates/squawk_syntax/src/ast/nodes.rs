// based on rust-analyzer's AST but SQL instead of Rust
// https://github.com/rust-lang/rust-analyzer/tree/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/ast
use crate::ast::{AstNode, AstToken};
use crate::syntax_node::SyntaxToken;
use crate::{ast, syntax_node::SyntaxNode};
use crate::{SyntaxKind, TokenText};

use super::node_ext::text_of_first_token;
use super::{support, AstChildren};

impl ArgList {
    pub fn args(&self) -> AstChildren<Expr> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasModuleItem for SourceFile {}
impl AstNode for SourceFile {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SOURCE_FILE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
    pub fn text(&self) -> TokenText<'_> {
        text_of_first_token(self.syntax())
    }
}
impl AstNode for Name {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::NAME)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::NAME => Name { syntax },
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ArgList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARG_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasArgList for CallExpr {}

impl CallExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    // TODO: why do we have arg list and param list?
    #[inline]
    pub fn param_list(&self) -> Option<ast::ParamList> {
        support::child(&self.syntax)
    }
}

impl AstNode for CallExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CALL_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr {
    pub(crate) syntax: SyntaxNode,
}

impl CastExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }
}

impl AstNode for CastExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CAST_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub(crate) syntax: SyntaxNode,
}

impl ArrayExpr {
    #[inline]
    pub fn array_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ARRAY_KW)
    }
}

impl AstNode for ArrayExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARRAY_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    CallExpr(CallExpr),
    CastExpr(CastExpr),
    ArrayExpr(ArrayExpr),
    Literal(Literal),
    NameRef(NameRef),
}

impl AstNode for Expr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::CALL_EXPR
                | SyntaxKind::CAST_EXPR
                | SyntaxKind::LITERAL
                | SyntaxKind::NAME_REF
                | SyntaxKind::ARRAY_EXPR
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
            SyntaxKind::CAST_EXPR => Expr::CastExpr(CastExpr { syntax }),
            SyntaxKind::LITERAL => Expr::Literal(Literal { syntax }),
            SyntaxKind::NAME_REF => Expr::NameRef(NameRef { syntax }),
            SyntaxKind::ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::CallExpr(it) => &it.syntax,
            Expr::CastExpr(it) => &it.syntax,
            Expr::Literal(it) => &it.syntax,
            Expr::NameRef(it) => &it.syntax,
            Expr::ArrayExpr(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl DefaultConstraint {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

impl AstNode for DefaultConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEFAULT_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl GeneratedConstraint {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

impl AstNode for GeneratedConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GENERATED_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferencesConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ReferencesConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REFERENCES_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsingIndex {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for UsingIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::USING_INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimaryKeyConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl PrimaryKeyConstraint {
    #[inline]
    pub fn using_index(&self) -> Option<ast::UsingIndex> {
        support::child(&self.syntax)
    }
}

impl AstNode for PrimaryKeyConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PRIMARY_KEY_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignKeyConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ForeignKeyConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FOREIGN_KEY_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ExcludeConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXCLUDE_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniqueConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl UniqueConstraint {
    #[inline]
    pub fn using_index(&self) -> Option<ast::UsingIndex> {
        support::child(&self.syntax)
    }
}

impl AstNode for UniqueConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UNIQUE_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for CheckConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CHECK_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NullConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NULL_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotNullConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NotNullConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_NULL_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotValid {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NotValid {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_VALID
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {
    DefaultConstraint(DefaultConstraint),
    GeneratedConstraint(GeneratedConstraint),
    ReferencesConstraint(ReferencesConstraint),
    PrimaryKeyConstraint(PrimaryKeyConstraint),
    ForeignKeyConstraint(ForeignKeyConstraint),
    ExcludeConstraint(ExcludeConstraint),
    UniqueConstraint(UniqueConstraint),
    CheckConstraint(CheckConstraint),
    NullConstraint(NullConstraint),
    NotNullConstraint(NotNullConstraint),
}

impl Constraint {
    #[inline]
    pub fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}

impl AstNode for Constraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::DEFAULT_CONSTRAINT
                | SyntaxKind::GENERATED_CONSTRAINT
                | SyntaxKind::REFERENCES_CONSTRAINT
                | SyntaxKind::PRIMARY_KEY_CONSTRAINT
                | SyntaxKind::FOREIGN_KEY_CONSTRAINT
                | SyntaxKind::EXCLUDE_CONSTRAINT
                | SyntaxKind::UNIQUE_CONSTRAINT
                | SyntaxKind::CHECK_CONSTRAINT
                | SyntaxKind::NULL_CONSTRAINT
                | SyntaxKind::NOT_NULL_CONSTRAINT
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::DEFAULT_CONSTRAINT => {
                Constraint::DefaultConstraint(DefaultConstraint { syntax })
            }
            SyntaxKind::GENERATED_CONSTRAINT => {
                Constraint::GeneratedConstraint(GeneratedConstraint { syntax })
            }
            SyntaxKind::REFERENCES_CONSTRAINT => {
                Constraint::ReferencesConstraint(ReferencesConstraint { syntax })
            }
            SyntaxKind::PRIMARY_KEY_CONSTRAINT => {
                Constraint::PrimaryKeyConstraint(PrimaryKeyConstraint { syntax })
            }
            SyntaxKind::FOREIGN_KEY_CONSTRAINT => {
                Constraint::ForeignKeyConstraint(ForeignKeyConstraint { syntax })
            }
            SyntaxKind::EXCLUDE_CONSTRAINT => {
                Constraint::ExcludeConstraint(ExcludeConstraint { syntax })
            }
            SyntaxKind::UNIQUE_CONSTRAINT => {
                Constraint::UniqueConstraint(UniqueConstraint { syntax })
            }
            SyntaxKind::CHECK_CONSTRAINT => Constraint::CheckConstraint(CheckConstraint { syntax }),
            SyntaxKind::NULL_CONSTRAINT => Constraint::NullConstraint(NullConstraint { syntax }),
            SyntaxKind::NOT_NULL_CONSTRAINT => {
                Constraint::NotNullConstraint(NotNullConstraint { syntax })
            }

            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Constraint::DefaultConstraint(t) => &t.syntax,
            Constraint::GeneratedConstraint(t) => &t.syntax,
            Constraint::ReferencesConstraint(t) => &t.syntax,
            Constraint::PrimaryKeyConstraint(t) => &t.syntax,
            Constraint::ForeignKeyConstraint(t) => &t.syntax,
            Constraint::ExcludeConstraint(t) => &t.syntax,
            Constraint::UniqueConstraint(t) => &t.syntax,
            Constraint::CheckConstraint(t) => &t.syntax,
            Constraint::NullConstraint(t) => &t.syntax,
            Constraint::NotNullConstraint(t) => &t.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidateConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl ValidateConstraint {
    #[inline]
    pub fn name_ref(&self) -> Option<ast::NameRef> {
        support::child(&self.syntax)
    }
}

impl AstNode for ValidateConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VALIDATE_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddColumn {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasIfNotExists for AddColumn {}
impl AddColumn {
    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn constraints(&self) -> AstChildren<Constraint> {
        support::children(&self.syntax)
    }
}

impl AstNode for AddColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ADD_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplicaIdentity {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ReplicaIdentity {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REPLICA_IDENTITY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OfType {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for OfType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OF_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotOf {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for NotOf {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_OF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForceRls {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ForceRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FORCE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inherit {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Inherit {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INHERIT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoInherit {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for NoInherit {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NO_INHERIT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableReplicaTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableReplicaTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_REPLICA_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableReplicaRule {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableReplicaRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_REPLICA_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableAlwaysTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableAlwaysTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_ALWAYS_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableAlwaysRule {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableAlwaysRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_ALWAYS_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableRule {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableRls {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EnableRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DisableTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableRls {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DisableRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableRule {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DisableRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableCluster {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DisableCluster {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_CLUSTER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnerTo {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for OwnerTo {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OWNER_TO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DetachPartition {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DetachPartition {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DETACH_PARTITION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropConstraint {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasIfExists for DropConstraint {}

impl DropConstraint {
    #[inline]
    pub fn name_ref(&self) -> Option<ast::NameRef> {
        support::child(&self.syntax)
    }
}
impl AstNode for DropConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropColumn {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasIfExists for DropColumn {}
impl AstNode for DropColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl AddConstraint {
    #[inline]
    pub fn constraint(&self) -> Option<ast::Constraint> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn not_valid(&self) -> Option<ast::NotValid> {
        support::child(self.syntax())
    }
}

impl AstNode for AddConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ADD_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttachPartition {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for AttachPartition {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ATTACH_PARTITION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetSchema {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetSchema {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_SCHEMA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetTablespace {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetTablespace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_TABLESPACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetWithoutCluster {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetWithoutCluster {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_WITHOUT_CLUSTER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetWithoutOids {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetWithoutOids {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_WITHOUT_OIDS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetAccessMethod {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetAccessMethod {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_ACCESS_METHOD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetLogged {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetLogged {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_LOGGED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetUnlogged {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetUnlogged {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_UNLOGGED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStorageParams {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SetStorageParams {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STORAGE_PARAMS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameTable {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RenameTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RenameConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameColumn {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RenameColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for AlterConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropDefault {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for DropDefault {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DEFAULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for DropExpression {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_EXPRESSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropIdentity {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for DropIdentity {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_IDENTITY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropNotNull {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for DropNotNull {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_NOT_NULL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Restart {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Restart {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESTART
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddGenerated {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for AddGenerated {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ADD_GENERATED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetOptions {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ResetOptions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET_OPTIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetType {
    pub(crate) syntax: SyntaxNode,
}

impl SetType {
    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }
}

impl AstNode for SetType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetGeneratedOptions {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetGeneratedOptions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_GENERATED_OPTIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetGenerated {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetGenerated {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_GENERATED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetSequenceOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetSequenceOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_SEQUENCE_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetDefault {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetDefault {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_DEFAULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetExpression {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_EXPRESSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStatistics {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetStatistics {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STATISTICS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetOptions {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetOptions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_OPTIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStorage {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetStorage {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STORAGE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetCompression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetCompression {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_COMPRESSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetNotNull {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetNotNull {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_NOT_NULL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlterColumnOption {
    DropDefault(DropDefault),
    DropExpression(DropExpression),
    DropIdentity(DropIdentity),
    DropNotNull(DropNotNull),
    Restart(Restart),
    AddGenerated(AddGenerated),
    ResetOptions(ResetOptions),
    SetType(SetType),
    SetGeneratedOptions(SetGeneratedOptions),
    SetGenerated(SetGenerated),
    SetSequenceOption(SetSequenceOption),
    SetDefault(SetDefault),
    SetExpression(SetExpression),
    SetStatistics(SetStatistics),
    SetOptions(SetOptions),
    SetStorage(SetStorage),
    SetCompression(SetCompression),
    SetNotNull(SetNotNull),
}

impl AstNode for AlterColumnOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::DROP_DEFAULT
                | SyntaxKind::DROP_EXPRESSION
                | SyntaxKind::DROP_IDENTITY
                | SyntaxKind::DROP_NOT_NULL
                | SyntaxKind::RESTART
                | SyntaxKind::ADD_GENERATED
                | SyntaxKind::RESET_OPTIONS
                | SyntaxKind::SET_TYPE
                | SyntaxKind::SET_GENERATED_OPTIONS
                | SyntaxKind::SET_GENERATED
                | SyntaxKind::SET_DEFAULT
                | SyntaxKind::SET_EXPRESSION
                | SyntaxKind::SET_STATISTICS
                | SyntaxKind::SET_OPTIONS
                | SyntaxKind::SET_STORAGE
                | SyntaxKind::COMPRESSION_KW
                | SyntaxKind::SET_NOT_NULL
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::DROP_DEFAULT => AlterColumnOption::DropDefault(DropDefault { syntax }),
            SyntaxKind::DROP_EXPRESSION => {
                AlterColumnOption::DropExpression(DropExpression { syntax })
            }
            SyntaxKind::DROP_IDENTITY => AlterColumnOption::DropIdentity(DropIdentity { syntax }),
            SyntaxKind::DROP_NOT_NULL => AlterColumnOption::DropNotNull(DropNotNull { syntax }),
            SyntaxKind::RESTART => AlterColumnOption::Restart(Restart { syntax }),
            SyntaxKind::ADD_GENERATED => AlterColumnOption::AddGenerated(AddGenerated { syntax }),
            SyntaxKind::RESET_OPTIONS => AlterColumnOption::ResetOptions(ResetOptions { syntax }),
            SyntaxKind::SET_TYPE => AlterColumnOption::SetType(SetType { syntax }),
            SyntaxKind::SET_GENERATED_OPTIONS => {
                AlterColumnOption::SetGeneratedOptions(SetGeneratedOptions { syntax })
            }
            SyntaxKind::SET_GENERATED => AlterColumnOption::SetGenerated(SetGenerated { syntax }),
            SyntaxKind::SET_SEQUENCE_OPTION => {
                AlterColumnOption::SetSequenceOption(SetSequenceOption { syntax })
            }
            SyntaxKind::SET_DEFAULT => AlterColumnOption::SetDefault(SetDefault { syntax }),
            SyntaxKind::SET_EXPRESSION => {
                AlterColumnOption::SetExpression(SetExpression { syntax })
            }
            SyntaxKind::SET_STATISTICS => {
                AlterColumnOption::SetStatistics(SetStatistics { syntax })
            }
            SyntaxKind::SET_OPTIONS => AlterColumnOption::SetOptions(SetOptions { syntax }),
            SyntaxKind::SET_STORAGE => AlterColumnOption::SetStorage(SetStorage { syntax }),
            SyntaxKind::SET_COMPRESSION => {
                AlterColumnOption::SetCompression(SetCompression { syntax })
            }
            SyntaxKind::SET_NOT_NULL => AlterColumnOption::SetNotNull(SetNotNull { syntax }),

            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AlterColumnOption::DropDefault(it) => &it.syntax,
            AlterColumnOption::DropExpression(it) => &it.syntax,
            AlterColumnOption::DropIdentity(it) => &it.syntax,
            AlterColumnOption::DropNotNull(it) => &it.syntax,
            AlterColumnOption::Restart(it) => &it.syntax,
            AlterColumnOption::AddGenerated(it) => &it.syntax,
            AlterColumnOption::ResetOptions(it) => &it.syntax,
            AlterColumnOption::SetType(it) => &it.syntax,
            AlterColumnOption::SetGeneratedOptions(it) => &it.syntax,
            AlterColumnOption::SetGenerated(it) => &it.syntax,
            AlterColumnOption::SetSequenceOption(it) => &it.syntax,
            AlterColumnOption::SetDefault(it) => &it.syntax,
            AlterColumnOption::SetExpression(it) => &it.syntax,
            AlterColumnOption::SetStatistics(it) => &it.syntax,
            AlterColumnOption::SetOptions(it) => &it.syntax,
            AlterColumnOption::SetStorage(it) => &it.syntax,
            AlterColumnOption::SetCompression(it) => &it.syntax,
            AlterColumnOption::SetNotNull(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDomain {
    pub(crate) syntax: SyntaxNode,
}

impl CreateDomain {
    #[inline]
    pub fn constraints(&self) -> AstChildren<Constraint> {
        support::children(&self.syntax)
    }
}

impl AstNode for CreateDomain {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_DOMAIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterColumn {
    pub(crate) syntax: SyntaxNode,
}
impl AlterColumn {
    #[inline]
    pub fn option(&self) -> Option<AlterColumnOption> {
        support::child(&self.syntax)
    }
}
impl AstNode for AlterColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoForceRls {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NoForceRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NO_FORCE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub enum AlterTableAction {
    ValidateConstraint(ValidateConstraint),
    ReplicaIdentity(ReplicaIdentity),
    OfType(OfType),
    NotOf(NotOf),
    ForceRls(ForceRls),
    NoForceRls(NoForceRls),
    Inherit(Inherit),
    NoInherit(NoInherit),
    EnableTrigger(EnableTrigger),
    EnableReplicaTrigger(EnableReplicaTrigger),
    EnableReplicaRule(EnableReplicaRule),
    EnableAlwaysTrigger(EnableAlwaysTrigger),
    EnableAlwaysRule(EnableAlwaysRule),
    EnableRule(EnableRule),
    EnableRls(EnableRls),
    DisableTrigger(DisableTrigger),
    DisableRls(DisableRls),
    DisableRule(DisableRule),
    DisableCluster(DisableCluster),
    OwnerTo(OwnerTo),
    DetachPartition(DetachPartition),
    DropConstraint(DropConstraint),
    DropColumn(DropColumn),
    AddConstraint(AddConstraint),
    AddColumn(AddColumn),
    AttachPartition(AttachPartition),
    SetSchema(SetSchema),
    SetTablespace(SetTablespace),
    SetWithoutCluster(SetWithoutCluster),
    SetWithoutOids(SetWithoutOids),
    SetAccessMethod(SetAccessMethod),
    SetLogged(SetLogged),
    SetUnlogged(SetUnlogged),
    SetStorageParams(SetStorageParams),
    RenameTable(RenameTable),
    RenameConstraint(RenameConstraint),
    RenameColumn(RenameColumn),
    AlterConstraint(AlterConstraint),
    AlterColumn(AlterColumn),
}

impl AstNode for AlterTableAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::VALIDATE_CONSTRAINT
                | SyntaxKind::FORCE_RLS
                | SyntaxKind::REPLICA_IDENTITY
                | SyntaxKind::OF_TYPE
                | SyntaxKind::NOT_OF
                | SyntaxKind::INHERIT
                | SyntaxKind::NO_INHERIT
                | SyntaxKind::ENABLE_TRIGGER
                | SyntaxKind::ENABLE_REPLICA_TRIGGER
                | SyntaxKind::ENABLE_REPLICA_RULE
                | SyntaxKind::ENABLE_ALWAYS_TRIGGER
                | SyntaxKind::ENABLE_ALWAYS_RULE
                | SyntaxKind::ENABLE_RULE
                | SyntaxKind::ENABLE_RLS
                | SyntaxKind::DISABLE_TRIGGER
                | SyntaxKind::DISABLE_RLS
                | SyntaxKind::DISABLE_RULE
                | SyntaxKind::DISABLE_CLUSTER
                | SyntaxKind::OWNER_TO
                | SyntaxKind::DETACH_PARTITION
                | SyntaxKind::DROP_CONSTRAINT
                | SyntaxKind::DROP_COLUMN
                | SyntaxKind::ADD_CONSTRAINT
                | SyntaxKind::ADD_COLUMN
                | SyntaxKind::ATTACH_PARTITION
                | SyntaxKind::SET_SCHEMA
                | SyntaxKind::SET_TABLESPACE
                | SyntaxKind::SET_WITHOUT_CLUSTER
                | SyntaxKind::SET_WITHOUT_OIDS
                | SyntaxKind::SET_ACCESS_METHOD
                | SyntaxKind::SET_LOGGED
                | SyntaxKind::SET_UNLOGGED
                | SyntaxKind::SET_STORAGE_PARAMS
                | SyntaxKind::RENAME_TABLE
                | SyntaxKind::RENAME_CONSTRAINT
                | SyntaxKind::RENAME_COLUMN
                | SyntaxKind::ALTER_CONSTRAINT
                | SyntaxKind::ALTER_COLUMN
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::VALIDATE_CONSTRAINT => {
                AlterTableAction::ValidateConstraint(ValidateConstraint { syntax })
            }
            SyntaxKind::ADD_COLUMN => AlterTableAction::AddColumn(AddColumn { syntax }),
            SyntaxKind::REPLICA_IDENTITY => {
                AlterTableAction::ReplicaIdentity(ReplicaIdentity { syntax })
            }
            SyntaxKind::OF_TYPE => AlterTableAction::OfType(OfType { syntax }),
            SyntaxKind::NOT_OF => AlterTableAction::NotOf(NotOf { syntax }),
            SyntaxKind::FORCE_RLS => AlterTableAction::ForceRls(ForceRls { syntax }),

            SyntaxKind::INHERIT => AlterTableAction::Inherit(Inherit { syntax }),
            SyntaxKind::NO_INHERIT => AlterTableAction::NoInherit(NoInherit { syntax }),
            SyntaxKind::ENABLE_TRIGGER => AlterTableAction::EnableTrigger(EnableTrigger { syntax }),
            SyntaxKind::ENABLE_REPLICA_TRIGGER => {
                AlterTableAction::EnableReplicaTrigger(EnableReplicaTrigger { syntax })
            }
            SyntaxKind::ENABLE_REPLICA_RULE => {
                AlterTableAction::EnableReplicaRule(EnableReplicaRule { syntax })
            }
            SyntaxKind::ENABLE_ALWAYS_TRIGGER => {
                AlterTableAction::EnableAlwaysTrigger(EnableAlwaysTrigger { syntax })
            }
            SyntaxKind::ENABLE_ALWAYS_RULE => {
                AlterTableAction::EnableAlwaysRule(EnableAlwaysRule { syntax })
            }
            SyntaxKind::ENABLE_RULE => AlterTableAction::EnableRule(EnableRule { syntax }),
            SyntaxKind::ENABLE_RLS => AlterTableAction::EnableRls(EnableRls { syntax }),
            SyntaxKind::DISABLE_TRIGGER => {
                AlterTableAction::DisableTrigger(DisableTrigger { syntax })
            }
            SyntaxKind::DISABLE_RLS => AlterTableAction::DisableRls(DisableRls { syntax }),
            SyntaxKind::DISABLE_RULE => AlterTableAction::DisableRule(DisableRule { syntax }),
            SyntaxKind::DISABLE_CLUSTER => {
                AlterTableAction::DisableCluster(DisableCluster { syntax })
            }
            SyntaxKind::OWNER_TO => AlterTableAction::OwnerTo(OwnerTo { syntax }),
            SyntaxKind::DETACH_PARTITION => {
                AlterTableAction::DetachPartition(DetachPartition { syntax })
            }
            SyntaxKind::DROP_CONSTRAINT => {
                AlterTableAction::DropConstraint(DropConstraint { syntax })
            }
            SyntaxKind::DROP_COLUMN => AlterTableAction::DropColumn(DropColumn { syntax }),
            SyntaxKind::ADD_CONSTRAINT => AlterTableAction::AddConstraint(AddConstraint { syntax }),
            SyntaxKind::ATTACH_PARTITION => {
                AlterTableAction::AttachPartition(AttachPartition { syntax })
            }
            SyntaxKind::SET_SCHEMA => AlterTableAction::SetSchema(SetSchema { syntax }),
            SyntaxKind::SET_TABLESPACE => AlterTableAction::SetTablespace(SetTablespace { syntax }),
            SyntaxKind::SET_WITHOUT_CLUSTER => {
                AlterTableAction::SetWithoutCluster(SetWithoutCluster { syntax })
            }
            SyntaxKind::SET_WITHOUT_OIDS => {
                AlterTableAction::SetWithoutOids(SetWithoutOids { syntax })
            }
            SyntaxKind::SET_ACCESS_METHOD => {
                AlterTableAction::SetAccessMethod(SetAccessMethod { syntax })
            }
            SyntaxKind::SET_LOGGED => AlterTableAction::SetLogged(SetLogged { syntax }),
            SyntaxKind::SET_UNLOGGED => AlterTableAction::SetUnlogged(SetUnlogged { syntax }),
            SyntaxKind::SET_STORAGE_PARAMS => {
                AlterTableAction::SetStorageParams(SetStorageParams { syntax })
            }
            SyntaxKind::RENAME_TABLE => AlterTableAction::RenameTable(RenameTable { syntax }),
            SyntaxKind::RENAME_CONSTRAINT => {
                AlterTableAction::RenameConstraint(RenameConstraint { syntax })
            }
            SyntaxKind::RENAME_COLUMN => AlterTableAction::RenameColumn(RenameColumn { syntax }),
            SyntaxKind::ALTER_CONSTRAINT => {
                AlterTableAction::AlterConstraint(AlterConstraint { syntax })
            }
            SyntaxKind::ALTER_COLUMN => AlterTableAction::AlterColumn(AlterColumn { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AlterTableAction::ValidateConstraint(it) => &it.syntax,
            AlterTableAction::AddColumn(it) => &it.syntax,
            AlterTableAction::ReplicaIdentity(it) => &it.syntax,
            AlterTableAction::OfType(it) => &it.syntax,
            AlterTableAction::NotOf(it) => &it.syntax,
            AlterTableAction::ForceRls(it) => &it.syntax,
            AlterTableAction::Inherit(it) => &it.syntax,
            AlterTableAction::NoInherit(it) => &it.syntax,
            AlterTableAction::EnableTrigger(it) => &it.syntax,
            AlterTableAction::EnableReplicaTrigger(it) => &it.syntax,
            AlterTableAction::EnableReplicaRule(it) => &it.syntax,
            AlterTableAction::EnableAlwaysTrigger(it) => &it.syntax,
            AlterTableAction::EnableAlwaysRule(it) => &it.syntax,
            AlterTableAction::EnableRule(it) => &it.syntax,
            AlterTableAction::EnableRls(it) => &it.syntax,
            AlterTableAction::NoForceRls(it) => &it.syntax,
            AlterTableAction::DisableTrigger(it) => &it.syntax,
            AlterTableAction::DisableRls(it) => &it.syntax,
            AlterTableAction::DisableRule(it) => &it.syntax,
            AlterTableAction::DisableCluster(it) => &it.syntax,
            AlterTableAction::OwnerTo(it) => &it.syntax,
            AlterTableAction::DetachPartition(it) => &it.syntax,
            AlterTableAction::DropConstraint(it) => &it.syntax,
            AlterTableAction::DropColumn(it) => &it.syntax,
            AlterTableAction::AddConstraint(it) => &it.syntax,
            AlterTableAction::AttachPartition(it) => &it.syntax,
            AlterTableAction::SetSchema(it) => &it.syntax,
            AlterTableAction::SetTablespace(it) => &it.syntax,
            AlterTableAction::SetWithoutCluster(it) => &it.syntax,
            AlterTableAction::SetWithoutOids(it) => &it.syntax,
            AlterTableAction::SetAccessMethod(it) => &it.syntax,
            AlterTableAction::SetLogged(it) => &it.syntax,
            AlterTableAction::SetUnlogged(it) => &it.syntax,
            AlterTableAction::SetStorageParams(it) => &it.syntax,
            AlterTableAction::RenameTable(it) => &it.syntax,
            AlterTableAction::RenameConstraint(it) => &it.syntax,
            AlterTableAction::RenameColumn(it) => &it.syntax,
            AlterTableAction::AlterConstraint(it) => &it.syntax,
            AlterTableAction::AlterColumn(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameTo {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for RenameTo {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_TO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlterDomainAction {
    SetDefault(SetDefault),
    DropDefault(DropDefault),
    SetNotNull(SetNotNull),
    DropNotNull(DropNotNull),
    AddConstraint(AddConstraint),
    DropConstraint(DropConstraint),
    RenameConstraint(RenameConstraint),
    ValidateConstraint(ValidateConstraint),
    OwnerTo(OwnerTo),
    RenameTo(RenameTo),
    SetSchema(SetSchema),
}

impl AstNode for AlterDomainAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::SET_DEFAULT
                | SyntaxKind::DROP_DEFAULT
                | SyntaxKind::SET_NOT_NULL
                | SyntaxKind::DROP_NOT_NULL
                | SyntaxKind::ADD_CONSTRAINT
                | SyntaxKind::DROP_CONSTRAINT
                | SyntaxKind::RENAME_CONSTRAINT
                | SyntaxKind::VALIDATE_CONSTRAINT
                | SyntaxKind::OWNER_TO
                | SyntaxKind::RENAME_TO
                | SyntaxKind::SET_SCHEMA
        )
    }

    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::SET_DEFAULT => AlterDomainAction::SetDefault(SetDefault { syntax }),
            SyntaxKind::DROP_DEFAULT => AlterDomainAction::DropDefault(DropDefault { syntax }),
            SyntaxKind::SET_NOT_NULL => AlterDomainAction::SetNotNull(SetNotNull { syntax }),
            SyntaxKind::DROP_NOT_NULL => AlterDomainAction::DropNotNull(DropNotNull { syntax }),
            SyntaxKind::ADD_CONSTRAINT => {
                AlterDomainAction::AddConstraint(AddConstraint { syntax })
            }
            SyntaxKind::DROP_CONSTRAINT => {
                AlterDomainAction::DropConstraint(DropConstraint { syntax })
            }
            SyntaxKind::RENAME_CONSTRAINT => {
                AlterDomainAction::RenameConstraint(RenameConstraint { syntax })
            }
            SyntaxKind::VALIDATE_CONSTRAINT => {
                AlterDomainAction::ValidateConstraint(ValidateConstraint { syntax })
            }
            SyntaxKind::OWNER_TO => AlterDomainAction::OwnerTo(OwnerTo { syntax }),
            SyntaxKind::RENAME_TO => AlterDomainAction::RenameTo(RenameTo { syntax }),
            SyntaxKind::SET_SCHEMA => AlterDomainAction::SetSchema(SetSchema { syntax }),
            _ => return None,
        };
        Some(res)
    }

    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AlterDomainAction::SetDefault(it) => &it.syntax,
            AlterDomainAction::DropDefault(it) => &it.syntax,
            AlterDomainAction::SetNotNull(it) => &it.syntax,
            AlterDomainAction::DropNotNull(it) => &it.syntax,
            AlterDomainAction::AddConstraint(it) => &it.syntax,
            AlterDomainAction::DropConstraint(it) => &it.syntax,
            AlterDomainAction::RenameConstraint(it) => &it.syntax,
            AlterDomainAction::ValidateConstraint(it) => &it.syntax,
            AlterDomainAction::OwnerTo(it) => &it.syntax,
            AlterDomainAction::RenameTo(it) => &it.syntax,
            AlterDomainAction::SetSchema(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterDomain {
    pub(crate) syntax: SyntaxNode,
}

impl AlterDomain {
    #[inline]
    pub fn actions(&self) -> AstChildren<AlterDomainAction> {
        support::children(&self.syntax)
    }
}

impl AstNode for AlterDomain {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DOMAIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTable {
    pub(crate) syntax: SyntaxNode,
}

impl AlterTable {
    #[inline]
    pub fn path(&self) -> Option<ast::Path> {
        support::child(&self.syntax)
    }
}

impl AlterTable {
    #[inline]
    pub fn actions(&self) -> AstChildren<AlterTableAction> {
        support::children(&self.syntax)
    }
}

impl AstNode for AlterTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropDatabase {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for DropDatabase {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DATABASE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    pub(crate) syntax: SyntaxNode,
}

impl Column {
    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }
}

impl AstNode for Column {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LikeClause {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LikeClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LIKE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableArg {
    Column(Column),
    LikeClause(LikeClause),
    Constraint(Constraint),
}

impl AstNode for TableArg {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::COLUMN | SyntaxKind::LIKE_CLAUSE)
            | ast::Constraint::can_cast(kind)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::COLUMN => TableArg::Column(Column { syntax }),
            SyntaxKind::LIKE_CLAUSE => TableArg::LikeClause(LikeClause { syntax }),
            _ => {
                if let Some(result) = ast::Constraint::cast(syntax) {
                    return Some(TableArg::Constraint(result));
                }
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableArg::Column(it) => &it.syntax,
            TableArg::LikeClause(it) => &it.syntax,
            TableArg::Constraint(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableArgs {
    pub(crate) syntax: SyntaxNode,
}

impl TableArgs {
    #[inline]
    pub fn args(&self) -> AstChildren<ast::TableArg> {
        support::children(&self.syntax)
    }
}

impl AstNode for TableArgs {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TABLE_ARGS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTable {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasIfNotExists for CreateTable {}

impl CreateTable {
    #[inline]
    pub fn path(&self) -> Option<ast::Path> {
        support::child(&self.syntax)
    }
}

impl CreateTable {
    #[inline]
    pub fn table_args(&self) -> Option<ast::TableArgs> {
        support::child(&self.syntax)
    }
}

impl AstNode for CreateTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Begin {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Begin {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BEGIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Commit {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Commit {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COMMIT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExists {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for IfExists {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IF_EXISTS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfNotExists {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for IfNotExists {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IF_NOT_EXISTS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateIndex {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasIfNotExists for CreateIndex {}

impl CreateIndex {
    #[inline]
    pub fn name(&self) -> Option<ast::Name> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn concurrently_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONCURRENTLY_KW)
    }
}

impl AstNode for CreateIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_INDEX_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropType {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasIfExists for DropType {}

impl AstNode for DropType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TYPE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropIndex {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasIfExists for DropIndex {}

impl DropIndex {
    #[inline]
    pub fn concurrently_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONCURRENTLY_KW)
    }
}

impl AstNode for DropIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_INDEX_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTable {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasIfExists for DropTable {}

impl AstNode for DropTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterAggregate {
    pub(crate) syntax: SyntaxNode,
}

impl AlterAggregate {
    #[inline]
    pub fn path(&self) -> Option<ast::Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_list(&self) -> Option<ast::ParamList> {
        support::child(&self.syntax)
    }
}

impl AstNode for AlterAggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_AGGREGATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PrefixExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREFIX_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomOp {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for CustomOp {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CUSTOM_OP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropAggregate {
    pub(crate) syntax: SyntaxNode,
}

impl DropAggregate {
    #[inline]
    pub fn aggregates(&self) -> AstChildren<ast::CallExpr> {
        support::children(&self.syntax)
    }
}

impl AstNode for DropAggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_AGGREGATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateAggregate {
    pub(crate) syntax: SyntaxNode,
}

impl CreateAggregate {
    #[inline]
    pub fn path(&self) -> Option<ast::Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_list(&self) -> Option<ast::ParamList> {
        support::child(&self.syntax)
    }
}

impl AstNode for CreateAggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_AGGREGATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rollback {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Rollback {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROLLBACK_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    AlterTable(AlterTable),
    AlterDomain(AlterDomain),
    AlterAggregate(AlterAggregate),
    CreateAggregate(CreateAggregate),
    Begin(Begin),
    Commit(Commit),
    Rollback(Rollback),
    CreateFunc(CreateFunc),
    CreateIndex(CreateIndex),
    CreateTable(CreateTable),
    CreateDomain(CreateDomain),
    DropDatabase(DropDatabase),
    DropAggregate(DropAggregate),
    DropTable(DropTable),
    DropIndex(DropIndex),
    DropType(DropType),
    Select(Select),
}
// impl ast::HasAttrs for Item {}
// impl ast::HasDocComments for Item {}

impl AstNode for Item {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::SELECT
                | SyntaxKind::CREATE_FUNCTION_STMT
                | SyntaxKind::ALTER_TABLE
                | SyntaxKind::DROP_DATABASE_STMT
                | SyntaxKind::CREATE_TABLE
                | SyntaxKind::BEGIN_STMT
                | SyntaxKind::COMMIT_STMT
                | SyntaxKind::CREATE_INDEX_STMT
                | SyntaxKind::DROP_TABLE
                | SyntaxKind::DROP_INDEX_STMT
                | SyntaxKind::DROP_TYPE_STMT
                | SyntaxKind::DROP_AGGREGATE_STMT
                | SyntaxKind::CREATE_DOMAIN_STMT
                | SyntaxKind::ALTER_DOMAIN_STMT
                | SyntaxKind::ALTER_AGGREGATE_STMT
                | SyntaxKind::CREATE_AGGREGATE_STMT
                | SyntaxKind::ROLLBACK_KW
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::SELECT => Item::Select(Select { syntax }),
            SyntaxKind::CREATE_FUNCTION_STMT => Item::CreateFunc(CreateFunc { syntax }),
            SyntaxKind::ALTER_TABLE => Item::AlterTable(AlterTable { syntax }),
            SyntaxKind::DROP_DATABASE_STMT => Item::DropDatabase(DropDatabase { syntax }),
            SyntaxKind::CREATE_TABLE => Item::CreateTable(CreateTable { syntax }),
            SyntaxKind::BEGIN_STMT => Item::Begin(Begin { syntax }),
            SyntaxKind::COMMIT_STMT => Item::Commit(Commit { syntax }),
            SyntaxKind::CREATE_INDEX_STMT => Item::CreateIndex(CreateIndex { syntax }),
            SyntaxKind::DROP_TABLE => Item::DropTable(DropTable { syntax }),
            SyntaxKind::DROP_INDEX_STMT => Item::DropIndex(DropIndex { syntax }),
            SyntaxKind::DROP_TYPE_STMT => Item::DropType(DropType { syntax }),
            SyntaxKind::CREATE_DOMAIN_STMT => Item::CreateDomain(CreateDomain { syntax }),
            SyntaxKind::ALTER_DOMAIN_STMT => Item::AlterDomain(AlterDomain { syntax }),
            SyntaxKind::ALTER_AGGREGATE_STMT => Item::AlterAggregate(AlterAggregate { syntax }),
            SyntaxKind::CREATE_AGGREGATE_STMT => Item::CreateAggregate(CreateAggregate { syntax }),
            SyntaxKind::DROP_AGGREGATE_STMT => Item::DropAggregate(DropAggregate { syntax }),
            SyntaxKind::ROLLBACK_STMT => Item::Rollback(Rollback { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Select(it) => &it.syntax,
            Item::CreateFunc(it) => &it.syntax,
            Item::AlterTable(it) => &it.syntax,
            Item::DropDatabase(it) => &it.syntax,
            Item::CreateTable(it) => &it.syntax,
            Item::Begin(it) => &it.syntax,
            Item::Commit(it) => &it.syntax,
            Item::CreateIndex(it) => &it.syntax,
            Item::DropTable(it) => &it.syntax,
            Item::DropIndex(it) => &it.syntax,
            Item::DropType(it) => &it.syntax,
            Item::CreateDomain(it) => &it.syntax,
            Item::AlterDomain(it) => &it.syntax,
            Item::AlterAggregate(it) => &it.syntax,
            Item::CreateAggregate(it) => &it.syntax,
            Item::DropAggregate(it) => &it.syntax,
            Item::Rollback(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Select {
    pub(crate) syntax: SyntaxNode,
}
// impl ast::HasModuleItem for SourceFile {}
impl AstNode for Select {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SELECT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl NameRef {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn text(&self) -> TokenText<'_> {
        text_of_first_token(self.syntax())
    }
}
impl AstNode for NameRef {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME_REF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}
impl PathSegment {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
}
impl AstNode for PathSegment {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PATH_SEGMENT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl Path {
    #[inline]
    pub fn qualifier(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn segment(&self) -> Option<PathSegment> {
        support::child(&self.syntax)
    }
}

impl AstNode for Path {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PATH
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayType {
    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }
}
impl AstNode for ArrayType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARRAY_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentType {
    pub(crate) syntax: SyntaxNode,
}
impl PercentType {
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
}
impl AstNode for PercentType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PERCENT_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasArgList for PathType {}
impl PathType {
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
}
impl AstNode for PathType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PATH_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharType {
    pub(crate) syntax: SyntaxNode,
}

impl ast::HasArgList for CharType {}

impl CharType {
    #[inline]
    pub fn text(&self) -> TokenText<'_> {
        text_of_first_token(self.syntax())
    }
}

impl AstNode for CharType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CHAR_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BitType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BIT_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoubleType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for DoubleType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DOUBLE_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithTimezone {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for WithTimezone {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITH_TIMEZONE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithoutTimezone {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for WithoutTimezone {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITHOUT_TIMEZONE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeType {
    pub(crate) syntax: SyntaxNode,
}

impl TimeType {
    #[inline]
    pub fn name_ref(&self) -> Option<ast::NameRef> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn without_timezone(&self) -> Option<ast::WithoutTimezone> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn with_timezone(&self) -> Option<ast::WithTimezone> {
        support::child(&self.syntax)
    }
}

impl AstNode for TimeType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TIME_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntervalType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for IntervalType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INTERVAL_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    ArrayType(ArrayType),
    PercentType(PercentType),
    PathType(PathType),
    CharType(CharType),
    BitType(BitType),
    DoubleType(DoubleType),
    TimeType(TimeType),
    IntervalType(IntervalType),
}

impl AstNode for Type {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ARRAY_TYPE
                | SyntaxKind::PERCENT_TYPE
                | SyntaxKind::PATH_TYPE
                | SyntaxKind::CHAR_TYPE
                | SyntaxKind::BIT_TYPE
                | SyntaxKind::DOUBLE_TYPE
                | SyntaxKind::TIME_TYPE
                | SyntaxKind::INTERVAL_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ARRAY_TYPE => Type::ArrayType(ArrayType { syntax }),
            SyntaxKind::PATH_TYPE => Type::PathType(PathType { syntax }),
            SyntaxKind::PERCENT_TYPE => Type::PercentType(PercentType { syntax }),
            SyntaxKind::CHAR_TYPE => Type::CharType(CharType { syntax }),
            SyntaxKind::BIT_TYPE => Type::BitType(BitType { syntax }),
            SyntaxKind::DOUBLE_TYPE => Type::DoubleType(DoubleType { syntax }),
            SyntaxKind::TIME_TYPE => Type::TimeType(TimeType { syntax }),
            SyntaxKind::INTERVAL_TYPE => Type::IntervalType(IntervalType { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Type::ArrayType(it) => &it.syntax,
            Type::PathType(it) => &it.syntax,
            Type::PercentType(it) => &it.syntax,
            Type::CharType(it) => &it.syntax,
            Type::BitType(it) => &it.syntax,
            Type::DoubleType(it) => &it.syntax,
            Type::TimeType(it) => &it.syntax,
            Type::IntervalType(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RetType {
    pub(crate) syntax: SyntaxNode,
}
impl RetType {
    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }
}
impl AstNode for RetType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RET_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BeginFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BeginFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BEGIN_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ReturnFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RETURN_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String {
    pub(crate) syntax: SyntaxToken,
}

impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRING
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Null {
    pub(crate) syntax: SyntaxToken,
}

impl AstToken for Null {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NULL_KW
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    String(ast::String),
    Null(ast::Null),
    // ByteString(ast::ByteString),
    // CString(ast::CString),
    // IntNumber(ast::IntNumber),
    // FloatNumber(ast::FloatNumber),
    // Char(ast::Char),
    // Byte(ast::Byte),
    // Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}

impl Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .find(|e| !e.kind().is_trivia())
            .and_then(|e| e.into_token())
            .unwrap()
    }

    #[inline]
    pub fn kind(&self) -> LiteralKind {
        let token = self.token();
        if let Some(t) = ast::String::cast(token.clone()) {
            return LiteralKind::String(t);
        }
        if let Some(t) = ast::Null::cast(token.clone()) {
            return LiteralKind::Null(t);
        }
        unreachable!()
    }
}

impl AstNode for Literal {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LITERAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AsFuncOption {
    #[inline]
    pub fn strings(&self) -> AstChildren<Literal> {
        support::children(&self.syntax)
    }
}

impl AstNode for AsFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AS_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SupportFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SUPPORT_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SetFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RowsFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for RowsFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROWS_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CostFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for CostFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COST_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParallelFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ParallelFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARALLEL_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecurityFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SecurityFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SECURITY_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrictFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for StrictFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SECURITY_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ResetFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeakproofFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LeakproofFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LEAKPROOF_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VolatilityFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for VolatilityFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VOLATILITY_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for WindowFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WINDOW_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransformFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for TransformFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TRANSFORM_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageFuncOption {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LanguageFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LANGUAGE_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FuncOption {
    BeginFuncOption(BeginFuncOption),
    ReturnFuncOption(ReturnFuncOption),
    AsFuncOption(AsFuncOption),
    SetFuncOption(SetFuncOption),
    SupportFuncOption(SupportFuncOption),
    RowsFuncOption(RowsFuncOption),
    CostFuncOption(CostFuncOption),
    ParallelFuncOption(ParallelFuncOption),
    SecurityFuncOption(SecurityFuncOption),
    StrictFuncOption(StrictFuncOption),
    LeakproofFuncOption(LeakproofFuncOption),
    ResetFuncOption(ResetFuncOption),
    VolatilityFuncOption(VolatilityFuncOption),
    WindowFuncOption(WindowFuncOption),
    TransformFuncOption(TransformFuncOption),
    LanguageFuncOption(LanguageFuncOption),
}

impl AstNode for FuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::BEGIN_FUNC_OPTION
                | SyntaxKind::RETURN_FUNC_OPTION
                | SyntaxKind::AS_FUNC_OPTION
                | SyntaxKind::SET_FUNC_OPTION
                | SyntaxKind::SUPPORT_FUNC_OPTION
                | SyntaxKind::ROWS_FUNC_OPTION
                | SyntaxKind::COST_FUNC_OPTION
                | SyntaxKind::PARALLEL_FUNC_OPTION
                | SyntaxKind::SECURITY_FUNC_OPTION
                | SyntaxKind::STRICT_FUNC_OPTION
                | SyntaxKind::LEAKPROOF_FUNC_OPTION
                | SyntaxKind::RESET_FUNC_OPTION
                | SyntaxKind::VOLATILITY_FUNC_OPTION
                | SyntaxKind::WINDOW_FUNC_OPTION
                | SyntaxKind::TRANSFORM_FUNC_OPTION
                | SyntaxKind::LANGUAGE_FUNC_OPTION
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::BEGIN_FUNC_OPTION => {
                FuncOption::BeginFuncOption(BeginFuncOption { syntax })
            }
            SyntaxKind::RETURN_FUNC_OPTION => {
                FuncOption::ReturnFuncOption(ReturnFuncOption { syntax })
            }
            SyntaxKind::AS_FUNC_OPTION => FuncOption::AsFuncOption(AsFuncOption { syntax }),
            SyntaxKind::SET_FUNC_OPTION => FuncOption::SetFuncOption(SetFuncOption { syntax }),
            SyntaxKind::SUPPORT_FUNC_OPTION => {
                FuncOption::SupportFuncOption(SupportFuncOption { syntax })
            }
            SyntaxKind::ROWS_FUNC_OPTION => FuncOption::RowsFuncOption(RowsFuncOption { syntax }),
            SyntaxKind::COST_FUNC_OPTION => FuncOption::CostFuncOption(CostFuncOption { syntax }),
            SyntaxKind::PARALLEL_FUNC_OPTION => {
                FuncOption::ParallelFuncOption(ParallelFuncOption { syntax })
            }
            SyntaxKind::SECURITY_FUNC_OPTION => {
                FuncOption::SecurityFuncOption(SecurityFuncOption { syntax })
            }
            SyntaxKind::STRICT_FUNC_OPTION => {
                FuncOption::StrictFuncOption(StrictFuncOption { syntax })
            }
            SyntaxKind::LEAKPROOF_FUNC_OPTION => {
                FuncOption::LeakproofFuncOption(LeakproofFuncOption { syntax })
            }
            SyntaxKind::RESET_FUNC_OPTION => {
                FuncOption::ResetFuncOption(ResetFuncOption { syntax })
            }
            SyntaxKind::VOLATILITY_FUNC_OPTION => {
                FuncOption::VolatilityFuncOption(VolatilityFuncOption { syntax })
            }
            SyntaxKind::WINDOW_FUNC_OPTION => {
                FuncOption::WindowFuncOption(WindowFuncOption { syntax })
            }
            SyntaxKind::TRANSFORM_FUNC_OPTION => {
                FuncOption::TransformFuncOption(TransformFuncOption { syntax })
            }
            SyntaxKind::LANGUAGE_FUNC_OPTION => {
                FuncOption::LanguageFuncOption(LanguageFuncOption { syntax })
            }

            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FuncOption::BeginFuncOption(it) => &it.syntax,
            FuncOption::ReturnFuncOption(it) => &it.syntax,
            FuncOption::AsFuncOption(it) => &it.syntax,
            FuncOption::SetFuncOption(it) => &it.syntax,
            FuncOption::ResetFuncOption(it) => &it.syntax,
            FuncOption::SupportFuncOption(it) => &it.syntax,
            FuncOption::RowsFuncOption(it) => &it.syntax,
            FuncOption::CostFuncOption(it) => &it.syntax,
            FuncOption::ParallelFuncOption(it) => &it.syntax,
            FuncOption::SecurityFuncOption(it) => &it.syntax,
            FuncOption::StrictFuncOption(it) => &it.syntax,
            FuncOption::LeakproofFuncOption(it) => &it.syntax,
            FuncOption::VolatilityFuncOption(it) => &it.syntax,
            FuncOption::WindowFuncOption(it) => &it.syntax,
            FuncOption::TransformFuncOption(it) => &it.syntax,
            FuncOption::LanguageFuncOption(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncOptionList {
    pub(crate) syntax: SyntaxNode,
}

impl FuncOptionList {
    #[inline]
    pub fn options(&self) -> AstChildren<FuncOption> {
        support::children(&self.syntax)
    }
}

impl AstNode for FuncOptionList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FUNC_OPTION_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamIn {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ParamIn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_IN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamOut {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ParamOut {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_OUT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamInOut {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ParamInOut {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_INOUT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamVariadic {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ParamVariadic {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_VARIADIC
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParamMode {
    ParamIn(ParamIn),
    ParamOut(ParamOut),
    ParamInOut(ParamInOut),
    ParamVariadic(ParamVariadic),
}

impl AstNode for ParamMode {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::PARAM_IN
                | SyntaxKind::PARAM_OUT
                | SyntaxKind::PARAM_INOUT
                | SyntaxKind::VARIADIC_KW
        )
    }

    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::PARAM_IN => ParamMode::ParamIn(ParamIn { syntax }),
            SyntaxKind::PARAM_OUT => ParamMode::ParamOut(ParamOut { syntax }),
            SyntaxKind::PARAM_INOUT => ParamMode::ParamInOut(ParamInOut { syntax }),
            SyntaxKind::PARAM_VARIADIC => ParamMode::ParamVariadic(ParamVariadic { syntax }),
            _ => return None,
        };
        Some(res)
    }

    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ParamMode::ParamIn(it) => &it.syntax,
            ParamMode::ParamOut(it) => &it.syntax,
            ParamMode::ParamInOut(it) => &it.syntax,
            ParamMode::ParamVariadic(it) => &it.syntax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}
impl Param {
    #[inline]
    pub fn name(&self) -> Option<ast::Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<ast::Type> {
        support::child(&self.syntax)
    }

    #[inline]
    pub fn mode(&self) -> Option<ast::ParamMode> {
        support::child(&self.syntax)
    }
}

impl AstNode for Param {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}

impl ParamList {
    #[inline]
    pub fn params(&self) -> AstChildren<Param> {
        support::children(&self.syntax)
    }
}

impl AstNode for ParamList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunc {
    pub(crate) syntax: SyntaxNode,
}
impl CreateFunc {
    #[inline]
    pub fn path(&self) -> Option<ast::Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_list(&self) -> Option<ast::ParamList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ret_type(&self) -> Option<ast::RetType> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn option_list(&self) -> Option<ast::FuncOptionList> {
        support::child(&self.syntax)
    }
}
impl AstNode for CreateFunc {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FUNCTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
