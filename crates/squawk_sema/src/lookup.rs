use crate::interner::Name;
use crate::ir::{ReferenceKind, TableKind, TypeId, TypeKind};

#[derive(Clone, Copy)]
pub(crate) enum Lookup {
    Relation,
    Type,
    Function,
    Operator,
    Collation,
    Conversion,
    OperatorClass,
    Statistics,
    TextSearch,
}

impl Lookup {
    pub(crate) fn searches_temp(self) -> bool {
        match self {
            Lookup::Relation | Lookup::Type | Lookup::Statistics => true,
            Lookup::Function
            | Lookup::Operator
            | Lookup::Collation
            | Lookup::Conversion
            | Lookup::OperatorClass
            | Lookup::TextSearch => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableLookup {
    Table,
    ForeignTable,
    TableOrForeignTable,
    TableOrView,
    TableOrMaterializedView,
    Any,
    FromItem,
    Like,
    View,
    MaterializedView,
    Sequence,
    PropertyGraph,
    CopyTo,
    CopyFrom,
    Truncate,
}

impl TableLookup {
    pub(crate) fn accepts(self, kind: TableKind) -> bool {
        match self {
            TableLookup::Table => kind == TableKind::Table,
            TableLookup::ForeignTable => kind == TableKind::ForeignTable,
            TableLookup::TableOrForeignTable => {
                matches!(kind, TableKind::Table | TableKind::ForeignTable)
            }
            TableLookup::TableOrView => matches!(kind, TableKind::Table | TableKind::View),
            TableLookup::TableOrMaterializedView => {
                matches!(kind, TableKind::Table | TableKind::MaterializedView)
            }
            TableLookup::Any | TableLookup::FromItem => true,
            TableLookup::Like => matches!(
                kind,
                TableKind::Table
                    | TableKind::View
                    | TableKind::MaterializedView
                    | TableKind::ForeignTable
            ),
            TableLookup::View => kind == TableKind::View,
            TableLookup::MaterializedView => kind == TableKind::MaterializedView,
            TableLookup::Sequence => kind == TableKind::Sequence,
            TableLookup::PropertyGraph => kind == TableKind::PropertyGraph,
            TableLookup::CopyTo => matches!(kind, TableKind::Table | TableKind::MaterializedView),
            TableLookup::CopyFrom | TableLookup::Truncate => {
                matches!(kind, TableKind::Table | TableKind::ForeignTable)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum RefSource {
    Column,
    /// table name, alias, CTE, or subquery alias
    /// ```sql
    /// -- x is the qualifier
    /// select x.a, x.*
    /// ```
    Qualifier,
    Table {
        schema: Option<Name>,
        lookup: TableLookup,
    },
    Type {
        schema: Option<Name>,
        candidates: TypeCandidates,
    },
    Window,
    EnumLabel {
        ty: TypeId,
    },
}

impl RefSource {
    pub(crate) fn kind(self) -> ReferenceKind {
        match self {
            RefSource::Column => ReferenceKind::Column,
            RefSource::Qualifier => ReferenceKind::Relation,
            RefSource::Table { .. } => ReferenceKind::Table,
            RefSource::Type { .. } => ReferenceKind::Type,
            RefSource::Window => ReferenceKind::Window,
            RefSource::EnumLabel { .. } => ReferenceKind::EnumLabel,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum TypeCandidates {
    TypeName,
    Types(Option<TypeKind>),
}

#[derive(Clone, Copy)]
pub(crate) enum RelationLookup {
    Any,
    OnlyTable,
}
