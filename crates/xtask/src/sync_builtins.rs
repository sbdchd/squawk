use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::path::project_root;

const BUILTIN_SCHEMAS_QUERY: &str = r"
select nspname
from pg_namespace
where nspname not like 'pg_temp%'
  and nspname not like 'pg_toast%'
  and nspname != 'public'
order by nspname;
";

const BUILTIN_TYPES_QUERY: &str = r"
select n.nspname, t.typname, t.typlen, case t.typalign
    when 'c' then 1
    when 's' then 2
    when 'i' then 4
    when 'd' then 8
  end as typalign
from pg_type t
join pg_namespace n on n.oid = t.typnamespace
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and n.nspname != 'public'
  and t.typtype in ('b', 'p', 'd', 'e')
  and t.typname not like '\_%'
order by n.nspname, t.typname;
";

const BUILTIN_RANGE_TYPES_QUERY: &str = r"
select n.nspname, t.typname, t.typlen, case t.typalign
    when 'c' then 1
    when 's' then 2
    when 'i' then 4
    when 'd' then 8
  end as typalign, format_type(r.rngsubtype, null) as subtype
from pg_type t
join pg_namespace n on n.oid = t.typnamespace
join pg_range r on r.rngtypid = t.oid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and n.nspname != 'public'
order by n.nspname, t.typname;
";

const BUILTIN_TABLES_QUERY: &str = r"
select n.nspname, c.relname, c.relkind, a.attname, format_type(a.atttypid, a.atttypmod) as type
from pg_class c
join pg_namespace n on n.oid = c.relnamespace
join pg_attribute a on a.attrelid = c.oid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and n.nspname != 'public'
  and c.relkind in ('r', 'v')
  and a.attnum > 0
  and not a.attisdropped
order by n.nspname, c.relname, a.attnum;
";

const BUILTIN_FUNCTIONS_QUERY: &str = r"
select n.nspname, p.proname, pg_get_function_arguments(p.oid) as args, pg_get_function_result(p.oid) as result
from pg_proc p
join pg_namespace n on n.oid = p.pronamespace
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and n.nspname != 'public'
  and pg_get_function_arguments(p.oid) not like '%ORDER BY%'
order by n.nspname, p.proname;
";

const BUILTIN_OPERATORS_QUERY: &str = r"
select n.nspname, o.oprname,
  format_type(o.oprleft, null) as left_type,
  format_type(o.oprright, null) as right_type,
  pn.nspname as func_schema,
  p.proname as func_name
from pg_operator o
join pg_namespace n on n.oid = o.oprnamespace
join pg_proc p on p.oid = o.oprcode
join pg_namespace pn on pn.oid = p.pronamespace
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and n.nspname != 'public'
order by n.nspname, o.oprname;
";

const PG_VERSION_QUERY: &str = "show server_version;";

fn write_table(sql: &mut String, schema: &str, table_name: &str, columns: &[(String, String)]) {
    sql.push_str(&format!("create table {schema}.{table_name} (\n"));
    for (i, (col_name, col_type)) in columns.iter().enumerate() {
        let comma = if i + 1 < columns.len() { "," } else { "" };
        sql.push_str(&format!("  {col_name} {col_type}{comma}\n"));
    }
    sql.push_str(");\n\n");
}

fn write_view(sql: &mut String, schema: &str, view_name: &str, columns: &[(String, String)]) {
    let col_names: Vec<_> = columns.iter().map(|(name, _)| name.as_str()).collect();
    sql.push_str(&format!(
        "create view {schema}.{view_name}({}) as\n  select\n",
        col_names.join(", ")
    ));
    for (i, (_, col_type)) in columns.iter().enumerate() {
        let comma = if i + 1 < columns.len() { "," } else { "" };
        sql.push_str(&format!("    null::{col_type}{comma}\n"));
    }
    sql.push_str(";\n\n");
}

fn run_sql(query: &str) -> Result<String> {
    let output = Command::new("psql")
        .args([
            "--tuples-only",
            "--no-align",
            "--field-separator",
            "\t",
            "--command",
            query,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to run psql.")?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).context("Invalid utf8")?;
        bail!("psql failed: {}", stderr);
    }

    String::from_utf8(output.stdout).context("Invalid utf8")
}

pub(crate) fn sync_builtins() -> Result<()> {
    let version = run_sql(PG_VERSION_QUERY)?;
    let version = version
        .split_whitespace()
        .next()
        .context("version not found")?;

    let mut sql = format!(
        "\
-- squawk-ignore-file
-- pg version: {version}
-- update via:
--   cargo xtask sync-builtins

"
    );

    for schema in run_sql(BUILTIN_SCHEMAS_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        sql.push_str(&format!("create schema {schema};\n"));
    }
    sql.push_str("create schema pg_temp;\n");
    sql.push('\n');

    for line in run_sql(BUILTIN_TYPES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let type_name = parts.next().context("expected type name")?;
        let type_size = parts.next().context("expected type size")?;
        let type_align = parts.next().context("expected type alignment")?;
        if type_align.is_empty() {
            bail!("unexpected type alignment for {schema}.{type_name}");
        }
        sql.push_str(&format!(
            "-- size: {type_size}, align: {type_align}\ncreate type {schema}.{type_name};\n\n"
        ));
    }

    for line in run_sql(BUILTIN_RANGE_TYPES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let type_name = parts.next().context("expected type name")?;
        let type_size = parts.next().context("expected type size")?;
        let type_align = parts.next().context("expected type alignment")?;
        let subtype = parts.next().context("expected subtype")?;
        sql.push_str(&format!(
            "-- size: {type_size}, align: {type_align}\ncreate type {schema}.{type_name} as range (subtype = {subtype});\n\n"
        ));
    }

    let mut current_relation: Option<(String, String, String)> = None;
    let mut columns: Vec<(String, String)> = vec![];

    for line in run_sql(BUILTIN_TABLES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let rel_name = parts.next().context("expected relation name")?;
        let relkind = parts.next().context("expected relkind")?;
        let col_name = parts.next().context("expected column name")?;
        let col_type = parts.next().context("expected column type")?;

        if current_relation
            .as_ref()
            .map(|(s, t, _)| (s.as_str(), t.as_str()))
            != Some((schema, rel_name))
        {
            if let Some((prev_schema, prev_rel, prev_kind)) = current_relation.take() {
                if prev_kind == "v" {
                    write_view(&mut sql, &prev_schema, &prev_rel, &columns);
                } else {
                    write_table(&mut sql, &prev_schema, &prev_rel, &columns);
                }
                columns.clear();
            }
            current_relation = Some((
                schema.to_string(),
                rel_name.to_string(),
                relkind.to_string(),
            ));
        }

        columns.push((col_name.to_string(), col_type.to_string()));
    }

    if let Some((schema, rel_name, relkind)) = current_relation {
        if relkind == "v" {
            write_view(&mut sql, &schema, &rel_name, &columns);
        } else {
            write_table(&mut sql, &schema, &rel_name, &columns);
        }
    }

    for line in run_sql(BUILTIN_FUNCTIONS_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let func_name = parts.next().context("expected function name")?;
        let args = parts.next().context("expected function arguments")?;
        let result = parts.next().context("expected function result")?;
        sql.push_str(&format!(
            "create function {schema}.{func_name}({args}) returns {result}\n  language internal;\n\n"
        ));
    }

    for line in run_sql(BUILTIN_OPERATORS_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let op_name = parts.next().context("expected operator name")?;
        let left_type = parts.next().context("expected left type")?;
        let right_type = parts.next().context("expected right type")?;
        let func_schema = parts.next().context("expected function schema")?;
        let func_name = parts.next().context("expected function name")?;

        let args = match (left_type, right_type) {
            ("-", r) => format!("  rightarg = {r},\n"),
            (l, "-") => format!("  leftarg = {l},\n"),
            (l, r) => format!("  leftarg = {l},\n  rightarg = {r},\n"),
        };
        sql.push_str(&format!(
            "create operator {schema}.{op_name} (\n{args}  function = {func_schema}.{func_name}\n);\n\n"
        ));
    }

    let builtins_path = project_root().join("crates/squawk_ide/src/builtins.sql");
    std::fs::write(&builtins_path, sql).context("Failed to write builtins.sql")?;

    Ok(())
}
