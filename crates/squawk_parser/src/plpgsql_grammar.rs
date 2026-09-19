use crate::{
    Parser, SyntaxKind,
    generated::token_sets::{
        ALL_KEYWORDS, PLPGSQL_RESERVED_CONTEXTUAL_KEYWORDS, PLPGSQL_RESERVED_KEYWORDS,
    },
    grammar,
    syntax_kind::SyntaxKind::*,
    token_set::TokenSet,
};

pub(crate) fn plpgsql_entry_point(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) && p.at(POUND) {
        comp_option(p);
    }
    while !p.at(EOF) {
        if at_block_start(p) {
            opt_block(p);
        } else {
            temp_unknown(p, "expected a block");
        }
    }
    m.complete(p, PLPGSQL);
}

const VARIABLE_CONFLICT_VALUES: [SyntaxKind; 3] = [ERROR_KW, USE_VARIABLE_KW, USE_COLUMN_KW];

fn comp_option(p: &mut Parser) {
    assert!(p.at(POUND));
    let m = p.start();
    p.bump(POUND);
    let kind = if p.at(OPTION_KW) {
        p.bump(OPTION_KW);
        expect_contextual_kw(p, DUMP_KW);
        PLPGSQL_COMP_OPTION_DUMP
    } else if p.nth_at_contextual_kw(0, VARIABLE_CONFLICT_KW) {
        p.bump_remap(VARIABLE_CONFLICT_KW);
        variable_conflict_value(p);
        PLPGSQL_COMP_OPTION_VARIABLE_CONFLICT
    } else if p.nth_at_contextual_kw(0, PRINT_STRICT_PARAMS_KW) {
        p.bump_remap(PRINT_STRICT_PARAMS_KW);
        option_value(p);
        PLPGSQL_COMP_OPTION_PRINT_STRICT_PARAMS
    } else {
        p.error("expected OPTION, PRINT_STRICT_PARAMS, or VARIABLE_CONFLICT");
        ERROR
    };
    m.complete(p, kind);
}

fn variable_conflict_value(p: &mut Parser) {
    match VARIABLE_CONFLICT_VALUES
        .into_iter()
        .find(|&kw| at_maybe_contextual_kw(p, 0, kw))
    {
        Some(kw) => bump_maybe_contextual_kw(p, kw),
        None => p.error("expected ERROR, USE_VARIABLE, or USE_COLUMN"),
    }
}

fn option_value(p: &mut Parser) {
    if !at_name(p, 0) {
        p.error(format!("expected an option value, got {:?}", p.current()));
        return;
    }
    name(p, PLPGSQL_OPTION_VALUE);
}

const BLOCK_FIRST: TokenSet = TokenSet::new(&[BEGIN_KW, DECLARE_KW]);

fn opt_block(p: &mut Parser) {
    if !at_block_start(p) {
        return;
    }
    let m = p.start();
    opt_label(p);
    opt_declare_section(p);
    p.expect(BEGIN_KW);
    body(p, BodyKind::Block);
    opt_exception_section(p);
    p.expect(END_KW);
    opt_label_name_ref(p);
    // TODO: add validation, sometimes this is required
    p.eat(SEMICOLON);
    m.complete(p, PLPGSQL_BLOCK);
}

// <<foo>>
fn opt_label(p: &mut Parser) {
    if !at_label(p) {
        return;
    }
    let m = p.start();
    p.bump(LESS_LESS);
    name(p, PLPGSQL_LABEL_NAME);
    p.bump(GREATER_GREATER);
    m.complete(p, PLPGSQL_LABEL);
}

fn opt_label_name_ref(p: &mut Parser) {
    if !at_name(p, 0) {
        return;
    }
    name(p, PLPGSQL_LABEL_NAME_REF);
}

fn name(p: &mut Parser, kind: SyntaxKind) {
    let m = p.start();
    p.bump_any();
    m.complete(p, kind);
}

fn opt_declare_section(p: &mut Parser) {
    if !p.at(DECLARE_KW) {
        return;
    }
    let m = p.start();
    p.bump(DECLARE_KW);
    while !p.at(EOF) && !p.at(BEGIN_KW) {
        if at_alias_decl(p) {
            alias_decl(p);
        } else if at_cursor_decl(p) {
            cursor_decl(p);
        } else if at_var_decl(p) {
            var_decl(p);
        } else {
            temp_unknown(p, "expected a declaration");
        }
    }
    m.complete(p, PLPGSQL_DECLARE_SECTION);
}

fn var_decl(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME);
    if p.nth_at_contextual_kw(0, CONSTANT_KW) {
        p.bump_remap(CONSTANT_KW);
    }
    decl_datatype(p);
    grammar::opt_collate(p);
    opt_not_null(p);
    opt_var_init(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_VAR_DECL);
}

fn alias_decl(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME);
    p.bump_remap(ALIAS_KW);
    p.expect(FOR_KW);
    alias_target(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_ALIAS_DECL);
}

fn alias_target(p: &mut Parser) {
    let m = p.start();
    if at_path(p).is_some() {
        path_name_ref(p);
    } else {
        p.error(format!("expected an alias target, got {:?}", p.current()));
    }
    m.complete(p, PLPGSQL_ALIAS_TARGET);
}

fn cursor_decl(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME);
    grammar::opt_cursor_scroll(p);
    p.expect(CURSOR_KW);
    if p.at(L_PAREN) {
        cursor_arg_list(p);
    }
    if !p.eat(IS_KW) {
        p.expect(FOR_KW);
    }
    if grammar::stmt(p, &grammar::StmtRestrictions::default()).is_none() {
        p.error("expected a query");
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_CURSOR_DECL);
}

fn cursor_arg_list(p: &mut Parser) {
    assert!(p.at(L_PAREN));
    let m = p.start();
    // TODO: use delimited
    p.bump(L_PAREN);
    if p.at(R_PAREN) {
        p.error("expected at least one cursor argument");
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        cursor_arg(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, PLPGSQL_CURSOR_ARG_LIST);
}

fn cursor_arg(p: &mut Parser) {
    let m = p.start();
    if at_name(p, 0) {
        name(p, PLPGSQL_VAR_NAME);
        decl_datatype(p);
    } else {
        p.error(format!(
            "expected a cursor argument name, got {:?}",
            p.current()
        ));
    }
    m.complete(p, PLPGSQL_CURSOR_ARG);
}

fn decl_datatype(p: &mut Parser) {
    if !at_percent_datatype(p) {
        grammar::func_type(p);
        return;
    }
    let m = p.start();
    path_name_ref(p);
    let kind = if grammar::opt_percent_type(p).is_some() {
        PERCENT_TYPE
    } else {
        p.bump(PERCENT);
        p.bump_remap(ROWTYPE_KW);
        PLPGSQL_PERCENT_ROWTYPE
    };
    p.eat(ARRAY_KW);
    while !p.at(EOF) && grammar::opt_array_bound(p) {}
    m.complete(p, kind);
}

fn path_name_ref(p: &mut Parser) {
    assert!(at_path(p).is_some());
    let m = p.start();
    name(p, PATH_SEGMENT_REF);
    let mut path = m.complete(p, PATH_REF);
    while !p.at(EOF) && p.at(DOT) {
        let m = path.precede(p);
        p.bump(DOT);
        name(p, PATH_SEGMENT_REF);
        path = m.complete(p, PATH_REF);
    }
}

fn at_percent_datatype(p: &Parser) -> bool {
    let Some(n) = at_path(p) else {
        return false;
    };
    p.nth_at(n, PERCENT) && (p.nth_at(n + 1, TYPE_KW) || p.nth_at_contextual_kw(n + 1, ROWTYPE_KW))
}

fn at_path(p: &Parser) -> Option<usize> {
    let mut n = 0;
    while !p.nth_at(n, EOF) && at_name(p, n) {
        n += 1;
        if !p.nth_at(n, DOT) {
            return Some(n);
        }
        n += 1;
    }
    None
}

fn opt_not_null(p: &mut Parser) {
    if !p.at(NOT_KW) {
        return;
    }
    let m = p.start();
    p.bump(NOT_KW);
    p.expect(NULL_KW);
    m.complete(p, PLPGSQL_NOT_NULL);
}

fn opt_var_init(p: &mut Parser) {
    if !p.at(COLON_EQ) && !p.at(EQ) && !p.at(DEFAULT_KW) {
        return;
    }
    let m = p.start();
    if !p.eat(COLON_EQ) && !p.eat(EQ) {
        p.bump(DEFAULT_KW);
    }
    expr(p);
    m.complete(p, PLPGSQL_VAR_INIT);
}

fn expr(p: &mut Parser) {
    if grammar::expr(p).is_none() {
        p.error("expected an expression");
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BodyKind {
    Block,
    ExceptionHandler,
    IfThen,
    IfElse,
    CaseWhen,
    CaseElse,
    Loop,
}

fn body(p: &mut Parser, kind: BodyKind) {
    let m = p.start();
    while !p.at(EOF) && !at_body_end(p, kind) {
        stmt(p);
    }
    m.complete(p, PLPGSQL_BODY);
}

fn stmt(p: &mut Parser) {
    if at_block_start(p) {
        opt_block(p);
    } else if at_loop_start(p) {
        loop_stmt(p);
    } else if at_assign_stmt(p) {
        assign_stmt(p);
    } else if at_exit_stmt(p) {
        exit_stmt(p);
    } else if p.at(CALL_KW) {
        call_stmt(p);
    } else if p.at(DO_KW) {
        do_stmt(p);
    } else if p.nth_at_contextual_kw(0, PERFORM_KW) {
        perform_stmt(p);
    } else if p.at(RETURN_KW) {
        return_stmt(p);
    } else if p.nth_at_contextual_kw(0, ASSERT_KW) {
        assert_stmt(p);
    } else if p.nth_at_contextual_kw(0, RAISE_KW) {
        raise_stmt(p);
    } else if at_stmt_kw(p, GET_KW) {
        get_diag_stmt(p);
    } else if at_stmt_kw(p, FETCH_KW) {
        fetch_stmt(p);
    } else if at_stmt_kw(p, OPEN_KW) {
        open_stmt(p);
    } else if at_stmt_kw(p, MOVE_KW) {
        move_stmt(p);
    } else if at_stmt_kw(p, CLOSE_KW) {
        close_stmt(p);
    } else if at_stmt_kw(p, EXECUTE_KW) {
        dyn_execute_stmt(p);
    } else if at_transaction_stmt(p) {
        transaction_stmt(p);
    } else if p.at(CASE_KW) {
        case_stmt(p);
    } else if p.at(IF_KW) {
        if_stmt(p);
    } else if p.at(NULL_KW) && p.nth_at(1, SEMICOLON) {
        let m = p.start();
        p.bump(NULL_KW);
        p.bump(SEMICOLON);
        m.complete(p, PLPGSQL_NULL_STMT);
    } else {
        temp_unknown(p, "expected a statement");
    }
}

fn call_stmt(p: &mut Parser) {
    assert!(p.at(CALL_KW));
    let m = p.start();
    match grammar::stmt(p, &grammar::StmtRestrictions::default()).map(|x| x.kind()) {
        Some(CALL) => (),
        _ => {
            p.error("expected a CALL statement");
        }
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_CALL_STMT);
}

fn do_stmt(p: &mut Parser) {
    assert!(p.at(DO_KW));
    let m = p.start();
    match grammar::stmt(p, &grammar::StmtRestrictions::default()).map(|x| x.kind()) {
        Some(DO) => (),
        _ => {
            p.error("expected a DO statement");
        }
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_DO_STMT);
}

fn perform_stmt(p: &mut Parser) {
    assert!(p.nth_at_contextual_kw(0, PERFORM_KW));
    let m = p.start();
    match grammar::perform_select(p).kind() {
        SELECT | SELECT_INTO | COMPOUND_SELECT => (),
        _ => {
            p.error("expected a SELECT statement");
        }
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_PERFORM_STMT);
}

fn return_stmt(p: &mut Parser) {
    assert!(p.at(RETURN_KW));
    let m = p.start();
    p.bump(RETURN_KW);
    let composite = p.nth_at(1, DOT);
    let kind = if p.at(NEXT_KW) && !composite {
        p.bump(NEXT_KW);
        if !p.at(SEMICOLON) {
            expr(p);
        }
        PLPGSQL_RETURN_NEXT_STMT
    } else if p.nth_at_contextual_kw(0, QUERY_KW) && !composite && p.nth_at(1, EXECUTE_KW) {
        p.bump_remap(QUERY_KW);
        p.bump(EXECUTE_KW);
        expr(p);
        opt_using_clause(p, expr);
        PLPGSQL_RETURN_QUERY_EXECUTE_STMT
    } else if p.nth_at_contextual_kw(0, QUERY_KW) && !composite {
        p.bump_remap(QUERY_KW);
        if grammar::stmt(p, &grammar::StmtRestrictions::default()).is_none() {
            p.error("expected a query");
        }
        PLPGSQL_RETURN_QUERY_STMT
    } else {
        if !p.at(SEMICOLON) {
            expr(p);
        }
        PLPGSQL_RETURN_STMT
    };
    p.expect(SEMICOLON);
    m.complete(p, kind);
}

fn assert_stmt(p: &mut Parser) {
    assert!(p.nth_at_contextual_kw(0, ASSERT_KW));
    let m = p.start();
    p.bump_remap(ASSERT_KW);
    expr(p);
    if p.eat(COMMA) {
        expr(p);
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_ASSERT_STMT);
}

fn opt_using_clause(p: &mut Parser, param: fn(&mut Parser)) {
    if !p.at(USING_KW) {
        return;
    }
    let m = p.start();
    p.bump(USING_KW);
    param(p);
    while !p.at(EOF) && p.eat(COMMA) {
        param(p);
    }
    m.complete(p, PLPGSQL_USING_CLAUSE);
}

const RAISE_LEVELS: [SyntaxKind; 6] = [
    EXCEPTION_KW,
    WARNING_KW,
    NOTICE_KW,
    INFO_KW,
    LOG_KW,
    DEBUG_KW,
];

const RAISE_OPTIONS: [(SyntaxKind, SyntaxKind); 9] = [
    (ERRCODE_KW, PLPGSQL_RAISE_OPTION_ERRCODE),
    (MESSAGE_KW, PLPGSQL_RAISE_OPTION_MESSAGE),
    (DETAIL_KW, PLPGSQL_RAISE_OPTION_DETAIL),
    (HINT_KW, PLPGSQL_RAISE_OPTION_HINT),
    (COLUMN_KW, PLPGSQL_RAISE_OPTION_COLUMN),
    (CONSTRAINT_KW, PLPGSQL_RAISE_OPTION_CONSTRAINT),
    (DATATYPE_KW, PLPGSQL_RAISE_OPTION_DATATYPE),
    (TABLE_KW, PLPGSQL_RAISE_OPTION_TABLE),
    (SCHEMA_KW, PLPGSQL_RAISE_OPTION_SCHEMA),
];

fn raise_stmt(p: &mut Parser) {
    assert!(p.nth_at_contextual_kw(0, RAISE_KW));
    let m = p.start();
    p.bump_remap(RAISE_KW);
    if !p.at(SEMICOLON) {
        opt_raise_level(p);
        if opt_raise_message(p) {
            while !p.at(EOF) && p.eat(COMMA) {
                expr(p);
            }
        } else if !p.at(USING_KW) {
            condition(p);
        }
        opt_raise_using_clause(p);
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_RAISE_STMT);
}

fn opt_raise_level(p: &mut Parser) {
    let Some(kw) = RAISE_LEVELS
        .into_iter()
        .find(|&kw| at_maybe_contextual_kw(p, 0, kw))
    else {
        return;
    };
    let m = p.start();
    bump_maybe_contextual_kw(p, kw);
    m.complete(p, PLPGSQL_RAISE_LEVEL);
}

fn opt_raise_message(p: &mut Parser) -> bool {
    let Some(literal) = grammar::opt_string_literal(p) else {
        return false;
    };
    literal.precede(p).complete(p, PLPGSQL_RAISE_MESSAGE);
    true
}

fn opt_raise_using_clause(p: &mut Parser) {
    if !p.at(USING_KW) {
        return;
    }
    let m = p.start();
    p.bump(USING_KW);
    raise_option(p);
    while !p.at(EOF) && p.eat(COMMA) {
        raise_option(p);
    }
    m.complete(p, PLPGSQL_RAISE_USING_CLAUSE);
}

fn raise_option(p: &mut Parser) {
    let m = p.start();
    let kind = match RAISE_OPTIONS
        .into_iter()
        .find(|&(kw, _)| at_maybe_contextual_kw(p, 0, kw))
    {
        Some((kw, kind)) => {
            bump_maybe_contextual_kw(p, kw);
            kind
        }
        None => {
            p.error("unrecognized RAISE statement option");
            ERROR
        }
    };
    if !p.eat(COLON_EQ) {
        p.expect(EQ);
    }
    expr(p);
    m.complete(p, kind);
}

fn transaction_stmt(p: &mut Parser) {
    assert!(at_transaction_stmt(p));
    let m = p.start();
    let kind = if p.eat(COMMIT_KW) {
        PLPGSQL_COMMIT_STMT
    } else {
        p.bump(ROLLBACK_KW);
        PLPGSQL_ROLLBACK_STMT
    };
    grammar::opt_chain_clause(p);
    p.expect(SEMICOLON);
    m.complete(p, kind);
}

const DIAG_ITEM_KINDS: [SyntaxKind; 13] = [
    ROW_COUNT_KW,
    PG_ROUTINE_OID_KW,
    PG_CONTEXT_KW,
    PG_EXCEPTION_DETAIL_KW,
    PG_EXCEPTION_HINT_KW,
    PG_EXCEPTION_CONTEXT_KW,
    COLUMN_NAME_KW,
    CONSTRAINT_NAME_KW,
    PG_DATATYPE_NAME_KW,
    MESSAGE_TEXT_KW,
    TABLE_NAME_KW,
    SCHEMA_NAME_KW,
    RETURNED_SQLSTATE_KW,
];

fn get_diag_stmt(p: &mut Parser) {
    assert!(at_stmt_kw(p, GET_KW));
    let m = p.start();
    p.bump_remap(GET_KW);
    opt_diag_area(p);
    expect_contextual_kw(p, DIAGNOSTICS_KW);
    diag_item_list(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_GET_DIAG_STMT);
}

fn opt_diag_area(p: &mut Parser) {
    if !p.at(CURRENT_KW) && !p.nth_at_contextual_kw(0, STACKED_KW) {
        return;
    }
    let m = p.start();
    if !p.eat(CURRENT_KW) {
        p.bump_remap(STACKED_KW);
    }
    m.complete(p, PLPGSQL_DIAG_AREA);
}

fn diag_item_list(p: &mut Parser) {
    let m = p.start();
    diag_item(p);
    while !p.at(EOF) && p.eat(COMMA) {
        diag_item(p);
    }
    m.complete(p, PLPGSQL_DIAG_ITEM_LIST);
}

fn diag_item(p: &mut Parser) {
    let m = p.start();
    if at_name(p, 0) {
        scalar_target(p, PLPGSQL_DIAG_TARGET);
        if !p.eat(COLON_EQ) {
            p.expect(EQ);
        }
        diag_kind(p);
    } else {
        p.error(format!(
            "expected a diagnostics target, got {:?}",
            p.current()
        ));
    }
    m.complete(p, PLPGSQL_DIAG_ITEM);
}

fn scalar_target(p: &mut Parser, kind: SyntaxKind) {
    assert!(at_name(p, 0));
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME_REF);
    while !p.at(EOF) && p.at(DOT) {
        grammar::field_accessor(p);
    }
    if p.at(L_BRACK) {
        let m = p.start();
        p.error("subscript not allowed");
        grammar::accessors(p);
        m.complete(p, ERROR);
    }
    m.complete(p, kind);
}

fn diag_kind(p: &mut Parser) {
    let m = p.start();
    match DIAG_ITEM_KINDS
        .into_iter()
        .find(|&kw| p.nth_at_contextual_kw(0, kw))
    {
        Some(kw) => p.bump_remap(kw),
        None => {
            p.error("unrecognized GET DIAGNOSTICS item");
            if at_name(p, 0) {
                p.bump_any();
            }
        }
    }
    m.complete(p, PLPGSQL_DIAG_KIND);
}

fn open_stmt(p: &mut Parser) {
    assert!(at_stmt_kw(p, OPEN_KW));
    let m = p.start();
    p.bump_remap(OPEN_KW);
    cursor_variable_ref(p);
    if p.at(L_PAREN) {
        grammar::arg_list(p);
    }
    grammar::opt_cursor_scroll(p);
    if p.at(FOR_KW) {
        let m = p.start();
        p.bump(FOR_KW);
        let kind = if p.eat(EXECUTE_KW) {
            expr(p);
            opt_using_clause(p, expr);
            PLPGSQL_OPEN_EXECUTE
        } else {
            if grammar::stmt(p, &grammar::StmtRestrictions::default()).is_none() {
                p.error("expected a query");
            }
            PLPGSQL_OPEN_QUERY
        };
        m.complete(p, kind);
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_OPEN_STMT);
}

fn fetch_stmt(p: &mut Parser) {
    assert!(at_stmt_kw(p, FETCH_KW));
    let m = p.start();
    p.bump(FETCH_KW);
    fetch_direction(p);
    cursor_variable_ref(p);
    into_clause(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_FETCH_STMT);
}

fn dyn_execute_stmt(p: &mut Parser) {
    assert!(at_stmt_kw(p, EXECUTE_KW));
    let m = p.start();
    p.bump(EXECUTE_KW);
    expr_until_into_or_using(p);
    let mut into = false;
    let mut using = false;
    while !p.at(EOF) {
        if !into && p.at(INTO_KW) {
            into = true;
            into_clause(p);
        } else if !using && p.at(USING_KW) {
            using = true;
            opt_using_clause(p, expr);
        } else {
            break;
        }
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_DYN_EXECUTE_STMT);
}

fn move_stmt(p: &mut Parser) {
    assert!(at_stmt_kw(p, MOVE_KW));
    let m = p.start();
    p.bump(MOVE_KW);
    fetch_direction(p);
    cursor_variable_ref(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_MOVE_STMT);
}

fn fetch_direction(p: &mut Parser) {
    let direction = grammar::opt_direction(p);
    let from_or_in = p.eat(FROM_KW) || p.eat(IN_KW);
    if direction && !from_or_in {
        p.error("expected FROM or IN");
    }
}

fn into_clause(p: &mut Parser) {
    if !p.at(INTO_KW) {
        p.error(format!("expected INTO, got {:?}", p.current()));
        return;
    }
    let m = p.start();
    p.bump(INTO_KW);
    p.eat(STRICT_KW);
    into_target_list(p);
    m.complete(p, PLPGSQL_INTO_CLAUSE);
}

fn into_target_list(p: &mut Parser) {
    let m = p.start();
    into_target(p);
    while !p.at(EOF) && p.eat(COMMA) {
        into_target(p);
    }
    m.complete(p, PLPGSQL_INTO_TARGET_LIST);
}

fn into_target(p: &mut Parser) {
    if !at_name(p, 0) {
        p.error(format!("expected an INTO target, got {:?}", p.current()));
        return;
    }
    scalar_target(p, PLPGSQL_INTO_TARGET);
}

fn close_stmt(p: &mut Parser) {
    assert!(at_stmt_kw(p, CLOSE_KW));
    let m = p.start();
    p.bump(CLOSE_KW);
    cursor_variable_ref(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_CLOSE_STMT);
}

fn cursor_variable_ref(p: &mut Parser) {
    if !at_name(p, 0) {
        p.error(format!("expected a cursor variable, got {:?}", p.current()));
        return;
    }
    name(p, PLPGSQL_CURSOR_VARIABLE_REF);
    if p.at(DOT) || p.at(L_BRACK) {
        let m = p.start();
        p.error("a cursor variable must be a simple variable");
        grammar::accessors(p);
        m.complete(p, ERROR);
    }
}

fn assign_stmt(p: &mut Parser) {
    assert!(at_assign_stmt(p));
    let m = p.start();
    assign_target(p);
    if !p.eat(COLON_EQ) {
        p.expect(EQ);
    }
    expr(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_ASSIGN_STMT);
}

fn assign_target(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME_REF);
    grammar::accessors(p);
    m.complete(p, PLPGSQL_ASSIGN_TARGET);
}

fn if_stmt(p: &mut Parser) {
    assert!(p.at(IF_KW));
    let m = p.start();
    p.bump(IF_KW);
    expr(p);
    p.expect(THEN_KW);
    body(p, BodyKind::IfThen);
    while !p.at(EOF) && opt_elsif_clause(p) {}
    opt_else_clause(p, BodyKind::IfElse);
    p.expect(END_KW);
    p.expect(IF_KW);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_IF_STMT);
}

fn case_stmt(p: &mut Parser) {
    assert!(p.at(CASE_KW));
    let m = p.start();
    p.bump(CASE_KW);
    if !p.at(WHEN_KW) {
        expr(p);
        if !p.at(WHEN_KW) {
            p.error("expected `when`");
        }
    }
    while !p.at(EOF) && p.at(WHEN_KW) {
        case_when(p);
    }
    opt_else_clause(p, BodyKind::CaseElse);
    p.expect(END_KW);
    p.expect(CASE_KW);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_CASE_STMT);
}

fn case_when(p: &mut Parser) {
    assert!(p.at(WHEN_KW));
    let m = p.start();
    p.bump(WHEN_KW);
    expr(p);
    while !p.at(EOF) && p.eat(COMMA) {
        expr(p);
    }
    p.expect(THEN_KW);
    body(p, BodyKind::CaseWhen);
    m.complete(p, PLPGSQL_CASE_WHEN);
}

fn loop_stmt(p: &mut Parser) {
    assert!(at_loop_start(p));
    let m = p.start();
    opt_label(p);
    let kind = if p.nth_at_contextual_kw(0, WHILE_KW) {
        p.bump_remap(WHILE_KW);
        expr(p);
        PLPGSQL_WHILE_STMT
    } else if p.at(FOR_KW) {
        for_head(p)
    } else if p.nth_at_contextual_kw(0, FOREACH_KW) {
        foreach_head(p)
    } else {
        PLPGSQL_LOOP_STMT
    };
    expect_contextual_kw(p, LOOP_KW);
    body(p, BodyKind::Loop);
    p.expect(END_KW);
    expect_contextual_kw(p, LOOP_KW);
    opt_label_name_ref(p);
    p.expect(SEMICOLON);
    m.complete(p, kind);
}

fn for_head(p: &mut Parser) -> SyntaxKind {
    assert!(p.at(FOR_KW));
    p.bump(FOR_KW);
    for_variable_list(p);
    p.expect(IN_KW);
    if p.eat(EXECUTE_KW) {
        expr_until_loop(p);
        opt_using_clause(p, expr_until_loop);
        return PLPGSQL_FOR_DYN_STMT;
    }
    if at_for_cursor(p, 0) {
        cursor_variable_ref(p);
        if p.at(L_PAREN) {
            grammar::arg_list(p);
        }
        return PLPGSQL_FOR_CURSOR_STMT;
    }
    if at_for_reverse(p, 0) {
        p.bump_remap(REVERSE_KW);
    }
    match for_header_terminator(p, 0) {
        Some((DOT_DOT, _)) => {
            for_range(p);
            PLPGSQL_FOR_I_STMT
        }
        terminator => {
            for_query(p, terminator.map(|(_, n)| n));
            PLPGSQL_FOR_QUERY_STMT
        }
    }
}

fn foreach_head(p: &mut Parser) -> SyntaxKind {
    assert!(p.nth_at_contextual_kw(0, FOREACH_KW));
    p.bump_remap(FOREACH_KW);
    for_variable_list(p);
    opt_foreach_slice(p);
    p.expect(IN_KW);
    p.expect(ARRAY_KW);
    expr_until_loop(p);
    PLPGSQL_FOR_EACH_STMT
}

fn opt_foreach_slice(p: &mut Parser) {
    if !p.nth_at_contextual_kw(0, SLICE_KW) {
        return;
    }
    let m = p.start();
    p.bump_remap(SLICE_KW);
    if grammar::opt_uint_literal(p).is_none() {
        let m = p.start();
        p.error("expected unsigned integer literal");
        if !p.at(IN_KW) {
            p.bump_any();
        }
        m.complete(p, ERROR);
    }
    m.complete(p, PLPGSQL_FOR_EACH_SLICE);
}

fn for_query(p: &mut Parser, loop_at: Option<usize>) {
    match loop_at {
        Some(0) => p.error("expected a query"),
        Some(n) => {
            p.with_limit(n, |p| {
                grammar::stmt(p, &grammar::StmtRestrictions::default());
            });
        }
        None => {
            grammar::stmt(p, &grammar::StmtRestrictions::default());
        }
    }
}

fn for_variable_list(p: &mut Parser) {
    for_variable(p);
    while !p.at(EOF) && p.eat(COMMA) {
        for_variable(p);
    }
}

fn for_variable(p: &mut Parser) {
    if !at_name(p, 0) {
        p.error(format!("expected a loop variable, got {:?}", p.current()));
        return;
    }
    scalar_target(p, PLPGSQL_FOR_VARIABLE);
}

fn for_range(p: &mut Parser) {
    let m = p.start();
    range_bound(p);
    p.expect(DOT_DOT);
    range_bound(p);
    if p.eat(BY_KW) {
        range_bound(p);
    }
    m.complete(p, PLPGSQL_FOR_RANGE);
}

fn range_bound(p: &mut Parser) {
    if p.nth_at_contextual_kw(0, LOOP_KW) || p.at(BY_KW) {
        p.error("expected an expression");
        return;
    }
    expr(p);
}

fn expr_until_into_or_using(p: &mut Parser) {
    if p.at(INTO_KW) || p.at(USING_KW) || p.at(SEMICOLON) {
        p.error("expected an expression");
        return;
    }
    expr(p);
}

fn expr_until_loop(p: &mut Parser) {
    if p.nth_at_contextual_kw(0, LOOP_KW) || p.at(USING_KW) {
        p.error("expected an expression");
        return;
    }
    expr(p);
}

fn exit_stmt(p: &mut Parser) {
    assert!(at_exit_stmt(p));
    let m = p.start();
    let kind = if p.eat(CONTINUE_KW) {
        PLPGSQL_CONTINUE_STMT
    } else {
        p.bump_remap(EXIT_KW);
        PLPGSQL_EXIT_STMT
    };
    opt_label_name_ref(p);
    opt_exit_when(p);
    p.expect(SEMICOLON);
    m.complete(p, kind);
}

fn opt_exit_when(p: &mut Parser) {
    if !p.at(WHEN_KW) {
        return;
    }
    let m = p.start();
    p.bump(WHEN_KW);
    expr(p);
    m.complete(p, PLPGSQL_EXIT_WHEN);
}

fn expect_contextual_kw(p: &mut Parser, kw: SyntaxKind) {
    if p.nth_at_contextual_kw(0, kw) {
        p.bump_remap(kw);
    } else {
        p.error(format!("expected {kw:?}"));
    }
}

fn opt_elsif_clause(p: &mut Parser) -> bool {
    if !at_elsif(p) {
        return false;
    }
    let m = p.start();
    if p.nth_at_contextual_kw(0, ELSEIF_KW) {
        p.bump_remap(ELSEIF_KW);
    } else {
        p.bump_remap(ELSIF_KW);
    }
    expr(p);
    p.expect(THEN_KW);
    body(p, BodyKind::IfThen);
    m.complete(p, PLPGSQL_ELSIF_CLAUSE);
    true
}

fn opt_else_clause(p: &mut Parser, kind: BodyKind) {
    if !p.at(ELSE_KW) {
        return;
    }
    let m = p.start();
    p.bump(ELSE_KW);
    body(p, kind);
    m.complete(p, PLPGSQL_ELSE_CLAUSE);
}

// TODO: remove this once we get all the ast nodes working
fn temp_unknown(p: &mut Parser, message: &str) {
    let m = p.start();
    p.error(format!("{message}, got {:?}", p.current()));

    let mut depth = 0;
    while !p.at(EOF) {
        if depth == 0 && p.at(SEMICOLON) {
            break;
        }
        if p.at(BEGIN_KW) {
            depth += 1;
        } else if p.at(END_KW) && depth > 0 {
            depth -= 1;
        }
        p.bump_any();
    }
    p.eat(SEMICOLON);
    m.complete(p, ERROR);
}

fn opt_exception_section(p: &mut Parser) {
    if !p.nth_at_contextual_kw(0, EXCEPTION_KW) {
        return;
    }
    let m = p.start();
    p.bump_remap(EXCEPTION_KW);
    while !p.at(EOF) && p.at(WHEN_KW) {
        exception_handler(p);
    }
    m.complete(p, PLPGSQL_EXCEPTION_SECTION);
}

fn exception_handler(p: &mut Parser) {
    assert!(p.at(WHEN_KW));
    let m = p.start();
    // TODO: use delimited
    p.bump(WHEN_KW);
    condition(p);
    while !p.at(EOF) && p.eat(OR_KW) {
        condition(p);
    }
    p.expect(THEN_KW);
    body(p, BodyKind::ExceptionHandler);
    m.complete(p, PLPGSQL_EXCEPTION_HANDLER);
}

fn condition(p: &mut Parser) {
    let m = p.start();
    if at_name(p, 0) {
        let sqlstate = p.nth_at_contextual_kw(0, SQLSTATE_KW);
        p.bump_any();
        if sqlstate {
            p.expect(STRING);
        }
    } else {
        p.error(format!("expected a condition name, got {:?}", p.current()));
    }
    m.complete(p, PLPGSQL_CONDITION);
}

fn at_alias_decl(p: &Parser) -> bool {
    at_name(p, 0) && p.nth_at_contextual_kw(1, ALIAS_KW)
}

fn at_cursor_decl(p: &Parser) -> bool {
    at_name(p, 0) && (p.nth_at(1, CURSOR_KW) || p.nth_at(1, SCROLL_KW) || p.nth_at(1, NO_KW))
}

fn at_var_decl(p: &Parser) -> bool {
    at_name(p, 0)
}

fn at_block_start(p: &Parser) -> bool {
    p.at_ts(BLOCK_FIRST) || at_block_label(p)
}

fn at_block_label(p: &Parser) -> bool {
    at_label(p) && p.nth_at_ts(5, BLOCK_FIRST)
}

fn at_label(p: &Parser) -> bool {
    p.at(LESS_LESS) && at_name(p, 2) && p.nth_at(3, GREATER_GREATER)
}

fn at_loop_start(p: &Parser) -> bool {
    at_loop_kw(p, 0) || (at_label(p) && at_loop_kw(p, 5))
}

fn at_loop_kw(p: &Parser, n: usize) -> bool {
    p.nth_at_contextual_kw(n, LOOP_KW)
        || p.nth_at_contextual_kw(n, WHILE_KW)
        || p.nth_at_contextual_kw(n, FOREACH_KW)
        || p.nth_at(n, FOR_KW)
}

const NOT_A_CURSOR_NAME: TokenSet = TokenSet::new(&[SELECT_KW, VALUES_KW]);

fn at_for_cursor(p: &Parser, n: usize) -> bool {
    if !at_name(p, n) || p.nth_at_ts(n, NOT_A_CURSOR_NAME) {
        return false;
    }
    let Some(n) = cursor_args_end(p, n + 1) else {
        return false;
    };
    p.nth_at_contextual_kw(n, LOOP_KW)
}

fn cursor_args_end(p: &Parser, mut n: usize) -> Option<usize> {
    if !p.nth_at(n, L_PAREN) {
        return Some(n);
    }
    let mut depth = 0i32;
    while !p.nth_at(n, EOF) && !p.nth_at(n, SEMICOLON) {
        match p.nth(n) {
            L_PAREN => depth += 1,
            R_PAREN => {
                depth -= 1;
                if depth == 0 {
                    return Some(n + 1);
                }
            }
            _ => (),
        }
        n += 1;
    }
    None
}

fn at_for_reverse(p: &Parser, n: usize) -> bool {
    p.nth_at_contextual_kw(n, REVERSE_KW) && !p.nth_at(n + 1, DOT)
}

// postgres does a similar thing to look ahead until it sees a loop token
fn for_header_terminator(p: &Parser, mut n: usize) -> Option<(SyntaxKind, usize)> {
    let mut depth = 0i32;
    while !p.nth_at(n, EOF) {
        match p.nth(n) {
            L_PAREN | L_BRACK => depth += 1,
            R_PAREN | R_BRACK => depth -= 1,
            SEMICOLON if depth == 0 => return None,
            DOT_DOT if depth == 0 => return Some((DOT_DOT, n)),
            _ if depth == 0 && p.nth_at_contextual_kw(n, LOOP_KW) => return Some((LOOP_KW, n)),
            _ => (),
        }
        n += 1;
    }
    None
}

fn at_assign_stmt(p: &Parser) -> bool {
    at_assign_target(p)
        && (p.nth_at(1, COLON_EQ) || p.nth_at(1, EQ) || p.nth_at(1, L_BRACK) || p.nth_at(1, DOT))
}

fn at_assign_target(p: &Parser) -> bool {
    at_name(p, 0) && (p.at_ts(grammar::NAME_FIRST) || p.at(POSITIONAL_PARAM))
}

fn at_stmt_kw(p: &Parser, kw: SyntaxKind) -> bool {
    at_maybe_contextual_kw(p, 0, kw) && !p.nth_at(1, DOT)
}

fn at_transaction_stmt(p: &Parser) -> bool {
    p.at(COMMIT_KW) || p.at(ROLLBACK_KW)
}

fn at_exit_stmt(p: &Parser) -> bool {
    p.at(CONTINUE_KW) || p.nth_at_contextual_kw(0, EXIT_KW)
}

fn at_name(p: &Parser, n: usize) -> bool {
    if p.nth_at(n, IDENT) {
        return !p.nth_at_contextual_ts(n, PLPGSQL_RESERVED_CONTEXTUAL_KEYWORDS);
    }
    if p.nth_at(n, POSITIONAL_PARAM) {
        return true;
    }
    p.nth_at_ts(n, ALL_KEYWORDS) && !p.nth_at_ts(n, PLPGSQL_RESERVED_KEYWORDS)
}

fn at_maybe_contextual_kw(p: &Parser, n: usize, kw: SyntaxKind) -> bool {
    p.nth_at(n, kw) || p.nth_at_contextual_kw(n, kw)
}

fn bump_maybe_contextual_kw(p: &mut Parser, kw: SyntaxKind) {
    if p.at(kw) {
        p.bump(kw);
    } else {
        p.bump_remap(kw);
    }
}

fn at_elsif(p: &Parser) -> bool {
    p.nth_at_contextual_kw(0, ELSIF_KW) || p.nth_at_contextual_kw(0, ELSEIF_KW)
}

fn at_body_end(p: &Parser, kind: BodyKind) -> bool {
    if at_block_end(p) {
        return true;
    }
    match kind {
        BodyKind::Block => p.nth_at_contextual_kw(0, EXCEPTION_KW),
        BodyKind::ExceptionHandler => p.nth_at_contextual_kw(0, EXCEPTION_KW) || p.at(WHEN_KW),
        BodyKind::IfThen => at_end_if(p) || p.at(ELSE_KW) || at_elsif(p),
        BodyKind::IfElse => at_end_if(p),
        BodyKind::CaseWhen => at_end_case(p) || p.at(WHEN_KW) || p.at(ELSE_KW),
        BodyKind::CaseElse => at_end_case(p),
        BodyKind::Loop => at_end_loop(p),
    }
}

fn at_end_if(p: &Parser) -> bool {
    p.at(END_KW) && p.nth_at(1, IF_KW)
}

fn at_end_case(p: &Parser) -> bool {
    p.at(END_KW) && p.nth_at(1, CASE_KW)
}

fn at_end_loop(p: &Parser) -> bool {
    p.at(END_KW) && p.nth_at_contextual_kw(1, LOOP_KW)
}

fn at_block_end(p: &Parser) -> bool {
    if !p.at(END_KW) {
        return false;
    }
    !p.nth_at(1, IF_KW) && !p.nth_at(1, CASE_KW) && !p.nth_at_contextual_kw(1, LOOP_KW)
}
