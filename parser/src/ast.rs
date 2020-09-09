use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum TransactionStmtKind {
    Begin,
    /// semantically identical to BEGIN
    Start,
    Commit,
    Rollback,
    Savepoint,
    Release,
    RollbackTo,
    Prepare,
    CommitPrepared,
    RollbackPrepared,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionStmt {
    pub kind: TransactionStmtKind,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawStmt {
    pub stmt: Stmt,
    #[serde(default)]
    pub stmt_location: i32,
    /// None when the statement doesn't have a closing semicolon
    pub stmt_len: Option<i32>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SetOperation {
    None,
    Union,
    Intersect,
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
    /// the target list (of ResTarget)
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
    /// WINDOW window_name AS (...), ...
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
    /// sort clause (a list of SortBy's)
    #[serde(rename = "sortClause")]
    sort_clause: Option<Value>,
    /// # of result tuples to skip
    #[serde(rename = "limitOffset")]
    limit_offset: Option<Value>,
    /// # of result tuples to return
    #[serde(rename = "limitCount")]
    limit_count: Option<Value>,
    /// FOR UPDATE (list of LockingClause's)
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
    pub larg: Option<Box<SelectChild>>,
    /// right child
    pub rarg: Option<Box<SelectChild>>,
}

/// Sort ordering options for ORDER BY and CREATE INDEX
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortByDir {
    Default,
    Asc,
    Desc,
    /// not allowed in CREATE INDEX ...
    Using,
}

impl Default for SortByDir {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortByNulls {
    Default,
    First,
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
    pub inh: bool,
    /// see RELPERSISTENCE_* in pg_class.h
    pub relpersistence: String,
    /// table alias & optional column aliases
    pub alias: Option<Value>,
    /// token location, or -1 if unknown
    pub location: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RelationKind {
    RangeVar(RangeVar),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexStmt {
    /// name of access method (eg. btree)
    #[serde(rename = "accessMethod")]
    pub access_method: String,
    /// name of new index, or NULL for default
    pub idxname: String,
    #[serde(rename = "indexParams")]
    pub index_params: Vec<IndexParams>,
    /// relation to build index on
    pub relation: RelationKind,
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
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ObjectType {
    AccessMethod,
    Aggregate,
    Amop,
    Amproc,
    /// type's attribute, when distinct from column
    Attribute,
    Cast,
    Column,
    Collation,
    Conversion,
    Database,
    Default,
    Defacl,
    Domain,
    Domconstraint,
    EventTrigger,
    Extension,
    Fdw,
    ForeignServer,
    ForeignTable,
    Function,
    Index,
    Language,
    Largeobject,
    Matview,
    Opclass,
    Operator,
    Opfamily,
    Policy,
    Publication,
    PublicationRel,
    Role,
    Rule,
    Schema,
    Sequence,
    Subscription,
    StatisticExt,
    TabConstraint,
    Table,
    Tablespace,
    Transform,
    Trigger,
    TsConfiguration,
    TsDictionary,
    TsParser,
    TsTemplate,
    Type,
    UserMapping,
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
    pub relation: RelationKind,
    pub relkind: ObjectType,
    #[serde(default)]
    pub missing_ok: bool,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DropBehavior {
    /// drop fails if any dependent objects
    Restrict,
    /// remove dependent objects too
    DropCascade,
}
impl Default for DropBehavior {
    fn default() -> Self {
        Self::Restrict
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum AlterTableType {
    /// add column
    AddColumn,
    /// internal to commands/tablecmds.c
    AddColumnRecurse,
    /// implicitly via CREATE OR REPLACE VIEW
    AddColumnToView,
    /// alter column default
    ColumnDefault,
    /// alter column drop not null
    DropNotNull,
    /// alter column set not null
    SetNotNull,
    /// alter column set statistics
    SetStatistics,
    /// alter column set ( options )
    SetOptions,
    /// alter column reset ( options )
    ResetOptions,
    /// alter column set storage
    SetStorage,
    /// drop column
    DropColumn,
    /// internal to commands/tablecmds.c
    DropColumnRecurse,
    /// add index
    AddIndex,
    /// internal to commands/tablecmds.c
    ReAddIndex,
    /// add constraint
    AddConstraint,
    /// internal to commands/tablecmds.c
    AddConstraintRecurse,
    /// internal to commands/tablecmds.c
    ReAddConstraint,
    /// alter constraint
    AlterConstraint,
    /// validate constraint
    ValidateConstraint,
    /// internal to commands/tablecmds.c
    ValidateConstraintRecurse,
    /// pre-processed add constraint (local in parser/parse_utilcmd.c)
    ProcessedConstraint,
    /// add constraint using existing index
    AddIndexConstraint,
    /// drop constraint
    DropConstraint,
    /// internal to commands/tablecmds.c
    DropConstraintRecurse,
    /// internal to commands/tablecmds.c
    ReAddComment,
    /// alter column type
    AlterColumnType,
    /// alter column OPTIONS (...)
    AlterColumnGenericOptions,
    /// change owner
    ChangeOwner,
    /// CLUSTER ON
    ClusterOn,
    /// SET WITHOUT CLUSTER
    DropCluster,
    /// SET LOGGED
    SetLogged,
    /// SET UNLOGGED
    SetUnLogged,
    /// SET WITH OIDS
    AddOids,
    /// internal to commands/tablecmds.c
    AddOidsRecurse,
    /// SET WITHOUT OIDS
    DropOids,
    /// SET TABLESPACE
    SetTableSpace,
    /// SET (...) -- AM specific parameters
    SetRelOptions,
    /// RESET (...) -- AM specific parameters
    ResetRelOptions,
    /// replace reloption list in its entirety
    ReplaceRelOptions,
    /// ENABLE TRIGGER name
    EnableTrig,
    /// ENABLE ALWAYS TRIGGER name
    EnableAlwaysTrig,
    /// ENABLE REPLICA TRIGGER name
    EnableReplicaTrig,
    /// DISABLE TRIGGER name
    DisableTrig,
    /// ENABLE TRIGGER ALL
    EnableTrigAll,
    /// DISABLE TRIGGER ALL
    DisableTrigAll,
    /// ENABLE TRIGGER USER
    EnableTrigUser,
    /// DISABLE TRIGGER USER
    DisableTrigUser,
    /// ENABLE RULE name
    EnableRule,
    /// ENABLE ALWAYS RULE name
    EnableAlwaysRule,
    /// ENABLE REPLICA RULE name
    EnableReplicaRule,
    /// DISABLE RULE name
    DisableRule,
    /// INHERIT parent
    AddInherit,
    /// NO INHERIT parent
    DropInherit,
    /// OF <type_name>
    AddOf,
    /// NOT OF
    DropOf,
    /// REPLICA IDENTITY
    ReplicaIdentity,
    /// ENABLE ROW SECURITY
    EnableRowSecurity,
    /// DISABLE ROW SECURITY
    DisableRowSecurity,
    /// FORCE ROW SECURITY
    ForceRowSecurity,
    /// NO FORCE ROW SECURITY
    NoForceRowSecurity,
    /// OPTIONS (...)
    GenericOptions,
    /// ATTACH PARTITION
    AttachPartition,
    /// DETACH PARTITION
    DetachPartition,
    /// ADD IDENTITY
    AddIdentity,
    /// SET identity column options
    SetIdentity,
    /// DROP IDENTITY
    DropIdentity,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ColumnDefConstraint {
    Constraint(Constraint),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PGString {
    pub str: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum QualifiedName {
    String(PGString),
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
pub enum ColumnDefTypeName {
    TypeName(TypeName),
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
    pub type_name: ColumnDefTypeName,
    #[serde(default)]
    pub constraints: Vec<ColumnDefConstraint>,
    /// column has local (non-inherited) def'n
    #[serde(default)]
    pub is_local: bool,
    pub location: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AlterTableDef {
    FuncCall(Value),
    Constraint(Constraint),
    ColumnDef(ColumnDef),
    #[serde(rename = "A_Const")]
    Constant(Value),
    ReplicaIdentityStmt(Value),
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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ConstrType {
    /// not standard SQL, but a lot of people expect it
    Null,
    NotNull,
    Default,
    Identity,
    Check,
    Primary,
    Unique,
    Exclusion,
    Foreign,
    /// attributes for previous constraint node
    AttrDeferrable,
    AttrNotDeferrable,
    AttrDeferred,
    AttrImmediate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Constraint {
    pub contype: ConstrType,

    /* Fields used for most/all constraint types: */
    // char	   *conname;		/* Constraint name, or NULL if unnamed */
    // bool		deferrable;		/* DEFERRABLE? */
    // bool		initdeferred;	/* INITIALLY DEFERRED? */
    /// token location, or -1 if unknown
    pub location: i32,

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
pub struct RenameStmt {
    // Node	   *object;			/* in case it's some other object */
    pub newname: String,
    pub behavior: DropBehavior,
    // in case it's a table
    pub relation: Option<RelationKind>,
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
}

/// What to do at commit time for temporary relations
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum OnCommitAction {
    /// No ON COMMIT clause (do nothing)
    Noop,
    /// ON COMMIT PRESERVE ROWS (do nothing)
    PreserveRows,
    /// ON COMMIT DELETE ROWS
    DeleteRows,
    /// ON COMMIT DROP
    Drop,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateStmt {
    /// relation to create
    pub relation: RelationKind,
    /// column definitions (list of ColumnDef)
    #[serde(rename = "tableElts")]
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
pub enum RootStmt {
    RawStmt(RawStmt),
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
    AlterDomainStmt(Value),
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
    DropStmt(Value),
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
    AlterFunctionStmt(Value),
    CreatePLangStmt(Value),
    CreateRoleStmt(Value),
    AlterRoleStmt(Value),
    AlterRoleSetStmt(Value),
    DropRoleStmt(Value),
    CreateSeqStmt(Value),
    AlterSeqStmt(Value),
    DefineStmt(Value),
    CreateDomainStmt(Value),
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
