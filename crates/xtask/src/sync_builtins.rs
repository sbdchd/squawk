use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::path::project_root;

const BUILTIN_TYPES_QUERY: &str = r"
select typname, typlen, case typalign
    when 'c' then 1
    when 's' then 2
    when 'i' then 4
    when 'd' then 8
  end as typalign
from pg_type
where typnamespace = 'pg_catalog'::regnamespace
  and typtype in ('b', 'p', 'r', 'd')
  and typname not like '\_%'
order by typname;
";

const PG_VERSION_QUERY: &str = "show server_version;";

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
        .trim()
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

    for line in run_sql(BUILTIN_TYPES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('|');
        let type_name = parts.next().context("expected type name")?;
        let type_size = parts.next().context("expected type size")?;
        let type_align = parts.next().context("expected type alignment")?;
        if type_align.is_empty() {
            bail!("unexpected type alignment for {type_name}");
        }
        sql.push_str(&format!(
            "-- size: {type_size}, align: {type_align}\ncreate type {type_name};\n\n"
        ));
    }

    let builtins_path = project_root().join("crates/squawk_ide/src/builtins.sql");
    std::fs::write(&builtins_path, sql).context("Failed to write builtins.sql")?;

    Ok(())
}
