use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L428-L444>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum TransactionStmtKind {
    #[serde(rename = "TRANS_STMT_BEGIN")]
    Begin,
    #[serde(rename = "TRANS_STMT_START")]
    Start,
    #[serde(rename = "TRANS_STMT_COMMIT")]
    Commit,
    #[serde(rename = "TRANS_STMT_ROLLBACK")]
    Rollback,
    #[serde(rename = "TRANS_STMT_SAVEPOINT")]
    Savepoint,
    #[serde(rename = "TRANS_STMT_RELEASE")]
    Release,
    #[serde(rename = "TRANS_STMT_ROLLBACK_TO")]
    RollbackTo,
    #[serde(rename = "TRANS_STMT_PREPARE")]
    Prepare,
    #[serde(rename = "TRANS_STMT_COMMIT_PREPARED")]
    CommitPrepared,
    #[serde(rename = "TRANS_STMT_ROLLBACK_PREPARED")]
    RollbackPrepared,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionStmt {
    pub kind: TransactionStmtKind,
}
#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: i32,
    pub len: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawStmt {
    pub stmt: Stmt,
    #[serde(default)]
    pub stmt_location: i32,
    /// None when the statement doesn't have a closing semicolon
    pub stmt_len: Option<i32>,
}

impl std::convert::From<&RawStmt> for Span {
    fn from(stmt: &RawStmt) -> Self {
        Self {
            start: stmt.stmt_location,
            len: stmt.stmt_len,
        }
    }
}

impl std::convert::From<&ColumnDef> for Span {
    fn from(stmt: &ColumnDef) -> Self {
        Self {
            start: stmt.location,
            // Use current line
            len: None,
        }
    }
}

impl RawStmt {
    #[must_use]
    pub fn span(&self) -> Span {
        Span {
            start: self.stmt_location,
            len: self.stmt_len,
        }
    }
}

/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L179-L189>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SetOperation {
    #[serde(rename = "SETOP_NONE")]
    None,
    #[serde(rename = "SETOP_UNION")]
    Union,
    #[serde(rename = "SETOP_INTERSECT")]
    Intersect,
    #[serde(rename = "SETOP_EXCEPT")]
    Except,
}

impl Default for SetOperation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SelectChild {
    SelectStmt(SelectStmt),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectStmt {
    //
    // These fields are used only in "leaf" SelectStmts.
    //
    /// NULL, list of DISTINCT ON exprs, or lcons(NIL,NIL) for all (SELECT
    /// DISTINCT)
    #[serde(rename = "distinctClause")]
    pub distinct_clause: Option<Value>,
    // target for SELECT INTO
    #[serde(rename = "intoClause")]
    pub into_clause: Option<Value>,
    /// the target list (of `ResTarget`)
    #[serde(rename = "targetList")]
    pub target_list: Option<Vec<Value>>,
    /// the FROM clause
    #[serde(rename = "fromClause")]
    pub from_clause: Option<Value>,
    /// WHERE qualification
    #[serde(rename = "whereClause")]
    pub where_clause: Option<Value>,
    /// GROUP BY clauses
    #[serde(rename = "groupClause")]
    pub group_clause: Option<Value>,
    /// HAVING conditional-expression
    #[serde(rename = "havingClause")]
    pub having_clause: Option<Value>,
    /// WINDOW `window_name` AS (...), ...
    #[serde(rename = "windowClause")]
    pub window_clause: Option<Value>,

    //
    // In a "leaf" node representing a VALUES list, the above fields are all
    // null, and instead this field is set.  Note that the elements of the
    // sublists are just expressions, without ResTarget decoration. Also note
    // that a list element can be DEFAULT (represented as a SetToDefault
    // node), regardless of the context of the VALUES list. It's up to parse
    // analysis to reject that where not valid.
    //
    /// untransformed list of expression lists
    #[serde(rename = "valuesLists")]
    values_lists: Option<Value>,

    //
    // These fields are used in both "leaf" SelectStmts and upper-level
    // SelectStmts.
    //
    /// sort clause (a list of `SortBy`'s)
    #[serde(rename = "sortClause")]
    sort_clause: Option<Value>,
    /// # of result tuples to skip
    #[serde(rename = "limitOffset")]
    limit_offset: Option<Value>,
    /// # of result tuples to return
    #[serde(rename = "limitCount")]
    limit_count: Option<Value>,
    /// FOR UPDATE (list of `LockingClause`'s)
    #[serde(rename = "lockingClause")]
    locking_clause: Option<Value>,
    /// WITH clause
    #[serde(rename = "withClause")]
    with_clause: Option<Value>,

    //
    // These fields are used only in upper-level SelectStmts.
    //
    /// type of set op
    #[serde(default)]
    pub op: SetOperation,
    /// ALL specified?
    #[serde(default)]
    pub all: bool,
    /// left child
    pub larg: Option<Box<SelectStmt>>,
    /// right child
    pub rarg: Option<Box<SelectStmt>>,
}

/// Sort ordering options for ORDER BY and CREATE INDEX
///
/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L27-L37>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SortByDir {
    #[serde(rename = "SORTBY_DEFAULT")]
    Default,
    #[serde(rename = "SORTBY_ASC")]
    Asc,
    #[serde(rename = "SORTBY_DESC")]
    Desc,
    #[serde(rename = "SORTBY_USING")]
    Using,
}

impl Default for SortByDir {
    fn default() -> Self {
        Self::Default
    }
}

/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L39-L48>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SortByNulls {
    #[serde(rename = "SORTBY_NULLS_DEFAULT")]
    Default,
    #[serde(rename = "SORTBY_NULLS_FIRST")]
    First,
    #[serde(rename = "SORTBY_NULLS_LAST")]
    Last,
}

impl Default for SortByNulls {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexElem {
    /// name of attribute to index, or NULL
    name: Option<String>,
    /// expression to index, or NULL
    expr: Option<Value>,
    /// name for index column; NULL = default
    indexcolname: Option<String>,
    /// name of collation; NIL = default
    collation: Option<Value>,
    /// name of desired opclass; NIL = default
    #[serde(default)]
    opclass: Vec<Value>,
    /// ASC/DESC/default
    #[serde(default)]
    ordering: SortByDir,
    /// FIRST/LAST/default
    #[serde(default)]
    nulls_ordering: SortByNulls,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum IndexParams {
    IndexElem(IndexElem),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RangeVar {
    /// the catalog (database) name, or NULL
    pub catalogname: Option<String>,
    /// the schema name, or NULL
    pub schemaname: Option<String>,
    /// the relation/sequence name
    pub relname: String,
    /// expand rel by inheritance? recursively act on children?
    #[serde(default)]
    pub inh: bool,
    /// see RELPERSISTENCE_* in `pg_class.h`
    pub relpersistence: String,
    /// table alias & optional column aliases
    pub alias: Option<Value>,
    /// token location, or -1 if unknown
    pub location: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexStmt {
    /// name of access method (eg. btree)
    #[serde(rename = "accessMethod")]
    pub access_method: String,
    /// name of new index, or NULL for default
    #[serde(default)]
    pub idxname: Option<String>,
    #[serde(rename = "indexParams")]
    pub index_params: Vec<IndexParams>,
    /// relation to build index on
    pub relation: RangeVar,
    #[serde(default)]
    pub concurrent: bool,
    /// is index unique
    #[serde(default)]
    pub unique: bool,
    /// is index a primary key?
    #[serde(default)]
    pub primary: bool,
    /// is it for a pkey/unique constraint?
    #[serde(default)]
    pub isconstraint: bool,
    /// is the constraint DEFERRABLE?
    #[serde(default)]
    pub deferrable: bool,
    /// is the constraint INITIALLY DEFERRED?
    #[serde(default)]
    pub initdeferred: bool,
    /// true when transformIndexStmt is finished
    #[serde(default)]
    pub transformed: bool,
    /// should this be a concurrent index build?
    /// just do nothing if index already exists?
    #[serde(default)]
    pub if_not_exists: bool,
    /// tablespace, or NULL for default
    #[serde(rename = "tableSpace")]
    pub table_space: Option<String>,
}

/// When a command can act on several kinds of objects with only one
/// parse structure required, use these constants to designate the
/// object type.  Note that commands typically don't support all the types.
///
/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L191-L247>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ObjectType {
    #[serde(rename = "OBJECT_ACCESS_METHOD")]
    AccessMethod,
    #[serde(rename = "OBJECT_AGGREGATE")]
    Aggregate,
    #[serde(rename = "OBJECT_AMOP")]
    Amop,
    #[serde(rename = "OBJECT_AMPROC")]
    Amproc,
    #[serde(rename = "OBJECT_ATTRIBUTE")]
    Attribute,
    #[serde(rename = "OBJECT_CAST")]
    Cast,
    #[serde(rename = "OBJECT_COLUMN")]
    Column,
    #[serde(rename = "OBJECT_COLLATION")]
    Collation,
    #[serde(rename = "OBJECT_CONVERSION")]
    Conversion,
    #[serde(rename = "OBJECT_DATABASE")]
    Database,
    #[serde(rename = "OBJECT_DEFAULT")]
    Default,
    #[serde(rename = "OBJECT_DEFACL")]
    Defacl,
    #[serde(rename = "OBJECT_DOMAIN")]
    Domain,
    #[serde(rename = "OBJECT_DOMCONSTRAINT")]
    Domconstraint,
    #[serde(rename = "OBJECT_EVENT_TRIGGER")]
    EventTrigger,
    #[serde(rename = "OBJECT_EXTENSION")]
    Extension,
    #[serde(rename = "OBJECT_FDW")]
    Fdw,
    #[serde(rename = "OBJECT_FOREIGN_SERVER")]
    ForeignServer,
    #[serde(rename = "OBJECT_FOREIGN_TABLE")]
    ForeignTable,
    #[serde(rename = "OBJECT_FUNCTION")]
    Function,
    #[serde(rename = "OBJECT_INDEX")]
    Index,
    #[serde(rename = "OBJECT_LANGUAGE")]
    Language,
    #[serde(rename = "OBJECT_LARGEOBJECT")]
    Largeobject,
    #[serde(rename = "OBJECT_MATVIEW")]
    Matview,
    #[serde(rename = "OBJECT_OPCLASS")]
    Opclass,
    #[serde(rename = "OBJECT_OPERATOR")]
    Operator,
    #[serde(rename = "OBJECT_OPFAMILY")]
    Opfamily,
    #[serde(rename = "OBJECT_POLICY")]
    Policy,
    #[serde(rename = "OBJECT_PROCEDURE")]
    Procedure,
    #[serde(rename = "OBJECT_PUBLICATION")]
    Publication,
    #[serde(rename = "OBJECT_PUBLICATION_REL")]
    PublicationRel,
    #[serde(rename = "OBJECT_ROLE")]
    Role,
    #[serde(rename = "OBJECT_ROUTINE")]
    Routine,
    #[serde(rename = "OBJECT_RULE")]
    Rule,
    #[serde(rename = "OBJECT_SCHEMA")]
    Schema,
    #[serde(rename = "OBJECT_SEQUENCE")]
    Sequence,
    #[serde(rename = "OBJECT_SUBSCRIPTION")]
    Subscription,
    #[serde(rename = "OBJECT_STATISTIC_EXT")]
    StatisticExt,
    #[serde(rename = "OBJECT_TABCONSTRAINT")]
    Tabconstraint,
    #[serde(rename = "OBJECT_TABLE")]
    Table,
    #[serde(rename = "OBJECT_TABLESPACE")]
    Tablespace,
    #[serde(rename = "OBJECT_TRANSFORM")]
    Transform,
    #[serde(rename = "OBJECT_TRIGGER")]
    Trigger,
    #[serde(rename = "OBJECT_TSCONFIGURATION")]
    Tsconfiguration,
    #[serde(rename = "OBJECT_TSDICTIONARY")]
    Tsdictionary,
    #[serde(rename = "OBJECT_TSPARSER")]
    Tsparser,
    #[serde(rename = "OBJECT_TSTEMPLATE")]
    Tstemplate,
    #[serde(rename = "OBJECT_TYPE")]
    Type,
    #[serde(rename = "OBJECT_USER_MAPPING")]
    UserMapping,
    #[serde(rename = "OBJECT_VIEW")]
    View,
}

// List	   *options;		/* WITH clause options: a list of DefElem */
// Node	   *whereClause;	/* qualification (partial-index predicate) */
// List	   *excludeOpNames; /* exclusion operator names, or NIL if none */
// char	   *idxcomment;		/* comment to apply to index, or NULL */
// Oid			indexOid;		/* OID of an existing index, if any */
// Oid			oldNode;		/* relfilenode of existing storage, if any */
#[derive(Debug, Deserialize, Serialize)]
pub enum AlterTableCmds {
    AlterTableCmd(AlterTableCmd),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlterTableStmt {
    pub cmds: Vec<AlterTableCmds>,
    pub relation: RangeVar,
    pub objtype: ObjectType,
    #[serde(default)]
    pub missing_ok: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDomainStmt {
    #[serde(rename = "domainname")]
    pub domain_name: Vec<QualifiedName>,
    #[serde(rename = "typeName")]
    pub typename: Value,
    #[serde(default)]
    pub constraints: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlterDomainStmt {
    pub behavior: DropBehavior,
    pub name: Option<String>,
    pub subtype: String,
    #[serde(rename = "typeName")]
    pub typename: Value,
    pub def: Option<Value>,
}

/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L249-L257>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DropBehavior {
    #[serde(rename = "DROP_RESTRICT")]
    Restrict,
    #[serde(rename = "DROP_CASCADE")]
    DropCascade,
}
impl Default for DropBehavior {
    fn default() -> Self {
        Self::Restrict
    }
}

/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L259-L332>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AlterTableType {
    #[serde(rename = "AT_AddColumn")]
    AddColumn,
    #[serde(rename = "AT_AddColumnRecurse")]
    AddColumnRecurse,
    #[serde(rename = "AT_AddColumnToView")]
    AddColumnToView,
    #[serde(rename = "AT_ColumnDefault")]
    ColumnDefault,
    #[serde(rename = "AT_CookedColumnDefault")]
    CookedColumnDefault,
    #[serde(rename = "AT_DropNotNull")]
    DropNotNull,
    #[serde(rename = "AT_SetNotNull")]
    SetNotNull,
    #[serde(rename = "AT_DropExpression")]
    DropExpression,
    #[serde(rename = "AT_CheckNotNull")]
    CheckNotNull,
    #[serde(rename = "AT_SetStatistics")]
    SetStatistics,
    #[serde(rename = "AT_SetOptions")]
    SetOptions,
    #[serde(rename = "AT_ResetOptions")]
    ResetOptions,
    #[serde(rename = "AT_SetStorage")]
    SetStorage,
    #[serde(rename = "AT_DropColumn")]
    DropColumn,
    #[serde(rename = "AT_DropColumnRecurse")]
    DropColumnRecurse,
    #[serde(rename = "AT_AddIndex")]
    AddIndex,
    #[serde(rename = "AT_ReAddIndex")]
    ReAddIndex,
    #[serde(rename = "AT_AddConstraint")]
    AddConstraint,
    #[serde(rename = "AT_AddConstraintRecurse")]
    AddConstraintRecurse,
    #[serde(rename = "AT_ReAddConstraint")]
    ReAddConstraint,
    #[serde(rename = "AT_ReAddDomainConstraint")]
    ReAddDomainConstraint,
    #[serde(rename = "AT_AlterConstraint")]
    AlterConstraint,
    #[serde(rename = "AT_ValidateConstraint")]
    ValidateConstraint,
    #[serde(rename = "AT_ValidateConstraintRecurse")]
    ValidateConstraintRecurse,
    #[serde(rename = "AT_AddIndexConstraint")]
    AddIndexConstraint,
    #[serde(rename = "AT_DropConstraint")]
    DropConstraint,
    #[serde(rename = "AT_DropConstraintRecurse")]
    DropConstraintRecurse,
    #[serde(rename = "AT_ReAddComment")]
    ReAddComment,
    #[serde(rename = "AT_AlterColumnType")]
    AlterColumnType,
    #[serde(rename = "AT_AlterColumnGenericOptions")]
    AlterColumnGenericOptions,
    #[serde(rename = "AT_ChangeOwner")]
    ChangeOwner,
    #[serde(rename = "AT_ClusterOn")]
    ClusterOn,
    #[serde(rename = "AT_DropCluster")]
    DropCluster,
    #[serde(rename = "AT_SetLogged")]
    SetLogged,
    #[serde(rename = "AT_SetUnLogged")]
    SetUnLogged,
    #[serde(rename = "AT_DropOids")]
    DropOids,
    #[serde(rename = "AT_SetTableSpace")]
    SetTableSpace,
    #[serde(rename = "AT_SetRelOptions")]
    SetRelOptions,
    #[serde(rename = "AT_ResetRelOptions")]
    ResetRelOptions,
    #[serde(rename = "AT_ReplaceRelOptions")]
    ReplaceRelOptions,
    #[serde(rename = "AT_EnableTrig")]
    EnableTrig,
    #[serde(rename = "AT_EnableAlwaysTrig")]
    EnableAlwaysTrig,
    #[serde(rename = "AT_EnableReplicaTrig")]
    EnableReplicaTrig,
    #[serde(rename = "AT_DisableTrig")]
    DisableTrig,
    #[serde(rename = "AT_EnableTrigAll")]
    EnableTrigAll,
    #[serde(rename = "AT_DisableTrigAll")]
    DisableTrigAll,
    #[serde(rename = "AT_EnableTrigUser")]
    EnableTrigUser,
    #[serde(rename = "AT_DisableTrigUser")]
    DisableTrigUser,
    #[serde(rename = "AT_EnableRule")]
    EnableRule,
    #[serde(rename = "AT_EnableAlwaysRule")]
    EnableAlwaysRule,
    #[serde(rename = "AT_EnableReplicaRule")]
    EnableReplicaRule,
    #[serde(rename = "AT_DisableRule")]
    DisableRule,
    #[serde(rename = "AT_AddInherit")]
    AddInherit,
    #[serde(rename = "AT_DropInherit")]
    DropInherit,
    #[serde(rename = "AT_AddOf")]
    AddOf,
    #[serde(rename = "AT_DropOf")]
    DropOf,
    #[serde(rename = "AT_ReplicaIdentity")]
    ReplicaIdentity,
    #[serde(rename = "AT_EnableRowSecurity")]
    EnableRowSecurity,
    #[serde(rename = "AT_DisableRowSecurity")]
    DisableRowSecurity,
    #[serde(rename = "AT_ForceRowSecurity")]
    ForceRowSecurity,
    #[serde(rename = "AT_NoForceRowSecurity")]
    NoForceRowSecurity,
    #[serde(rename = "AT_GenericOptions")]
    GenericOptions,
    #[serde(rename = "AT_AttachPartition")]
    AttachPartition,
    #[serde(rename = "AT_DetachPartition")]
    DetachPartition,
    #[serde(rename = "AT_AddIdentity")]
    AddIdentity,
    #[serde(rename = "AT_SetIdentity")]
    SetIdentity,
    #[serde(rename = "AT_DropIdentity")]
    DropIdentity,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ColumnDefConstraint {
    Constraint(Constraint),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PGString {
    pub sval: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct QualifiedName {
    #[serde(rename = "String")]
    pub string: PGString,
}

///
/// `TypeName` - specifies a type in definitions
///
/// For `TypeName` structures generated internally, it is often easier to
/// specify the type by OID than by name.  If `names` is NIL then the
/// actual type OID is given by `type_oid`, otherwise `type_oid` is unused.
/// Similarly, if `typmods` is NIL then the actual typmod is expected to
/// be prespecified in typemod, otherwise typemod is unused.
///
/// If `pct_type` is true, then `names` is actually a field name and we look up
/// the type of that field.  Otherwise (the normal case), `names` is a type
/// name possibly qualified with schema and database name.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeName {
    /// qualified name (list of Value strings)
    #[serde(default)]
    pub names: Vec<QualifiedName>,
    /// type identified by OID
    #[serde(rename = "typeOid")]
    pub type_oid: Option<Value>,
    /// is a set?
    #[serde(default)]
    pub setof: bool,
    /// %TYPE specified?
    #[serde(default)]
    pub pct_type: bool,
    /// type modifier expression(s)
    #[serde(default)]
    pub typmods: Vec<Value>,
    /// prespecified type modifier
    pub typemod: i32,
    #[serde(rename = "arrayBounds", default)]
    pub array_bounds: Vec<Value>,
    /// token location, or -1 if unknown
    pub location: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColumnDef {
    // int			inhcount;		/* number of times column is inherited */
    // bool		is_not_null;	/* NOT NULL constraint specified? */
    // bool		is_from_type;	/* column definition came from table type */
    // bool		is_from_parent; /* column def came from partition parent */
    // char		storage;		/* attstorage setting, or 0 for default */
    /// default value (untransformed parse tree)
    // raw_default: Value,
    // Node	   *cooked_default; /* default value (transformed expr tree) */
    // char		identity;		/* attidentity setting */
    // RangeVar   *identitySequence; /* to store identity sequence name for ALTER
    // 							   * TABLE ... ADD COLUMN */
    // CollateClause *collClause;	/* untransformed COLLATE spec, if any */
    // Oid			collOid;		/* collation OID (InvalidOid if not set) */
    // List	   *fdwoptions;		/* per-column FDW options */
    pub colname: Option<String>,
    #[serde(rename = "typeName")]
    pub type_name: TypeName,
    #[serde(default)]
    pub constraints: Vec<ColumnDefConstraint>,
    /// column has local (non-inherited) def'n
    #[serde(default)]
    pub is_local: bool,
    pub location: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AlterTableDef {
    TypeCast(Value),
    FuncCall(Value),
    Constraint(Constraint),
    ColumnDef(ColumnDef),
    #[serde(rename = "A_Const")]
    Constant(Value),
    ReplicaIdentityStmt(Value),
    SQLValueFunction(Value),
    List(Value),
    PartitionCmd(Value),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlterTableCmd {
    /// Type of table alteration to apply
    pub subtype: AlterTableType,
    /// column, constraint, or trigger to act on, or tablespace
    pub name: Option<String>,
    /// definition of new column, index, constraint, or parent table
    pub def: Option<AlterTableDef>,
    #[serde(default)]
    pub behavior: DropBehavior,
    // RoleSpec   *newowner;
    /// skip error if missing?
    #[serde(default)]
    pub missing_ok: bool,
}

/// Source: <https://github.com/pganalyze/libpg_query/blob/b2790f8140721ff7f047167ecd7d44267b0a3880/src/pg_query_enum_defs.c#L359-L379>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ConstrType {
    /// not standard SQL, but a lot of people expect it
    #[serde(rename = "CONSTR_NULL")]
    Null,
    #[serde(rename = "CONSTR_NOTNULL")]
    NotNull,
    #[serde(rename = "CONSTR_DEFAULT")]
    Default,
    #[serde(rename = "CONSTR_IDENTITY")]
    Identity,
    #[serde(rename = "CONSTR_GENERATED")]
    Generated,
    #[serde(rename = "CONSTR_CHECK")]
    Check,
    #[serde(rename = "CONSTR_PRIMARY")]
    Primary,
    #[serde(rename = "CONSTR_UNIQUE")]
    Unique,
    #[serde(rename = "CONSTR_EXCLUSION")]
    Exclusion,
    #[serde(rename = "CONSTR_FOREIGN")]
    Foreign,
    #[serde(rename = "CONSTR_ATTR_DEFERRABLE")]
    AttrDeferrable,
    /// attributes for previous constraint node
    #[serde(rename = "CONSTR_ATTR_NOT_DEFERRABLE")]
    AttrNotDeferrable,
    #[serde(rename = "CONSTR_ATTR_DEFERRED")]
    AttrDeferred,
    #[serde(rename = "CONSTR_ATTR_IMMEDIATE")]
    AttrImmediate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Constraint {
    #[serde(default)]
    pub conname: Option<String>,
    pub contype: ConstrType,

    /* Fields used for most/all constraint types: */
    // char	   *conname;		/* Constraint name, or NULL if unnamed */
    // bool		deferrable;		/* DEFERRABLE? */
    // bool		initdeferred;	/* INITIALLY DEFERRED? */
    /// token location, or -1 if unknown
    #[serde(default)]
    pub location: Option<i32>,

    /* Fields used for constraints with expressions (CHECK and DEFAULT): */
    // bool		is_no_inherit;	/* is constraint non-inheritable? */
    /// expr, as untransformed parse tree
    pub raw_expr: Option<Value>,
    // char	   *cooked_expr;	/* expr, as nodeToString representation */
    // char		generated_when;

    // Fields used for unique constraints (UNIQUE and PRIMARY KEY): */
    /// String nodes naming referenced column(s)
    pub keys: Option<Value>,

    /* Fields used for EXCLUSION constraints: */
    // List	   *exclusions;		/* list of (IndexElem, operator name) pairs */

    /* Fields used for index constraints (UNIQUE, PRIMARY KEY, EXCLUSION): */
    // List	   *options;		/* options from WITH clause */
    /// existing index to use; otherwise NULL
    pub indexname: Option<String>,
    // char	   *indexspace;		/* index tablespace; NULL for default */
    /* These could be, but currently are not, used for UNIQUE/PKEY: */
    // char	   *access_method;	/* index access method; NULL for default */
    // Node	   *where_clause;	/* partial index predicate */

    /* Fields used for FOREIGN KEY constraints: */
    // RangeVar   *pktable;		/* Primary key table */
    // List	   *fk_attrs;		/* Attributes of foreign key */
    // List	   *pk_attrs;		/* Corresponding attrs in PK table */
    // char		fk_matchtype;	/* FULL, PARTIAL, SIMPLE */
    // char		fk_upd_action;	/* ON UPDATE action */
    // char		fk_del_action;	/* ON DELETE action */
    // List	   *old_conpfeqop;	/* pg_constraint.conpfeqop of my former self */
    // Oid			old_pktable_oid;	/* pg_constraint.confrelid of my former
    /// skip validation of existing rows?
    #[serde(default)]
    pub skip_validation: bool,
    /// mark the new constraint as valid?
    #[serde(default)]
    pub initially_valid: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableLikeClause {
    pub relation: RangeVar,
    pub options: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RenameStmt {
    // Node	   *object;			/* in case it's some other object */
    pub newname: String,
    pub behavior: DropBehavior,
    // in case it's a table
    pub relation: Option<RangeVar>,
    #[serde(rename = "relationType")]
    pub relation_type: ObjectType,
    #[serde(rename = "renameType")]
    pub rename_type: ObjectType,
    /// name of contained object (column, rule, trigger, etc)
    pub subname: Option<String>,
    // bool		missing_ok;		/* skip error if missing? */
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TableElt {
    ColumnDef(ColumnDef),
    Constraint(Constraint),
    TableLikeClause(TableLikeClause),
}

/// What to do at commit time for temporary relations
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum OnCommitAction {
    #[serde(rename = "ONCOMMIT_NOOP")]
    Noop,
    #[serde(rename = "ONCOMMIT_PRESERVE_ROWS")]
    PreserveRows,
    #[serde(rename = "ONCOMMIT_DELETE_ROWS")]
    DeleteRows,
    #[serde(rename = "ONCOMMIT_DROP")]
    Drop,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateStmt {
    /// relation to create
    pub relation: RangeVar,
    /// column definitions (list of `ColumnDef`)
    #[serde(rename = "tableElts", default)]
    pub table_elts: Vec<TableElt>,
    /// relations to inherit from (list of inhRelation)
    #[serde(rename = "inhRelations")]
    #[serde(default)]
    pub inh_relations: Vec<Value>,
    /// FOR VALUES clause
    pub partbound: Option<Value>,
    /// PARTITION BY clause
    pub partspec: Option<Value>,
    /// OF typename
    #[serde(rename = "ofTypename")]
    pub of_typename: Option<Value>,
    /// constraints (list of Constraint nodes)
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    /// options from WITH clause
    #[serde(default)]
    pub options: Vec<Value>,
    /// what do we do at COMMIT?
    pub oncommit: OnCommitAction,
    /// table space to use, or NULL
    pub tablespacename: Option<String>,
    /// just do nothing if it already exists?
    #[serde(default)]
    pub if_not_exists: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DropStmt {
    pub behavior: DropBehavior,
    #[serde(default)]
    pub concurrent: bool,
    #[serde(default)]
    pub missing_ok: bool,
    #[serde(rename = "removeType")]
    pub remove_type: ObjectType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StmtRoot {
    pub stmts: Vec<RawStmt>,
}

/// case for each node type found in Postgres' parsenodes.h
/// <https://github.com/lfittl/libpg_query/blob/6b1c3a582d38701593c5cadd260445737b9f7043/src/postgres/include/nodes/parsenodes.h>
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize, Serialize)]
pub enum Stmt {
    TransactionStmt(TransactionStmt),
    SelectStmt(SelectStmt),
    IndexStmt(IndexStmt),
    AlterTableStmt(AlterTableStmt),
    RenameStmt(RenameStmt),
    CreateStmt(CreateStmt),
    InsertStmt(Value),
    UpdateStmt(Value),
    DeleteStmt(Value),
    CreateSchemaStmt(Value),
    AlterDomainStmt(AlterDomainStmt),
    GrantStmt(Value),
    GrantRoleStmt(Value),
    AlterDefaultPrivilegesStmt(Value),
    CopyStmt(Value),
    VariableSetStmt(Value),
    VariableShowStmt(Value),
    CreateTableSpaceStmt(Value),
    DropTableSpaceStmt(Value),
    CreateExtensionStmt(Value),
    AlterExtensionStmt(Value),
    DropStmt(DropStmt),
    AlterObjectSchemaStmt(Value),
    AlterExtensionContentsStmt(Value),
    CreateFdwStmt(Value),
    AlterFdwStmt(Value),
    CreateForeignServerStmt(Value),
    AlterForeignServerStmt(Value),
    CreateForeignTableStmt(Value),
    CreateUserMappingStmt(Value),
    AlterUserMappingStmt(Value),
    DropUserMappingStmt(Value),
    ImportForeignSchemaStmt(Value),
    CreatePolicyStmt(Value),
    AlterPolicyStmt(Value),
    CreateAmStmt(Value),
    CreateTrigStmt(Value),
    CreateEventTrigStmt(Value),
    AlterEventTrigStmt(Value),
    CreateFunctionStmt(Value),
    CallStmt(Value),
    AlterFunctionStmt(Value),
    CreatePLangStmt(Value),
    CreateRoleStmt(Value),
    AlterRoleStmt(Value),
    AlterRoleSetStmt(Value),
    DropRoleStmt(Value),
    CreateSeqStmt(Value),
    AlterSeqStmt(Value),
    DefineStmt(Value),
    CreateDomainStmt(CreateDomainStmt),
    CreateOpClassStmt(Value),
    CreateOpFamilyStmt(Value),
    AlterOpFamilyStmt(Value),
    TruncateStmt(Value),
    CommentStmt(Value),
    SecLabelStmt(Value),
    DeclareCursorStmt(Value),
    ClosePortalStmt(Value),
    FetchStmt(Value),
    CreateStatsStmt(Value),
    ExplainStmt(Value),
    AlterOwnerStmt(Value),
    DoStmt(Value),
    AlterObjectDependsStmt(Value),
    AlterOperatorStmt(Value),
    RuleStmt(Value),
    NotifyStmt(Value),
    ListenStmt(Value),
    UnlistenStmt(Value),
    CompositeTypeStmt(Value),
    CreateEnumStmt(Value),
    CreateRangeStmt(Value),
    AlterEnumStmt(Value),
    ViewStmt(Value),
    LoadStmt(Value),
    CreatedbStmt(Value),
    AlterDatabaseRefreshCollStmt(Value),
    AlterDatabaseStmt(Value),
    AlterDatabaseSetStmt(Value),
    DropdbStmt(Value),
    AlterSystemStmt(Value),
    ClusterStmt(Value),
    VacuumStmt(Value),
    CreateTableAsStmt(Value),
    RefreshMatViewStmt(Value),
    CheckPointStmt(Value),
    DiscardStmt(Value),
    LockStmt(Value),
    ConstraintsSetStmt(Value),
    ReindexStmt(Value),
    CreateConversionStmt(Value),
    CreateCastStmt(Value),
    CreateTransformStmt(Value),
    PrepareStmt(Value),
    ExecuteStmt(Value),
    DeallocateStmt(Value),
    DropOwnedStmt(Value),
    ReassignOwnedStmt(Value),
    AlterTSDictionaryStmt(Value),
    AlterTSConfigurationStmt(Value),
    CreatePublicationStmt(Value),
    AlterPublicationStmt(Value),
    CreateSubscriptionStmt(Value),
    AlterSubscriptionStmt(Value),
    DropSubscriptionStmt(Value),
}
