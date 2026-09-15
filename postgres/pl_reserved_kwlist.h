// synced from:
//   commit: dde9a87d4d02a67a5bd459045f03a6f467a00a80
//   committed at: 2026-07-18T08:00:26+05:30
//   file: https://github.com/postgres/postgres/blob/dde9a87d4d02a67a5bd459045f03a6f467a00a80/src/pl/plpgsql/src/pl_reserved_kwlist.h
//
// update via:
//   cargo xtask sync-pg

/*-------------------------------------------------------------------------
 *
 * pl_reserved_kwlist.h
 *
 * The keyword lists are kept in their own source files for use by
 * automatic tools.  The exact representation of a keyword is determined
 * by the PG_KEYWORD macro, which is not defined in this file; it can
 * be defined by the caller for special purposes.
 *
 * Portions Copyright (c) 1996-2026, PostgreSQL Global Development Group
 * Portions Copyright (c) 1994, Regents of the University of California
 *
 * src/pl/plpgsql/src/pl_reserved_kwlist.h
 *
 *-------------------------------------------------------------------------
 */

/* There is deliberately not an #ifndef PL_RESERVED_KWLIST_H here. */

/*
 * List of (keyword-name, keyword-token-value) pairs.
 *
 * Be careful not to put the same word into pl_unreserved_kwlist.h.
 *
 * Note: gen_keywordlist.pl requires the entries to appear in ASCII order.
 */

/* name, value */
PG_KEYWORD("all", K_ALL)
PG_KEYWORD("begin", K_BEGIN)
PG_KEYWORD("by", K_BY)
PG_KEYWORD("case", K_CASE)
PG_KEYWORD("declare", K_DECLARE)
PG_KEYWORD("else", K_ELSE)
PG_KEYWORD("end", K_END)
PG_KEYWORD("for", K_FOR)
PG_KEYWORD("foreach", K_FOREACH)
PG_KEYWORD("from", K_FROM)
PG_KEYWORD("if", K_IF)
PG_KEYWORD("in", K_IN)
PG_KEYWORD("into", K_INTO)
PG_KEYWORD("loop", K_LOOP)
PG_KEYWORD("not", K_NOT)
PG_KEYWORD("null", K_NULL)
PG_KEYWORD("or", K_OR)
PG_KEYWORD("then", K_THEN)
PG_KEYWORD("to", K_TO)
PG_KEYWORD("using", K_USING)
PG_KEYWORD("when", K_WHEN)
PG_KEYWORD("while", K_WHILE)
