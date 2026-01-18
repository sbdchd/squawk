use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::path::project_root;

const BUILTIN_SCHEMAS_QUERY: &str = r"
select nspname
from pg_namespace
where nspname not like 'pg_temp%'
  and nspname not like 'pg_toast%'
  and nspname <> 'public'
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
  and n.nspname <> 'public'
  and t.typtype in ('b', 'p', 'r', 'd')
  and t.typname not like '\_%'
order by n.nspname, t.typname;
";

const BUILTIN_TABLES_QUERY: &str = r"
select n.nspname, c.relname, a.attname, format_type(a.atttypid, a.atttypmod) as type
from pg_class c
join pg_namespace n on n.oid = c.relnamespace
join pg_attribute a on a.attrelid = c.oid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and n.nspname <> 'public'
  and c.relkind = 'r'
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
  and n.nspname <> 'public'
  and pg_get_function_arguments(p.oid) not like '%ORDER BY%'
order by n.nspname, p.proname;
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

fn run_sql(query: &str) -> Result<String> {
    let output = Command::new("psql")
        .args(["--tuples-only", "--no-align", "--command", query])
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
        let mut parts = line.split('|');
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

    let mut current_table: Option<(String, String)> = None;
    let mut columns: Vec<(String, String)> = vec![];

    for line in run_sql(BUILTIN_TABLES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('|');
        let schema = parts.next().context("expected schema name")?;
        let table_name = parts.next().context("expected table name")?;
        let col_name = parts.next().context("expected column name")?;
        let col_type = parts.next().context("expected column type")?;

        if current_table
            .as_ref()
            .map(|(s, t)| (s.as_str(), t.as_str()))
            != Some((schema, table_name))
        {
            if let Some((prev_schema, prev_table)) = current_table.take() {
                write_table(&mut sql, &prev_schema, &prev_table, &columns);
                columns.clear();
            }
            current_table = Some((schema.to_string(), table_name.to_string()));
        }

        columns.push((col_name.to_string(), col_type.to_string()));
    }

    if let Some((schema, table_name)) = current_table {
        write_table(&mut sql, &schema, &table_name, &columns);
    }

    for line in run_sql(BUILTIN_FUNCTIONS_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('|');
        let schema = parts.next().context("expected schema name")?;
        let func_name = parts.next().context("expected function name")?;
        let args = parts.next().context("expected function arguments")?;
        let result = parts.next().context("expected function result")?;
        sql.push_str(&format!(
            "create function {schema}.{func_name}({args}) returns {result} language internal;\n\n"
        ));
    }

    let builtins_path = project_root().join("crates/squawk_ide/src/builtins.sql");
    std::fs::write(&builtins_path, sql).context("Failed to write builtins.sql")?;

    Ok(())
}
