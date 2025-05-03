// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/syntax_kind/generated.rs
#![allow(bad_style, missing_docs, clippy::upper_case_acronyms)]

use crate::token_set::TokenSet;
#[doc = r" The kind of syntax node, e.g. `IDENT`, `SELECT_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
/// Needs to be compatible with [`rowan::SyntaxKind`]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,

    // symbols
    /// `;`
    SEMICOLON,
    /// `,`
    COMMA,
    /// `(`
    L_PAREN,
    /// `)`
    R_PAREN,
    /// `[`
    L_BRACK,
    /// `]`
    R_BRACK,
    /// `<`
    L_ANGLE,
    /// `>`
    R_ANGLE,
    /// `@`
    AT,
    /// `#`
    POUND,
    /// `~`
    TILDE,
    /// `?`
    QUESTION,
    /// `&`
    AMP,
    /// `|`
    PIPE,
    /// `+`
    PLUS,
    /// `*`
    STAR,
    /// `/`
    SLASH,
    /// `^`
    CARET,
    /// `%`
    PERCENT,
    /// `.`
    DOT,
    /// `:`
    COLON,
    /// `::`
    COLON2,
    /// `:=`
    COLONEQ,
    /// `=`
    EQ,
    /// `=>`
    /// Used for `NAMED_ARG`
    FAT_ARROW,
    /// ```
    /// `
    /// ```
    BACKTICK,
    /// `!`
    BANG,
    /// `!=`
    NEQ,
    /// `<>`
    NEQB,
    /// `-`
    MINUS,
    /// `<=`
    LTEQ,
    /// `>=`
    GTEQ,
    // catch all for custom operators
    CUSTOM_OP,

    // keywords -- generated via: cargo xtrask generate-keywords
    /// `abort`
    ABORT_KW,
    /// `absent`
    ABSENT_KW,
    /// `absolute`
    ABSOLUTE_KW,
    /// `access`
    ACCESS_KW,
    /// `action`
    ACTION_KW,
    /// `add`
    ADD_KW,
    /// `admin`
    ADMIN_KW,
    /// `after`
    AFTER_KW,
    /// `aggregate`
    AGGREGATE_KW,
    /// `all`
    ALL_KW,
    /// `also`
    ALSO_KW,
    /// `alter`
    ALTER_KW,
    /// `always`
    ALWAYS_KW,
    /// `analyse`
    ANALYSE_KW,
    /// `analyze`
    ANALYZE_KW,
    /// `and`
    AND_KW,
    /// `any`
    ANY_KW,
    /// `array`
    ARRAY_KW,
    /// `as`
    AS_KW,
    /// `asc`
    ASC_KW,
    /// `asensitive`
    ASENSITIVE_KW,
    /// `assertion`
    ASSERTION_KW,
    /// `assignment`
    ASSIGNMENT_KW,
    /// `asymmetric`
    ASYMMETRIC_KW,
    /// `at`
    AT_KW,
    /// `atomic`
    ATOMIC_KW,
    /// `attach`
    ATTACH_KW,
    /// `attribute`
    ATTRIBUTE_KW,
    /// `authorization`
    AUTHORIZATION_KW,
    /// `backward`
    BACKWARD_KW,
    /// `before`
    BEFORE_KW,
    /// `begin`
    BEGIN_KW,
    /// `between`
    BETWEEN_KW,
    /// `bigint`
    BIGINT_KW,
    /// `binary`
    BINARY_KW,
    /// `bit`
    BIT_KW,
    /// `boolean`
    BOOLEAN_KW,
    /// `both`
    BOTH_KW,
    /// `breadth`
    BREADTH_KW,
    /// `by`
    BY_KW,
    /// `cache`
    CACHE_KW,
    /// `call`
    CALL_KW,
    /// `called`
    CALLED_KW,
    /// `cascade`
    CASCADE_KW,
    /// `cascaded`
    CASCADED_KW,
    /// `case`
    CASE_KW,
    /// `cast`
    CAST_KW,
    /// `catalog`
    CATALOG_KW,
    /// `chain`
    CHAIN_KW,
    /// `char`
    CHAR_KW,
    /// `character`
    CHARACTER_KW,
    /// `characteristics`
    CHARACTERISTICS_KW,
    /// `check`
    CHECK_KW,
    /// `checkpoint`
    CHECKPOINT_KW,
    /// `class`
    CLASS_KW,
    /// `close`
    CLOSE_KW,
    /// `cluster`
    CLUSTER_KW,
    /// `coalesce`
    COALESCE_KW,
    /// `collate`
    COLLATE_KW,
    /// `collation`
    COLLATION_KW,
    /// `column`
    COLUMN_KW,
    /// `columns`
    COLUMNS_KW,
    /// `comment`
    COMMENT_KW,
    /// `comments`
    COMMENTS_KW,
    /// `commit`
    COMMIT_KW,
    /// `committed`
    COMMITTED_KW,
    /// `compression`
    COMPRESSION_KW,
    /// `concurrently`
    CONCURRENTLY_KW,
    /// `conditional`
    CONDITIONAL_KW,
    /// `configuration`
    CONFIGURATION_KW,
    /// `conflict`
    CONFLICT_KW,
    /// `connection`
    CONNECTION_KW,
    /// `constraint`
    CONSTRAINT_KW,
    /// `constraints`
    CONSTRAINTS_KW,
    /// `content`
    CONTENT_KW,
    /// `continue`
    CONTINUE_KW,
    /// `conversion`
    CONVERSION_KW,
    /// `copy`
    COPY_KW,
    /// `cost`
    COST_KW,
    /// `create`
    CREATE_KW,
    /// `cross`
    CROSS_KW,
    /// `csv`
    CSV_KW,
    /// `cube`
    CUBE_KW,
    /// `current`
    CURRENT_KW,
    /// `current_catalog`
    CURRENT_CATALOG_KW,
    /// `current_date`
    CURRENT_DATE_KW,
    /// `current_role`
    CURRENT_ROLE_KW,
    /// `current_schema`
    CURRENT_SCHEMA_KW,
    /// `current_time`
    CURRENT_TIME_KW,
    /// `current_timestamp`
    CURRENT_TIMESTAMP_KW,
    /// `current_user`
    CURRENT_USER_KW,
    /// `cursor`
    CURSOR_KW,
    /// `cycle`
    CYCLE_KW,
    /// `data`
    DATA_KW,
    /// `database`
    DATABASE_KW,
    /// `day`
    DAY_KW,
    /// `deallocate`
    DEALLOCATE_KW,
    /// `dec`
    DEC_KW,
    /// `decimal`
    DECIMAL_KW,
    /// `declare`
    DECLARE_KW,
    /// `default`
    DEFAULT_KW,
    /// `defaults`
    DEFAULTS_KW,
    /// `deferrable`
    DEFERRABLE_KW,
    /// `deferred`
    DEFERRED_KW,
    /// `definer`
    DEFINER_KW,
    /// `delete`
    DELETE_KW,
    /// `delimiter`
    DELIMITER_KW,
    /// `delimiters`
    DELIMITERS_KW,
    /// `depends`
    DEPENDS_KW,
    /// `depth`
    DEPTH_KW,
    /// `desc`
    DESC_KW,
    /// `detach`
    DETACH_KW,
    /// `dictionary`
    DICTIONARY_KW,
    /// `disable`
    DISABLE_KW,
    /// `discard`
    DISCARD_KW,
    /// `distinct`
    DISTINCT_KW,
    /// `do`
    DO_KW,
    /// `document`
    DOCUMENT_KW,
    /// `domain`
    DOMAIN_KW,
    /// `double`
    DOUBLE_KW,
    /// `drop`
    DROP_KW,
    /// `each`
    EACH_KW,
    /// `else`
    ELSE_KW,
    /// `empty`
    EMPTY_KW,
    /// `enable`
    ENABLE_KW,
    /// `encoding`
    ENCODING_KW,
    /// `encrypted`
    ENCRYPTED_KW,
    /// `end`
    END_KW,
    /// `enum`
    ENUM_KW,
    /// `error`
    ERROR_KW,
    /// `escape`
    ESCAPE_KW,
    /// `event`
    EVENT_KW,
    /// `except`
    EXCEPT_KW,
    /// `exclude`
    EXCLUDE_KW,
    /// `excluding`
    EXCLUDING_KW,
    /// `exclusive`
    EXCLUSIVE_KW,
    /// `execute`
    EXECUTE_KW,
    /// `exists`
    EXISTS_KW,
    /// `explain`
    EXPLAIN_KW,
    /// `expression`
    EXPRESSION_KW,
    /// `extension`
    EXTENSION_KW,
    /// `external`
    EXTERNAL_KW,
    /// `extract`
    EXTRACT_KW,
    /// `false`
    FALSE_KW,
    /// `family`
    FAMILY_KW,
    /// `fetch`
    FETCH_KW,
    /// `filter`
    FILTER_KW,
    /// `finalize`
    FINALIZE_KW,
    /// `first`
    FIRST_KW,
    /// `float`
    FLOAT_KW,
    /// `following`
    FOLLOWING_KW,
    /// `for`
    FOR_KW,
    /// `force`
    FORCE_KW,
    /// `foreign`
    FOREIGN_KW,
    /// `format`
    FORMAT_KW,
    /// `forward`
    FORWARD_KW,
    /// `freeze`
    FREEZE_KW,
    /// `from`
    FROM_KW,
    /// `full`
    FULL_KW,
    /// `function`
    FUNCTION_KW,
    /// `functions`
    FUNCTIONS_KW,
    /// `generated`
    GENERATED_KW,
    /// `global`
    GLOBAL_KW,
    /// `grant`
    GRANT_KW,
    /// `granted`
    GRANTED_KW,
    /// `greatest`
    GREATEST_KW,
    /// `group`
    GROUP_KW,
    /// `grouping`
    GROUPING_KW,
    /// `groups`
    GROUPS_KW,
    /// `handler`
    HANDLER_KW,
    /// `having`
    HAVING_KW,
    /// `header`
    HEADER_KW,
    /// `hold`
    HOLD_KW,
    /// `hour`
    HOUR_KW,
    /// `identity`
    IDENTITY_KW,
    /// `if`
    IF_KW,
    /// `ilike`
    ILIKE_KW,
    /// `immediate`
    IMMEDIATE_KW,
    /// `immutable`
    IMMUTABLE_KW,
    /// `implicit`
    IMPLICIT_KW,
    /// `import`
    IMPORT_KW,
    /// `in`
    IN_KW,
    /// `include`
    INCLUDE_KW,
    /// `including`
    INCLUDING_KW,
    /// `increment`
    INCREMENT_KW,
    /// `indent`
    INDENT_KW,
    /// `index`
    INDEX_KW,
    /// `indexes`
    INDEXES_KW,
    /// `inherit`
    INHERIT_KW,
    /// `inherits`
    INHERITS_KW,
    /// `initially`
    INITIALLY_KW,
    /// `inline`
    INLINE_KW,
    /// `inner`
    INNER_KW,
    /// `inout`
    INOUT_KW,
    /// `input`
    INPUT_KW,
    /// `insensitive`
    INSENSITIVE_KW,
    /// `insert`
    INSERT_KW,
    /// `instead`
    INSTEAD_KW,
    /// `int`
    INT_KW,
    /// `integer`
    INTEGER_KW,
    /// `intersect`
    INTERSECT_KW,
    /// `interval`
    INTERVAL_KW,
    /// `into`
    INTO_KW,
    /// `invoker`
    INVOKER_KW,
    /// `is`
    IS_KW,
    /// `isnull`
    ISNULL_KW,
    /// `isolation`
    ISOLATION_KW,
    /// `join`
    JOIN_KW,
    /// `json`
    JSON_KW,
    /// `json_array`
    JSON_ARRAY_KW,
    /// `json_arrayagg`
    JSON_ARRAYAGG_KW,
    /// `json_exists`
    JSON_EXISTS_KW,
    /// `json_object`
    JSON_OBJECT_KW,
    /// `json_objectagg`
    JSON_OBJECTAGG_KW,
    /// `json_query`
    JSON_QUERY_KW,
    /// `json_scalar`
    JSON_SCALAR_KW,
    /// `json_serialize`
    JSON_SERIALIZE_KW,
    /// `json_table`
    JSON_TABLE_KW,
    /// `json_value`
    JSON_VALUE_KW,
    /// `keep`
    KEEP_KW,
    /// `key`
    KEY_KW,
    /// `keys`
    KEYS_KW,
    /// `label`
    LABEL_KW,
    /// `language`
    LANGUAGE_KW,
    /// `large`
    LARGE_KW,
    /// `last`
    LAST_KW,
    /// `lateral`
    LATERAL_KW,
    /// `leading`
    LEADING_KW,
    /// `leakproof`
    LEAKPROOF_KW,
    /// `least`
    LEAST_KW,
    /// `left`
    LEFT_KW,
    /// `level`
    LEVEL_KW,
    /// `like`
    LIKE_KW,
    /// `limit`
    LIMIT_KW,
    /// `listen`
    LISTEN_KW,
    /// `load`
    LOAD_KW,
    /// `local`
    LOCAL_KW,
    /// `localtime`
    LOCALTIME_KW,
    /// `localtimestamp`
    LOCALTIMESTAMP_KW,
    /// `location`
    LOCATION_KW,
    /// `lock`
    LOCK_KW,
    /// `locked`
    LOCKED_KW,
    /// `logged`
    LOGGED_KW,
    /// `mapping`
    MAPPING_KW,
    /// `match`
    MATCH_KW,
    /// `matched`
    MATCHED_KW,
    /// `materialized`
    MATERIALIZED_KW,
    /// `maxvalue`
    MAXVALUE_KW,
    /// `merge`
    MERGE_KW,
    /// `merge_action`
    MERGE_ACTION_KW,
    /// `method`
    METHOD_KW,
    /// `minute`
    MINUTE_KW,
    /// `minvalue`
    MINVALUE_KW,
    /// `mode`
    MODE_KW,
    /// `month`
    MONTH_KW,
    /// `move`
    MOVE_KW,
    /// `name`
    NAME_KW,
    /// `names`
    NAMES_KW,
    /// `national`
    NATIONAL_KW,
    /// `natural`
    NATURAL_KW,
    /// `nchar`
    NCHAR_KW,
    /// `nested`
    NESTED_KW,
    /// `new`
    NEW_KW,
    /// `next`
    NEXT_KW,
    /// `nfc`
    NFC_KW,
    /// `nfd`
    NFD_KW,
    /// `nfkc`
    NFKC_KW,
    /// `nfkd`
    NFKD_KW,
    /// `no`
    NO_KW,
    /// `none`
    NONE_KW,
    /// `normalize`
    NORMALIZE_KW,
    /// `normalized`
    NORMALIZED_KW,
    /// `not`
    NOT_KW,
    /// `nothing`
    NOTHING_KW,
    /// `notify`
    NOTIFY_KW,
    /// `notnull`
    NOTNULL_KW,
    /// `nowait`
    NOWAIT_KW,
    /// `null`
    NULL_KW,
    /// `nullif`
    NULLIF_KW,
    /// `nulls`
    NULLS_KW,
    /// `numeric`
    NUMERIC_KW,
    /// `object`
    OBJECT_KW,
    /// `of`
    OF_KW,
    /// `off`
    OFF_KW,
    /// `offset`
    OFFSET_KW,
    /// `oids`
    OIDS_KW,
    /// `old`
    OLD_KW,
    /// `omit`
    OMIT_KW,
    /// `on`
    ON_KW,
    /// `only`
    ONLY_KW,
    /// `operator`
    OPERATOR_KW,
    /// `option`
    OPTION_KW,
    /// `options`
    OPTIONS_KW,
    /// `or`
    OR_KW,
    /// `order`
    ORDER_KW,
    /// `ordinality`
    ORDINALITY_KW,
    /// `others`
    OTHERS_KW,
    /// `out`
    OUT_KW,
    /// `outer`
    OUTER_KW,
    /// `over`
    OVER_KW,
    /// `overlaps`
    OVERLAPS_KW,
    /// `overlay`
    OVERLAY_KW,
    /// `overriding`
    OVERRIDING_KW,
    /// `owned`
    OWNED_KW,
    /// `owner`
    OWNER_KW,
    /// `parallel`
    PARALLEL_KW,
    /// `parameter`
    PARAMETER_KW,
    /// `parser`
    PARSER_KW,
    /// `partial`
    PARTIAL_KW,
    /// `partition`
    PARTITION_KW,
    /// `passing`
    PASSING_KW,
    /// `password`
    PASSWORD_KW,
    /// `path`
    PATH_KW,
    /// `period`
    PERIOD_KW,
    /// `placing`
    PLACING_KW,
    /// `plan`
    PLAN_KW,
    /// `plans`
    PLANS_KW,
    /// `policy`
    POLICY_KW,
    /// `position`
    POSITION_KW,
    /// `preceding`
    PRECEDING_KW,
    /// `precision`
    PRECISION_KW,
    /// `prepare`
    PREPARE_KW,
    /// `prepared`
    PREPARED_KW,
    /// `preserve`
    PRESERVE_KW,
    /// `primary`
    PRIMARY_KW,
    /// `prior`
    PRIOR_KW,
    /// `privileges`
    PRIVILEGES_KW,
    /// `procedural`
    PROCEDURAL_KW,
    /// `procedure`
    PROCEDURE_KW,
    /// `procedures`
    PROCEDURES_KW,
    /// `program`
    PROGRAM_KW,
    /// `publication`
    PUBLICATION_KW,
    /// `quote`
    QUOTE_KW,
    /// `quotes`
    QUOTES_KW,
    /// `range`
    RANGE_KW,
    /// `read`
    READ_KW,
    /// `real`
    REAL_KW,
    /// `reassign`
    REASSIGN_KW,
    /// `recursive`
    RECURSIVE_KW,
    /// `ref`
    REF_KW,
    /// `references`
    REFERENCES_KW,
    /// `referencing`
    REFERENCING_KW,
    /// `refresh`
    REFRESH_KW,
    /// `reindex`
    REINDEX_KW,
    /// `relative`
    RELATIVE_KW,
    /// `release`
    RELEASE_KW,
    /// `rename`
    RENAME_KW,
    /// `repeatable`
    REPEATABLE_KW,
    /// `replace`
    REPLACE_KW,
    /// `replica`
    REPLICA_KW,
    /// `reset`
    RESET_KW,
    /// `restart`
    RESTART_KW,
    /// `restrict`
    RESTRICT_KW,
    /// `return`
    RETURN_KW,
    /// `returning`
    RETURNING_KW,
    /// `returns`
    RETURNS_KW,
    /// `revoke`
    REVOKE_KW,
    /// `right`
    RIGHT_KW,
    /// `role`
    ROLE_KW,
    /// `rollback`
    ROLLBACK_KW,
    /// `rollup`
    ROLLUP_KW,
    /// `routine`
    ROUTINE_KW,
    /// `routines`
    ROUTINES_KW,
    /// `row`
    ROW_KW,
    /// `rows`
    ROWS_KW,
    /// `rule`
    RULE_KW,
    /// `savepoint`
    SAVEPOINT_KW,
    /// `scalar`
    SCALAR_KW,
    /// `schema`
    SCHEMA_KW,
    /// `schemas`
    SCHEMAS_KW,
    /// `scroll`
    SCROLL_KW,
    /// `search`
    SEARCH_KW,
    /// `second`
    SECOND_KW,
    /// `security`
    SECURITY_KW,
    /// `select`
    SELECT_KW,
    /// `sequence`
    SEQUENCE_KW,
    /// `sequences`
    SEQUENCES_KW,
    /// `serializable`
    SERIALIZABLE_KW,
    /// `server`
    SERVER_KW,
    /// `session`
    SESSION_KW,
    /// `session_user`
    SESSION_USER_KW,
    /// `set`
    SET_KW,
    /// `setof`
    SETOF_KW,
    /// `sets`
    SETS_KW,
    /// `share`
    SHARE_KW,
    /// `show`
    SHOW_KW,
    /// `similar`
    SIMILAR_KW,
    /// `simple`
    SIMPLE_KW,
    /// `skip`
    SKIP_KW,
    /// `smallint`
    SMALLINT_KW,
    /// `snapshot`
    SNAPSHOT_KW,
    /// `some`
    SOME_KW,
    /// `source`
    SOURCE_KW,
    /// `sql`
    SQL_KW,
    /// `stable`
    STABLE_KW,
    /// `standalone`
    STANDALONE_KW,
    /// `start`
    START_KW,
    /// `statement`
    STATEMENT_KW,
    /// `statistics`
    STATISTICS_KW,
    /// `stdin`
    STDIN_KW,
    /// `stdout`
    STDOUT_KW,
    /// `storage`
    STORAGE_KW,
    /// `stored`
    STORED_KW,
    /// `strict`
    STRICT_KW,
    /// `string`
    STRING_KW,
    /// `strip`
    STRIP_KW,
    /// `subscription`
    SUBSCRIPTION_KW,
    /// `substring`
    SUBSTRING_KW,
    /// `support`
    SUPPORT_KW,
    /// `symmetric`
    SYMMETRIC_KW,
    /// `sysid`
    SYSID_KW,
    /// `system`
    SYSTEM_KW,
    /// `system_user`
    SYSTEM_USER_KW,
    /// `table`
    TABLE_KW,
    /// `tables`
    TABLES_KW,
    /// `tablesample`
    TABLESAMPLE_KW,
    /// `tablespace`
    TABLESPACE_KW,
    /// `target`
    TARGET_KW,
    /// `temp`
    TEMP_KW,
    /// `template`
    TEMPLATE_KW,
    /// `temporary`
    TEMPORARY_KW,
    /// `text`
    TEXT_KW,
    /// `then`
    THEN_KW,
    /// `ties`
    TIES_KW,
    /// `time`
    TIME_KW,
    /// `timestamp`
    TIMESTAMP_KW,
    /// `to`
    TO_KW,
    /// `trailing`
    TRAILING_KW,
    /// `transaction`
    TRANSACTION_KW,
    /// `transform`
    TRANSFORM_KW,
    /// `treat`
    TREAT_KW,
    /// `trigger`
    TRIGGER_KW,
    /// `trim`
    TRIM_KW,
    /// `true`
    TRUE_KW,
    /// `truncate`
    TRUNCATE_KW,
    /// `trusted`
    TRUSTED_KW,
    /// `type`
    TYPE_KW,
    /// `types`
    TYPES_KW,
    /// `uescape`
    UESCAPE_KW,
    /// `unbounded`
    UNBOUNDED_KW,
    /// `uncommitted`
    UNCOMMITTED_KW,
    /// `unconditional`
    UNCONDITIONAL_KW,
    /// `unencrypted`
    UNENCRYPTED_KW,
    /// `union`
    UNION_KW,
    /// `unique`
    UNIQUE_KW,
    /// `unknown`
    UNKNOWN_KW,
    /// `unlisten`
    UNLISTEN_KW,
    /// `unlogged`
    UNLOGGED_KW,
    /// `until`
    UNTIL_KW,
    /// `update`
    UPDATE_KW,
    /// `user`
    USER_KW,
    /// `using`
    USING_KW,
    /// `vacuum`
    VACUUM_KW,
    /// `valid`
    VALID_KW,
    /// `validate`
    VALIDATE_KW,
    /// `validator`
    VALIDATOR_KW,
    /// `value`
    VALUE_KW,
    /// `values`
    VALUES_KW,
    /// `varchar`
    VARCHAR_KW,
    /// `variadic`
    VARIADIC_KW,
    /// `varying`
    VARYING_KW,
    /// `verbose`
    VERBOSE_KW,
    /// `version`
    VERSION_KW,
    /// `view`
    VIEW_KW,
    /// `views`
    VIEWS_KW,
    /// `volatile`
    VOLATILE_KW,
    /// `when`
    WHEN_KW,
    /// `where`
    WHERE_KW,
    /// `whitespace`
    WHITESPACE_KW,
    /// `window`
    WINDOW_KW,
    /// `with`
    WITH_KW,
    /// `within`
    WITHIN_KW,
    /// `without`
    WITHOUT_KW,
    /// `work`
    WORK_KW,
    /// `wrapper`
    WRAPPER_KW,
    /// `write`
    WRITE_KW,
    /// `xml`
    XML_KW,
    /// `xmlattributes`
    XMLATTRIBUTES_KW,
    /// `xmlconcat`
    XMLCONCAT_KW,
    /// `xmlelement`
    XMLELEMENT_KW,
    /// `xmlexists`
    XMLEXISTS_KW,
    /// `xmlforest`
    XMLFOREST_KW,
    /// `xmlnamespaces`
    XMLNAMESPACES_KW,
    /// `xmlparse`
    XMLPARSE_KW,
    /// `xmlpi`
    XMLPI_KW,
    /// `xmlroot`
    XMLROOT_KW,
    /// `xmlserialize`
    XMLSERIALIZE_KW,
    /// `xmltable`
    XMLTABLE_KW,
    /// `year`
    YEAR_KW,
    /// `yes`
    YES_KW,
    /// `zone`
    ZONE_KW,

    // literals
    /// `1.0`
    FLOAT_NUMBER,
    /// `1`
    INT_NUMBER,
    /// `'foo'`
    STRING,
    /// `X'1FF'`, `U&'d\0061t\+000061'`
    ///
    /// see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE>
    /// see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-BIT-STRINGS>
    BYTE_STRING,
    /// `B'1001'`
    ///
    /// see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-BIT-STRINGS>
    BIT_STRING,
    /// `$$Dianne's horse$$`
    ///
    /// see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING>
    DOLLAR_QUOTED_STRING,
    /// `E'foo'`
    ///
    /// see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html>
    ESC_STRING,
    /// `-- foo`
    /// or
    /// `/* foo */`
    ///
    /// see: <https://www.postgresql.org/docs/17/sql-syntax-lexical.html#SQL-SYNTAX-COMMENTS>
    COMMENT,
    IDENT,
    PARAM,
    ERROR,
    WHITESPACE,

    // nodes
    /// `100, bar, buzz`
    ARG_LIST,
    ARG,
    PARAM_LIST,
    COLLATE,
    TARGET_LIST,
    TARGET,
    ARRAY_EXPR,
    IS_NULL,
    IS_NOT,
    IS_NOT_DISTINCT_FROM,
    /// <https://www.postgresql.org/docs/17/sql-expressions.html#SQL-EXPRESSIONS-OPERATOR-CALLS>
    /// Left assoc in postgres/gram.y
    /// Same precedence as operators
    OPERATOR_CALL,
    /// `at time zone`
    AT_TIME_ZONE,
    SIMILAR_TO,
    IS_DISTINCT_FROM,
    NOT_LIKE,
    NOT_IN,
    BIN_EXPR,
    POSTFIX_EXPR,
    /// `foo()`
    CALL_EXPR,
    BETWEEN_EXPR,
    /// `foo::bar`, `cast(foo as bar)`, `treat(foo as bar)`, or `numeric '123'`
    CAST_EXPR,
    CASE_EXPR,
    ALIAS,
    /// `foo.bar`
    FIELD_EXPR,
    /// `foo[1]`
    INDEX_EXPR,
    LITERAL,
    NAME,
    /// `a := b`, `a => b`
    NAMED_ARG,
    /// `a: b`, `a value b`
    JSON_KEY_VALUE,
    PAREN_EXPR,
    PATH,
    PATH_SEGMENT,
    PATH_TYPE,
    CHAR_TYPE,
    BIT_TYPE,
    PERCENT_TYPE,
    DOUBLE_TYPE,
    TIME_TYPE,
    INTERVAL_TYPE,
    ARRAY_TYPE,
    PERCENT_TYPE_CLAUSE,
    WITH_TIMEZONE,
    WITHOUT_TIMEZONE,
    PREFIX_EXPR,
    COLUMN,
    SOURCE_FILE,
    RET_TYPE,
    STMT,
    ALTER_AGGREGATE_STMT,
    ALTER_COLLATION_STMT,
    ALTER_CONVERSION_STMT,
    ALTER_DATABASE_STMT,
    ALTER_DEFAULT_PRIVILEGES_STMT,
    ALTER_DOMAIN_STMT,
    ALTER_EVENT_TRIGGER_STMT,
    ALTER_EXTENSION_STMT,
    ALTER_FOREIGN_DATA_WRAPPER_STMT,
    ALTER_FOREIGN_TABLE_STMT,
    ALTER_FUNCTION_STMT,
    ALTER_GROUP_STMT,
    ALTER_INDEX_STMT,
    ALTER_LANGUAGE_STMT,
    ALTER_LARGE_OBJECT_STMT,
    ALTER_MATERIALIZED_VIEW_STMT,
    ALTER_OPERATOR_STMT,
    ALTER_OPERATOR_CLASS_STMT,
    ALTER_OPERATOR_FAMILY_STMT,
    ALTER_POLICY_STMT,
    ALTER_PROCEDURE_STMT,
    ALTER_PUBLICATION_STMT,
    ALTER_ROLE_STMT,
    ALTER_ROUTINE_STMT,
    ALTER_RULE_STMT,
    ALTER_SCHEMA_STMT,
    ALTER_SEQUENCE_STMT,
    ALTER_SERVER_STMT,
    ALTER_STATISTICS_STMT,
    ALTER_SUBSCRIPTION_STMT,
    ALTER_SYSTEM_STMT,
    ALTER_TABLESPACE_STMT,
    ALTER_TEXT_SEARCH_CONFIGURATION_STMT,
    ALTER_TEXT_SEARCH_DICTIONARY_STMT,
    ALTER_TEXT_SEARCH_PARSER_STMT,
    ALTER_TEXT_SEARCH_TEMPLATE_STMT,
    ALTER_TRIGGER_STMT,
    ALTER_TYPE_STMT,
    ALTER_USER_STMT,
    ALTER_USER_MAPPING_STMT,
    ALTER_VIEW_STMT,
    ANALYZE_STMT,
    CLUSTER_STMT,
    COMMENT_STMT,
    COMMIT_STMT,
    CREATE_EXTENSION_STMT,
    CREATE_ACCESS_METHOD_STMT,
    CREATE_AGGREGATE_STMT,
    CREATE_CAST_STMT,
    CREATE_COLLATION_STMT,
    CREATE_CONVERSION_STMT,
    CREATE_DATABASE_STMT,
    CREATE_DOMAIN_STMT,
    CREATE_EVENT_TRIGGER_STMT,
    CREATE_FOREIGN_DATA_WRAPPER_STMT,
    CREATE_FOREIGN_TABLE_STMT,
    CREATE_GROUP_STMT,
    CREATE_LANGUAGE_STMT,
    CREATE_MATERIALIZED_VIEW_STMT,
    CREATE_OPERATOR_STMT,
    CREATE_OPERATOR_CLASS_STMT,
    CREATE_OPERATOR_FAMILY_STMT,
    CREATE_POLICY_STMT,
    CREATE_PROCEDURE_STMT,
    CREATE_PUBLICATION_STMT,
    CREATE_ROLE_STMT,
    CREATE_RULE_STMT,
    CREATE_SEQUENCE_STMT,
    CREATE_SERVER_STMT,
    CREATE_STATISTICS_STMT,
    CREATE_SUBSCRIPTION_STMT,
    CREATE_TABLE_AS_STMT,
    CREATE_TABLESPACE_STMT,
    CREATE_TEXT_SEARCH_CONFIGURATION_STMT,
    CREATE_TEXT_SEARCH_DICTIONARY_STMT,
    CREATE_TEXT_SEARCH_PARSER_STMT,
    CREATE_TEXT_SEARCH_TEMPLATE_STMT,
    CREATE_TRANSFORM_STMT,
    CREATE_INDEX_STMT,
    CREATE_TYPE_STMT,
    CREATE_TRIGGER_STMT,
    CREATE_FUNCTION_STMT,
    PARAM_IN,
    PARAM_OUT,
    PARAM_INOUT,
    PARAM_VARIADIC,
    BEGIN_FUNC_OPTION,
    RETURN_FUNC_OPTION,
    AS_FUNC_OPTION,
    SET_FUNC_OPTION,
    SUPPORT_FUNC_OPTION,
    ROWS_FUNC_OPTION,
    COST_FUNC_OPTION,
    PARALLEL_FUNC_OPTION,
    SECURITY_FUNC_OPTION,
    STRICT_FUNC_OPTION,
    LEAKPROOF_FUNC_OPTION,
    RESET_FUNC_OPTION,
    VOLATILITY_FUNC_OPTION,
    WINDOW_FUNC_OPTION,
    TRANSFORM_FUNC_OPTION,
    LANGUAGE_FUNC_OPTION,
    PARAM_DEFAULT,
    FUNC_OPTION_LIST,
    IF_EXISTS,
    IF_NOT_EXISTS,
    OR_REPLACE,
    DROP_INDEX_STMT,
    DROP_TRIGGER_STMT,
    BEGIN_STMT,
    SHOW_STMT,
    SET_STMT,
    PREPARE_TRANSACTION_STMT,
    DROP_DATABASE_STMT,
    DROP_TYPE_STMT,
    CALL_STMT,
    TRUNCATE_STMT,
    MOVE_STMT,
    FETCH_STMT,
    DECLARE_STMT,
    DO_STMT,
    DISCARD_STMT,
    RESET_STMT,
    LISTEN_STMT,
    LOAD_STMT,
    DEALLOCATE_STMT,
    CHECKPOINT_STMT,
    PREPARE_STMT,
    UNLISTEN_STMT,
    NOTIFY_STMT,
    CLOSE_STMT,
    VACUUM_STMT,
    COPY_STMT,
    DELETE_STMT,
    MERGE_STMT,
    LOCK_STMT,
    EXPLAIN_STMT,
    DROP_USER_STMT,
    DROP_TRANSFORM_STMT,
    DROP_TEXT_SEARCH_TEMPLATE_STMT,
    DROP_TEXT_SEARCH_PARSER_STMT,
    DROP_TEXT_SEARCH_DICT_STMT,
    DROP_TEXT_SEARCH_CONFIG_STMT,
    DROP_TABLESPACE_STMT,
    DROP_SUBSCRIPTION_STMT,
    DROP_STATISTICS_STMT,
    DROP_SERVER_STMT,
    DROP_SEQUENCE_STMT,
    DROP_RULE_STMT,
    DROP_ROUTINE_STMT,
    DROP_ROLE_STMT,
    DROP_PUBLICATION_STMT,
    DROP_PROCEDURE_STMT,
    DROP_POLICY_STMT,
    DROP_OWNED_STMT,
    DROP_OPERATOR_FAMILY_STMT,
    DROP_OPERATOR_CLASS_STMT,
    DROP_MATERIALIZED_VIEW_STMT,
    DROP_OPERATOR_STMT,
    DROP_LANGUAGE_STMT,
    DROP_GROUP_STMT,
    DROP_FUNCTION_STMT,
    DROP_FOREIGN_TABLE_STMT,
    DROP_FOREIGN_DATA_WRAPPER_STMT,
    DROP_EXTENSION_STMT,
    DROP_EVENT_TRIGGER_STMT,
    DROP_DOMAIN_STMT,
    DROP_CONVERSION_STMT,
    DROP_COLLATION_STMT,
    DROP_CAST_STMT,
    DROP_AGGREGATE_STMT,
    DROP_ACCESS_METHOD_STMT,
    DROP_USER_MAPPING_STMT,
    IMPORT_FOREIGN_SCHEMA,
    EXECUTE_STMT,
    CREATE_VIEW_STMT,
    SAVEPOINT_STMT,
    RELEASE_SAVEPOINT_STMT,
    DROP_SCHEMA_STMT,
    DROP_VIEW_STMT,
    REINDEX_STMT,
    UPDATE_STMT,
    ROLLBACK_STMT,
    INSERT_STMT,
    CREATE_SCHEMA_STMT,
    SELECT,
    SELECT_INTO_STMT,
    SECURITY_LABEL_STMT,
    REVOKE_STMT,
    GRANT_STMT,
    REFRESH_STMT,
    REASSIGN_STMT,
    SET_SESSION_AUTH_STMT,
    CREATE_USER_MAPPING_STMT,
    CREATE_USER_STMT,
    SET_ROLE_STMT,
    SET_CONSTRAINTS_STMT,
    SET_TRANSACTION_STMT,
    INTO_CLAUSE,
    COMPOUND_SELECT,
    DROP_TABLE,
    JOIN,
    CREATE_TABLE,
    ALTER_TABLE,
    WINDOW_DEF,
    JSON_VALUE_EXPR,
    JSON_FORMAT_CLAUSE,
    JSON_RETURNING_CLAUSE,
    JSON_QUOTES_CLAUSE,
    JSON_WRAPPER_BEHAVIOR_CLAUSE,
    JSON_BEHAVIOR_CLAUSE,
    JSON_PASSING_CLAUSE,
    JSON_ON_ERROR_CLAUSE,
    JSON_NULL_CLAUSE,
    JSON_KEYS_UNIQUE_CLAUSE,
    SELECT_CLAUSE,
    LIKE_CLAUSE,
    REFERENCES_CONSTRAINT,
    PRIMARY_KEY_CONSTRAINT,
    FOREIGN_KEY_CONSTRAINT,
    EXCLUDE_CONSTRAINT,
    UNIQUE_CONSTRAINT,
    GENERATED_CONSTRAINT,
    DEFAULT_CONSTRAINT,
    CHECK_CONSTRAINT,
    NULL_CONSTRAINT,
    NOT_NULL_CONSTRAINT,
    INDEX_PARAMS,
    CONSTRAINT_INDEX_TABLESPACE,
    CONSTRAINT_STORAGE_PARAMS,
    CONSTRAINT_INCLUDE_CLAUSE,
    CONSTRAINT_WHERE_CLAUSE,
    CONSTRAINT_INDEX_METHOD,
    CONSTRAINT_EXCLUSIONS,
    DEFERRABLE_CONSTRAINT_OPTION,
    NOT_DEFERRABLE_CONSTRAINT_OPTION,
    INITALLY_DEFERRED_CONSTRAINT_OPTION,
    INITIALLY_IMMEDIATE_CONSTRAINT_OPTION,
    CONSTRAINT_OPTION_LIST,
    SEQUENCE_OPTION_LIST,
    USING_INDEX,
    // alter table actions
    VALIDATE_CONSTRAINT,
    REPLICA_IDENTITY,
    OF_TYPE,
    NOT_OF,
    FORCE_RLS,
    NO_FORCE_RLS,
    INHERIT,
    NO_INHERIT,
    ENABLE_TRIGGER,
    ENABLE_REPLICA_TRIGGER,
    ENABLE_REPLICA_RULE,
    ENABLE_ALWAYS_TRIGGER,
    ENABLE_ALWAYS_RULE,
    ENABLE_RULE,
    ENABLE_RLS,
    DISABLE_TRIGGER,
    DISABLE_RLS,
    DISABLE_RULE,
    DISABLE_CLUSTER,
    OWNER_TO,
    DETACH_PARTITION,
    DROP_CONSTRAINT,
    DROP_COLUMN,
    ADD_CONSTRAINT,
    ADD_COLUMN,
    ATTACH_PARTITION,
    SET_SCHEMA,
    SET_TABLESPACE,
    SET_WITHOUT_CLUSTER,
    SET_WITHOUT_OIDS,
    SET_ACCESS_METHOD,
    SET_LOGGED,
    SET_UNLOGGED,
    SET_STORAGE_PARAMS,
    RESET_STORAGE_PARAMS,
    RENAME_TABLE,
    RENAME_CONSTRAINT,
    RENAME_COLUMN,
    RENAME_TO,
    NOT_VALID,
    ALTER_CONSTRAINT,
    ALTER_COLUMN,
    // alter table actions end
    // alter column options
    DROP_DEFAULT,
    DROP_EXPRESSION,
    DROP_IDENTITY,
    DROP_NOT_NULL,
    RESTART,
    ADD_GENERATED,
    RESET_OPTIONS,
    SET_TYPE,
    SET_GENERATED_OPTIONS,
    SET_GENERATED,
    SET_SEQUENCE_OPTION,
    SET_DEFAULT,
    SET_EXPRESSION,
    SET_STATISTICS,
    SET_OPTIONS,
    SET_OPTIONS_LIST,
    SET_STORAGE,
    SET_COMPRESSION,
    SET_NOT_NULL,
    // alter column options end
    TABLE_ARGS,
    COLUMN_LIST,
    WHEN_CLAUSE,
    USING_CLAUSE,
    WITHIN_CLAUSE,
    FILTER_CLAUSE,
    OVER_CLAUSE,
    DISTINCT_CLAUSE,
    WITH_TABLE,
    WITH_CLAUSE,
    FROM_CLAUSE,
    WHERE_CLAUSE,
    GROUP_BY_CLAUSE,
    HAVING_CLAUSE,
    WINDOW_CLAUSE,
    LIMIT_CLAUSE,
    OFFSET_CLAUSE,
    ORDER_BY_CLAUSE,
    LOCKING_CLAUSE,
    TUPLE_EXPR,
    NAME_REF,
    #[doc(hidden)]
    __LAST,
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}

impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WHITESPACE | SyntaxKind::COMMENT)
    }

    // generated via: cargo xtask generate_keywords
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

// Generated TokenSet start
pub(crate) const COLUMN_OR_TABLE_KEYWORDS: TokenSet = TokenSet::new(&[
    SyntaxKind::ABORT_KW,
    SyntaxKind::ABSENT_KW,
    SyntaxKind::ABSOLUTE_KW,
    SyntaxKind::ACCESS_KW,
    SyntaxKind::ACTION_KW,
    SyntaxKind::ADD_KW,
    SyntaxKind::ADMIN_KW,
    SyntaxKind::AFTER_KW,
    SyntaxKind::AGGREGATE_KW,
    SyntaxKind::ALSO_KW,
    SyntaxKind::ALTER_KW,
    SyntaxKind::ALWAYS_KW,
    SyntaxKind::ASENSITIVE_KW,
    SyntaxKind::ASSERTION_KW,
    SyntaxKind::ASSIGNMENT_KW,
    SyntaxKind::AT_KW,
    SyntaxKind::ATOMIC_KW,
    SyntaxKind::ATTACH_KW,
    SyntaxKind::ATTRIBUTE_KW,
    SyntaxKind::BACKWARD_KW,
    SyntaxKind::BEFORE_KW,
    SyntaxKind::BEGIN_KW,
    SyntaxKind::BETWEEN_KW,
    SyntaxKind::BIGINT_KW,
    SyntaxKind::BIT_KW,
    SyntaxKind::BOOLEAN_KW,
    SyntaxKind::BREADTH_KW,
    SyntaxKind::BY_KW,
    SyntaxKind::CACHE_KW,
    SyntaxKind::CALL_KW,
    SyntaxKind::CALLED_KW,
    SyntaxKind::CASCADE_KW,
    SyntaxKind::CASCADED_KW,
    SyntaxKind::CATALOG_KW,
    SyntaxKind::CHAIN_KW,
    SyntaxKind::CHAR_KW,
    SyntaxKind::CHARACTER_KW,
    SyntaxKind::CHARACTERISTICS_KW,
    SyntaxKind::CHECKPOINT_KW,
    SyntaxKind::CLASS_KW,
    SyntaxKind::CLOSE_KW,
    SyntaxKind::CLUSTER_KW,
    SyntaxKind::COALESCE_KW,
    SyntaxKind::COLUMNS_KW,
    SyntaxKind::COMMENT_KW,
    SyntaxKind::COMMENTS_KW,
    SyntaxKind::COMMIT_KW,
    SyntaxKind::COMMITTED_KW,
    SyntaxKind::COMPRESSION_KW,
    SyntaxKind::CONDITIONAL_KW,
    SyntaxKind::CONFIGURATION_KW,
    SyntaxKind::CONFLICT_KW,
    SyntaxKind::CONNECTION_KW,
    SyntaxKind::CONSTRAINTS_KW,
    SyntaxKind::CONTENT_KW,
    SyntaxKind::CONTINUE_KW,
    SyntaxKind::CONVERSION_KW,
    SyntaxKind::COPY_KW,
    SyntaxKind::COST_KW,
    SyntaxKind::CSV_KW,
    SyntaxKind::CUBE_KW,
    SyntaxKind::CURRENT_KW,
    SyntaxKind::CURSOR_KW,
    SyntaxKind::CYCLE_KW,
    SyntaxKind::DATA_KW,
    SyntaxKind::DATABASE_KW,
    SyntaxKind::DAY_KW,
    SyntaxKind::DEALLOCATE_KW,
    SyntaxKind::DEC_KW,
    SyntaxKind::DECIMAL_KW,
    SyntaxKind::DECLARE_KW,
    SyntaxKind::DEFAULTS_KW,
    SyntaxKind::DEFERRED_KW,
    SyntaxKind::DEFINER_KW,
    SyntaxKind::DELETE_KW,
    SyntaxKind::DELIMITER_KW,
    SyntaxKind::DELIMITERS_KW,
    SyntaxKind::DEPENDS_KW,
    SyntaxKind::DEPTH_KW,
    SyntaxKind::DETACH_KW,
    SyntaxKind::DICTIONARY_KW,
    SyntaxKind::DISABLE_KW,
    SyntaxKind::DISCARD_KW,
    SyntaxKind::DOCUMENT_KW,
    SyntaxKind::DOMAIN_KW,
    SyntaxKind::DOUBLE_KW,
    SyntaxKind::DROP_KW,
    SyntaxKind::EACH_KW,
    SyntaxKind::EMPTY_KW,
    SyntaxKind::ENABLE_KW,
    SyntaxKind::ENCODING_KW,
    SyntaxKind::ENCRYPTED_KW,
    SyntaxKind::ENUM_KW,
    SyntaxKind::ERROR_KW,
    SyntaxKind::ESCAPE_KW,
    SyntaxKind::EVENT_KW,
    SyntaxKind::EXCLUDE_KW,
    SyntaxKind::EXCLUDING_KW,
    SyntaxKind::EXCLUSIVE_KW,
    SyntaxKind::EXECUTE_KW,
    SyntaxKind::EXISTS_KW,
    SyntaxKind::EXPLAIN_KW,
    SyntaxKind::EXPRESSION_KW,
    SyntaxKind::EXTENSION_KW,
    SyntaxKind::EXTERNAL_KW,
    SyntaxKind::EXTRACT_KW,
    SyntaxKind::FAMILY_KW,
    SyntaxKind::FILTER_KW,
    SyntaxKind::FINALIZE_KW,
    SyntaxKind::FIRST_KW,
    SyntaxKind::FLOAT_KW,
    SyntaxKind::FOLLOWING_KW,
    SyntaxKind::FORCE_KW,
    SyntaxKind::FORMAT_KW,
    SyntaxKind::FORWARD_KW,
    SyntaxKind::FUNCTION_KW,
    SyntaxKind::FUNCTIONS_KW,
    SyntaxKind::GENERATED_KW,
    SyntaxKind::GLOBAL_KW,
    SyntaxKind::GRANTED_KW,
    SyntaxKind::GREATEST_KW,
    SyntaxKind::GROUPING_KW,
    SyntaxKind::GROUPS_KW,
    SyntaxKind::HANDLER_KW,
    SyntaxKind::HEADER_KW,
    SyntaxKind::HOLD_KW,
    SyntaxKind::HOUR_KW,
    SyntaxKind::IDENTITY_KW,
    SyntaxKind::IF_KW,
    SyntaxKind::IMMEDIATE_KW,
    SyntaxKind::IMMUTABLE_KW,
    SyntaxKind::IMPLICIT_KW,
    SyntaxKind::IMPORT_KW,
    SyntaxKind::INCLUDE_KW,
    SyntaxKind::INCLUDING_KW,
    SyntaxKind::INCREMENT_KW,
    SyntaxKind::INDENT_KW,
    SyntaxKind::INDEX_KW,
    SyntaxKind::INDEXES_KW,
    SyntaxKind::INHERIT_KW,
    SyntaxKind::INHERITS_KW,
    SyntaxKind::INLINE_KW,
    SyntaxKind::INOUT_KW,
    SyntaxKind::INPUT_KW,
    SyntaxKind::INSENSITIVE_KW,
    SyntaxKind::INSERT_KW,
    SyntaxKind::INSTEAD_KW,
    SyntaxKind::INT_KW,
    SyntaxKind::INTEGER_KW,
    SyntaxKind::INTERVAL_KW,
    SyntaxKind::INVOKER_KW,
    SyntaxKind::ISOLATION_KW,
    SyntaxKind::JSON_KW,
    SyntaxKind::JSON_ARRAY_KW,
    SyntaxKind::JSON_ARRAYAGG_KW,
    SyntaxKind::JSON_EXISTS_KW,
    SyntaxKind::JSON_OBJECT_KW,
    SyntaxKind::JSON_OBJECTAGG_KW,
    SyntaxKind::JSON_QUERY_KW,
    SyntaxKind::JSON_SCALAR_KW,
    SyntaxKind::JSON_SERIALIZE_KW,
    SyntaxKind::JSON_TABLE_KW,
    SyntaxKind::JSON_VALUE_KW,
    SyntaxKind::KEEP_KW,
    SyntaxKind::KEY_KW,
    SyntaxKind::KEYS_KW,
    SyntaxKind::LABEL_KW,
    SyntaxKind::LANGUAGE_KW,
    SyntaxKind::LARGE_KW,
    SyntaxKind::LAST_KW,
    SyntaxKind::LEAKPROOF_KW,
    SyntaxKind::LEAST_KW,
    SyntaxKind::LEVEL_KW,
    SyntaxKind::LISTEN_KW,
    SyntaxKind::LOAD_KW,
    SyntaxKind::LOCAL_KW,
    SyntaxKind::LOCATION_KW,
    SyntaxKind::LOCK_KW,
    SyntaxKind::LOCKED_KW,
    SyntaxKind::LOGGED_KW,
    SyntaxKind::MAPPING_KW,
    SyntaxKind::MATCH_KW,
    SyntaxKind::MATCHED_KW,
    SyntaxKind::MATERIALIZED_KW,
    SyntaxKind::MAXVALUE_KW,
    SyntaxKind::MERGE_KW,
    SyntaxKind::MERGE_ACTION_KW,
    SyntaxKind::METHOD_KW,
    SyntaxKind::MINUTE_KW,
    SyntaxKind::MINVALUE_KW,
    SyntaxKind::MODE_KW,
    SyntaxKind::MONTH_KW,
    SyntaxKind::MOVE_KW,
    SyntaxKind::NAME_KW,
    SyntaxKind::NAMES_KW,
    SyntaxKind::NATIONAL_KW,
    SyntaxKind::NCHAR_KW,
    SyntaxKind::NESTED_KW,
    SyntaxKind::NEW_KW,
    SyntaxKind::NEXT_KW,
    SyntaxKind::NFC_KW,
    SyntaxKind::NFD_KW,
    SyntaxKind::NFKC_KW,
    SyntaxKind::NFKD_KW,
    SyntaxKind::NO_KW,
    SyntaxKind::NONE_KW,
    SyntaxKind::NORMALIZE_KW,
    SyntaxKind::NORMALIZED_KW,
    SyntaxKind::NOTHING_KW,
    SyntaxKind::NOTIFY_KW,
    SyntaxKind::NOWAIT_KW,
    SyntaxKind::NULLIF_KW,
    SyntaxKind::NULLS_KW,
    SyntaxKind::NUMERIC_KW,
    SyntaxKind::OBJECT_KW,
    SyntaxKind::OF_KW,
    SyntaxKind::OFF_KW,
    SyntaxKind::OIDS_KW,
    SyntaxKind::OLD_KW,
    SyntaxKind::OMIT_KW,
    SyntaxKind::OPERATOR_KW,
    SyntaxKind::OPTION_KW,
    SyntaxKind::OPTIONS_KW,
    SyntaxKind::ORDINALITY_KW,
    SyntaxKind::OTHERS_KW,
    SyntaxKind::OUT_KW,
    SyntaxKind::OVER_KW,
    SyntaxKind::OVERLAY_KW,
    SyntaxKind::OVERRIDING_KW,
    SyntaxKind::OWNED_KW,
    SyntaxKind::OWNER_KW,
    SyntaxKind::PARALLEL_KW,
    SyntaxKind::PARAMETER_KW,
    SyntaxKind::PARSER_KW,
    SyntaxKind::PARTIAL_KW,
    SyntaxKind::PARTITION_KW,
    SyntaxKind::PASSING_KW,
    SyntaxKind::PASSWORD_KW,
    SyntaxKind::PATH_KW,
    SyntaxKind::PERIOD_KW,
    SyntaxKind::PLAN_KW,
    SyntaxKind::PLANS_KW,
    SyntaxKind::POLICY_KW,
    SyntaxKind::POSITION_KW,
    SyntaxKind::PRECEDING_KW,
    SyntaxKind::PRECISION_KW,
    SyntaxKind::PREPARE_KW,
    SyntaxKind::PREPARED_KW,
    SyntaxKind::PRESERVE_KW,
    SyntaxKind::PRIOR_KW,
    SyntaxKind::PRIVILEGES_KW,
    SyntaxKind::PROCEDURAL_KW,
    SyntaxKind::PROCEDURE_KW,
    SyntaxKind::PROCEDURES_KW,
    SyntaxKind::PROGRAM_KW,
    SyntaxKind::PUBLICATION_KW,
    SyntaxKind::QUOTE_KW,
    SyntaxKind::QUOTES_KW,
    SyntaxKind::RANGE_KW,
    SyntaxKind::READ_KW,
    SyntaxKind::REAL_KW,
    SyntaxKind::REASSIGN_KW,
    SyntaxKind::RECURSIVE_KW,
    SyntaxKind::REF_KW,
    SyntaxKind::REFERENCING_KW,
    SyntaxKind::REFRESH_KW,
    SyntaxKind::REINDEX_KW,
    SyntaxKind::RELATIVE_KW,
    SyntaxKind::RELEASE_KW,
    SyntaxKind::RENAME_KW,
    SyntaxKind::REPEATABLE_KW,
    SyntaxKind::REPLACE_KW,
    SyntaxKind::REPLICA_KW,
    SyntaxKind::RESET_KW,
    SyntaxKind::RESTART_KW,
    SyntaxKind::RESTRICT_KW,
    SyntaxKind::RETURN_KW,
    SyntaxKind::RETURNS_KW,
    SyntaxKind::REVOKE_KW,
    SyntaxKind::ROLE_KW,
    SyntaxKind::ROLLBACK_KW,
    SyntaxKind::ROLLUP_KW,
    SyntaxKind::ROUTINE_KW,
    SyntaxKind::ROUTINES_KW,
    SyntaxKind::ROW_KW,
    SyntaxKind::ROWS_KW,
    SyntaxKind::RULE_KW,
    SyntaxKind::SAVEPOINT_KW,
    SyntaxKind::SCALAR_KW,
    SyntaxKind::SCHEMA_KW,
    SyntaxKind::SCHEMAS_KW,
    SyntaxKind::SCROLL_KW,
    SyntaxKind::SEARCH_KW,
    SyntaxKind::SECOND_KW,
    SyntaxKind::SECURITY_KW,
    SyntaxKind::SEQUENCE_KW,
    SyntaxKind::SEQUENCES_KW,
    SyntaxKind::SERIALIZABLE_KW,
    SyntaxKind::SERVER_KW,
    SyntaxKind::SESSION_KW,
    SyntaxKind::SET_KW,
    SyntaxKind::SETOF_KW,
    SyntaxKind::SETS_KW,
    SyntaxKind::SHARE_KW,
    SyntaxKind::SHOW_KW,
    SyntaxKind::SIMPLE_KW,
    SyntaxKind::SKIP_KW,
    SyntaxKind::SMALLINT_KW,
    SyntaxKind::SNAPSHOT_KW,
    SyntaxKind::SOURCE_KW,
    SyntaxKind::SQL_KW,
    SyntaxKind::STABLE_KW,
    SyntaxKind::STANDALONE_KW,
    SyntaxKind::START_KW,
    SyntaxKind::STATEMENT_KW,
    SyntaxKind::STATISTICS_KW,
    SyntaxKind::STDIN_KW,
    SyntaxKind::STDOUT_KW,
    SyntaxKind::STORAGE_KW,
    SyntaxKind::STORED_KW,
    SyntaxKind::STRICT_KW,
    SyntaxKind::STRING_KW,
    SyntaxKind::STRIP_KW,
    SyntaxKind::SUBSCRIPTION_KW,
    SyntaxKind::SUBSTRING_KW,
    SyntaxKind::SUPPORT_KW,
    SyntaxKind::SYSID_KW,
    SyntaxKind::SYSTEM_KW,
    SyntaxKind::TABLES_KW,
    SyntaxKind::TABLESPACE_KW,
    SyntaxKind::TARGET_KW,
    SyntaxKind::TEMP_KW,
    SyntaxKind::TEMPLATE_KW,
    SyntaxKind::TEMPORARY_KW,
    SyntaxKind::TEXT_KW,
    SyntaxKind::TIES_KW,
    SyntaxKind::TIME_KW,
    SyntaxKind::TIMESTAMP_KW,
    SyntaxKind::TRANSACTION_KW,
    SyntaxKind::TRANSFORM_KW,
    SyntaxKind::TREAT_KW,
    SyntaxKind::TRIGGER_KW,
    SyntaxKind::TRIM_KW,
    SyntaxKind::TRUNCATE_KW,
    SyntaxKind::TRUSTED_KW,
    SyntaxKind::TYPE_KW,
    SyntaxKind::TYPES_KW,
    SyntaxKind::UESCAPE_KW,
    SyntaxKind::UNBOUNDED_KW,
    SyntaxKind::UNCOMMITTED_KW,
    SyntaxKind::UNCONDITIONAL_KW,
    SyntaxKind::UNENCRYPTED_KW,
    SyntaxKind::UNKNOWN_KW,
    SyntaxKind::UNLISTEN_KW,
    SyntaxKind::UNLOGGED_KW,
    SyntaxKind::UNTIL_KW,
    SyntaxKind::UPDATE_KW,
    SyntaxKind::VACUUM_KW,
    SyntaxKind::VALID_KW,
    SyntaxKind::VALIDATE_KW,
    SyntaxKind::VALIDATOR_KW,
    SyntaxKind::VALUE_KW,
    SyntaxKind::VALUES_KW,
    SyntaxKind::VARCHAR_KW,
    SyntaxKind::VARYING_KW,
    SyntaxKind::VERSION_KW,
    SyntaxKind::VIEW_KW,
    SyntaxKind::VIEWS_KW,
    SyntaxKind::VOLATILE_KW,
    SyntaxKind::WHITESPACE_KW,
    SyntaxKind::WITHIN_KW,
    SyntaxKind::WITHOUT_KW,
    SyntaxKind::WORK_KW,
    SyntaxKind::WRAPPER_KW,
    SyntaxKind::WRITE_KW,
    SyntaxKind::XML_KW,
    SyntaxKind::XMLATTRIBUTES_KW,
    SyntaxKind::XMLCONCAT_KW,
    SyntaxKind::XMLELEMENT_KW,
    SyntaxKind::XMLEXISTS_KW,
    SyntaxKind::XMLFOREST_KW,
    SyntaxKind::XMLNAMESPACES_KW,
    SyntaxKind::XMLPARSE_KW,
    SyntaxKind::XMLPI_KW,
    SyntaxKind::XMLROOT_KW,
    SyntaxKind::XMLSERIALIZE_KW,
    SyntaxKind::XMLTABLE_KW,
    SyntaxKind::YEAR_KW,
    SyntaxKind::YES_KW,
    SyntaxKind::ZONE_KW,
]);

pub(crate) const TYPE_KEYWORDS: TokenSet = TokenSet::new(&[
    SyntaxKind::ABORT_KW,
    SyntaxKind::ABSENT_KW,
    SyntaxKind::ABSOLUTE_KW,
    SyntaxKind::ACCESS_KW,
    SyntaxKind::ACTION_KW,
    SyntaxKind::ADD_KW,
    SyntaxKind::ADMIN_KW,
    SyntaxKind::AFTER_KW,
    SyntaxKind::AGGREGATE_KW,
    SyntaxKind::ALSO_KW,
    SyntaxKind::ALTER_KW,
    SyntaxKind::ALWAYS_KW,
    SyntaxKind::ASENSITIVE_KW,
    SyntaxKind::ASSERTION_KW,
    SyntaxKind::ASSIGNMENT_KW,
    SyntaxKind::AT_KW,
    SyntaxKind::ATOMIC_KW,
    SyntaxKind::ATTACH_KW,
    SyntaxKind::ATTRIBUTE_KW,
    SyntaxKind::AUTHORIZATION_KW,
    SyntaxKind::BACKWARD_KW,
    SyntaxKind::BEFORE_KW,
    SyntaxKind::BEGIN_KW,
    SyntaxKind::BETWEEN_KW,
    SyntaxKind::BIGINT_KW,
    SyntaxKind::BINARY_KW,
    SyntaxKind::BIT_KW,
    SyntaxKind::BOOLEAN_KW,
    SyntaxKind::BREADTH_KW,
    SyntaxKind::BY_KW,
    SyntaxKind::CACHE_KW,
    SyntaxKind::CALL_KW,
    SyntaxKind::CALLED_KW,
    SyntaxKind::CASCADE_KW,
    SyntaxKind::CASCADED_KW,
    SyntaxKind::CATALOG_KW,
    SyntaxKind::CHAIN_KW,
    SyntaxKind::CHAR_KW,
    SyntaxKind::CHARACTER_KW,
    SyntaxKind::CHARACTERISTICS_KW,
    SyntaxKind::CHECKPOINT_KW,
    SyntaxKind::CLASS_KW,
    SyntaxKind::CLOSE_KW,
    SyntaxKind::CLUSTER_KW,
    SyntaxKind::COALESCE_KW,
    SyntaxKind::COLLATION_KW,
    SyntaxKind::COLUMNS_KW,
    SyntaxKind::COMMENT_KW,
    SyntaxKind::COMMENTS_KW,
    SyntaxKind::COMMIT_KW,
    SyntaxKind::COMMITTED_KW,
    SyntaxKind::COMPRESSION_KW,
    SyntaxKind::CONCURRENTLY_KW,
    SyntaxKind::CONDITIONAL_KW,
    SyntaxKind::CONFIGURATION_KW,
    SyntaxKind::CONFLICT_KW,
    SyntaxKind::CONNECTION_KW,
    SyntaxKind::CONSTRAINTS_KW,
    SyntaxKind::CONTENT_KW,
    SyntaxKind::CONTINUE_KW,
    SyntaxKind::CONVERSION_KW,
    SyntaxKind::COPY_KW,
    SyntaxKind::COST_KW,
    SyntaxKind::CROSS_KW,
    SyntaxKind::CSV_KW,
    SyntaxKind::CUBE_KW,
    SyntaxKind::CURRENT_KW,
    SyntaxKind::CURRENT_SCHEMA_KW,
    SyntaxKind::CURSOR_KW,
    SyntaxKind::CYCLE_KW,
    SyntaxKind::DATA_KW,
    SyntaxKind::DATABASE_KW,
    SyntaxKind::DAY_KW,
    SyntaxKind::DEALLOCATE_KW,
    SyntaxKind::DEC_KW,
    SyntaxKind::DECIMAL_KW,
    SyntaxKind::DECLARE_KW,
    SyntaxKind::DEFAULTS_KW,
    SyntaxKind::DEFERRED_KW,
    SyntaxKind::DEFINER_KW,
    SyntaxKind::DELETE_KW,
    SyntaxKind::DELIMITER_KW,
    SyntaxKind::DELIMITERS_KW,
    SyntaxKind::DEPENDS_KW,
    SyntaxKind::DEPTH_KW,
    SyntaxKind::DETACH_KW,
    SyntaxKind::DICTIONARY_KW,
    SyntaxKind::DISABLE_KW,
    SyntaxKind::DISCARD_KW,
    SyntaxKind::DOCUMENT_KW,
    SyntaxKind::DOMAIN_KW,
    SyntaxKind::DOUBLE_KW,
    SyntaxKind::DROP_KW,
    SyntaxKind::EACH_KW,
    SyntaxKind::EMPTY_KW,
    SyntaxKind::ENABLE_KW,
    SyntaxKind::ENCODING_KW,
    SyntaxKind::ENCRYPTED_KW,
    SyntaxKind::ENUM_KW,
    SyntaxKind::ERROR_KW,
    SyntaxKind::ESCAPE_KW,
    SyntaxKind::EVENT_KW,
    SyntaxKind::EXCLUDE_KW,
    SyntaxKind::EXCLUDING_KW,
    SyntaxKind::EXCLUSIVE_KW,
    SyntaxKind::EXECUTE_KW,
    SyntaxKind::EXISTS_KW,
    SyntaxKind::EXPLAIN_KW,
    SyntaxKind::EXPRESSION_KW,
    SyntaxKind::EXTENSION_KW,
    SyntaxKind::EXTERNAL_KW,
    SyntaxKind::EXTRACT_KW,
    SyntaxKind::FAMILY_KW,
    SyntaxKind::FILTER_KW,
    SyntaxKind::FINALIZE_KW,
    SyntaxKind::FIRST_KW,
    SyntaxKind::FLOAT_KW,
    SyntaxKind::FOLLOWING_KW,
    SyntaxKind::FORCE_KW,
    SyntaxKind::FORMAT_KW,
    SyntaxKind::FORWARD_KW,
    SyntaxKind::FREEZE_KW,
    SyntaxKind::FULL_KW,
    SyntaxKind::FUNCTION_KW,
    SyntaxKind::FUNCTIONS_KW,
    SyntaxKind::GENERATED_KW,
    SyntaxKind::GLOBAL_KW,
    SyntaxKind::GRANTED_KW,
    SyntaxKind::GREATEST_KW,
    SyntaxKind::GROUPING_KW,
    SyntaxKind::GROUPS_KW,
    SyntaxKind::HANDLER_KW,
    SyntaxKind::HEADER_KW,
    SyntaxKind::HOLD_KW,
    SyntaxKind::HOUR_KW,
    SyntaxKind::IDENTITY_KW,
    SyntaxKind::IF_KW,
    SyntaxKind::ILIKE_KW,
    SyntaxKind::IMMEDIATE_KW,
    SyntaxKind::IMMUTABLE_KW,
    SyntaxKind::IMPLICIT_KW,
    SyntaxKind::IMPORT_KW,
    SyntaxKind::INCLUDE_KW,
    SyntaxKind::INCLUDING_KW,
    SyntaxKind::INCREMENT_KW,
    SyntaxKind::INDENT_KW,
    SyntaxKind::INDEX_KW,
    SyntaxKind::INDEXES_KW,
    SyntaxKind::INHERIT_KW,
    SyntaxKind::INHERITS_KW,
    SyntaxKind::INLINE_KW,
    SyntaxKind::INNER_KW,
    SyntaxKind::INOUT_KW,
    SyntaxKind::INPUT_KW,
    SyntaxKind::INSENSITIVE_KW,
    SyntaxKind::INSERT_KW,
    SyntaxKind::INSTEAD_KW,
    SyntaxKind::INT_KW,
    SyntaxKind::INTEGER_KW,
    SyntaxKind::INTERVAL_KW,
    SyntaxKind::INVOKER_KW,
    SyntaxKind::IS_KW,
    SyntaxKind::ISNULL_KW,
    SyntaxKind::ISOLATION_KW,
    SyntaxKind::JOIN_KW,
    SyntaxKind::JSON_KW,
    SyntaxKind::JSON_ARRAY_KW,
    SyntaxKind::JSON_ARRAYAGG_KW,
    SyntaxKind::JSON_EXISTS_KW,
    SyntaxKind::JSON_OBJECT_KW,
    SyntaxKind::JSON_OBJECTAGG_KW,
    SyntaxKind::JSON_QUERY_KW,
    SyntaxKind::JSON_SCALAR_KW,
    SyntaxKind::JSON_SERIALIZE_KW,
    SyntaxKind::JSON_TABLE_KW,
    SyntaxKind::JSON_VALUE_KW,
    SyntaxKind::KEEP_KW,
    SyntaxKind::KEY_KW,
    SyntaxKind::KEYS_KW,
    SyntaxKind::LABEL_KW,
    SyntaxKind::LANGUAGE_KW,
    SyntaxKind::LARGE_KW,
    SyntaxKind::LAST_KW,
    SyntaxKind::LEAKPROOF_KW,
    SyntaxKind::LEAST_KW,
    SyntaxKind::LEFT_KW,
    SyntaxKind::LEVEL_KW,
    SyntaxKind::LIKE_KW,
    SyntaxKind::LISTEN_KW,
    SyntaxKind::LOAD_KW,
    SyntaxKind::LOCAL_KW,
    SyntaxKind::LOCATION_KW,
    SyntaxKind::LOCK_KW,
    SyntaxKind::LOCKED_KW,
    SyntaxKind::LOGGED_KW,
    SyntaxKind::MAPPING_KW,
    SyntaxKind::MATCH_KW,
    SyntaxKind::MATCHED_KW,
    SyntaxKind::MATERIALIZED_KW,
    SyntaxKind::MAXVALUE_KW,
    SyntaxKind::MERGE_KW,
    SyntaxKind::MERGE_ACTION_KW,
    SyntaxKind::METHOD_KW,
    SyntaxKind::MINUTE_KW,
    SyntaxKind::MINVALUE_KW,
    SyntaxKind::MODE_KW,
    SyntaxKind::MONTH_KW,
    SyntaxKind::MOVE_KW,
    SyntaxKind::NAME_KW,
    SyntaxKind::NAMES_KW,
    SyntaxKind::NATIONAL_KW,
    SyntaxKind::NATURAL_KW,
    SyntaxKind::NCHAR_KW,
    SyntaxKind::NESTED_KW,
    SyntaxKind::NEW_KW,
    SyntaxKind::NEXT_KW,
    SyntaxKind::NFC_KW,
    SyntaxKind::NFD_KW,
    SyntaxKind::NFKC_KW,
    SyntaxKind::NFKD_KW,
    SyntaxKind::NO_KW,
    SyntaxKind::NONE_KW,
    SyntaxKind::NORMALIZE_KW,
    SyntaxKind::NORMALIZED_KW,
    SyntaxKind::NOTHING_KW,
    SyntaxKind::NOTIFY_KW,
    SyntaxKind::NOTNULL_KW,
    SyntaxKind::NOWAIT_KW,
    SyntaxKind::NULLIF_KW,
    SyntaxKind::NULLS_KW,
    SyntaxKind::NUMERIC_KW,
    SyntaxKind::OBJECT_KW,
    SyntaxKind::OF_KW,
    SyntaxKind::OFF_KW,
    SyntaxKind::OIDS_KW,
    SyntaxKind::OLD_KW,
    SyntaxKind::OMIT_KW,
    SyntaxKind::OPERATOR_KW,
    SyntaxKind::OPTION_KW,
    SyntaxKind::OPTIONS_KW,
    SyntaxKind::ORDINALITY_KW,
    SyntaxKind::OTHERS_KW,
    SyntaxKind::OUT_KW,
    SyntaxKind::OUTER_KW,
    SyntaxKind::OVER_KW,
    SyntaxKind::OVERLAPS_KW,
    SyntaxKind::OVERLAY_KW,
    SyntaxKind::OVERRIDING_KW,
    SyntaxKind::OWNED_KW,
    SyntaxKind::OWNER_KW,
    SyntaxKind::PARALLEL_KW,
    SyntaxKind::PARAMETER_KW,
    SyntaxKind::PARSER_KW,
    SyntaxKind::PARTIAL_KW,
    SyntaxKind::PARTITION_KW,
    SyntaxKind::PASSING_KW,
    SyntaxKind::PASSWORD_KW,
    SyntaxKind::PATH_KW,
    SyntaxKind::PERIOD_KW,
    SyntaxKind::PLAN_KW,
    SyntaxKind::PLANS_KW,
    SyntaxKind::POLICY_KW,
    SyntaxKind::POSITION_KW,
    SyntaxKind::PRECEDING_KW,
    SyntaxKind::PRECISION_KW,
    SyntaxKind::PREPARE_KW,
    SyntaxKind::PREPARED_KW,
    SyntaxKind::PRESERVE_KW,
    SyntaxKind::PRIOR_KW,
    SyntaxKind::PRIVILEGES_KW,
    SyntaxKind::PROCEDURAL_KW,
    SyntaxKind::PROCEDURE_KW,
    SyntaxKind::PROCEDURES_KW,
    SyntaxKind::PROGRAM_KW,
    SyntaxKind::PUBLICATION_KW,
    SyntaxKind::QUOTE_KW,
    SyntaxKind::QUOTES_KW,
    SyntaxKind::RANGE_KW,
    SyntaxKind::READ_KW,
    SyntaxKind::REAL_KW,
    SyntaxKind::REASSIGN_KW,
    SyntaxKind::RECURSIVE_KW,
    SyntaxKind::REF_KW,
    SyntaxKind::REFERENCING_KW,
    SyntaxKind::REFRESH_KW,
    SyntaxKind::REINDEX_KW,
    SyntaxKind::RELATIVE_KW,
    SyntaxKind::RELEASE_KW,
    SyntaxKind::RENAME_KW,
    SyntaxKind::REPEATABLE_KW,
    SyntaxKind::REPLACE_KW,
    SyntaxKind::REPLICA_KW,
    SyntaxKind::RESET_KW,
    SyntaxKind::RESTART_KW,
    SyntaxKind::RESTRICT_KW,
    SyntaxKind::RETURN_KW,
    SyntaxKind::RETURNS_KW,
    SyntaxKind::REVOKE_KW,
    SyntaxKind::RIGHT_KW,
    SyntaxKind::ROLE_KW,
    SyntaxKind::ROLLBACK_KW,
    SyntaxKind::ROLLUP_KW,
    SyntaxKind::ROUTINE_KW,
    SyntaxKind::ROUTINES_KW,
    SyntaxKind::ROW_KW,
    SyntaxKind::ROWS_KW,
    SyntaxKind::RULE_KW,
    SyntaxKind::SAVEPOINT_KW,
    SyntaxKind::SCALAR_KW,
    SyntaxKind::SCHEMA_KW,
    SyntaxKind::SCHEMAS_KW,
    SyntaxKind::SCROLL_KW,
    SyntaxKind::SEARCH_KW,
    SyntaxKind::SECOND_KW,
    SyntaxKind::SECURITY_KW,
    SyntaxKind::SEQUENCE_KW,
    SyntaxKind::SEQUENCES_KW,
    SyntaxKind::SERIALIZABLE_KW,
    SyntaxKind::SERVER_KW,
    SyntaxKind::SESSION_KW,
    SyntaxKind::SET_KW,
    SyntaxKind::SETOF_KW,
    SyntaxKind::SETS_KW,
    SyntaxKind::SHARE_KW,
    SyntaxKind::SHOW_KW,
    SyntaxKind::SIMILAR_KW,
    SyntaxKind::SIMPLE_KW,
    SyntaxKind::SKIP_KW,
    SyntaxKind::SMALLINT_KW,
    SyntaxKind::SNAPSHOT_KW,
    SyntaxKind::SOURCE_KW,
    SyntaxKind::SQL_KW,
    SyntaxKind::STABLE_KW,
    SyntaxKind::STANDALONE_KW,
    SyntaxKind::START_KW,
    SyntaxKind::STATEMENT_KW,
    SyntaxKind::STATISTICS_KW,
    SyntaxKind::STDIN_KW,
    SyntaxKind::STDOUT_KW,
    SyntaxKind::STORAGE_KW,
    SyntaxKind::STORED_KW,
    SyntaxKind::STRICT_KW,
    SyntaxKind::STRING_KW,
    SyntaxKind::STRIP_KW,
    SyntaxKind::SUBSCRIPTION_KW,
    SyntaxKind::SUBSTRING_KW,
    SyntaxKind::SUPPORT_KW,
    SyntaxKind::SYSID_KW,
    SyntaxKind::SYSTEM_KW,
    SyntaxKind::TABLES_KW,
    SyntaxKind::TABLESAMPLE_KW,
    SyntaxKind::TABLESPACE_KW,
    SyntaxKind::TARGET_KW,
    SyntaxKind::TEMP_KW,
    SyntaxKind::TEMPLATE_KW,
    SyntaxKind::TEMPORARY_KW,
    SyntaxKind::TEXT_KW,
    SyntaxKind::TIES_KW,
    SyntaxKind::TIME_KW,
    SyntaxKind::TIMESTAMP_KW,
    SyntaxKind::TRANSACTION_KW,
    SyntaxKind::TRANSFORM_KW,
    SyntaxKind::TREAT_KW,
    SyntaxKind::TRIGGER_KW,
    SyntaxKind::TRIM_KW,
    SyntaxKind::TRUNCATE_KW,
    SyntaxKind::TRUSTED_KW,
    SyntaxKind::TYPE_KW,
    SyntaxKind::TYPES_KW,
    SyntaxKind::UESCAPE_KW,
    SyntaxKind::UNBOUNDED_KW,
    SyntaxKind::UNCOMMITTED_KW,
    SyntaxKind::UNCONDITIONAL_KW,
    SyntaxKind::UNENCRYPTED_KW,
    SyntaxKind::UNKNOWN_KW,
    SyntaxKind::UNLISTEN_KW,
    SyntaxKind::UNLOGGED_KW,
    SyntaxKind::UNTIL_KW,
    SyntaxKind::UPDATE_KW,
    SyntaxKind::VACUUM_KW,
    SyntaxKind::VALID_KW,
    SyntaxKind::VALIDATE_KW,
    SyntaxKind::VALIDATOR_KW,
    SyntaxKind::VALUE_KW,
    SyntaxKind::VALUES_KW,
    SyntaxKind::VARCHAR_KW,
    SyntaxKind::VARYING_KW,
    SyntaxKind::VERBOSE_KW,
    SyntaxKind::VERSION_KW,
    SyntaxKind::VIEW_KW,
    SyntaxKind::VIEWS_KW,
    SyntaxKind::VOLATILE_KW,
    SyntaxKind::WHITESPACE_KW,
    SyntaxKind::WITHIN_KW,
    SyntaxKind::WITHOUT_KW,
    SyntaxKind::WORK_KW,
    SyntaxKind::WRAPPER_KW,
    SyntaxKind::WRITE_KW,
    SyntaxKind::XML_KW,
    SyntaxKind::XMLATTRIBUTES_KW,
    SyntaxKind::XMLCONCAT_KW,
    SyntaxKind::XMLELEMENT_KW,
    SyntaxKind::XMLEXISTS_KW,
    SyntaxKind::XMLFOREST_KW,
    SyntaxKind::XMLNAMESPACES_KW,
    SyntaxKind::XMLPARSE_KW,
    SyntaxKind::XMLPI_KW,
    SyntaxKind::XMLROOT_KW,
    SyntaxKind::XMLSERIALIZE_KW,
    SyntaxKind::XMLTABLE_KW,
    SyntaxKind::YEAR_KW,
    SyntaxKind::YES_KW,
    SyntaxKind::ZONE_KW,
]);

pub(crate) const ALL_KEYWORDS: TokenSet = TokenSet::new(&[
    SyntaxKind::ABORT_KW,
    SyntaxKind::ABSENT_KW,
    SyntaxKind::ABSOLUTE_KW,
    SyntaxKind::ACCESS_KW,
    SyntaxKind::ACTION_KW,
    SyntaxKind::ADD_KW,
    SyntaxKind::ADMIN_KW,
    SyntaxKind::AFTER_KW,
    SyntaxKind::AGGREGATE_KW,
    SyntaxKind::ALL_KW,
    SyntaxKind::ALSO_KW,
    SyntaxKind::ALTER_KW,
    SyntaxKind::ALWAYS_KW,
    SyntaxKind::ANALYSE_KW,
    SyntaxKind::ANALYZE_KW,
    SyntaxKind::AND_KW,
    SyntaxKind::ANY_KW,
    SyntaxKind::ARRAY_KW,
    SyntaxKind::AS_KW,
    SyntaxKind::ASC_KW,
    SyntaxKind::ASENSITIVE_KW,
    SyntaxKind::ASSERTION_KW,
    SyntaxKind::ASSIGNMENT_KW,
    SyntaxKind::ASYMMETRIC_KW,
    SyntaxKind::AT_KW,
    SyntaxKind::ATOMIC_KW,
    SyntaxKind::ATTACH_KW,
    SyntaxKind::ATTRIBUTE_KW,
    SyntaxKind::AUTHORIZATION_KW,
    SyntaxKind::BACKWARD_KW,
    SyntaxKind::BEFORE_KW,
    SyntaxKind::BEGIN_KW,
    SyntaxKind::BETWEEN_KW,
    SyntaxKind::BIGINT_KW,
    SyntaxKind::BINARY_KW,
    SyntaxKind::BIT_KW,
    SyntaxKind::BOOLEAN_KW,
    SyntaxKind::BOTH_KW,
    SyntaxKind::BREADTH_KW,
    SyntaxKind::BY_KW,
    SyntaxKind::CACHE_KW,
    SyntaxKind::CALL_KW,
    SyntaxKind::CALLED_KW,
    SyntaxKind::CASCADE_KW,
    SyntaxKind::CASCADED_KW,
    SyntaxKind::CASE_KW,
    SyntaxKind::CAST_KW,
    SyntaxKind::CATALOG_KW,
    SyntaxKind::CHAIN_KW,
    SyntaxKind::CHAR_KW,
    SyntaxKind::CHARACTER_KW,
    SyntaxKind::CHARACTERISTICS_KW,
    SyntaxKind::CHECK_KW,
    SyntaxKind::CHECKPOINT_KW,
    SyntaxKind::CLASS_KW,
    SyntaxKind::CLOSE_KW,
    SyntaxKind::CLUSTER_KW,
    SyntaxKind::COALESCE_KW,
    SyntaxKind::COLLATE_KW,
    SyntaxKind::COLLATION_KW,
    SyntaxKind::COLUMN_KW,
    SyntaxKind::COLUMNS_KW,
    SyntaxKind::COMMENT_KW,
    SyntaxKind::COMMENTS_KW,
    SyntaxKind::COMMIT_KW,
    SyntaxKind::COMMITTED_KW,
    SyntaxKind::COMPRESSION_KW,
    SyntaxKind::CONCURRENTLY_KW,
    SyntaxKind::CONDITIONAL_KW,
    SyntaxKind::CONFIGURATION_KW,
    SyntaxKind::CONFLICT_KW,
    SyntaxKind::CONNECTION_KW,
    SyntaxKind::CONSTRAINT_KW,
    SyntaxKind::CONSTRAINTS_KW,
    SyntaxKind::CONTENT_KW,
    SyntaxKind::CONTINUE_KW,
    SyntaxKind::CONVERSION_KW,
    SyntaxKind::COPY_KW,
    SyntaxKind::COST_KW,
    SyntaxKind::CREATE_KW,
    SyntaxKind::CROSS_KW,
    SyntaxKind::CSV_KW,
    SyntaxKind::CUBE_KW,
    SyntaxKind::CURRENT_KW,
    SyntaxKind::CURRENT_CATALOG_KW,
    SyntaxKind::CURRENT_DATE_KW,
    SyntaxKind::CURRENT_ROLE_KW,
    SyntaxKind::CURRENT_SCHEMA_KW,
    SyntaxKind::CURRENT_TIME_KW,
    SyntaxKind::CURRENT_TIMESTAMP_KW,
    SyntaxKind::CURRENT_USER_KW,
    SyntaxKind::CURSOR_KW,
    SyntaxKind::CYCLE_KW,
    SyntaxKind::DATA_KW,
    SyntaxKind::DATABASE_KW,
    SyntaxKind::DAY_KW,
    SyntaxKind::DEALLOCATE_KW,
    SyntaxKind::DEC_KW,
    SyntaxKind::DECIMAL_KW,
    SyntaxKind::DECLARE_KW,
    SyntaxKind::DEFAULT_KW,
    SyntaxKind::DEFAULTS_KW,
    SyntaxKind::DEFERRABLE_KW,
    SyntaxKind::DEFERRED_KW,
    SyntaxKind::DEFINER_KW,
    SyntaxKind::DELETE_KW,
    SyntaxKind::DELIMITER_KW,
    SyntaxKind::DELIMITERS_KW,
    SyntaxKind::DEPENDS_KW,
    SyntaxKind::DEPTH_KW,
    SyntaxKind::DESC_KW,
    SyntaxKind::DETACH_KW,
    SyntaxKind::DICTIONARY_KW,
    SyntaxKind::DISABLE_KW,
    SyntaxKind::DISCARD_KW,
    SyntaxKind::DISTINCT_KW,
    SyntaxKind::DO_KW,
    SyntaxKind::DOCUMENT_KW,
    SyntaxKind::DOMAIN_KW,
    SyntaxKind::DOUBLE_KW,
    SyntaxKind::DROP_KW,
    SyntaxKind::EACH_KW,
    SyntaxKind::ELSE_KW,
    SyntaxKind::EMPTY_KW,
    SyntaxKind::ENABLE_KW,
    SyntaxKind::ENCODING_KW,
    SyntaxKind::ENCRYPTED_KW,
    SyntaxKind::END_KW,
    SyntaxKind::ENUM_KW,
    SyntaxKind::ERROR_KW,
    SyntaxKind::ESCAPE_KW,
    SyntaxKind::EVENT_KW,
    SyntaxKind::EXCEPT_KW,
    SyntaxKind::EXCLUDE_KW,
    SyntaxKind::EXCLUDING_KW,
    SyntaxKind::EXCLUSIVE_KW,
    SyntaxKind::EXECUTE_KW,
    SyntaxKind::EXISTS_KW,
    SyntaxKind::EXPLAIN_KW,
    SyntaxKind::EXPRESSION_KW,
    SyntaxKind::EXTENSION_KW,
    SyntaxKind::EXTERNAL_KW,
    SyntaxKind::EXTRACT_KW,
    SyntaxKind::FALSE_KW,
    SyntaxKind::FAMILY_KW,
    SyntaxKind::FETCH_KW,
    SyntaxKind::FILTER_KW,
    SyntaxKind::FINALIZE_KW,
    SyntaxKind::FIRST_KW,
    SyntaxKind::FLOAT_KW,
    SyntaxKind::FOLLOWING_KW,
    SyntaxKind::FOR_KW,
    SyntaxKind::FORCE_KW,
    SyntaxKind::FOREIGN_KW,
    SyntaxKind::FORMAT_KW,
    SyntaxKind::FORWARD_KW,
    SyntaxKind::FREEZE_KW,
    SyntaxKind::FROM_KW,
    SyntaxKind::FULL_KW,
    SyntaxKind::FUNCTION_KW,
    SyntaxKind::FUNCTIONS_KW,
    SyntaxKind::GENERATED_KW,
    SyntaxKind::GLOBAL_KW,
    SyntaxKind::GRANT_KW,
    SyntaxKind::GRANTED_KW,
    SyntaxKind::GREATEST_KW,
    SyntaxKind::GROUP_KW,
    SyntaxKind::GROUPING_KW,
    SyntaxKind::GROUPS_KW,
    SyntaxKind::HANDLER_KW,
    SyntaxKind::HAVING_KW,
    SyntaxKind::HEADER_KW,
    SyntaxKind::HOLD_KW,
    SyntaxKind::HOUR_KW,
    SyntaxKind::IDENTITY_KW,
    SyntaxKind::IF_KW,
    SyntaxKind::ILIKE_KW,
    SyntaxKind::IMMEDIATE_KW,
    SyntaxKind::IMMUTABLE_KW,
    SyntaxKind::IMPLICIT_KW,
    SyntaxKind::IMPORT_KW,
    SyntaxKind::IN_KW,
    SyntaxKind::INCLUDE_KW,
    SyntaxKind::INCLUDING_KW,
    SyntaxKind::INCREMENT_KW,
    SyntaxKind::INDENT_KW,
    SyntaxKind::INDEX_KW,
    SyntaxKind::INDEXES_KW,
    SyntaxKind::INHERIT_KW,
    SyntaxKind::INHERITS_KW,
    SyntaxKind::INITIALLY_KW,
    SyntaxKind::INLINE_KW,
    SyntaxKind::INNER_KW,
    SyntaxKind::INOUT_KW,
    SyntaxKind::INPUT_KW,
    SyntaxKind::INSENSITIVE_KW,
    SyntaxKind::INSERT_KW,
    SyntaxKind::INSTEAD_KW,
    SyntaxKind::INT_KW,
    SyntaxKind::INTEGER_KW,
    SyntaxKind::INTERSECT_KW,
    SyntaxKind::INTERVAL_KW,
    SyntaxKind::INTO_KW,
    SyntaxKind::INVOKER_KW,
    SyntaxKind::IS_KW,
    SyntaxKind::ISNULL_KW,
    SyntaxKind::ISOLATION_KW,
    SyntaxKind::JOIN_KW,
    SyntaxKind::JSON_KW,
    SyntaxKind::JSON_ARRAY_KW,
    SyntaxKind::JSON_ARRAYAGG_KW,
    SyntaxKind::JSON_EXISTS_KW,
    SyntaxKind::JSON_OBJECT_KW,
    SyntaxKind::JSON_OBJECTAGG_KW,
    SyntaxKind::JSON_QUERY_KW,
    SyntaxKind::JSON_SCALAR_KW,
    SyntaxKind::JSON_SERIALIZE_KW,
    SyntaxKind::JSON_TABLE_KW,
    SyntaxKind::JSON_VALUE_KW,
    SyntaxKind::KEEP_KW,
    SyntaxKind::KEY_KW,
    SyntaxKind::KEYS_KW,
    SyntaxKind::LABEL_KW,
    SyntaxKind::LANGUAGE_KW,
    SyntaxKind::LARGE_KW,
    SyntaxKind::LAST_KW,
    SyntaxKind::LATERAL_KW,
    SyntaxKind::LEADING_KW,
    SyntaxKind::LEAKPROOF_KW,
    SyntaxKind::LEAST_KW,
    SyntaxKind::LEFT_KW,
    SyntaxKind::LEVEL_KW,
    SyntaxKind::LIKE_KW,
    SyntaxKind::LIMIT_KW,
    SyntaxKind::LISTEN_KW,
    SyntaxKind::LOAD_KW,
    SyntaxKind::LOCAL_KW,
    SyntaxKind::LOCALTIME_KW,
    SyntaxKind::LOCALTIMESTAMP_KW,
    SyntaxKind::LOCATION_KW,
    SyntaxKind::LOCK_KW,
    SyntaxKind::LOCKED_KW,
    SyntaxKind::LOGGED_KW,
    SyntaxKind::MAPPING_KW,
    SyntaxKind::MATCH_KW,
    SyntaxKind::MATCHED_KW,
    SyntaxKind::MATERIALIZED_KW,
    SyntaxKind::MAXVALUE_KW,
    SyntaxKind::MERGE_KW,
    SyntaxKind::MERGE_ACTION_KW,
    SyntaxKind::METHOD_KW,
    SyntaxKind::MINUTE_KW,
    SyntaxKind::MINVALUE_KW,
    SyntaxKind::MODE_KW,
    SyntaxKind::MONTH_KW,
    SyntaxKind::MOVE_KW,
    SyntaxKind::NAME_KW,
    SyntaxKind::NAMES_KW,
    SyntaxKind::NATIONAL_KW,
    SyntaxKind::NATURAL_KW,
    SyntaxKind::NCHAR_KW,
    SyntaxKind::NESTED_KW,
    SyntaxKind::NEW_KW,
    SyntaxKind::NEXT_KW,
    SyntaxKind::NFC_KW,
    SyntaxKind::NFD_KW,
    SyntaxKind::NFKC_KW,
    SyntaxKind::NFKD_KW,
    SyntaxKind::NO_KW,
    SyntaxKind::NONE_KW,
    SyntaxKind::NORMALIZE_KW,
    SyntaxKind::NORMALIZED_KW,
    SyntaxKind::NOT_KW,
    SyntaxKind::NOTHING_KW,
    SyntaxKind::NOTIFY_KW,
    SyntaxKind::NOTNULL_KW,
    SyntaxKind::NOWAIT_KW,
    SyntaxKind::NULL_KW,
    SyntaxKind::NULLIF_KW,
    SyntaxKind::NULLS_KW,
    SyntaxKind::NUMERIC_KW,
    SyntaxKind::OBJECT_KW,
    SyntaxKind::OF_KW,
    SyntaxKind::OFF_KW,
    SyntaxKind::OFFSET_KW,
    SyntaxKind::OIDS_KW,
    SyntaxKind::OLD_KW,
    SyntaxKind::OMIT_KW,
    SyntaxKind::ON_KW,
    SyntaxKind::ONLY_KW,
    SyntaxKind::OPERATOR_KW,
    SyntaxKind::OPTION_KW,
    SyntaxKind::OPTIONS_KW,
    SyntaxKind::OR_KW,
    SyntaxKind::ORDER_KW,
    SyntaxKind::ORDINALITY_KW,
    SyntaxKind::OTHERS_KW,
    SyntaxKind::OUT_KW,
    SyntaxKind::OUTER_KW,
    SyntaxKind::OVER_KW,
    SyntaxKind::OVERLAPS_KW,
    SyntaxKind::OVERLAY_KW,
    SyntaxKind::OVERRIDING_KW,
    SyntaxKind::OWNED_KW,
    SyntaxKind::OWNER_KW,
    SyntaxKind::PARALLEL_KW,
    SyntaxKind::PARAMETER_KW,
    SyntaxKind::PARSER_KW,
    SyntaxKind::PARTIAL_KW,
    SyntaxKind::PARTITION_KW,
    SyntaxKind::PASSING_KW,
    SyntaxKind::PASSWORD_KW,
    SyntaxKind::PATH_KW,
    SyntaxKind::PERIOD_KW,
    SyntaxKind::PLACING_KW,
    SyntaxKind::PLAN_KW,
    SyntaxKind::PLANS_KW,
    SyntaxKind::POLICY_KW,
    SyntaxKind::POSITION_KW,
    SyntaxKind::PRECEDING_KW,
    SyntaxKind::PRECISION_KW,
    SyntaxKind::PREPARE_KW,
    SyntaxKind::PREPARED_KW,
    SyntaxKind::PRESERVE_KW,
    SyntaxKind::PRIMARY_KW,
    SyntaxKind::PRIOR_KW,
    SyntaxKind::PRIVILEGES_KW,
    SyntaxKind::PROCEDURAL_KW,
    SyntaxKind::PROCEDURE_KW,
    SyntaxKind::PROCEDURES_KW,
    SyntaxKind::PROGRAM_KW,
    SyntaxKind::PUBLICATION_KW,
    SyntaxKind::QUOTE_KW,
    SyntaxKind::QUOTES_KW,
    SyntaxKind::RANGE_KW,
    SyntaxKind::READ_KW,
    SyntaxKind::REAL_KW,
    SyntaxKind::REASSIGN_KW,
    SyntaxKind::RECURSIVE_KW,
    SyntaxKind::REF_KW,
    SyntaxKind::REFERENCES_KW,
    SyntaxKind::REFERENCING_KW,
    SyntaxKind::REFRESH_KW,
    SyntaxKind::REINDEX_KW,
    SyntaxKind::RELATIVE_KW,
    SyntaxKind::RELEASE_KW,
    SyntaxKind::RENAME_KW,
    SyntaxKind::REPEATABLE_KW,
    SyntaxKind::REPLACE_KW,
    SyntaxKind::REPLICA_KW,
    SyntaxKind::RESET_KW,
    SyntaxKind::RESTART_KW,
    SyntaxKind::RESTRICT_KW,
    SyntaxKind::RETURN_KW,
    SyntaxKind::RETURNING_KW,
    SyntaxKind::RETURNS_KW,
    SyntaxKind::REVOKE_KW,
    SyntaxKind::RIGHT_KW,
    SyntaxKind::ROLE_KW,
    SyntaxKind::ROLLBACK_KW,
    SyntaxKind::ROLLUP_KW,
    SyntaxKind::ROUTINE_KW,
    SyntaxKind::ROUTINES_KW,
    SyntaxKind::ROW_KW,
    SyntaxKind::ROWS_KW,
    SyntaxKind::RULE_KW,
    SyntaxKind::SAVEPOINT_KW,
    SyntaxKind::SCALAR_KW,
    SyntaxKind::SCHEMA_KW,
    SyntaxKind::SCHEMAS_KW,
    SyntaxKind::SCROLL_KW,
    SyntaxKind::SEARCH_KW,
    SyntaxKind::SECOND_KW,
    SyntaxKind::SECURITY_KW,
    SyntaxKind::SELECT_KW,
    SyntaxKind::SEQUENCE_KW,
    SyntaxKind::SEQUENCES_KW,
    SyntaxKind::SERIALIZABLE_KW,
    SyntaxKind::SERVER_KW,
    SyntaxKind::SESSION_KW,
    SyntaxKind::SESSION_USER_KW,
    SyntaxKind::SET_KW,
    SyntaxKind::SETOF_KW,
    SyntaxKind::SETS_KW,
    SyntaxKind::SHARE_KW,
    SyntaxKind::SHOW_KW,
    SyntaxKind::SIMILAR_KW,
    SyntaxKind::SIMPLE_KW,
    SyntaxKind::SKIP_KW,
    SyntaxKind::SMALLINT_KW,
    SyntaxKind::SNAPSHOT_KW,
    SyntaxKind::SOME_KW,
    SyntaxKind::SOURCE_KW,
    SyntaxKind::SQL_KW,
    SyntaxKind::STABLE_KW,
    SyntaxKind::STANDALONE_KW,
    SyntaxKind::START_KW,
    SyntaxKind::STATEMENT_KW,
    SyntaxKind::STATISTICS_KW,
    SyntaxKind::STDIN_KW,
    SyntaxKind::STDOUT_KW,
    SyntaxKind::STORAGE_KW,
    SyntaxKind::STORED_KW,
    SyntaxKind::STRICT_KW,
    SyntaxKind::STRING_KW,
    SyntaxKind::STRIP_KW,
    SyntaxKind::SUBSCRIPTION_KW,
    SyntaxKind::SUBSTRING_KW,
    SyntaxKind::SUPPORT_KW,
    SyntaxKind::SYMMETRIC_KW,
    SyntaxKind::SYSID_KW,
    SyntaxKind::SYSTEM_KW,
    SyntaxKind::SYSTEM_USER_KW,
    SyntaxKind::TABLE_KW,
    SyntaxKind::TABLES_KW,
    SyntaxKind::TABLESAMPLE_KW,
    SyntaxKind::TABLESPACE_KW,
    SyntaxKind::TARGET_KW,
    SyntaxKind::TEMP_KW,
    SyntaxKind::TEMPLATE_KW,
    SyntaxKind::TEMPORARY_KW,
    SyntaxKind::TEXT_KW,
    SyntaxKind::THEN_KW,
    SyntaxKind::TIES_KW,
    SyntaxKind::TIME_KW,
    SyntaxKind::TIMESTAMP_KW,
    SyntaxKind::TO_KW,
    SyntaxKind::TRAILING_KW,
    SyntaxKind::TRANSACTION_KW,
    SyntaxKind::TRANSFORM_KW,
    SyntaxKind::TREAT_KW,
    SyntaxKind::TRIGGER_KW,
    SyntaxKind::TRIM_KW,
    SyntaxKind::TRUE_KW,
    SyntaxKind::TRUNCATE_KW,
    SyntaxKind::TRUSTED_KW,
    SyntaxKind::TYPE_KW,
    SyntaxKind::TYPES_KW,
    SyntaxKind::UESCAPE_KW,
    SyntaxKind::UNBOUNDED_KW,
    SyntaxKind::UNCOMMITTED_KW,
    SyntaxKind::UNCONDITIONAL_KW,
    SyntaxKind::UNENCRYPTED_KW,
    SyntaxKind::UNION_KW,
    SyntaxKind::UNIQUE_KW,
    SyntaxKind::UNKNOWN_KW,
    SyntaxKind::UNLISTEN_KW,
    SyntaxKind::UNLOGGED_KW,
    SyntaxKind::UNTIL_KW,
    SyntaxKind::UPDATE_KW,
    SyntaxKind::USER_KW,
    SyntaxKind::USING_KW,
    SyntaxKind::VACUUM_KW,
    SyntaxKind::VALID_KW,
    SyntaxKind::VALIDATE_KW,
    SyntaxKind::VALIDATOR_KW,
    SyntaxKind::VALUE_KW,
    SyntaxKind::VALUES_KW,
    SyntaxKind::VARCHAR_KW,
    SyntaxKind::VARIADIC_KW,
    SyntaxKind::VARYING_KW,
    SyntaxKind::VERBOSE_KW,
    SyntaxKind::VERSION_KW,
    SyntaxKind::VIEW_KW,
    SyntaxKind::VIEWS_KW,
    SyntaxKind::VOLATILE_KW,
    SyntaxKind::WHEN_KW,
    SyntaxKind::WHERE_KW,
    SyntaxKind::WHITESPACE_KW,
    SyntaxKind::WINDOW_KW,
    SyntaxKind::WITH_KW,
    SyntaxKind::WITHIN_KW,
    SyntaxKind::WITHOUT_KW,
    SyntaxKind::WORK_KW,
    SyntaxKind::WRAPPER_KW,
    SyntaxKind::WRITE_KW,
    SyntaxKind::XML_KW,
    SyntaxKind::XMLATTRIBUTES_KW,
    SyntaxKind::XMLCONCAT_KW,
    SyntaxKind::XMLELEMENT_KW,
    SyntaxKind::XMLEXISTS_KW,
    SyntaxKind::XMLFOREST_KW,
    SyntaxKind::XMLNAMESPACES_KW,
    SyntaxKind::XMLPARSE_KW,
    SyntaxKind::XMLPI_KW,
    SyntaxKind::XMLROOT_KW,
    SyntaxKind::XMLSERIALIZE_KW,
    SyntaxKind::XMLTABLE_KW,
    SyntaxKind::YEAR_KW,
    SyntaxKind::YES_KW,
    SyntaxKind::ZONE_KW,
]);

pub(crate) const BARE_LABEL_KEYWORDS: TokenSet = TokenSet::new(&[
    SyntaxKind::ABORT_KW,
    SyntaxKind::ABSENT_KW,
    SyntaxKind::ABSOLUTE_KW,
    SyntaxKind::ACCESS_KW,
    SyntaxKind::ACTION_KW,
    SyntaxKind::ADD_KW,
    SyntaxKind::ADMIN_KW,
    SyntaxKind::AFTER_KW,
    SyntaxKind::AGGREGATE_KW,
    SyntaxKind::ALL_KW,
    SyntaxKind::ALSO_KW,
    SyntaxKind::ALTER_KW,
    SyntaxKind::ALWAYS_KW,
    SyntaxKind::ANALYSE_KW,
    SyntaxKind::ANALYZE_KW,
    SyntaxKind::AND_KW,
    SyntaxKind::ANY_KW,
    SyntaxKind::ASC_KW,
    SyntaxKind::ASENSITIVE_KW,
    SyntaxKind::ASSERTION_KW,
    SyntaxKind::ASSIGNMENT_KW,
    SyntaxKind::ASYMMETRIC_KW,
    SyntaxKind::AT_KW,
    SyntaxKind::ATOMIC_KW,
    SyntaxKind::ATTACH_KW,
    SyntaxKind::ATTRIBUTE_KW,
    SyntaxKind::AUTHORIZATION_KW,
    SyntaxKind::BACKWARD_KW,
    SyntaxKind::BEFORE_KW,
    SyntaxKind::BEGIN_KW,
    SyntaxKind::BETWEEN_KW,
    SyntaxKind::BIGINT_KW,
    SyntaxKind::BINARY_KW,
    SyntaxKind::BIT_KW,
    SyntaxKind::BOOLEAN_KW,
    SyntaxKind::BOTH_KW,
    SyntaxKind::BREADTH_KW,
    SyntaxKind::BY_KW,
    SyntaxKind::CACHE_KW,
    SyntaxKind::CALL_KW,
    SyntaxKind::CALLED_KW,
    SyntaxKind::CASCADE_KW,
    SyntaxKind::CASCADED_KW,
    SyntaxKind::CASE_KW,
    SyntaxKind::CAST_KW,
    SyntaxKind::CATALOG_KW,
    SyntaxKind::CHAIN_KW,
    SyntaxKind::CHARACTERISTICS_KW,
    SyntaxKind::CHECK_KW,
    SyntaxKind::CHECKPOINT_KW,
    SyntaxKind::CLASS_KW,
    SyntaxKind::CLOSE_KW,
    SyntaxKind::CLUSTER_KW,
    SyntaxKind::COALESCE_KW,
    SyntaxKind::COLLATE_KW,
    SyntaxKind::COLLATION_KW,
    SyntaxKind::COLUMN_KW,
    SyntaxKind::COLUMNS_KW,
    SyntaxKind::COMMENT_KW,
    SyntaxKind::COMMENTS_KW,
    SyntaxKind::COMMIT_KW,
    SyntaxKind::COMMITTED_KW,
    SyntaxKind::COMPRESSION_KW,
    SyntaxKind::CONCURRENTLY_KW,
    SyntaxKind::CONDITIONAL_KW,
    SyntaxKind::CONFIGURATION_KW,
    SyntaxKind::CONFLICT_KW,
    SyntaxKind::CONNECTION_KW,
    SyntaxKind::CONSTRAINT_KW,
    SyntaxKind::CONSTRAINTS_KW,
    SyntaxKind::CONTENT_KW,
    SyntaxKind::CONTINUE_KW,
    SyntaxKind::CONVERSION_KW,
    SyntaxKind::COPY_KW,
    SyntaxKind::COST_KW,
    SyntaxKind::CROSS_KW,
    SyntaxKind::CSV_KW,
    SyntaxKind::CUBE_KW,
    SyntaxKind::CURRENT_KW,
    SyntaxKind::CURRENT_CATALOG_KW,
    SyntaxKind::CURRENT_DATE_KW,
    SyntaxKind::CURRENT_ROLE_KW,
    SyntaxKind::CURRENT_SCHEMA_KW,
    SyntaxKind::CURRENT_TIME_KW,
    SyntaxKind::CURRENT_TIMESTAMP_KW,
    SyntaxKind::CURRENT_USER_KW,
    SyntaxKind::CURSOR_KW,
    SyntaxKind::CYCLE_KW,
    SyntaxKind::DATA_KW,
    SyntaxKind::DATABASE_KW,
    SyntaxKind::DEALLOCATE_KW,
    SyntaxKind::DEC_KW,
    SyntaxKind::DECIMAL_KW,
    SyntaxKind::DECLARE_KW,
    SyntaxKind::DEFAULT_KW,
    SyntaxKind::DEFAULTS_KW,
    SyntaxKind::DEFERRABLE_KW,
    SyntaxKind::DEFERRED_KW,
    SyntaxKind::DEFINER_KW,
    SyntaxKind::DELETE_KW,
    SyntaxKind::DELIMITER_KW,
    SyntaxKind::DELIMITERS_KW,
    SyntaxKind::DEPENDS_KW,
    SyntaxKind::DEPTH_KW,
    SyntaxKind::DESC_KW,
    SyntaxKind::DETACH_KW,
    SyntaxKind::DICTIONARY_KW,
    SyntaxKind::DISABLE_KW,
    SyntaxKind::DISCARD_KW,
    SyntaxKind::DISTINCT_KW,
    SyntaxKind::DO_KW,
    SyntaxKind::DOCUMENT_KW,
    SyntaxKind::DOMAIN_KW,
    SyntaxKind::DOUBLE_KW,
    SyntaxKind::DROP_KW,
    SyntaxKind::EACH_KW,
    SyntaxKind::ELSE_KW,
    SyntaxKind::EMPTY_KW,
    SyntaxKind::ENABLE_KW,
    SyntaxKind::ENCODING_KW,
    SyntaxKind::ENCRYPTED_KW,
    SyntaxKind::END_KW,
    SyntaxKind::ENUM_KW,
    SyntaxKind::ERROR_KW,
    SyntaxKind::ESCAPE_KW,
    SyntaxKind::EVENT_KW,
    SyntaxKind::EXCLUDE_KW,
    SyntaxKind::EXCLUDING_KW,
    SyntaxKind::EXCLUSIVE_KW,
    SyntaxKind::EXECUTE_KW,
    SyntaxKind::EXISTS_KW,
    SyntaxKind::EXPLAIN_KW,
    SyntaxKind::EXPRESSION_KW,
    SyntaxKind::EXTENSION_KW,
    SyntaxKind::EXTERNAL_KW,
    SyntaxKind::EXTRACT_KW,
    SyntaxKind::FALSE_KW,
    SyntaxKind::FAMILY_KW,
    SyntaxKind::FINALIZE_KW,
    SyntaxKind::FIRST_KW,
    SyntaxKind::FLOAT_KW,
    SyntaxKind::FOLLOWING_KW,
    SyntaxKind::FORCE_KW,
    SyntaxKind::FOREIGN_KW,
    SyntaxKind::FORMAT_KW,
    SyntaxKind::FORWARD_KW,
    SyntaxKind::FREEZE_KW,
    SyntaxKind::FULL_KW,
    SyntaxKind::FUNCTION_KW,
    SyntaxKind::FUNCTIONS_KW,
    SyntaxKind::GENERATED_KW,
    SyntaxKind::GLOBAL_KW,
    SyntaxKind::GRANTED_KW,
    SyntaxKind::GREATEST_KW,
    SyntaxKind::GROUPING_KW,
    SyntaxKind::GROUPS_KW,
    SyntaxKind::HANDLER_KW,
    SyntaxKind::HEADER_KW,
    SyntaxKind::HOLD_KW,
    SyntaxKind::IDENTITY_KW,
    SyntaxKind::IF_KW,
    SyntaxKind::ILIKE_KW,
    SyntaxKind::IMMEDIATE_KW,
    SyntaxKind::IMMUTABLE_KW,
    SyntaxKind::IMPLICIT_KW,
    SyntaxKind::IMPORT_KW,
    SyntaxKind::IN_KW,
    SyntaxKind::INCLUDE_KW,
    SyntaxKind::INCLUDING_KW,
    SyntaxKind::INCREMENT_KW,
    SyntaxKind::INDENT_KW,
    SyntaxKind::INDEX_KW,
    SyntaxKind::INDEXES_KW,
    SyntaxKind::INHERIT_KW,
    SyntaxKind::INHERITS_KW,
    SyntaxKind::INITIALLY_KW,
    SyntaxKind::INLINE_KW,
    SyntaxKind::INNER_KW,
    SyntaxKind::INOUT_KW,
    SyntaxKind::INPUT_KW,
    SyntaxKind::INSENSITIVE_KW,
    SyntaxKind::INSERT_KW,
    SyntaxKind::INSTEAD_KW,
    SyntaxKind::INT_KW,
    SyntaxKind::INTEGER_KW,
    SyntaxKind::INTERVAL_KW,
    SyntaxKind::INVOKER_KW,
    SyntaxKind::IS_KW,
    SyntaxKind::ISOLATION_KW,
    SyntaxKind::JOIN_KW,
    SyntaxKind::JSON_KW,
    SyntaxKind::JSON_ARRAY_KW,
    SyntaxKind::JSON_ARRAYAGG_KW,
    SyntaxKind::JSON_EXISTS_KW,
    SyntaxKind::JSON_OBJECT_KW,
    SyntaxKind::JSON_OBJECTAGG_KW,
    SyntaxKind::JSON_QUERY_KW,
    SyntaxKind::JSON_SCALAR_KW,
    SyntaxKind::JSON_SERIALIZE_KW,
    SyntaxKind::JSON_TABLE_KW,
    SyntaxKind::JSON_VALUE_KW,
    SyntaxKind::KEEP_KW,
    SyntaxKind::KEY_KW,
    SyntaxKind::KEYS_KW,
    SyntaxKind::LABEL_KW,
    SyntaxKind::LANGUAGE_KW,
    SyntaxKind::LARGE_KW,
    SyntaxKind::LAST_KW,
    SyntaxKind::LATERAL_KW,
    SyntaxKind::LEADING_KW,
    SyntaxKind::LEAKPROOF_KW,
    SyntaxKind::LEAST_KW,
    SyntaxKind::LEFT_KW,
    SyntaxKind::LEVEL_KW,
    SyntaxKind::LIKE_KW,
    SyntaxKind::LISTEN_KW,
    SyntaxKind::LOAD_KW,
    SyntaxKind::LOCAL_KW,
    SyntaxKind::LOCALTIME_KW,
    SyntaxKind::LOCALTIMESTAMP_KW,
    SyntaxKind::LOCATION_KW,
    SyntaxKind::LOCK_KW,
    SyntaxKind::LOCKED_KW,
    SyntaxKind::LOGGED_KW,
    SyntaxKind::MAPPING_KW,
    SyntaxKind::MATCH_KW,
    SyntaxKind::MATCHED_KW,
    SyntaxKind::MATERIALIZED_KW,
    SyntaxKind::MAXVALUE_KW,
    SyntaxKind::MERGE_KW,
    SyntaxKind::MERGE_ACTION_KW,
    SyntaxKind::METHOD_KW,
    SyntaxKind::MINVALUE_KW,
    SyntaxKind::MODE_KW,
    SyntaxKind::MOVE_KW,
    SyntaxKind::NAME_KW,
    SyntaxKind::NAMES_KW,
    SyntaxKind::NATIONAL_KW,
    SyntaxKind::NATURAL_KW,
    SyntaxKind::NCHAR_KW,
    SyntaxKind::NESTED_KW,
    SyntaxKind::NEW_KW,
    SyntaxKind::NEXT_KW,
    SyntaxKind::NFC_KW,
    SyntaxKind::NFD_KW,
    SyntaxKind::NFKC_KW,
    SyntaxKind::NFKD_KW,
    SyntaxKind::NO_KW,
    SyntaxKind::NONE_KW,
    SyntaxKind::NORMALIZE_KW,
    SyntaxKind::NORMALIZED_KW,
    SyntaxKind::NOT_KW,
    SyntaxKind::NOTHING_KW,
    SyntaxKind::NOTIFY_KW,
    SyntaxKind::NOWAIT_KW,
    SyntaxKind::NULL_KW,
    SyntaxKind::NULLIF_KW,
    SyntaxKind::NULLS_KW,
    SyntaxKind::NUMERIC_KW,
    SyntaxKind::OBJECT_KW,
    SyntaxKind::OF_KW,
    SyntaxKind::OFF_KW,
    SyntaxKind::OIDS_KW,
    SyntaxKind::OLD_KW,
    SyntaxKind::OMIT_KW,
    SyntaxKind::ONLY_KW,
    SyntaxKind::OPERATOR_KW,
    SyntaxKind::OPTION_KW,
    SyntaxKind::OPTIONS_KW,
    SyntaxKind::OR_KW,
    SyntaxKind::ORDINALITY_KW,
    SyntaxKind::OTHERS_KW,
    SyntaxKind::OUT_KW,
    SyntaxKind::OUTER_KW,
    SyntaxKind::OVERLAY_KW,
    SyntaxKind::OVERRIDING_KW,
    SyntaxKind::OWNED_KW,
    SyntaxKind::OWNER_KW,
    SyntaxKind::PARALLEL_KW,
    SyntaxKind::PARAMETER_KW,
    SyntaxKind::PARSER_KW,
    SyntaxKind::PARTIAL_KW,
    SyntaxKind::PARTITION_KW,
    SyntaxKind::PASSING_KW,
    SyntaxKind::PASSWORD_KW,
    SyntaxKind::PATH_KW,
    SyntaxKind::PERIOD_KW,
    SyntaxKind::PLACING_KW,
    SyntaxKind::PLAN_KW,
    SyntaxKind::PLANS_KW,
    SyntaxKind::POLICY_KW,
    SyntaxKind::POSITION_KW,
    SyntaxKind::PRECEDING_KW,
    SyntaxKind::PREPARE_KW,
    SyntaxKind::PREPARED_KW,
    SyntaxKind::PRESERVE_KW,
    SyntaxKind::PRIMARY_KW,
    SyntaxKind::PRIOR_KW,
    SyntaxKind::PRIVILEGES_KW,
    SyntaxKind::PROCEDURAL_KW,
    SyntaxKind::PROCEDURE_KW,
    SyntaxKind::PROCEDURES_KW,
    SyntaxKind::PROGRAM_KW,
    SyntaxKind::PUBLICATION_KW,
    SyntaxKind::QUOTE_KW,
    SyntaxKind::QUOTES_KW,
    SyntaxKind::RANGE_KW,
    SyntaxKind::READ_KW,
    SyntaxKind::REAL_KW,
    SyntaxKind::REASSIGN_KW,
    SyntaxKind::RECURSIVE_KW,
    SyntaxKind::REF_KW,
    SyntaxKind::REFERENCES_KW,
    SyntaxKind::REFERENCING_KW,
    SyntaxKind::REFRESH_KW,
    SyntaxKind::REINDEX_KW,
    SyntaxKind::RELATIVE_KW,
    SyntaxKind::RELEASE_KW,
    SyntaxKind::RENAME_KW,
    SyntaxKind::REPEATABLE_KW,
    SyntaxKind::REPLACE_KW,
    SyntaxKind::REPLICA_KW,
    SyntaxKind::RESET_KW,
    SyntaxKind::RESTART_KW,
    SyntaxKind::RESTRICT_KW,
    SyntaxKind::RETURN_KW,
    SyntaxKind::RETURNS_KW,
    SyntaxKind::REVOKE_KW,
    SyntaxKind::RIGHT_KW,
    SyntaxKind::ROLE_KW,
    SyntaxKind::ROLLBACK_KW,
    SyntaxKind::ROLLUP_KW,
    SyntaxKind::ROUTINE_KW,
    SyntaxKind::ROUTINES_KW,
    SyntaxKind::ROW_KW,
    SyntaxKind::ROWS_KW,
    SyntaxKind::RULE_KW,
    SyntaxKind::SAVEPOINT_KW,
    SyntaxKind::SCALAR_KW,
    SyntaxKind::SCHEMA_KW,
    SyntaxKind::SCHEMAS_KW,
    SyntaxKind::SCROLL_KW,
    SyntaxKind::SEARCH_KW,
    SyntaxKind::SECURITY_KW,
    SyntaxKind::SELECT_KW,
    SyntaxKind::SEQUENCE_KW,
    SyntaxKind::SEQUENCES_KW,
    SyntaxKind::SERIALIZABLE_KW,
    SyntaxKind::SERVER_KW,
    SyntaxKind::SESSION_KW,
    SyntaxKind::SESSION_USER_KW,
    SyntaxKind::SET_KW,
    SyntaxKind::SETOF_KW,
    SyntaxKind::SETS_KW,
    SyntaxKind::SHARE_KW,
    SyntaxKind::SHOW_KW,
    SyntaxKind::SIMILAR_KW,
    SyntaxKind::SIMPLE_KW,
    SyntaxKind::SKIP_KW,
    SyntaxKind::SMALLINT_KW,
    SyntaxKind::SNAPSHOT_KW,
    SyntaxKind::SOME_KW,
    SyntaxKind::SOURCE_KW,
    SyntaxKind::SQL_KW,
    SyntaxKind::STABLE_KW,
    SyntaxKind::STANDALONE_KW,
    SyntaxKind::START_KW,
    SyntaxKind::STATEMENT_KW,
    SyntaxKind::STATISTICS_KW,
    SyntaxKind::STDIN_KW,
    SyntaxKind::STDOUT_KW,
    SyntaxKind::STORAGE_KW,
    SyntaxKind::STORED_KW,
    SyntaxKind::STRICT_KW,
    SyntaxKind::STRING_KW,
    SyntaxKind::STRIP_KW,
    SyntaxKind::SUBSCRIPTION_KW,
    SyntaxKind::SUBSTRING_KW,
    SyntaxKind::SUPPORT_KW,
    SyntaxKind::SYMMETRIC_KW,
    SyntaxKind::SYSID_KW,
    SyntaxKind::SYSTEM_KW,
    SyntaxKind::SYSTEM_USER_KW,
    SyntaxKind::TABLE_KW,
    SyntaxKind::TABLES_KW,
    SyntaxKind::TABLESAMPLE_KW,
    SyntaxKind::TABLESPACE_KW,
    SyntaxKind::TARGET_KW,
    SyntaxKind::TEMP_KW,
    SyntaxKind::TEMPLATE_KW,
    SyntaxKind::TEMPORARY_KW,
    SyntaxKind::TEXT_KW,
    SyntaxKind::THEN_KW,
    SyntaxKind::TIES_KW,
    SyntaxKind::TIME_KW,
    SyntaxKind::TIMESTAMP_KW,
    SyntaxKind::TRAILING_KW,
    SyntaxKind::TRANSACTION_KW,
    SyntaxKind::TRANSFORM_KW,
    SyntaxKind::TREAT_KW,
    SyntaxKind::TRIGGER_KW,
    SyntaxKind::TRIM_KW,
    SyntaxKind::TRUE_KW,
    SyntaxKind::TRUNCATE_KW,
    SyntaxKind::TRUSTED_KW,
    SyntaxKind::TYPE_KW,
    SyntaxKind::TYPES_KW,
    SyntaxKind::UESCAPE_KW,
    SyntaxKind::UNBOUNDED_KW,
    SyntaxKind::UNCOMMITTED_KW,
    SyntaxKind::UNCONDITIONAL_KW,
    SyntaxKind::UNENCRYPTED_KW,
    SyntaxKind::UNIQUE_KW,
    SyntaxKind::UNKNOWN_KW,
    SyntaxKind::UNLISTEN_KW,
    SyntaxKind::UNLOGGED_KW,
    SyntaxKind::UNTIL_KW,
    SyntaxKind::UPDATE_KW,
    SyntaxKind::USER_KW,
    SyntaxKind::USING_KW,
    SyntaxKind::VACUUM_KW,
    SyntaxKind::VALID_KW,
    SyntaxKind::VALIDATE_KW,
    SyntaxKind::VALIDATOR_KW,
    SyntaxKind::VALUE_KW,
    SyntaxKind::VALUES_KW,
    SyntaxKind::VARCHAR_KW,
    SyntaxKind::VARIADIC_KW,
    SyntaxKind::VERBOSE_KW,
    SyntaxKind::VERSION_KW,
    SyntaxKind::VIEW_KW,
    SyntaxKind::VIEWS_KW,
    SyntaxKind::VOLATILE_KW,
    SyntaxKind::WHEN_KW,
    SyntaxKind::WHITESPACE_KW,
    SyntaxKind::WORK_KW,
    SyntaxKind::WRAPPER_KW,
    SyntaxKind::WRITE_KW,
    SyntaxKind::XML_KW,
    SyntaxKind::XMLATTRIBUTES_KW,
    SyntaxKind::XMLCONCAT_KW,
    SyntaxKind::XMLELEMENT_KW,
    SyntaxKind::XMLEXISTS_KW,
    SyntaxKind::XMLFOREST_KW,
    SyntaxKind::XMLNAMESPACES_KW,
    SyntaxKind::XMLPARSE_KW,
    SyntaxKind::XMLPI_KW,
    SyntaxKind::XMLROOT_KW,
    SyntaxKind::XMLSERIALIZE_KW,
    SyntaxKind::XMLTABLE_KW,
    SyntaxKind::YES_KW,
    SyntaxKind::ZONE_KW,
]);

pub(crate) const UNRESERVED_KEYWORDS: TokenSet = TokenSet::new(&[
    SyntaxKind::ABORT_KW,
    SyntaxKind::ABSENT_KW,
    SyntaxKind::ABSOLUTE_KW,
    SyntaxKind::ACCESS_KW,
    SyntaxKind::ACTION_KW,
    SyntaxKind::ADD_KW,
    SyntaxKind::ADMIN_KW,
    SyntaxKind::AFTER_KW,
    SyntaxKind::AGGREGATE_KW,
    SyntaxKind::ALSO_KW,
    SyntaxKind::ALTER_KW,
    SyntaxKind::ALWAYS_KW,
    SyntaxKind::ASENSITIVE_KW,
    SyntaxKind::ASSERTION_KW,
    SyntaxKind::ASSIGNMENT_KW,
    SyntaxKind::AT_KW,
    SyntaxKind::ATOMIC_KW,
    SyntaxKind::ATTACH_KW,
    SyntaxKind::ATTRIBUTE_KW,
    SyntaxKind::BACKWARD_KW,
    SyntaxKind::BEFORE_KW,
    SyntaxKind::BEGIN_KW,
    SyntaxKind::BREADTH_KW,
    SyntaxKind::BY_KW,
    SyntaxKind::CACHE_KW,
    SyntaxKind::CALL_KW,
    SyntaxKind::CALLED_KW,
    SyntaxKind::CASCADE_KW,
    SyntaxKind::CASCADED_KW,
    SyntaxKind::CATALOG_KW,
    SyntaxKind::CHAIN_KW,
    SyntaxKind::CHARACTERISTICS_KW,
    SyntaxKind::CHECKPOINT_KW,
    SyntaxKind::CLASS_KW,
    SyntaxKind::CLOSE_KW,
    SyntaxKind::CLUSTER_KW,
    SyntaxKind::COLUMNS_KW,
    SyntaxKind::COMMENT_KW,
    SyntaxKind::COMMENTS_KW,
    SyntaxKind::COMMIT_KW,
    SyntaxKind::COMMITTED_KW,
    SyntaxKind::COMPRESSION_KW,
    SyntaxKind::CONDITIONAL_KW,
    SyntaxKind::CONFIGURATION_KW,
    SyntaxKind::CONFLICT_KW,
    SyntaxKind::CONNECTION_KW,
    SyntaxKind::CONSTRAINTS_KW,
    SyntaxKind::CONTENT_KW,
    SyntaxKind::CONTINUE_KW,
    SyntaxKind::CONVERSION_KW,
    SyntaxKind::COPY_KW,
    SyntaxKind::COST_KW,
    SyntaxKind::CSV_KW,
    SyntaxKind::CUBE_KW,
    SyntaxKind::CURRENT_KW,
    SyntaxKind::CURSOR_KW,
    SyntaxKind::CYCLE_KW,
    SyntaxKind::DATA_KW,
    SyntaxKind::DATABASE_KW,
    SyntaxKind::DAY_KW,
    SyntaxKind::DEALLOCATE_KW,
    SyntaxKind::DECLARE_KW,
    SyntaxKind::DEFAULTS_KW,
    SyntaxKind::DEFERRED_KW,
    SyntaxKind::DEFINER_KW,
    SyntaxKind::DELETE_KW,
    SyntaxKind::DELIMITER_KW,
    SyntaxKind::DELIMITERS_KW,
    SyntaxKind::DEPENDS_KW,
    SyntaxKind::DEPTH_KW,
    SyntaxKind::DETACH_KW,
    SyntaxKind::DICTIONARY_KW,
    SyntaxKind::DISABLE_KW,
    SyntaxKind::DISCARD_KW,
    SyntaxKind::DOCUMENT_KW,
    SyntaxKind::DOMAIN_KW,
    SyntaxKind::DOUBLE_KW,
    SyntaxKind::DROP_KW,
    SyntaxKind::EACH_KW,
    SyntaxKind::EMPTY_KW,
    SyntaxKind::ENABLE_KW,
    SyntaxKind::ENCODING_KW,
    SyntaxKind::ENCRYPTED_KW,
    SyntaxKind::ENUM_KW,
    SyntaxKind::ERROR_KW,
    SyntaxKind::ESCAPE_KW,
    SyntaxKind::EVENT_KW,
    SyntaxKind::EXCLUDE_KW,
    SyntaxKind::EXCLUDING_KW,
    SyntaxKind::EXCLUSIVE_KW,
    SyntaxKind::EXECUTE_KW,
    SyntaxKind::EXPLAIN_KW,
    SyntaxKind::EXPRESSION_KW,
    SyntaxKind::EXTENSION_KW,
    SyntaxKind::EXTERNAL_KW,
    SyntaxKind::FAMILY_KW,
    SyntaxKind::FILTER_KW,
    SyntaxKind::FINALIZE_KW,
    SyntaxKind::FIRST_KW,
    SyntaxKind::FOLLOWING_KW,
    SyntaxKind::FORCE_KW,
    SyntaxKind::FORMAT_KW,
    SyntaxKind::FORWARD_KW,
    SyntaxKind::FUNCTION_KW,
    SyntaxKind::FUNCTIONS_KW,
    SyntaxKind::GENERATED_KW,
    SyntaxKind::GLOBAL_KW,
    SyntaxKind::GRANTED_KW,
    SyntaxKind::GROUPS_KW,
    SyntaxKind::HANDLER_KW,
    SyntaxKind::HEADER_KW,
    SyntaxKind::HOLD_KW,
    SyntaxKind::HOUR_KW,
    SyntaxKind::IDENTITY_KW,
    SyntaxKind::IF_KW,
    SyntaxKind::IMMEDIATE_KW,
    SyntaxKind::IMMUTABLE_KW,
    SyntaxKind::IMPLICIT_KW,
    SyntaxKind::IMPORT_KW,
    SyntaxKind::INCLUDE_KW,
    SyntaxKind::INCLUDING_KW,
    SyntaxKind::INCREMENT_KW,
    SyntaxKind::INDENT_KW,
    SyntaxKind::INDEX_KW,
    SyntaxKind::INDEXES_KW,
    SyntaxKind::INHERIT_KW,
    SyntaxKind::INHERITS_KW,
    SyntaxKind::INLINE_KW,
    SyntaxKind::INPUT_KW,
    SyntaxKind::INSENSITIVE_KW,
    SyntaxKind::INSERT_KW,
    SyntaxKind::INSTEAD_KW,
    SyntaxKind::INVOKER_KW,
    SyntaxKind::ISOLATION_KW,
    SyntaxKind::KEEP_KW,
    SyntaxKind::KEY_KW,
    SyntaxKind::KEYS_KW,
    SyntaxKind::LABEL_KW,
    SyntaxKind::LANGUAGE_KW,
    SyntaxKind::LARGE_KW,
    SyntaxKind::LAST_KW,
    SyntaxKind::LEAKPROOF_KW,
    SyntaxKind::LEVEL_KW,
    SyntaxKind::LISTEN_KW,
    SyntaxKind::LOAD_KW,
    SyntaxKind::LOCAL_KW,
    SyntaxKind::LOCATION_KW,
    SyntaxKind::LOCK_KW,
    SyntaxKind::LOCKED_KW,
    SyntaxKind::LOGGED_KW,
    SyntaxKind::MAPPING_KW,
    SyntaxKind::MATCH_KW,
    SyntaxKind::MATCHED_KW,
    SyntaxKind::MATERIALIZED_KW,
    SyntaxKind::MAXVALUE_KW,
    SyntaxKind::MERGE_KW,
    SyntaxKind::METHOD_KW,
    SyntaxKind::MINUTE_KW,
    SyntaxKind::MINVALUE_KW,
    SyntaxKind::MODE_KW,
    SyntaxKind::MONTH_KW,
    SyntaxKind::MOVE_KW,
    SyntaxKind::NAME_KW,
    SyntaxKind::NAMES_KW,
    SyntaxKind::NESTED_KW,
    SyntaxKind::NEW_KW,
    SyntaxKind::NEXT_KW,
    SyntaxKind::NFC_KW,
    SyntaxKind::NFD_KW,
    SyntaxKind::NFKC_KW,
    SyntaxKind::NFKD_KW,
    SyntaxKind::NO_KW,
    SyntaxKind::NORMALIZED_KW,
    SyntaxKind::NOTHING_KW,
    SyntaxKind::NOTIFY_KW,
    SyntaxKind::NOWAIT_KW,
    SyntaxKind::NULLS_KW,
    SyntaxKind::OBJECT_KW,
    SyntaxKind::OF_KW,
    SyntaxKind::OFF_KW,
    SyntaxKind::OIDS_KW,
    SyntaxKind::OLD_KW,
    SyntaxKind::OMIT_KW,
    SyntaxKind::OPERATOR_KW,
    SyntaxKind::OPTION_KW,
    SyntaxKind::OPTIONS_KW,
    SyntaxKind::ORDINALITY_KW,
    SyntaxKind::OTHERS_KW,
    SyntaxKind::OVER_KW,
    SyntaxKind::OVERRIDING_KW,
    SyntaxKind::OWNED_KW,
    SyntaxKind::OWNER_KW,
    SyntaxKind::PARALLEL_KW,
    SyntaxKind::PARAMETER_KW,
    SyntaxKind::PARSER_KW,
    SyntaxKind::PARTIAL_KW,
    SyntaxKind::PARTITION_KW,
    SyntaxKind::PASSING_KW,
    SyntaxKind::PASSWORD_KW,
    SyntaxKind::PATH_KW,
    SyntaxKind::PERIOD_KW,
    SyntaxKind::PLAN_KW,
    SyntaxKind::PLANS_KW,
    SyntaxKind::POLICY_KW,
    SyntaxKind::PRECEDING_KW,
    SyntaxKind::PREPARE_KW,
    SyntaxKind::PREPARED_KW,
    SyntaxKind::PRESERVE_KW,
    SyntaxKind::PRIOR_KW,
    SyntaxKind::PRIVILEGES_KW,
    SyntaxKind::PROCEDURAL_KW,
    SyntaxKind::PROCEDURE_KW,
    SyntaxKind::PROCEDURES_KW,
    SyntaxKind::PROGRAM_KW,
    SyntaxKind::PUBLICATION_KW,
    SyntaxKind::QUOTE_KW,
    SyntaxKind::QUOTES_KW,
    SyntaxKind::RANGE_KW,
    SyntaxKind::READ_KW,
    SyntaxKind::REASSIGN_KW,
    SyntaxKind::RECURSIVE_KW,
    SyntaxKind::REF_KW,
    SyntaxKind::REFERENCING_KW,
    SyntaxKind::REFRESH_KW,
    SyntaxKind::REINDEX_KW,
    SyntaxKind::RELATIVE_KW,
    SyntaxKind::RELEASE_KW,
    SyntaxKind::RENAME_KW,
    SyntaxKind::REPEATABLE_KW,
    SyntaxKind::REPLACE_KW,
    SyntaxKind::REPLICA_KW,
    SyntaxKind::RESET_KW,
    SyntaxKind::RESTART_KW,
    SyntaxKind::RESTRICT_KW,
    SyntaxKind::RETURN_KW,
    SyntaxKind::RETURNS_KW,
    SyntaxKind::REVOKE_KW,
    SyntaxKind::ROLE_KW,
    SyntaxKind::ROLLBACK_KW,
    SyntaxKind::ROLLUP_KW,
    SyntaxKind::ROUTINE_KW,
    SyntaxKind::ROUTINES_KW,
    SyntaxKind::ROWS_KW,
    SyntaxKind::RULE_KW,
    SyntaxKind::SAVEPOINT_KW,
    SyntaxKind::SCALAR_KW,
    SyntaxKind::SCHEMA_KW,
    SyntaxKind::SCHEMAS_KW,
    SyntaxKind::SCROLL_KW,
    SyntaxKind::SEARCH_KW,
    SyntaxKind::SECOND_KW,
    SyntaxKind::SECURITY_KW,
    SyntaxKind::SEQUENCE_KW,
    SyntaxKind::SEQUENCES_KW,
    SyntaxKind::SERIALIZABLE_KW,
    SyntaxKind::SERVER_KW,
    SyntaxKind::SESSION_KW,
    SyntaxKind::SET_KW,
    SyntaxKind::SETS_KW,
    SyntaxKind::SHARE_KW,
    SyntaxKind::SHOW_KW,
    SyntaxKind::SIMPLE_KW,
    SyntaxKind::SKIP_KW,
    SyntaxKind::SNAPSHOT_KW,
    SyntaxKind::SOURCE_KW,
    SyntaxKind::SQL_KW,
    SyntaxKind::STABLE_KW,
    SyntaxKind::STANDALONE_KW,
    SyntaxKind::START_KW,
    SyntaxKind::STATEMENT_KW,
    SyntaxKind::STATISTICS_KW,
    SyntaxKind::STDIN_KW,
    SyntaxKind::STDOUT_KW,
    SyntaxKind::STORAGE_KW,
    SyntaxKind::STORED_KW,
    SyntaxKind::STRICT_KW,
    SyntaxKind::STRING_KW,
    SyntaxKind::STRIP_KW,
    SyntaxKind::SUBSCRIPTION_KW,
    SyntaxKind::SUPPORT_KW,
    SyntaxKind::SYSID_KW,
    SyntaxKind::SYSTEM_KW,
    SyntaxKind::TABLES_KW,
    SyntaxKind::TABLESPACE_KW,
    SyntaxKind::TARGET_KW,
    SyntaxKind::TEMP_KW,
    SyntaxKind::TEMPLATE_KW,
    SyntaxKind::TEMPORARY_KW,
    SyntaxKind::TEXT_KW,
    SyntaxKind::TIES_KW,
    SyntaxKind::TRANSACTION_KW,
    SyntaxKind::TRANSFORM_KW,
    SyntaxKind::TRIGGER_KW,
    SyntaxKind::TRUNCATE_KW,
    SyntaxKind::TRUSTED_KW,
    SyntaxKind::TYPE_KW,
    SyntaxKind::TYPES_KW,
    SyntaxKind::UESCAPE_KW,
    SyntaxKind::UNBOUNDED_KW,
    SyntaxKind::UNCOMMITTED_KW,
    SyntaxKind::UNCONDITIONAL_KW,
    SyntaxKind::UNENCRYPTED_KW,
    SyntaxKind::UNKNOWN_KW,
    SyntaxKind::UNLISTEN_KW,
    SyntaxKind::UNLOGGED_KW,
    SyntaxKind::UNTIL_KW,
    SyntaxKind::UPDATE_KW,
    SyntaxKind::VACUUM_KW,
    SyntaxKind::VALID_KW,
    SyntaxKind::VALIDATE_KW,
    SyntaxKind::VALIDATOR_KW,
    SyntaxKind::VALUE_KW,
    SyntaxKind::VARYING_KW,
    SyntaxKind::VERSION_KW,
    SyntaxKind::VIEW_KW,
    SyntaxKind::VIEWS_KW,
    SyntaxKind::VOLATILE_KW,
    SyntaxKind::WHITESPACE_KW,
    SyntaxKind::WITHIN_KW,
    SyntaxKind::WITHOUT_KW,
    SyntaxKind::WORK_KW,
    SyntaxKind::WRAPPER_KW,
    SyntaxKind::WRITE_KW,
    SyntaxKind::XML_KW,
    SyntaxKind::YEAR_KW,
    SyntaxKind::YES_KW,
    SyntaxKind::ZONE_KW,
]);

pub(crate) const RESERVED_KEYWORDS: TokenSet = TokenSet::new(&[
    SyntaxKind::ALL_KW,
    SyntaxKind::ANALYSE_KW,
    SyntaxKind::ANALYZE_KW,
    SyntaxKind::AND_KW,
    SyntaxKind::ANY_KW,
    SyntaxKind::ARRAY_KW,
    SyntaxKind::AS_KW,
    SyntaxKind::ASC_KW,
    SyntaxKind::ASYMMETRIC_KW,
    SyntaxKind::BOTH_KW,
    SyntaxKind::CASE_KW,
    SyntaxKind::CAST_KW,
    SyntaxKind::CHECK_KW,
    SyntaxKind::COLLATE_KW,
    SyntaxKind::COLUMN_KW,
    SyntaxKind::CONSTRAINT_KW,
    SyntaxKind::CREATE_KW,
    SyntaxKind::CURRENT_CATALOG_KW,
    SyntaxKind::CURRENT_DATE_KW,
    SyntaxKind::CURRENT_ROLE_KW,
    SyntaxKind::CURRENT_TIME_KW,
    SyntaxKind::CURRENT_TIMESTAMP_KW,
    SyntaxKind::CURRENT_USER_KW,
    SyntaxKind::DEFAULT_KW,
    SyntaxKind::DEFERRABLE_KW,
    SyntaxKind::DESC_KW,
    SyntaxKind::DISTINCT_KW,
    SyntaxKind::DO_KW,
    SyntaxKind::ELSE_KW,
    SyntaxKind::END_KW,
    SyntaxKind::EXCEPT_KW,
    SyntaxKind::FALSE_KW,
    SyntaxKind::FETCH_KW,
    SyntaxKind::FOR_KW,
    SyntaxKind::FOREIGN_KW,
    SyntaxKind::FROM_KW,
    SyntaxKind::GRANT_KW,
    SyntaxKind::GROUP_KW,
    SyntaxKind::HAVING_KW,
    SyntaxKind::IN_KW,
    SyntaxKind::INITIALLY_KW,
    SyntaxKind::INTERSECT_KW,
    SyntaxKind::INTO_KW,
    SyntaxKind::LATERAL_KW,
    SyntaxKind::LEADING_KW,
    SyntaxKind::LIMIT_KW,
    SyntaxKind::LOCALTIME_KW,
    SyntaxKind::LOCALTIMESTAMP_KW,
    SyntaxKind::NOT_KW,
    SyntaxKind::NULL_KW,
    SyntaxKind::OFFSET_KW,
    SyntaxKind::ON_KW,
    SyntaxKind::ONLY_KW,
    SyntaxKind::OR_KW,
    SyntaxKind::ORDER_KW,
    SyntaxKind::PLACING_KW,
    SyntaxKind::PRIMARY_KW,
    SyntaxKind::REFERENCES_KW,
    SyntaxKind::RETURNING_KW,
    SyntaxKind::SELECT_KW,
    SyntaxKind::SESSION_USER_KW,
    SyntaxKind::SOME_KW,
    SyntaxKind::SYMMETRIC_KW,
    SyntaxKind::SYSTEM_USER_KW,
    SyntaxKind::TABLE_KW,
    SyntaxKind::THEN_KW,
    SyntaxKind::TO_KW,
    SyntaxKind::TRAILING_KW,
    SyntaxKind::TRUE_KW,
    SyntaxKind::UNION_KW,
    SyntaxKind::UNIQUE_KW,
    SyntaxKind::USER_KW,
    SyntaxKind::USING_KW,
    SyntaxKind::VARIADIC_KW,
    SyntaxKind::WHEN_KW,
    SyntaxKind::WHERE_KW,
    SyntaxKind::WINDOW_KW,
    SyntaxKind::WITH_KW,
]);

// Generated TokenSet end
