#![allow(bad_style, missing_docs, clippy::upper_case_acronyms)]
#[doc = r"The kind of syntax node, e.g. `IDENT`, `SELECT_KW`, or `WHERE_CLAUSE`. Needs to be compatible with [`rowan::SyntaxKind`]"]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    DOLLAR,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    AT,
    POUND,
    TILDE,
    QUESTION,
    AMP,
    PIPE,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    UNDERSCORE,
    DOT,
    COLON,
    EQ,
    BANG,
    MINUS,
    BACKTICK,
    ABORT_KW,
    ABSENT_KW,
    ABSOLUTE_KW,
    ACCESS_KW,
    ACTION_KW,
    ADD_KW,
    ADMIN_KW,
    AFTER_KW,
    AGGREGATE_KW,
    ALL_KW,
    ALSO_KW,
    ALTER_KW,
    ALWAYS_KW,
    ANALYSE_KW,
    ANALYZE_KW,
    AND_KW,
    ANY_KW,
    ARRAY_KW,
    AS_KW,
    ASC_KW,
    ASENSITIVE_KW,
    ASSERTION_KW,
    ASSIGNMENT_KW,
    ASYMMETRIC_KW,
    AT_KW,
    ATOMIC_KW,
    ATTACH_KW,
    ATTRIBUTE_KW,
    AUTHORIZATION_KW,
    BACKWARD_KW,
    BEFORE_KW,
    BEGIN_KW,
    BETWEEN_KW,
    BIGINT_KW,
    BINARY_KW,
    BIT_KW,
    BOOLEAN_KW,
    BOTH_KW,
    BREADTH_KW,
    BY_KW,
    CACHE_KW,
    CALL_KW,
    CALLED_KW,
    CASCADE_KW,
    CASCADED_KW,
    CASE_KW,
    CAST_KW,
    CATALOG_KW,
    CHAIN_KW,
    CHAR_KW,
    CHARACTER_KW,
    CHARACTERISTICS_KW,
    CHECK_KW,
    CHECKPOINT_KW,
    CLASS_KW,
    CLOSE_KW,
    CLUSTER_KW,
    COALESCE_KW,
    COLLATE_KW,
    COLLATION_KW,
    COLUMN_KW,
    COLUMNS_KW,
    COMMENT_KW,
    COMMENTS_KW,
    COMMIT_KW,
    COMMITTED_KW,
    COMPRESSION_KW,
    CONCURRENTLY_KW,
    CONDITIONAL_KW,
    CONFIGURATION_KW,
    CONFLICT_KW,
    CONNECTION_KW,
    CONSTRAINT_KW,
    CONSTRAINTS_KW,
    CONTENT_KW,
    CONTINUE_KW,
    CONVERSION_KW,
    COPY_KW,
    COST_KW,
    CREATE_KW,
    CROSS_KW,
    CSV_KW,
    CUBE_KW,
    CURRENT_KW,
    CURRENT_CATALOG_KW,
    CURRENT_DATE_KW,
    CURRENT_ROLE_KW,
    CURRENT_SCHEMA_KW,
    CURRENT_TIME_KW,
    CURRENT_TIMESTAMP_KW,
    CURRENT_USER_KW,
    CURSOR_KW,
    CYCLE_KW,
    DATA_KW,
    DATABASE_KW,
    DAY_KW,
    DEALLOCATE_KW,
    DEC_KW,
    DECIMAL_KW,
    DECLARE_KW,
    DEFAULT_KW,
    DEFAULTS_KW,
    DEFERRABLE_KW,
    DEFERRED_KW,
    DEFINER_KW,
    DELETE_KW,
    DELIMITER_KW,
    DELIMITERS_KW,
    DEPENDS_KW,
    DEPTH_KW,
    DESC_KW,
    DETACH_KW,
    DICTIONARY_KW,
    DISABLE_KW,
    DISCARD_KW,
    DISTINCT_KW,
    DO_KW,
    DOCUMENT_KW,
    DOMAIN_KW,
    DOUBLE_KW,
    DROP_KW,
    EACH_KW,
    ELSE_KW,
    EMPTY_KW,
    ENABLE_KW,
    ENCODING_KW,
    ENCRYPTED_KW,
    END_KW,
    ENFORCED_KW,
    ENUM_KW,
    ERROR_KW,
    ESCAPE_KW,
    EVENT_KW,
    EXCEPT_KW,
    EXCLUDE_KW,
    EXCLUDING_KW,
    EXCLUSIVE_KW,
    EXECUTE_KW,
    EXISTS_KW,
    EXPLAIN_KW,
    EXPRESSION_KW,
    EXTENSION_KW,
    EXTERNAL_KW,
    EXTRACT_KW,
    FALSE_KW,
    FAMILY_KW,
    FETCH_KW,
    FILTER_KW,
    FINALIZE_KW,
    FIRST_KW,
    FLOAT_KW,
    FOLLOWING_KW,
    FOR_KW,
    FORCE_KW,
    FOREIGN_KW,
    FORMAT_KW,
    FORWARD_KW,
    FREEZE_KW,
    FROM_KW,
    FULL_KW,
    FUNCTION_KW,
    FUNCTIONS_KW,
    GENERATED_KW,
    GLOBAL_KW,
    GRANT_KW,
    GRANTED_KW,
    GREATEST_KW,
    GROUP_KW,
    GROUPING_KW,
    GROUPS_KW,
    HANDLER_KW,
    HAVING_KW,
    HEADER_KW,
    HOLD_KW,
    HOUR_KW,
    IDENTITY_KW,
    IF_KW,
    ILIKE_KW,
    IMMEDIATE_KW,
    IMMUTABLE_KW,
    IMPLICIT_KW,
    IMPORT_KW,
    IN_KW,
    INCLUDE_KW,
    INCLUDING_KW,
    INCREMENT_KW,
    INDENT_KW,
    INDEX_KW,
    INDEXES_KW,
    INHERIT_KW,
    INHERITS_KW,
    INITIALLY_KW,
    INLINE_KW,
    INNER_KW,
    INOUT_KW,
    INPUT_KW,
    INSENSITIVE_KW,
    INSERT_KW,
    INSTEAD_KW,
    INT_KW,
    INTEGER_KW,
    INTERSECT_KW,
    INTERVAL_KW,
    INTO_KW,
    INVOKER_KW,
    IS_KW,
    ISNULL_KW,
    ISOLATION_KW,
    JOIN_KW,
    JSON_KW,
    JSON_ARRAY_KW,
    JSON_ARRAYAGG_KW,
    JSON_EXISTS_KW,
    JSON_OBJECT_KW,
    JSON_OBJECTAGG_KW,
    JSON_QUERY_KW,
    JSON_SCALAR_KW,
    JSON_SERIALIZE_KW,
    JSON_TABLE_KW,
    JSON_VALUE_KW,
    KEEP_KW,
    KEY_KW,
    KEYS_KW,
    LABEL_KW,
    LANGUAGE_KW,
    LARGE_KW,
    LAST_KW,
    LATERAL_KW,
    LEADING_KW,
    LEAKPROOF_KW,
    LEAST_KW,
    LEFT_KW,
    LEVEL_KW,
    LIKE_KW,
    LIMIT_KW,
    LISTEN_KW,
    LOAD_KW,
    LOCAL_KW,
    LOCALTIME_KW,
    LOCALTIMESTAMP_KW,
    LOCATION_KW,
    LOCK_KW,
    LOCKED_KW,
    LOGGED_KW,
    MAPPING_KW,
    MATCH_KW,
    MATCHED_KW,
    MATERIALIZED_KW,
    MAXVALUE_KW,
    MERGE_KW,
    MERGE_ACTION_KW,
    METHOD_KW,
    MINUTE_KW,
    MINVALUE_KW,
    MODE_KW,
    MONTH_KW,
    MOVE_KW,
    NAME_KW,
    NAMES_KW,
    NATIONAL_KW,
    NATURAL_KW,
    NCHAR_KW,
    NESTED_KW,
    NEW_KW,
    NEXT_KW,
    NFC_KW,
    NFD_KW,
    NFKC_KW,
    NFKD_KW,
    NO_KW,
    NONE_KW,
    NORMALIZE_KW,
    NORMALIZED_KW,
    NOT_KW,
    NOTHING_KW,
    NOTIFY_KW,
    NOTNULL_KW,
    NOWAIT_KW,
    NULL_KW,
    NULLIF_KW,
    NULLS_KW,
    NUMERIC_KW,
    OBJECT_KW,
    OBJECTS_KW,
    OF_KW,
    OFF_KW,
    OFFSET_KW,
    OIDS_KW,
    OLD_KW,
    OMIT_KW,
    ON_KW,
    ONLY_KW,
    OPERATOR_KW,
    OPTION_KW,
    OPTIONS_KW,
    OR_KW,
    ORDER_KW,
    ORDINALITY_KW,
    OTHERS_KW,
    OUT_KW,
    OUTER_KW,
    OVER_KW,
    OVERLAPS_KW,
    OVERLAY_KW,
    OVERRIDING_KW,
    OWNED_KW,
    OWNER_KW,
    PARALLEL_KW,
    PARAMETER_KW,
    PARSER_KW,
    PARTIAL_KW,
    PARTITION_KW,
    PASSING_KW,
    PASSWORD_KW,
    PATH_KW,
    PERIOD_KW,
    PLACING_KW,
    PLAN_KW,
    PLANS_KW,
    POLICY_KW,
    POSITION_KW,
    PRECEDING_KW,
    PRECISION_KW,
    PREPARE_KW,
    PREPARED_KW,
    PRESERVE_KW,
    PRIMARY_KW,
    PRIOR_KW,
    PRIVILEGES_KW,
    PROCEDURAL_KW,
    PROCEDURE_KW,
    PROCEDURES_KW,
    PROGRAM_KW,
    PUBLICATION_KW,
    QUOTE_KW,
    QUOTES_KW,
    RANGE_KW,
    READ_KW,
    REAL_KW,
    REASSIGN_KW,
    RECURSIVE_KW,
    REF_KW,
    REFERENCES_KW,
    REFERENCING_KW,
    REFRESH_KW,
    REINDEX_KW,
    RELATIVE_KW,
    RELEASE_KW,
    RENAME_KW,
    REPEATABLE_KW,
    REPLACE_KW,
    REPLICA_KW,
    RESET_KW,
    RESTART_KW,
    RESTRICT_KW,
    RETURN_KW,
    RETURNING_KW,
    RETURNS_KW,
    REVOKE_KW,
    RIGHT_KW,
    ROLE_KW,
    ROLLBACK_KW,
    ROLLUP_KW,
    ROUTINE_KW,
    ROUTINES_KW,
    ROW_KW,
    ROWS_KW,
    RULE_KW,
    SAVEPOINT_KW,
    SCALAR_KW,
    SCHEMA_KW,
    SCHEMAS_KW,
    SCROLL_KW,
    SEARCH_KW,
    SECOND_KW,
    SECURITY_KW,
    SELECT_KW,
    SEQUENCE_KW,
    SEQUENCES_KW,
    SERIALIZABLE_KW,
    SERVER_KW,
    SESSION_KW,
    SESSION_USER_KW,
    SET_KW,
    SETOF_KW,
    SETS_KW,
    SHARE_KW,
    SHOW_KW,
    SIMILAR_KW,
    SIMPLE_KW,
    SKIP_KW,
    SMALLINT_KW,
    SNAPSHOT_KW,
    SOME_KW,
    SOURCE_KW,
    SQL_KW,
    STABLE_KW,
    STANDALONE_KW,
    START_KW,
    STATEMENT_KW,
    STATISTICS_KW,
    STDIN_KW,
    STDOUT_KW,
    STORAGE_KW,
    STORED_KW,
    STRICT_KW,
    STRING_KW,
    STRIP_KW,
    SUBSCRIPTION_KW,
    SUBSTRING_KW,
    SUPPORT_KW,
    SYMMETRIC_KW,
    SYSID_KW,
    SYSTEM_KW,
    SYSTEM_USER_KW,
    TABLE_KW,
    TABLES_KW,
    TABLESAMPLE_KW,
    TABLESPACE_KW,
    TARGET_KW,
    TEMP_KW,
    TEMPLATE_KW,
    TEMPORARY_KW,
    TEXT_KW,
    THEN_KW,
    TIES_KW,
    TIME_KW,
    TIMESTAMP_KW,
    TO_KW,
    TRAILING_KW,
    TRANSACTION_KW,
    TRANSFORM_KW,
    TREAT_KW,
    TRIGGER_KW,
    TRIM_KW,
    TRUE_KW,
    TRUNCATE_KW,
    TRUSTED_KW,
    TYPE_KW,
    TYPES_KW,
    UESCAPE_KW,
    UNBOUNDED_KW,
    UNCOMMITTED_KW,
    UNCONDITIONAL_KW,
    UNENCRYPTED_KW,
    UNION_KW,
    UNIQUE_KW,
    UNKNOWN_KW,
    UNLISTEN_KW,
    UNLOGGED_KW,
    UNTIL_KW,
    UPDATE_KW,
    USER_KW,
    USING_KW,
    VACUUM_KW,
    VALID_KW,
    VALIDATE_KW,
    VALIDATOR_KW,
    VALUE_KW,
    VALUES_KW,
    VARCHAR_KW,
    VARIADIC_KW,
    VARYING_KW,
    VERBOSE_KW,
    VERSION_KW,
    VIEW_KW,
    VIEWS_KW,
    VIRTUAL_KW,
    VOLATILE_KW,
    WHEN_KW,
    WHERE_KW,
    WHITESPACE_KW,
    WINDOW_KW,
    WITH_KW,
    WITHIN_KW,
    WITHOUT_KW,
    WORK_KW,
    WRAPPER_KW,
    WRITE_KW,
    XML_KW,
    XMLATTRIBUTES_KW,
    XMLCONCAT_KW,
    XMLELEMENT_KW,
    XMLEXISTS_KW,
    XMLFOREST_KW,
    XMLNAMESPACES_KW,
    XMLPARSE_KW,
    XMLPI_KW,
    XMLROOT_KW,
    XMLSERIALIZE_KW,
    XMLTABLE_KW,
    YEAR_KW,
    YES_KW,
    ZONE_KW,
    BIT_STRING,
    BYTE_STRING,
    DOLLAR_QUOTED_STRING,
    ESC_STRING,
    FLOAT_NUMBER,
    INT_NUMBER,
    NULL,
    POSITIONAL_PARAM,
    STRING,
    COMMENT,
    ERROR,
    IDENT,
    WHITESPACE,
    ADD_COLUMN,
    ADD_CONSTRAINT,
    ADD_GENERATED,
    AGGREGATE,
    ALIAS,
    ALTER_AGGREGATE,
    ALTER_COLLATION,
    ALTER_COLUMN,
    ALTER_CONSTRAINT,
    ALTER_CONVERSION,
    ALTER_DATABASE,
    ALTER_DEFAULT_PRIVILEGES,
    ALTER_DOMAIN,
    ALTER_EVENT_TRIGGER,
    ALTER_EXTENSION,
    ALTER_FOREIGN_DATA_WRAPPER,
    ALTER_FOREIGN_TABLE,
    ALTER_FUNCTION,
    ALTER_GROUP,
    ALTER_INDEX,
    ALTER_LANGUAGE,
    ALTER_LARGE_OBJECT,
    ALTER_MATERIALIZED_VIEW,
    ALTER_OPERATOR,
    ALTER_OPERATOR_CLASS,
    ALTER_OPERATOR_FAMILY,
    ALTER_POLICY,
    ALTER_PROCEDURE,
    ALTER_PUBLICATION,
    ALTER_ROLE,
    ALTER_ROUTINE,
    ALTER_RULE,
    ALTER_SCHEMA,
    ALTER_SEQUENCE,
    ALTER_SERVER,
    ALTER_STATISTICS,
    ALTER_SUBSCRIPTION,
    ALTER_SYSTEM,
    ALTER_TABLE,
    ALTER_TABLESPACE,
    ALTER_TEXT_SEARCH_CONFIGURATION,
    ALTER_TEXT_SEARCH_DICTIONARY,
    ALTER_TEXT_SEARCH_PARSER,
    ALTER_TEXT_SEARCH_TEMPLATE,
    ALTER_TRIGGER,
    ALTER_TYPE,
    ALTER_USER,
    ALTER_USER_MAPPING,
    ALTER_VIEW,
    ANALYZE,
    ARG,
    ARG_LIST,
    ARRAY_EXPR,
    ARRAY_TYPE,
    AS_FUNC_OPTION,
    ATTACH_PARTITION,
    ATTRIBUTE_LIST,
    ATTRIBUTE_OPTION,
    AT_TIME_ZONE,
    BEGIN,
    BEGIN_FUNC_OPTION,
    BETWEEN_EXPR,
    BIN_EXPR,
    BIT_TYPE,
    CALL,
    CALL_EXPR,
    CASCADE,
    CASE_EXPR,
    CAST_EXPR,
    CHAR_TYPE,
    CHECKPOINT,
    CHECK_CONSTRAINT,
    CLOSE,
    CLUSTER,
    CLUSTER_ON,
    COLLATE,
    COLON_COLON,
    COLON_EQ,
    COLUMN,
    COLUMN_LIST,
    COMMENT_ON,
    COMMIT,
    COMPOUND_SELECT,
    COMPRESSION_METHOD,
    CONSTRAINT_EXCLUSIONS,
    CONSTRAINT_INCLUDE_CLAUSE,
    CONSTRAINT_INDEX_METHOD,
    CONSTRAINT_INDEX_TABLESPACE,
    CONSTRAINT_WHERE_CLAUSE,
    COPY,
    COST_FUNC_OPTION,
    CREATE_ACCESS_METHOD,
    CREATE_AGGREGATE,
    CREATE_CAST,
    CREATE_COLLATION,
    CREATE_CONVERSION,
    CREATE_DATABASE,
    CREATE_DOMAIN,
    CREATE_EVENT_TRIGGER,
    CREATE_EXTENSION,
    CREATE_FOREIGN_DATA_WRAPPER,
    CREATE_FOREIGN_TABLE,
    CREATE_FUNCTION,
    CREATE_GROUP,
    CREATE_INDEX,
    CREATE_LANGUAGE,
    CREATE_MATERIALIZED_VIEW,
    CREATE_OPERATOR,
    CREATE_OPERATOR_CLASS,
    CREATE_OPERATOR_FAMILY,
    CREATE_POLICY,
    CREATE_PROCEDURE,
    CREATE_PUBLICATION,
    CREATE_ROLE,
    CREATE_RULE,
    CREATE_SCHEMA,
    CREATE_SEQUENCE,
    CREATE_SERVER,
    CREATE_STATISTICS,
    CREATE_SUBSCRIPTION,
    CREATE_TABLE,
    CREATE_TABLESPACE,
    CREATE_TABLE_AS,
    CREATE_TEXT_SEARCH_CONFIGURATION,
    CREATE_TEXT_SEARCH_DICTIONARY,
    CREATE_TEXT_SEARCH_PARSER,
    CREATE_TEXT_SEARCH_TEMPLATE,
    CREATE_TRANSFORM,
    CREATE_TRIGGER,
    CREATE_TYPE,
    CREATE_USER,
    CREATE_USER_MAPPING,
    CREATE_VIEW,
    CUSTOM_OP,
    DEALLOCATE,
    DECLARE,
    DEFAULT_CONSTRAINT,
    DEFERRABLE,
    DEFERRABLE_CONSTRAINT_OPTION,
    DELETE,
    DELETE_ROWS,
    DETACH_PARTITION,
    DISABLE_RLS,
    DISABLE_RULE,
    DISABLE_TRIGGER,
    DISCARD,
    DISTINCT_CLAUSE,
    DO,
    DOUBLE_TYPE,
    DROP,
    DROP_ACCESS_METHOD,
    DROP_AGGREGATE,
    DROP_CAST,
    DROP_COLLATION,
    DROP_COLUMN,
    DROP_CONSTRAINT,
    DROP_CONVERSION,
    DROP_DATABASE,
    DROP_DEFAULT,
    DROP_DOMAIN,
    DROP_EVENT_TRIGGER,
    DROP_EXPRESSION,
    DROP_EXTENSION,
    DROP_FOREIGN_DATA_WRAPPER,
    DROP_FOREIGN_TABLE,
    DROP_FUNCTION,
    DROP_GROUP,
    DROP_IDENTITY,
    DROP_INDEX,
    DROP_LANGUAGE,
    DROP_MATERIALIZED_VIEW,
    DROP_NOT_NULL,
    DROP_OPERATOR,
    DROP_OPERATOR_CLASS,
    DROP_OPERATOR_FAMILY,
    DROP_OWNED,
    DROP_POLICY,
    DROP_PROCEDURE,
    DROP_PUBLICATION,
    DROP_ROLE,
    DROP_ROUTINE,
    DROP_RULE,
    DROP_SCHEMA,
    DROP_SEQUENCE,
    DROP_SERVER,
    DROP_STATISTICS,
    DROP_SUBSCRIPTION,
    DROP_TABLE,
    DROP_TABLESPACE,
    DROP_TEXT_SEARCH_CONFIG,
    DROP_TEXT_SEARCH_DICT,
    DROP_TEXT_SEARCH_PARSER,
    DROP_TEXT_SEARCH_TEMPLATE,
    DROP_TRANSFORM,
    DROP_TRIGGER,
    DROP_TYPE,
    DROP_USER,
    DROP_USER_MAPPING,
    DROP_VIEW,
    ENABLE_ALWAYS_RULE,
    ENABLE_ALWAYS_TRIGGER,
    ENABLE_REPLICA_RULE,
    ENABLE_REPLICA_TRIGGER,
    ENABLE_RLS,
    ENABLE_RULE,
    ENABLE_TRIGGER,
    ENFORCED,
    EXCLUDE_CONSTRAINT,
    EXECUTE,
    EXPLAIN,
    FAT_ARROW,
    FETCH,
    FETCH_CLAUSE,
    FIELD_EXPR,
    FILTER_CLAUSE,
    FORCE_RLS,
    FOREIGN_KEY_CONSTRAINT,
    FRAME_CLAUSE,
    FROM_CLAUSE,
    FROM_ITEM,
    FUNC_OPTION_LIST,
    GENERATED_CONSTRAINT,
    GRANT,
    GROUPING_CUBE,
    GROUPING_EXPR,
    GROUPING_ROLLUP,
    GROUPING_SETS,
    GROUP_BY_CLAUSE,
    GTEQ,
    HAVING_CLAUSE,
    IF_EXISTS,
    IF_NOT_EXISTS,
    IMPORT_FOREIGN_SCHEMA,
    INDEX_EXPR,
    INDEX_PARAMS,
    INHERIT,
    INHERITS,
    INITIALLY_DEFERRED_CONSTRAINT_OPTION,
    INITIALLY_IMMEDIATE_CONSTRAINT_OPTION,
    INSERT,
    INTERVAL_TYPE,
    INTO_CLAUSE,
    IS_DISTINCT_FROM,
    IS_JSON,
    IS_JSON_ARRAY,
    IS_JSON_OBJECT,
    IS_JSON_SCALAR,
    IS_JSON_VALUE,
    IS_NORMALIZED,
    IS_NOT,
    IS_NOT_DISTINCT_FROM,
    IS_NOT_JSON,
    IS_NOT_JSON_ARRAY,
    IS_NOT_JSON_OBJECT,
    IS_NOT_JSON_SCALAR,
    IS_NOT_JSON_VALUE,
    IS_NOT_NORMALIZED,
    JOIN,
    JOIN_CROSS,
    JOIN_EXPR,
    JOIN_FULL,
    JOIN_INNER,
    JOIN_LEFT,
    JOIN_RIGHT,
    JOIN_USING_CLAUSE,
    JSON_BEHAVIOR_DEFAULT,
    JSON_BEHAVIOR_EMPTY_ARRAY,
    JSON_BEHAVIOR_EMPTY_OBJECT,
    JSON_BEHAVIOR_ERROR,
    JSON_BEHAVIOR_FALSE,
    JSON_BEHAVIOR_NULL,
    JSON_BEHAVIOR_TRUE,
    JSON_BEHAVIOR_UNKNOWN,
    JSON_FORMAT_CLAUSE,
    JSON_KEYS_UNIQUE_CLAUSE,
    JSON_KEY_VALUE,
    JSON_NULL_CLAUSE,
    JSON_ON_EMPTY_CLAUSE,
    JSON_ON_ERROR_CLAUSE,
    JSON_PASSING_ARG,
    JSON_PASSING_CLAUSE,
    JSON_QUOTES_CLAUSE,
    JSON_RETURNING_CLAUSE,
    JSON_TABLE_COLUMN,
    JSON_TABLE_COLUMN_LIST,
    JSON_VALUE_EXPR,
    JSON_WRAPPER_BEHAVIOR_CLAUSE,
    LANGUAGE_FUNC_OPTION,
    LEAKPROOF_FUNC_OPTION,
    LIKE_CLAUSE,
    LIMIT_CLAUSE,
    LISTEN,
    LITERAL,
    LOAD,
    LOCK,
    LOCKING_CLAUSE,
    LTEQ,
    MATCH_FULL,
    MATCH_PARTIAL,
    MATCH_SIMPLE,
    MATERIALIZED,
    MERGE,
    MOVE,
    NAME,
    NAMED_ARG,
    NAME_REF,
    NEQ,
    NEQB,
    NON_STANDARD_PARAM,
    NOTIFY,
    NOT_DEFERRABLE,
    NOT_DEFERRABLE_CONSTRAINT_OPTION,
    NOT_ENFORCED,
    NOT_ILIKE,
    NOT_IN,
    NOT_LIKE,
    NOT_MATERIALIZED,
    NOT_NULL_CONSTRAINT,
    NOT_OF,
    NOT_SIMILAR_TO,
    NOT_VALID,
    NO_ACTION,
    NO_FORCE_RLS,
    NO_INHERIT,
    NULLS_DISTINCT,
    NULLS_FIRST,
    NULLS_LAST,
    NULLS_NOT_DISTINCT,
    NULL_CONSTRAINT,
    OFFSET_CLAUSE,
    OF_TYPE,
    ON_CLAUSE,
    ON_COMMIT,
    ON_DELETE_ACTION,
    ON_UPDATE_ACTION,
    OP,
    OPERATOR_CALL,
    OPTIONS_LIST,
    ORDER_BY_CLAUSE,
    OR_REPLACE,
    OVER_CLAUSE,
    OWNER_TO,
    PARALLEL_FUNC_OPTION,
    PARAM,
    PARAM_DEFAULT,
    PARAM_IN,
    PARAM_IN_OUT,
    PARAM_LIST,
    PARAM_OUT,
    PARAM_VARIADIC,
    PAREN_EXPR,
    PAREN_SELECT,
    PARTITION_BY,
    PARTITION_DEFAULT,
    PARTITION_FOR_VALUES_FROM,
    PARTITION_FOR_VALUES_IN,
    PARTITION_FOR_VALUES_WITH,
    PARTITION_ITEM,
    PARTITION_OF,
    PATH,
    PATH_SEGMENT,
    PATH_TYPE,
    PERCENT_TYPE,
    PERCENT_TYPE_CLAUSE,
    POSTFIX_EXPR,
    PREFIX_EXPR,
    PREPARE,
    PREPARE_TRANSACTION,
    PRESERVE_ROWS,
    PRIMARY_KEY_CONSTRAINT,
    READ_COMMITTED,
    READ_ONLY,
    READ_UNCOMMITTED,
    READ_WRITE,
    REASSIGN,
    REFERENCES_CONSTRAINT,
    REFRESH,
    REINDEX,
    RELATION_NAME,
    RELEASE_SAVEPOINT,
    RENAME_COLUMN,
    RENAME_CONSTRAINT,
    RENAME_TABLE,
    RENAME_TO,
    REPEATABLE_READ,
    REPLICA_IDENTITY,
    RESET,
    RESET_FUNC_OPTION,
    RESET_OPTIONS,
    RESET_STORAGE_PARAMS,
    RESTART,
    RESTRICT,
    RETURNING_CLAUSE,
    RETURN_FUNC_OPTION,
    RET_TYPE,
    REVOKE,
    ROLE,
    ROLLBACK,
    ROWS_FUNC_OPTION,
    SAVEPOINT,
    SECURITY_FUNC_OPTION,
    SECURITY_LABEL,
    SELECT,
    SELECT_CLAUSE,
    SELECT_INTO,
    SEQUENCE_OPTION_LIST,
    SERIALIZABLE,
    SET,
    SET_ACCESS_METHOD,
    SET_COMPRESSION,
    SET_CONSTRAINTS,
    SET_DEFAULT,
    SET_DEFAULT_COLUMNS,
    SET_EXPRESSION,
    SET_FUNC_OPTION,
    SET_GENERATED,
    SET_GENERATED_OPTIONS,
    SET_LOGGED,
    SET_NOT_NULL,
    SET_NULL_COLUMNS,
    SET_OPTIONS,
    SET_OPTIONS_LIST,
    SET_ROLE,
    SET_SCHEMA,
    SET_SEQUENCE_OPTION,
    SET_SESSION_AUTH,
    SET_STATISTICS,
    SET_STORAGE,
    SET_STORAGE_PARAMS,
    SET_TABLESPACE,
    SET_TRANSACTION,
    SET_TYPE,
    SET_UNLOGGED,
    SET_WITHOUT_CLUSTER,
    SET_WITHOUT_OIDS,
    SHOW,
    SIMILAR_TO,
    SORT_ASC,
    SORT_BY,
    SORT_DESC,
    SORT_USING,
    SOURCE_FILE,
    STORAGE,
    STRICT_FUNC_OPTION,
    SUPPORT_FUNC_OPTION,
    TABLE,
    TABLESPACE,
    TABLE_ARG_LIST,
    TABLE_LIST,
    TARGET,
    TARGET_LIST,
    TIME_TYPE,
    TRANSACTION_MODE_LIST,
    TRANSFORM_FUNC_OPTION,
    TRUNCATE,
    TUPLE_EXPR,
    UNICODE_NORMAL_FORM,
    UNIQUE_CONSTRAINT,
    UNLISTEN,
    UPDATE,
    USING_CLAUSE,
    USING_INDEX,
    USING_METHOD,
    VACUUM,
    VALIDATE_CONSTRAINT,
    VALUES,
    VOLATILITY_FUNC_OPTION,
    WHEN_CLAUSE,
    WHERE_CLAUSE,
    WINDOW_CLAUSE,
    WINDOW_DEF,
    WINDOW_FUNC_OPTION,
    WINDOW_SPEC,
    WITHIN_CLAUSE,
    WITHOUT_OIDS,
    WITHOUT_TIMEZONE,
    WITH_CLAUSE,
    WITH_DATA,
    WITH_NO_DATA,
    WITH_OPTIONS,
    WITH_PARAMS,
    WITH_TABLE,
    WITH_TIMEZONE,
    XML_COLUMN_OPTION,
    XML_COLUMN_OPTION_LIST,
    XML_TABLE_COLUMN,
    XML_TABLE_COLUMN_LIST,

    #[doc(hidden)]
    __LAST,
}

impl SyntaxKind {
    pub(crate) fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = if ident.eq_ignore_ascii_case("abort") {
            SyntaxKind::ABORT_KW
        } else if ident.eq_ignore_ascii_case("absent") {
            SyntaxKind::ABSENT_KW
        } else if ident.eq_ignore_ascii_case("absolute") {
            SyntaxKind::ABSOLUTE_KW
        } else if ident.eq_ignore_ascii_case("access") {
            SyntaxKind::ACCESS_KW
        } else if ident.eq_ignore_ascii_case("action") {
            SyntaxKind::ACTION_KW
        } else if ident.eq_ignore_ascii_case("add") {
            SyntaxKind::ADD_KW
        } else if ident.eq_ignore_ascii_case("admin") {
            SyntaxKind::ADMIN_KW
        } else if ident.eq_ignore_ascii_case("after") {
            SyntaxKind::AFTER_KW
        } else if ident.eq_ignore_ascii_case("aggregate") {
            SyntaxKind::AGGREGATE_KW
        } else if ident.eq_ignore_ascii_case("all") {
            SyntaxKind::ALL_KW
        } else if ident.eq_ignore_ascii_case("also") {
            SyntaxKind::ALSO_KW
        } else if ident.eq_ignore_ascii_case("alter") {
            SyntaxKind::ALTER_KW
        } else if ident.eq_ignore_ascii_case("always") {
            SyntaxKind::ALWAYS_KW
        } else if ident.eq_ignore_ascii_case("analyse") {
            SyntaxKind::ANALYSE_KW
        } else if ident.eq_ignore_ascii_case("analyze") {
            SyntaxKind::ANALYZE_KW
        } else if ident.eq_ignore_ascii_case("and") {
            SyntaxKind::AND_KW
        } else if ident.eq_ignore_ascii_case("any") {
            SyntaxKind::ANY_KW
        } else if ident.eq_ignore_ascii_case("array") {
            SyntaxKind::ARRAY_KW
        } else if ident.eq_ignore_ascii_case("as") {
            SyntaxKind::AS_KW
        } else if ident.eq_ignore_ascii_case("asc") {
            SyntaxKind::ASC_KW
        } else if ident.eq_ignore_ascii_case("asensitive") {
            SyntaxKind::ASENSITIVE_KW
        } else if ident.eq_ignore_ascii_case("assertion") {
            SyntaxKind::ASSERTION_KW
        } else if ident.eq_ignore_ascii_case("assignment") {
            SyntaxKind::ASSIGNMENT_KW
        } else if ident.eq_ignore_ascii_case("asymmetric") {
            SyntaxKind::ASYMMETRIC_KW
        } else if ident.eq_ignore_ascii_case("at") {
            SyntaxKind::AT_KW
        } else if ident.eq_ignore_ascii_case("atomic") {
            SyntaxKind::ATOMIC_KW
        } else if ident.eq_ignore_ascii_case("attach") {
            SyntaxKind::ATTACH_KW
        } else if ident.eq_ignore_ascii_case("attribute") {
            SyntaxKind::ATTRIBUTE_KW
        } else if ident.eq_ignore_ascii_case("authorization") {
            SyntaxKind::AUTHORIZATION_KW
        } else if ident.eq_ignore_ascii_case("backward") {
            SyntaxKind::BACKWARD_KW
        } else if ident.eq_ignore_ascii_case("before") {
            SyntaxKind::BEFORE_KW
        } else if ident.eq_ignore_ascii_case("begin") {
            SyntaxKind::BEGIN_KW
        } else if ident.eq_ignore_ascii_case("between") {
            SyntaxKind::BETWEEN_KW
        } else if ident.eq_ignore_ascii_case("bigint") {
            SyntaxKind::BIGINT_KW
        } else if ident.eq_ignore_ascii_case("binary") {
            SyntaxKind::BINARY_KW
        } else if ident.eq_ignore_ascii_case("bit") {
            SyntaxKind::BIT_KW
        } else if ident.eq_ignore_ascii_case("boolean") {
            SyntaxKind::BOOLEAN_KW
        } else if ident.eq_ignore_ascii_case("both") {
            SyntaxKind::BOTH_KW
        } else if ident.eq_ignore_ascii_case("breadth") {
            SyntaxKind::BREADTH_KW
        } else if ident.eq_ignore_ascii_case("by") {
            SyntaxKind::BY_KW
        } else if ident.eq_ignore_ascii_case("cache") {
            SyntaxKind::CACHE_KW
        } else if ident.eq_ignore_ascii_case("call") {
            SyntaxKind::CALL_KW
        } else if ident.eq_ignore_ascii_case("called") {
            SyntaxKind::CALLED_KW
        } else if ident.eq_ignore_ascii_case("cascade") {
            SyntaxKind::CASCADE_KW
        } else if ident.eq_ignore_ascii_case("cascaded") {
            SyntaxKind::CASCADED_KW
        } else if ident.eq_ignore_ascii_case("case") {
            SyntaxKind::CASE_KW
        } else if ident.eq_ignore_ascii_case("cast") {
            SyntaxKind::CAST_KW
        } else if ident.eq_ignore_ascii_case("catalog") {
            SyntaxKind::CATALOG_KW
        } else if ident.eq_ignore_ascii_case("chain") {
            SyntaxKind::CHAIN_KW
        } else if ident.eq_ignore_ascii_case("char") {
            SyntaxKind::CHAR_KW
        } else if ident.eq_ignore_ascii_case("character") {
            SyntaxKind::CHARACTER_KW
        } else if ident.eq_ignore_ascii_case("characteristics") {
            SyntaxKind::CHARACTERISTICS_KW
        } else if ident.eq_ignore_ascii_case("check") {
            SyntaxKind::CHECK_KW
        } else if ident.eq_ignore_ascii_case("checkpoint") {
            SyntaxKind::CHECKPOINT_KW
        } else if ident.eq_ignore_ascii_case("class") {
            SyntaxKind::CLASS_KW
        } else if ident.eq_ignore_ascii_case("close") {
            SyntaxKind::CLOSE_KW
        } else if ident.eq_ignore_ascii_case("cluster") {
            SyntaxKind::CLUSTER_KW
        } else if ident.eq_ignore_ascii_case("coalesce") {
            SyntaxKind::COALESCE_KW
        } else if ident.eq_ignore_ascii_case("collate") {
            SyntaxKind::COLLATE_KW
        } else if ident.eq_ignore_ascii_case("collation") {
            SyntaxKind::COLLATION_KW
        } else if ident.eq_ignore_ascii_case("column") {
            SyntaxKind::COLUMN_KW
        } else if ident.eq_ignore_ascii_case("columns") {
            SyntaxKind::COLUMNS_KW
        } else if ident.eq_ignore_ascii_case("comment") {
            SyntaxKind::COMMENT_KW
        } else if ident.eq_ignore_ascii_case("comments") {
            SyntaxKind::COMMENTS_KW
        } else if ident.eq_ignore_ascii_case("commit") {
            SyntaxKind::COMMIT_KW
        } else if ident.eq_ignore_ascii_case("committed") {
            SyntaxKind::COMMITTED_KW
        } else if ident.eq_ignore_ascii_case("compression") {
            SyntaxKind::COMPRESSION_KW
        } else if ident.eq_ignore_ascii_case("concurrently") {
            SyntaxKind::CONCURRENTLY_KW
        } else if ident.eq_ignore_ascii_case("conditional") {
            SyntaxKind::CONDITIONAL_KW
        } else if ident.eq_ignore_ascii_case("configuration") {
            SyntaxKind::CONFIGURATION_KW
        } else if ident.eq_ignore_ascii_case("conflict") {
            SyntaxKind::CONFLICT_KW
        } else if ident.eq_ignore_ascii_case("connection") {
            SyntaxKind::CONNECTION_KW
        } else if ident.eq_ignore_ascii_case("constraint") {
            SyntaxKind::CONSTRAINT_KW
        } else if ident.eq_ignore_ascii_case("constraints") {
            SyntaxKind::CONSTRAINTS_KW
        } else if ident.eq_ignore_ascii_case("content") {
            SyntaxKind::CONTENT_KW
        } else if ident.eq_ignore_ascii_case("continue") {
            SyntaxKind::CONTINUE_KW
        } else if ident.eq_ignore_ascii_case("conversion") {
            SyntaxKind::CONVERSION_KW
        } else if ident.eq_ignore_ascii_case("copy") {
            SyntaxKind::COPY_KW
        } else if ident.eq_ignore_ascii_case("cost") {
            SyntaxKind::COST_KW
        } else if ident.eq_ignore_ascii_case("create") {
            SyntaxKind::CREATE_KW
        } else if ident.eq_ignore_ascii_case("cross") {
            SyntaxKind::CROSS_KW
        } else if ident.eq_ignore_ascii_case("csv") {
            SyntaxKind::CSV_KW
        } else if ident.eq_ignore_ascii_case("cube") {
            SyntaxKind::CUBE_KW
        } else if ident.eq_ignore_ascii_case("current") {
            SyntaxKind::CURRENT_KW
        } else if ident.eq_ignore_ascii_case("current_catalog") {
            SyntaxKind::CURRENT_CATALOG_KW
        } else if ident.eq_ignore_ascii_case("current_date") {
            SyntaxKind::CURRENT_DATE_KW
        } else if ident.eq_ignore_ascii_case("current_role") {
            SyntaxKind::CURRENT_ROLE_KW
        } else if ident.eq_ignore_ascii_case("current_schema") {
            SyntaxKind::CURRENT_SCHEMA_KW
        } else if ident.eq_ignore_ascii_case("current_time") {
            SyntaxKind::CURRENT_TIME_KW
        } else if ident.eq_ignore_ascii_case("current_timestamp") {
            SyntaxKind::CURRENT_TIMESTAMP_KW
        } else if ident.eq_ignore_ascii_case("current_user") {
            SyntaxKind::CURRENT_USER_KW
        } else if ident.eq_ignore_ascii_case("cursor") {
            SyntaxKind::CURSOR_KW
        } else if ident.eq_ignore_ascii_case("cycle") {
            SyntaxKind::CYCLE_KW
        } else if ident.eq_ignore_ascii_case("data") {
            SyntaxKind::DATA_KW
        } else if ident.eq_ignore_ascii_case("database") {
            SyntaxKind::DATABASE_KW
        } else if ident.eq_ignore_ascii_case("day") {
            SyntaxKind::DAY_KW
        } else if ident.eq_ignore_ascii_case("deallocate") {
            SyntaxKind::DEALLOCATE_KW
        } else if ident.eq_ignore_ascii_case("dec") {
            SyntaxKind::DEC_KW
        } else if ident.eq_ignore_ascii_case("decimal") {
            SyntaxKind::DECIMAL_KW
        } else if ident.eq_ignore_ascii_case("declare") {
            SyntaxKind::DECLARE_KW
        } else if ident.eq_ignore_ascii_case("default") {
            SyntaxKind::DEFAULT_KW
        } else if ident.eq_ignore_ascii_case("defaults") {
            SyntaxKind::DEFAULTS_KW
        } else if ident.eq_ignore_ascii_case("deferrable") {
            SyntaxKind::DEFERRABLE_KW
        } else if ident.eq_ignore_ascii_case("deferred") {
            SyntaxKind::DEFERRED_KW
        } else if ident.eq_ignore_ascii_case("definer") {
            SyntaxKind::DEFINER_KW
        } else if ident.eq_ignore_ascii_case("delete") {
            SyntaxKind::DELETE_KW
        } else if ident.eq_ignore_ascii_case("delimiter") {
            SyntaxKind::DELIMITER_KW
        } else if ident.eq_ignore_ascii_case("delimiters") {
            SyntaxKind::DELIMITERS_KW
        } else if ident.eq_ignore_ascii_case("depends") {
            SyntaxKind::DEPENDS_KW
        } else if ident.eq_ignore_ascii_case("depth") {
            SyntaxKind::DEPTH_KW
        } else if ident.eq_ignore_ascii_case("desc") {
            SyntaxKind::DESC_KW
        } else if ident.eq_ignore_ascii_case("detach") {
            SyntaxKind::DETACH_KW
        } else if ident.eq_ignore_ascii_case("dictionary") {
            SyntaxKind::DICTIONARY_KW
        } else if ident.eq_ignore_ascii_case("disable") {
            SyntaxKind::DISABLE_KW
        } else if ident.eq_ignore_ascii_case("discard") {
            SyntaxKind::DISCARD_KW
        } else if ident.eq_ignore_ascii_case("distinct") {
            SyntaxKind::DISTINCT_KW
        } else if ident.eq_ignore_ascii_case("do") {
            SyntaxKind::DO_KW
        } else if ident.eq_ignore_ascii_case("document") {
            SyntaxKind::DOCUMENT_KW
        } else if ident.eq_ignore_ascii_case("domain") {
            SyntaxKind::DOMAIN_KW
        } else if ident.eq_ignore_ascii_case("double") {
            SyntaxKind::DOUBLE_KW
        } else if ident.eq_ignore_ascii_case("drop") {
            SyntaxKind::DROP_KW
        } else if ident.eq_ignore_ascii_case("each") {
            SyntaxKind::EACH_KW
        } else if ident.eq_ignore_ascii_case("else") {
            SyntaxKind::ELSE_KW
        } else if ident.eq_ignore_ascii_case("empty") {
            SyntaxKind::EMPTY_KW
        } else if ident.eq_ignore_ascii_case("enable") {
            SyntaxKind::ENABLE_KW
        } else if ident.eq_ignore_ascii_case("encoding") {
            SyntaxKind::ENCODING_KW
        } else if ident.eq_ignore_ascii_case("encrypted") {
            SyntaxKind::ENCRYPTED_KW
        } else if ident.eq_ignore_ascii_case("end") {
            SyntaxKind::END_KW
        } else if ident.eq_ignore_ascii_case("enforced") {
            SyntaxKind::ENFORCED_KW
        } else if ident.eq_ignore_ascii_case("enum") {
            SyntaxKind::ENUM_KW
        } else if ident.eq_ignore_ascii_case("error") {
            SyntaxKind::ERROR_KW
        } else if ident.eq_ignore_ascii_case("escape") {
            SyntaxKind::ESCAPE_KW
        } else if ident.eq_ignore_ascii_case("event") {
            SyntaxKind::EVENT_KW
        } else if ident.eq_ignore_ascii_case("except") {
            SyntaxKind::EXCEPT_KW
        } else if ident.eq_ignore_ascii_case("exclude") {
            SyntaxKind::EXCLUDE_KW
        } else if ident.eq_ignore_ascii_case("excluding") {
            SyntaxKind::EXCLUDING_KW
        } else if ident.eq_ignore_ascii_case("exclusive") {
            SyntaxKind::EXCLUSIVE_KW
        } else if ident.eq_ignore_ascii_case("execute") {
            SyntaxKind::EXECUTE_KW
        } else if ident.eq_ignore_ascii_case("exists") {
            SyntaxKind::EXISTS_KW
        } else if ident.eq_ignore_ascii_case("explain") {
            SyntaxKind::EXPLAIN_KW
        } else if ident.eq_ignore_ascii_case("expression") {
            SyntaxKind::EXPRESSION_KW
        } else if ident.eq_ignore_ascii_case("extension") {
            SyntaxKind::EXTENSION_KW
        } else if ident.eq_ignore_ascii_case("external") {
            SyntaxKind::EXTERNAL_KW
        } else if ident.eq_ignore_ascii_case("extract") {
            SyntaxKind::EXTRACT_KW
        } else if ident.eq_ignore_ascii_case("false") {
            SyntaxKind::FALSE_KW
        } else if ident.eq_ignore_ascii_case("family") {
            SyntaxKind::FAMILY_KW
        } else if ident.eq_ignore_ascii_case("fetch") {
            SyntaxKind::FETCH_KW
        } else if ident.eq_ignore_ascii_case("filter") {
            SyntaxKind::FILTER_KW
        } else if ident.eq_ignore_ascii_case("finalize") {
            SyntaxKind::FINALIZE_KW
        } else if ident.eq_ignore_ascii_case("first") {
            SyntaxKind::FIRST_KW
        } else if ident.eq_ignore_ascii_case("float") {
            SyntaxKind::FLOAT_KW
        } else if ident.eq_ignore_ascii_case("following") {
            SyntaxKind::FOLLOWING_KW
        } else if ident.eq_ignore_ascii_case("for") {
            SyntaxKind::FOR_KW
        } else if ident.eq_ignore_ascii_case("force") {
            SyntaxKind::FORCE_KW
        } else if ident.eq_ignore_ascii_case("foreign") {
            SyntaxKind::FOREIGN_KW
        } else if ident.eq_ignore_ascii_case("format") {
            SyntaxKind::FORMAT_KW
        } else if ident.eq_ignore_ascii_case("forward") {
            SyntaxKind::FORWARD_KW
        } else if ident.eq_ignore_ascii_case("freeze") {
            SyntaxKind::FREEZE_KW
        } else if ident.eq_ignore_ascii_case("from") {
            SyntaxKind::FROM_KW
        } else if ident.eq_ignore_ascii_case("full") {
            SyntaxKind::FULL_KW
        } else if ident.eq_ignore_ascii_case("function") {
            SyntaxKind::FUNCTION_KW
        } else if ident.eq_ignore_ascii_case("functions") {
            SyntaxKind::FUNCTIONS_KW
        } else if ident.eq_ignore_ascii_case("generated") {
            SyntaxKind::GENERATED_KW
        } else if ident.eq_ignore_ascii_case("global") {
            SyntaxKind::GLOBAL_KW
        } else if ident.eq_ignore_ascii_case("grant") {
            SyntaxKind::GRANT_KW
        } else if ident.eq_ignore_ascii_case("granted") {
            SyntaxKind::GRANTED_KW
        } else if ident.eq_ignore_ascii_case("greatest") {
            SyntaxKind::GREATEST_KW
        } else if ident.eq_ignore_ascii_case("group") {
            SyntaxKind::GROUP_KW
        } else if ident.eq_ignore_ascii_case("grouping") {
            SyntaxKind::GROUPING_KW
        } else if ident.eq_ignore_ascii_case("groups") {
            SyntaxKind::GROUPS_KW
        } else if ident.eq_ignore_ascii_case("handler") {
            SyntaxKind::HANDLER_KW
        } else if ident.eq_ignore_ascii_case("having") {
            SyntaxKind::HAVING_KW
        } else if ident.eq_ignore_ascii_case("header") {
            SyntaxKind::HEADER_KW
        } else if ident.eq_ignore_ascii_case("hold") {
            SyntaxKind::HOLD_KW
        } else if ident.eq_ignore_ascii_case("hour") {
            SyntaxKind::HOUR_KW
        } else if ident.eq_ignore_ascii_case("identity") {
            SyntaxKind::IDENTITY_KW
        } else if ident.eq_ignore_ascii_case("if") {
            SyntaxKind::IF_KW
        } else if ident.eq_ignore_ascii_case("ilike") {
            SyntaxKind::ILIKE_KW
        } else if ident.eq_ignore_ascii_case("immediate") {
            SyntaxKind::IMMEDIATE_KW
        } else if ident.eq_ignore_ascii_case("immutable") {
            SyntaxKind::IMMUTABLE_KW
        } else if ident.eq_ignore_ascii_case("implicit") {
            SyntaxKind::IMPLICIT_KW
        } else if ident.eq_ignore_ascii_case("import") {
            SyntaxKind::IMPORT_KW
        } else if ident.eq_ignore_ascii_case("in") {
            SyntaxKind::IN_KW
        } else if ident.eq_ignore_ascii_case("include") {
            SyntaxKind::INCLUDE_KW
        } else if ident.eq_ignore_ascii_case("including") {
            SyntaxKind::INCLUDING_KW
        } else if ident.eq_ignore_ascii_case("increment") {
            SyntaxKind::INCREMENT_KW
        } else if ident.eq_ignore_ascii_case("indent") {
            SyntaxKind::INDENT_KW
        } else if ident.eq_ignore_ascii_case("index") {
            SyntaxKind::INDEX_KW
        } else if ident.eq_ignore_ascii_case("indexes") {
            SyntaxKind::INDEXES_KW
        } else if ident.eq_ignore_ascii_case("inherit") {
            SyntaxKind::INHERIT_KW
        } else if ident.eq_ignore_ascii_case("inherits") {
            SyntaxKind::INHERITS_KW
        } else if ident.eq_ignore_ascii_case("initially") {
            SyntaxKind::INITIALLY_KW
        } else if ident.eq_ignore_ascii_case("inline") {
            SyntaxKind::INLINE_KW
        } else if ident.eq_ignore_ascii_case("inner") {
            SyntaxKind::INNER_KW
        } else if ident.eq_ignore_ascii_case("inout") {
            SyntaxKind::INOUT_KW
        } else if ident.eq_ignore_ascii_case("input") {
            SyntaxKind::INPUT_KW
        } else if ident.eq_ignore_ascii_case("insensitive") {
            SyntaxKind::INSENSITIVE_KW
        } else if ident.eq_ignore_ascii_case("insert") {
            SyntaxKind::INSERT_KW
        } else if ident.eq_ignore_ascii_case("instead") {
            SyntaxKind::INSTEAD_KW
        } else if ident.eq_ignore_ascii_case("int") {
            SyntaxKind::INT_KW
        } else if ident.eq_ignore_ascii_case("integer") {
            SyntaxKind::INTEGER_KW
        } else if ident.eq_ignore_ascii_case("intersect") {
            SyntaxKind::INTERSECT_KW
        } else if ident.eq_ignore_ascii_case("interval") {
            SyntaxKind::INTERVAL_KW
        } else if ident.eq_ignore_ascii_case("into") {
            SyntaxKind::INTO_KW
        } else if ident.eq_ignore_ascii_case("invoker") {
            SyntaxKind::INVOKER_KW
        } else if ident.eq_ignore_ascii_case("is") {
            SyntaxKind::IS_KW
        } else if ident.eq_ignore_ascii_case("isnull") {
            SyntaxKind::ISNULL_KW
        } else if ident.eq_ignore_ascii_case("isolation") {
            SyntaxKind::ISOLATION_KW
        } else if ident.eq_ignore_ascii_case("join") {
            SyntaxKind::JOIN_KW
        } else if ident.eq_ignore_ascii_case("json") {
            SyntaxKind::JSON_KW
        } else if ident.eq_ignore_ascii_case("json_array") {
            SyntaxKind::JSON_ARRAY_KW
        } else if ident.eq_ignore_ascii_case("json_arrayagg") {
            SyntaxKind::JSON_ARRAYAGG_KW
        } else if ident.eq_ignore_ascii_case("json_exists") {
            SyntaxKind::JSON_EXISTS_KW
        } else if ident.eq_ignore_ascii_case("json_object") {
            SyntaxKind::JSON_OBJECT_KW
        } else if ident.eq_ignore_ascii_case("json_objectagg") {
            SyntaxKind::JSON_OBJECTAGG_KW
        } else if ident.eq_ignore_ascii_case("json_query") {
            SyntaxKind::JSON_QUERY_KW
        } else if ident.eq_ignore_ascii_case("json_scalar") {
            SyntaxKind::JSON_SCALAR_KW
        } else if ident.eq_ignore_ascii_case("json_serialize") {
            SyntaxKind::JSON_SERIALIZE_KW
        } else if ident.eq_ignore_ascii_case("json_table") {
            SyntaxKind::JSON_TABLE_KW
        } else if ident.eq_ignore_ascii_case("json_value") {
            SyntaxKind::JSON_VALUE_KW
        } else if ident.eq_ignore_ascii_case("keep") {
            SyntaxKind::KEEP_KW
        } else if ident.eq_ignore_ascii_case("key") {
            SyntaxKind::KEY_KW
        } else if ident.eq_ignore_ascii_case("keys") {
            SyntaxKind::KEYS_KW
        } else if ident.eq_ignore_ascii_case("label") {
            SyntaxKind::LABEL_KW
        } else if ident.eq_ignore_ascii_case("language") {
            SyntaxKind::LANGUAGE_KW
        } else if ident.eq_ignore_ascii_case("large") {
            SyntaxKind::LARGE_KW
        } else if ident.eq_ignore_ascii_case("last") {
            SyntaxKind::LAST_KW
        } else if ident.eq_ignore_ascii_case("lateral") {
            SyntaxKind::LATERAL_KW
        } else if ident.eq_ignore_ascii_case("leading") {
            SyntaxKind::LEADING_KW
        } else if ident.eq_ignore_ascii_case("leakproof") {
            SyntaxKind::LEAKPROOF_KW
        } else if ident.eq_ignore_ascii_case("least") {
            SyntaxKind::LEAST_KW
        } else if ident.eq_ignore_ascii_case("left") {
            SyntaxKind::LEFT_KW
        } else if ident.eq_ignore_ascii_case("level") {
            SyntaxKind::LEVEL_KW
        } else if ident.eq_ignore_ascii_case("like") {
            SyntaxKind::LIKE_KW
        } else if ident.eq_ignore_ascii_case("limit") {
            SyntaxKind::LIMIT_KW
        } else if ident.eq_ignore_ascii_case("listen") {
            SyntaxKind::LISTEN_KW
        } else if ident.eq_ignore_ascii_case("load") {
            SyntaxKind::LOAD_KW
        } else if ident.eq_ignore_ascii_case("local") {
            SyntaxKind::LOCAL_KW
        } else if ident.eq_ignore_ascii_case("localtime") {
            SyntaxKind::LOCALTIME_KW
        } else if ident.eq_ignore_ascii_case("localtimestamp") {
            SyntaxKind::LOCALTIMESTAMP_KW
        } else if ident.eq_ignore_ascii_case("location") {
            SyntaxKind::LOCATION_KW
        } else if ident.eq_ignore_ascii_case("lock") {
            SyntaxKind::LOCK_KW
        } else if ident.eq_ignore_ascii_case("locked") {
            SyntaxKind::LOCKED_KW
        } else if ident.eq_ignore_ascii_case("logged") {
            SyntaxKind::LOGGED_KW
        } else if ident.eq_ignore_ascii_case("mapping") {
            SyntaxKind::MAPPING_KW
        } else if ident.eq_ignore_ascii_case("match") {
            SyntaxKind::MATCH_KW
        } else if ident.eq_ignore_ascii_case("matched") {
            SyntaxKind::MATCHED_KW
        } else if ident.eq_ignore_ascii_case("materialized") {
            SyntaxKind::MATERIALIZED_KW
        } else if ident.eq_ignore_ascii_case("maxvalue") {
            SyntaxKind::MAXVALUE_KW
        } else if ident.eq_ignore_ascii_case("merge") {
            SyntaxKind::MERGE_KW
        } else if ident.eq_ignore_ascii_case("merge_action") {
            SyntaxKind::MERGE_ACTION_KW
        } else if ident.eq_ignore_ascii_case("method") {
            SyntaxKind::METHOD_KW
        } else if ident.eq_ignore_ascii_case("minute") {
            SyntaxKind::MINUTE_KW
        } else if ident.eq_ignore_ascii_case("minvalue") {
            SyntaxKind::MINVALUE_KW
        } else if ident.eq_ignore_ascii_case("mode") {
            SyntaxKind::MODE_KW
        } else if ident.eq_ignore_ascii_case("month") {
            SyntaxKind::MONTH_KW
        } else if ident.eq_ignore_ascii_case("move") {
            SyntaxKind::MOVE_KW
        } else if ident.eq_ignore_ascii_case("name") {
            SyntaxKind::NAME_KW
        } else if ident.eq_ignore_ascii_case("names") {
            SyntaxKind::NAMES_KW
        } else if ident.eq_ignore_ascii_case("national") {
            SyntaxKind::NATIONAL_KW
        } else if ident.eq_ignore_ascii_case("natural") {
            SyntaxKind::NATURAL_KW
        } else if ident.eq_ignore_ascii_case("nchar") {
            SyntaxKind::NCHAR_KW
        } else if ident.eq_ignore_ascii_case("nested") {
            SyntaxKind::NESTED_KW
        } else if ident.eq_ignore_ascii_case("new") {
            SyntaxKind::NEW_KW
        } else if ident.eq_ignore_ascii_case("next") {
            SyntaxKind::NEXT_KW
        } else if ident.eq_ignore_ascii_case("nfc") {
            SyntaxKind::NFC_KW
        } else if ident.eq_ignore_ascii_case("nfd") {
            SyntaxKind::NFD_KW
        } else if ident.eq_ignore_ascii_case("nfkc") {
            SyntaxKind::NFKC_KW
        } else if ident.eq_ignore_ascii_case("nfkd") {
            SyntaxKind::NFKD_KW
        } else if ident.eq_ignore_ascii_case("no") {
            SyntaxKind::NO_KW
        } else if ident.eq_ignore_ascii_case("none") {
            SyntaxKind::NONE_KW
        } else if ident.eq_ignore_ascii_case("normalize") {
            SyntaxKind::NORMALIZE_KW
        } else if ident.eq_ignore_ascii_case("normalized") {
            SyntaxKind::NORMALIZED_KW
        } else if ident.eq_ignore_ascii_case("not") {
            SyntaxKind::NOT_KW
        } else if ident.eq_ignore_ascii_case("nothing") {
            SyntaxKind::NOTHING_KW
        } else if ident.eq_ignore_ascii_case("notify") {
            SyntaxKind::NOTIFY_KW
        } else if ident.eq_ignore_ascii_case("notnull") {
            SyntaxKind::NOTNULL_KW
        } else if ident.eq_ignore_ascii_case("nowait") {
            SyntaxKind::NOWAIT_KW
        } else if ident.eq_ignore_ascii_case("null") {
            SyntaxKind::NULL_KW
        } else if ident.eq_ignore_ascii_case("nullif") {
            SyntaxKind::NULLIF_KW
        } else if ident.eq_ignore_ascii_case("nulls") {
            SyntaxKind::NULLS_KW
        } else if ident.eq_ignore_ascii_case("numeric") {
            SyntaxKind::NUMERIC_KW
        } else if ident.eq_ignore_ascii_case("object") {
            SyntaxKind::OBJECT_KW
        } else if ident.eq_ignore_ascii_case("objects") {
            SyntaxKind::OBJECTS_KW
        } else if ident.eq_ignore_ascii_case("of") {
            SyntaxKind::OF_KW
        } else if ident.eq_ignore_ascii_case("off") {
            SyntaxKind::OFF_KW
        } else if ident.eq_ignore_ascii_case("offset") {
            SyntaxKind::OFFSET_KW
        } else if ident.eq_ignore_ascii_case("oids") {
            SyntaxKind::OIDS_KW
        } else if ident.eq_ignore_ascii_case("old") {
            SyntaxKind::OLD_KW
        } else if ident.eq_ignore_ascii_case("omit") {
            SyntaxKind::OMIT_KW
        } else if ident.eq_ignore_ascii_case("on") {
            SyntaxKind::ON_KW
        } else if ident.eq_ignore_ascii_case("only") {
            SyntaxKind::ONLY_KW
        } else if ident.eq_ignore_ascii_case("operator") {
            SyntaxKind::OPERATOR_KW
        } else if ident.eq_ignore_ascii_case("option") {
            SyntaxKind::OPTION_KW
        } else if ident.eq_ignore_ascii_case("options") {
            SyntaxKind::OPTIONS_KW
        } else if ident.eq_ignore_ascii_case("or") {
            SyntaxKind::OR_KW
        } else if ident.eq_ignore_ascii_case("order") {
            SyntaxKind::ORDER_KW
        } else if ident.eq_ignore_ascii_case("ordinality") {
            SyntaxKind::ORDINALITY_KW
        } else if ident.eq_ignore_ascii_case("others") {
            SyntaxKind::OTHERS_KW
        } else if ident.eq_ignore_ascii_case("out") {
            SyntaxKind::OUT_KW
        } else if ident.eq_ignore_ascii_case("outer") {
            SyntaxKind::OUTER_KW
        } else if ident.eq_ignore_ascii_case("over") {
            SyntaxKind::OVER_KW
        } else if ident.eq_ignore_ascii_case("overlaps") {
            SyntaxKind::OVERLAPS_KW
        } else if ident.eq_ignore_ascii_case("overlay") {
            SyntaxKind::OVERLAY_KW
        } else if ident.eq_ignore_ascii_case("overriding") {
            SyntaxKind::OVERRIDING_KW
        } else if ident.eq_ignore_ascii_case("owned") {
            SyntaxKind::OWNED_KW
        } else if ident.eq_ignore_ascii_case("owner") {
            SyntaxKind::OWNER_KW
        } else if ident.eq_ignore_ascii_case("parallel") {
            SyntaxKind::PARALLEL_KW
        } else if ident.eq_ignore_ascii_case("parameter") {
            SyntaxKind::PARAMETER_KW
        } else if ident.eq_ignore_ascii_case("parser") {
            SyntaxKind::PARSER_KW
        } else if ident.eq_ignore_ascii_case("partial") {
            SyntaxKind::PARTIAL_KW
        } else if ident.eq_ignore_ascii_case("partition") {
            SyntaxKind::PARTITION_KW
        } else if ident.eq_ignore_ascii_case("passing") {
            SyntaxKind::PASSING_KW
        } else if ident.eq_ignore_ascii_case("password") {
            SyntaxKind::PASSWORD_KW
        } else if ident.eq_ignore_ascii_case("path") {
            SyntaxKind::PATH_KW
        } else if ident.eq_ignore_ascii_case("period") {
            SyntaxKind::PERIOD_KW
        } else if ident.eq_ignore_ascii_case("placing") {
            SyntaxKind::PLACING_KW
        } else if ident.eq_ignore_ascii_case("plan") {
            SyntaxKind::PLAN_KW
        } else if ident.eq_ignore_ascii_case("plans") {
            SyntaxKind::PLANS_KW
        } else if ident.eq_ignore_ascii_case("policy") {
            SyntaxKind::POLICY_KW
        } else if ident.eq_ignore_ascii_case("position") {
            SyntaxKind::POSITION_KW
        } else if ident.eq_ignore_ascii_case("preceding") {
            SyntaxKind::PRECEDING_KW
        } else if ident.eq_ignore_ascii_case("precision") {
            SyntaxKind::PRECISION_KW
        } else if ident.eq_ignore_ascii_case("prepare") {
            SyntaxKind::PREPARE_KW
        } else if ident.eq_ignore_ascii_case("prepared") {
            SyntaxKind::PREPARED_KW
        } else if ident.eq_ignore_ascii_case("preserve") {
            SyntaxKind::PRESERVE_KW
        } else if ident.eq_ignore_ascii_case("primary") {
            SyntaxKind::PRIMARY_KW
        } else if ident.eq_ignore_ascii_case("prior") {
            SyntaxKind::PRIOR_KW
        } else if ident.eq_ignore_ascii_case("privileges") {
            SyntaxKind::PRIVILEGES_KW
        } else if ident.eq_ignore_ascii_case("procedural") {
            SyntaxKind::PROCEDURAL_KW
        } else if ident.eq_ignore_ascii_case("procedure") {
            SyntaxKind::PROCEDURE_KW
        } else if ident.eq_ignore_ascii_case("procedures") {
            SyntaxKind::PROCEDURES_KW
        } else if ident.eq_ignore_ascii_case("program") {
            SyntaxKind::PROGRAM_KW
        } else if ident.eq_ignore_ascii_case("publication") {
            SyntaxKind::PUBLICATION_KW
        } else if ident.eq_ignore_ascii_case("quote") {
            SyntaxKind::QUOTE_KW
        } else if ident.eq_ignore_ascii_case("quotes") {
            SyntaxKind::QUOTES_KW
        } else if ident.eq_ignore_ascii_case("range") {
            SyntaxKind::RANGE_KW
        } else if ident.eq_ignore_ascii_case("read") {
            SyntaxKind::READ_KW
        } else if ident.eq_ignore_ascii_case("real") {
            SyntaxKind::REAL_KW
        } else if ident.eq_ignore_ascii_case("reassign") {
            SyntaxKind::REASSIGN_KW
        } else if ident.eq_ignore_ascii_case("recursive") {
            SyntaxKind::RECURSIVE_KW
        } else if ident.eq_ignore_ascii_case("ref") {
            SyntaxKind::REF_KW
        } else if ident.eq_ignore_ascii_case("references") {
            SyntaxKind::REFERENCES_KW
        } else if ident.eq_ignore_ascii_case("referencing") {
            SyntaxKind::REFERENCING_KW
        } else if ident.eq_ignore_ascii_case("refresh") {
            SyntaxKind::REFRESH_KW
        } else if ident.eq_ignore_ascii_case("reindex") {
            SyntaxKind::REINDEX_KW
        } else if ident.eq_ignore_ascii_case("relative") {
            SyntaxKind::RELATIVE_KW
        } else if ident.eq_ignore_ascii_case("release") {
            SyntaxKind::RELEASE_KW
        } else if ident.eq_ignore_ascii_case("rename") {
            SyntaxKind::RENAME_KW
        } else if ident.eq_ignore_ascii_case("repeatable") {
            SyntaxKind::REPEATABLE_KW
        } else if ident.eq_ignore_ascii_case("replace") {
            SyntaxKind::REPLACE_KW
        } else if ident.eq_ignore_ascii_case("replica") {
            SyntaxKind::REPLICA_KW
        } else if ident.eq_ignore_ascii_case("reset") {
            SyntaxKind::RESET_KW
        } else if ident.eq_ignore_ascii_case("restart") {
            SyntaxKind::RESTART_KW
        } else if ident.eq_ignore_ascii_case("restrict") {
            SyntaxKind::RESTRICT_KW
        } else if ident.eq_ignore_ascii_case("return") {
            SyntaxKind::RETURN_KW
        } else if ident.eq_ignore_ascii_case("returning") {
            SyntaxKind::RETURNING_KW
        } else if ident.eq_ignore_ascii_case("returns") {
            SyntaxKind::RETURNS_KW
        } else if ident.eq_ignore_ascii_case("revoke") {
            SyntaxKind::REVOKE_KW
        } else if ident.eq_ignore_ascii_case("right") {
            SyntaxKind::RIGHT_KW
        } else if ident.eq_ignore_ascii_case("role") {
            SyntaxKind::ROLE_KW
        } else if ident.eq_ignore_ascii_case("rollback") {
            SyntaxKind::ROLLBACK_KW
        } else if ident.eq_ignore_ascii_case("rollup") {
            SyntaxKind::ROLLUP_KW
        } else if ident.eq_ignore_ascii_case("routine") {
            SyntaxKind::ROUTINE_KW
        } else if ident.eq_ignore_ascii_case("routines") {
            SyntaxKind::ROUTINES_KW
        } else if ident.eq_ignore_ascii_case("row") {
            SyntaxKind::ROW_KW
        } else if ident.eq_ignore_ascii_case("rows") {
            SyntaxKind::ROWS_KW
        } else if ident.eq_ignore_ascii_case("rule") {
            SyntaxKind::RULE_KW
        } else if ident.eq_ignore_ascii_case("savepoint") {
            SyntaxKind::SAVEPOINT_KW
        } else if ident.eq_ignore_ascii_case("scalar") {
            SyntaxKind::SCALAR_KW
        } else if ident.eq_ignore_ascii_case("schema") {
            SyntaxKind::SCHEMA_KW
        } else if ident.eq_ignore_ascii_case("schemas") {
            SyntaxKind::SCHEMAS_KW
        } else if ident.eq_ignore_ascii_case("scroll") {
            SyntaxKind::SCROLL_KW
        } else if ident.eq_ignore_ascii_case("search") {
            SyntaxKind::SEARCH_KW
        } else if ident.eq_ignore_ascii_case("second") {
            SyntaxKind::SECOND_KW
        } else if ident.eq_ignore_ascii_case("security") {
            SyntaxKind::SECURITY_KW
        } else if ident.eq_ignore_ascii_case("select") {
            SyntaxKind::SELECT_KW
        } else if ident.eq_ignore_ascii_case("sequence") {
            SyntaxKind::SEQUENCE_KW
        } else if ident.eq_ignore_ascii_case("sequences") {
            SyntaxKind::SEQUENCES_KW
        } else if ident.eq_ignore_ascii_case("serializable") {
            SyntaxKind::SERIALIZABLE_KW
        } else if ident.eq_ignore_ascii_case("server") {
            SyntaxKind::SERVER_KW
        } else if ident.eq_ignore_ascii_case("session") {
            SyntaxKind::SESSION_KW
        } else if ident.eq_ignore_ascii_case("session_user") {
            SyntaxKind::SESSION_USER_KW
        } else if ident.eq_ignore_ascii_case("set") {
            SyntaxKind::SET_KW
        } else if ident.eq_ignore_ascii_case("setof") {
            SyntaxKind::SETOF_KW
        } else if ident.eq_ignore_ascii_case("sets") {
            SyntaxKind::SETS_KW
        } else if ident.eq_ignore_ascii_case("share") {
            SyntaxKind::SHARE_KW
        } else if ident.eq_ignore_ascii_case("show") {
            SyntaxKind::SHOW_KW
        } else if ident.eq_ignore_ascii_case("similar") {
            SyntaxKind::SIMILAR_KW
        } else if ident.eq_ignore_ascii_case("simple") {
            SyntaxKind::SIMPLE_KW
        } else if ident.eq_ignore_ascii_case("skip") {
            SyntaxKind::SKIP_KW
        } else if ident.eq_ignore_ascii_case("smallint") {
            SyntaxKind::SMALLINT_KW
        } else if ident.eq_ignore_ascii_case("snapshot") {
            SyntaxKind::SNAPSHOT_KW
        } else if ident.eq_ignore_ascii_case("some") {
            SyntaxKind::SOME_KW
        } else if ident.eq_ignore_ascii_case("source") {
            SyntaxKind::SOURCE_KW
        } else if ident.eq_ignore_ascii_case("sql") {
            SyntaxKind::SQL_KW
        } else if ident.eq_ignore_ascii_case("stable") {
            SyntaxKind::STABLE_KW
        } else if ident.eq_ignore_ascii_case("standalone") {
            SyntaxKind::STANDALONE_KW
        } else if ident.eq_ignore_ascii_case("start") {
            SyntaxKind::START_KW
        } else if ident.eq_ignore_ascii_case("statement") {
            SyntaxKind::STATEMENT_KW
        } else if ident.eq_ignore_ascii_case("statistics") {
            SyntaxKind::STATISTICS_KW
        } else if ident.eq_ignore_ascii_case("stdin") {
            SyntaxKind::STDIN_KW
        } else if ident.eq_ignore_ascii_case("stdout") {
            SyntaxKind::STDOUT_KW
        } else if ident.eq_ignore_ascii_case("storage") {
            SyntaxKind::STORAGE_KW
        } else if ident.eq_ignore_ascii_case("stored") {
            SyntaxKind::STORED_KW
        } else if ident.eq_ignore_ascii_case("strict") {
            SyntaxKind::STRICT_KW
        } else if ident.eq_ignore_ascii_case("string") {
            SyntaxKind::STRING_KW
        } else if ident.eq_ignore_ascii_case("strip") {
            SyntaxKind::STRIP_KW
        } else if ident.eq_ignore_ascii_case("subscription") {
            SyntaxKind::SUBSCRIPTION_KW
        } else if ident.eq_ignore_ascii_case("substring") {
            SyntaxKind::SUBSTRING_KW
        } else if ident.eq_ignore_ascii_case("support") {
            SyntaxKind::SUPPORT_KW
        } else if ident.eq_ignore_ascii_case("symmetric") {
            SyntaxKind::SYMMETRIC_KW
        } else if ident.eq_ignore_ascii_case("sysid") {
            SyntaxKind::SYSID_KW
        } else if ident.eq_ignore_ascii_case("system") {
            SyntaxKind::SYSTEM_KW
        } else if ident.eq_ignore_ascii_case("system_user") {
            SyntaxKind::SYSTEM_USER_KW
        } else if ident.eq_ignore_ascii_case("table") {
            SyntaxKind::TABLE_KW
        } else if ident.eq_ignore_ascii_case("tables") {
            SyntaxKind::TABLES_KW
        } else if ident.eq_ignore_ascii_case("tablesample") {
            SyntaxKind::TABLESAMPLE_KW
        } else if ident.eq_ignore_ascii_case("tablespace") {
            SyntaxKind::TABLESPACE_KW
        } else if ident.eq_ignore_ascii_case("target") {
            SyntaxKind::TARGET_KW
        } else if ident.eq_ignore_ascii_case("temp") {
            SyntaxKind::TEMP_KW
        } else if ident.eq_ignore_ascii_case("template") {
            SyntaxKind::TEMPLATE_KW
        } else if ident.eq_ignore_ascii_case("temporary") {
            SyntaxKind::TEMPORARY_KW
        } else if ident.eq_ignore_ascii_case("text") {
            SyntaxKind::TEXT_KW
        } else if ident.eq_ignore_ascii_case("then") {
            SyntaxKind::THEN_KW
        } else if ident.eq_ignore_ascii_case("ties") {
            SyntaxKind::TIES_KW
        } else if ident.eq_ignore_ascii_case("time") {
            SyntaxKind::TIME_KW
        } else if ident.eq_ignore_ascii_case("timestamp") {
            SyntaxKind::TIMESTAMP_KW
        } else if ident.eq_ignore_ascii_case("to") {
            SyntaxKind::TO_KW
        } else if ident.eq_ignore_ascii_case("trailing") {
            SyntaxKind::TRAILING_KW
        } else if ident.eq_ignore_ascii_case("transaction") {
            SyntaxKind::TRANSACTION_KW
        } else if ident.eq_ignore_ascii_case("transform") {
            SyntaxKind::TRANSFORM_KW
        } else if ident.eq_ignore_ascii_case("treat") {
            SyntaxKind::TREAT_KW
        } else if ident.eq_ignore_ascii_case("trigger") {
            SyntaxKind::TRIGGER_KW
        } else if ident.eq_ignore_ascii_case("trim") {
            SyntaxKind::TRIM_KW
        } else if ident.eq_ignore_ascii_case("true") {
            SyntaxKind::TRUE_KW
        } else if ident.eq_ignore_ascii_case("truncate") {
            SyntaxKind::TRUNCATE_KW
        } else if ident.eq_ignore_ascii_case("trusted") {
            SyntaxKind::TRUSTED_KW
        } else if ident.eq_ignore_ascii_case("type") {
            SyntaxKind::TYPE_KW
        } else if ident.eq_ignore_ascii_case("types") {
            SyntaxKind::TYPES_KW
        } else if ident.eq_ignore_ascii_case("uescape") {
            SyntaxKind::UESCAPE_KW
        } else if ident.eq_ignore_ascii_case("unbounded") {
            SyntaxKind::UNBOUNDED_KW
        } else if ident.eq_ignore_ascii_case("uncommitted") {
            SyntaxKind::UNCOMMITTED_KW
        } else if ident.eq_ignore_ascii_case("unconditional") {
            SyntaxKind::UNCONDITIONAL_KW
        } else if ident.eq_ignore_ascii_case("unencrypted") {
            SyntaxKind::UNENCRYPTED_KW
        } else if ident.eq_ignore_ascii_case("union") {
            SyntaxKind::UNION_KW
        } else if ident.eq_ignore_ascii_case("unique") {
            SyntaxKind::UNIQUE_KW
        } else if ident.eq_ignore_ascii_case("unknown") {
            SyntaxKind::UNKNOWN_KW
        } else if ident.eq_ignore_ascii_case("unlisten") {
            SyntaxKind::UNLISTEN_KW
        } else if ident.eq_ignore_ascii_case("unlogged") {
            SyntaxKind::UNLOGGED_KW
        } else if ident.eq_ignore_ascii_case("until") {
            SyntaxKind::UNTIL_KW
        } else if ident.eq_ignore_ascii_case("update") {
            SyntaxKind::UPDATE_KW
        } else if ident.eq_ignore_ascii_case("user") {
            SyntaxKind::USER_KW
        } else if ident.eq_ignore_ascii_case("using") {
            SyntaxKind::USING_KW
        } else if ident.eq_ignore_ascii_case("vacuum") {
            SyntaxKind::VACUUM_KW
        } else if ident.eq_ignore_ascii_case("valid") {
            SyntaxKind::VALID_KW
        } else if ident.eq_ignore_ascii_case("validate") {
            SyntaxKind::VALIDATE_KW
        } else if ident.eq_ignore_ascii_case("validator") {
            SyntaxKind::VALIDATOR_KW
        } else if ident.eq_ignore_ascii_case("value") {
            SyntaxKind::VALUE_KW
        } else if ident.eq_ignore_ascii_case("values") {
            SyntaxKind::VALUES_KW
        } else if ident.eq_ignore_ascii_case("varchar") {
            SyntaxKind::VARCHAR_KW
        } else if ident.eq_ignore_ascii_case("variadic") {
            SyntaxKind::VARIADIC_KW
        } else if ident.eq_ignore_ascii_case("varying") {
            SyntaxKind::VARYING_KW
        } else if ident.eq_ignore_ascii_case("verbose") {
            SyntaxKind::VERBOSE_KW
        } else if ident.eq_ignore_ascii_case("version") {
            SyntaxKind::VERSION_KW
        } else if ident.eq_ignore_ascii_case("view") {
            SyntaxKind::VIEW_KW
        } else if ident.eq_ignore_ascii_case("views") {
            SyntaxKind::VIEWS_KW
        } else if ident.eq_ignore_ascii_case("virtual") {
            SyntaxKind::VIRTUAL_KW
        } else if ident.eq_ignore_ascii_case("volatile") {
            SyntaxKind::VOLATILE_KW
        } else if ident.eq_ignore_ascii_case("when") {
            SyntaxKind::WHEN_KW
        } else if ident.eq_ignore_ascii_case("where") {
            SyntaxKind::WHERE_KW
        } else if ident.eq_ignore_ascii_case("whitespace") {
            SyntaxKind::WHITESPACE_KW
        } else if ident.eq_ignore_ascii_case("window") {
            SyntaxKind::WINDOW_KW
        } else if ident.eq_ignore_ascii_case("with") {
            SyntaxKind::WITH_KW
        } else if ident.eq_ignore_ascii_case("within") {
            SyntaxKind::WITHIN_KW
        } else if ident.eq_ignore_ascii_case("without") {
            SyntaxKind::WITHOUT_KW
        } else if ident.eq_ignore_ascii_case("work") {
            SyntaxKind::WORK_KW
        } else if ident.eq_ignore_ascii_case("wrapper") {
            SyntaxKind::WRAPPER_KW
        } else if ident.eq_ignore_ascii_case("write") {
            SyntaxKind::WRITE_KW
        } else if ident.eq_ignore_ascii_case("xml") {
            SyntaxKind::XML_KW
        } else if ident.eq_ignore_ascii_case("xmlattributes") {
            SyntaxKind::XMLATTRIBUTES_KW
        } else if ident.eq_ignore_ascii_case("xmlconcat") {
            SyntaxKind::XMLCONCAT_KW
        } else if ident.eq_ignore_ascii_case("xmlelement") {
            SyntaxKind::XMLELEMENT_KW
        } else if ident.eq_ignore_ascii_case("xmlexists") {
            SyntaxKind::XMLEXISTS_KW
        } else if ident.eq_ignore_ascii_case("xmlforest") {
            SyntaxKind::XMLFOREST_KW
        } else if ident.eq_ignore_ascii_case("xmlnamespaces") {
            SyntaxKind::XMLNAMESPACES_KW
        } else if ident.eq_ignore_ascii_case("xmlparse") {
            SyntaxKind::XMLPARSE_KW
        } else if ident.eq_ignore_ascii_case("xmlpi") {
            SyntaxKind::XMLPI_KW
        } else if ident.eq_ignore_ascii_case("xmlroot") {
            SyntaxKind::XMLROOT_KW
        } else if ident.eq_ignore_ascii_case("xmlserialize") {
            SyntaxKind::XMLSERIALIZE_KW
        } else if ident.eq_ignore_ascii_case("xmltable") {
            SyntaxKind::XMLTABLE_KW
        } else if ident.eq_ignore_ascii_case("year") {
            SyntaxKind::YEAR_KW
        } else if ident.eq_ignore_ascii_case("yes") {
            SyntaxKind::YES_KW
        } else if ident.eq_ignore_ascii_case("zone") {
            SyntaxKind::ZONE_KW
        } else {
            return None;
        };
        Some(kw)
    }
}
