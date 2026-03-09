use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::path::project_root;

const PG_TEMP_SCHEMA_SQL: &str = "create schema pg_temp;\n\n";

const SCHEMAS_QUERY: &str = r"
select
  n.nspname,
  coalesce(d.description, ''),
  coalesce(ext.extname, 'builtins') as extension_name
from pg_namespace n
  left join pg_description d on d.objoid = n.oid and d.classoid = 'pg_namespace'::regclass
  left join pg_depend dep on dep.classid = 'pg_namespace'::regclass and dep.objid = n.oid and dep.objsubid = 0 and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
order by n.nspname, ext.extname, n.oid;
";

const TYPES_QUERY: &str = r"
select
  n.nspname,
  t.typname,
  t.typlen,
  case t.typalign
    when 'c' then 1
    when 's' then 2
    when 'i' then 4
    when 'd' then 8
  end as typalign,
  coalesce(format_type(r.rngsubtype, null), '') as subtype,
  case when r.rngtypid is null then 0 else 1 end as is_range,
  coalesce(d.description, ''),
  coalesce(ext.extname, 'builtins') as extension_name
from pg_type t
  join pg_namespace n on n.oid = t.typnamespace
  left join pg_range r on r.rngtypid = t.oid
  left join pg_description d on d.objoid = t.oid and d.classoid = 'pg_type'::regclass
  left join pg_depend dep on dep.classid = 'pg_type'::regclass and dep.objid = t.oid and dep.objsubid = 0 and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and (
    r.rngtypid is not null
    or (t.typtype in ('b', 'p', 'd', 'e') and t.typname not like '\_%')
  )
order by n.nspname, t.typname, t.oid;
";

const TABLES_QUERY: &str = r"
select
  n.nspname,
  c.relname,
  c.relkind,
  coalesce(d.description, ''),
  coalesce(ext.extname, 'builtins') as extension_name,
  json_agg(
    json_build_object(
      'name', a.attname,
      'data_type', format_type(a.atttypid, a.atttypmod)
    )
    order by a.attnum
  ) as columns
from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  join pg_attribute a on a.attrelid = c.oid
  left join pg_description d on d.objoid = c.oid and d.classoid = 'pg_class'::regclass and d.objsubid = 0
  left join pg_depend dep on dep.classid = 'pg_class'::regclass and dep.objid = c.oid and dep.objsubid = 0 and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and c.relkind in ('r', 'v')
  and a.attnum > 0
  and not a.attisdropped
group by c.oid, n.nspname, c.relname, c.relkind, coalesce(d.description, ''), coalesce(ext.extname, 'builtins')
order by n.nspname, c.relname, c.oid;
";

const FUNCTIONS_QUERY: &str = r"
select
  n.nspname,
  p.proname,
  pg_get_function_arguments(p.oid) as args,
  pg_get_function_result(p.oid) as result,
  l.lanname,
  coalesce(d.description, ''),
  p.prokind,
  a.aggtransfn::regproc::text as trans_fn,
  format_type(a.aggtranstype, null) as trans_type,
  a.aggfinalfn::regproc::text as final_fn,
  a.aggcombinefn::regproc::text as combine_fn,
  coalesce(quote_literal(a.agginitval), '') as init_val,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_proc p
  join pg_namespace n on n.oid = p.pronamespace
  join pg_language l on l.oid = p.prolang
  left join pg_description d on d.objoid = p.oid and d.classoid = 'pg_proc'::regclass
  left join pg_aggregate a on a.aggfnoid = p.oid
  left join pg_depend dep on dep.classid = 'pg_proc'::regclass and dep.objid = p.oid and dep.objsubid = 0 and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and pg_get_function_arguments(p.oid) not like '%ORDER BY%'
order by n.nspname, p.proname, pg_get_function_arguments(p.oid), pg_get_function_result(p.oid), p.oid;
";

const OPERATORS_QUERY: &str = r"
select
  n.nspname,
  o.oprname,
  format_type(o.oprleft, null) as left_type,
  format_type(o.oprright, null) as right_type,
  pn.nspname as func_schema,
  p.proname as func_name,
  coalesce(d.description, ''),
  coalesce(ext.extname, 'builtins') as extension_name
from pg_operator o
  join pg_namespace n on n.oid = o.oprnamespace
  join pg_proc p on p.oid = o.oprcode
  join pg_namespace pn on pn.oid = p.pronamespace
  left join pg_description d on d.objoid = o.oid and d.classoid = 'pg_operator'::regclass
  left join pg_depend dep on dep.classid = 'pg_operator'::regclass and dep.objid = o.oid and dep.objsubid = 0 and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, o.oprname, format_type(o.oprleft, null), format_type(o.oprright, null), pn.nspname, p.proname, o.oid;
";

const PG_VERSION_QUERY: &str = "show server_version;";

const CREATE_EXTENSIONS_QUERY: &str = r"
do $$
declare
  extension_name text;
begin
  foreach extension_name in array array[
    'bloom',
    'citext',
    'cube',
    'hstore',
    'isn',
    'ltree',
    'pg_stat_statements',
    'pg_trgm',
    'pgcrypto',
    'plpgsql',
    'postgis',
    'postgres_fdw',
    'vector'
  ] loop
    execute format('create extension if not exists %I', extension_name);
  end loop;
end $$;
";

fn write_description<W: Write>(f: &mut W, description: &str) -> io::Result<()> {
    if !description.is_empty() {
        writeln!(f, "-- {}", description.replace('\n', "\n-- "))?;
    }

    Ok(())
}

trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()>;
}

#[derive(Clone, Deserialize)]
struct Column {
    name: String,
    data_type: String,
}

struct SchemaDef {
    schema: String,
    description: String,
}

impl WriteSql for SchemaDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create schema {};", self.schema)?;
        writeln!(f)?;
        Ok(())
    }
}

struct TypeDef {
    schema: String,
    name: String,
    size: String,
    align: String,
    description: String,
}

impl WriteSql for TypeDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "-- size: {}, align: {}", self.size, self.align)?;
        writeln!(f, "create type {}.{};", self.schema, self.name)?;
        writeln!(f)?;
        Ok(())
    }
}

struct RangeTypeDef {
    schema: String,
    name: String,
    size: String,
    align: String,
    subtype: String,
    description: String,
}

impl WriteSql for RangeTypeDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "-- size: {}, align: {}", self.size, self.align)?;
        writeln!(
            f,
            "create type {}.{} as range (subtype = {});",
            self.schema, self.name, self.subtype
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct TableDef {
    schema: String,
    name: String,
    description: String,
    columns: Vec<Column>,
}

impl WriteSql for TableDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create table {}.{} (", self.schema, self.name)?;
        for (index, column) in self.columns.iter().enumerate() {
            let comma = if index + 1 < self.columns.len() {
                ","
            } else {
                ""
            };
            writeln!(f, "  {} {}{comma}", column.name, column.data_type)?;
        }
        writeln!(f, ");")?;
        writeln!(f)?;
        Ok(())
    }
}

struct ViewDef {
    schema: String,
    name: String,
    description: String,
    columns: Vec<Column>,
}

impl WriteSql for ViewDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        let col_names: Vec<_> = self
            .columns
            .iter()
            .map(|column| column.name.as_str())
            .collect();
        writeln!(
            f,
            "create view {}.{}({}) as",
            self.schema,
            self.name,
            col_names.join(", ")
        )?;
        writeln!(f, "  select")?;
        for (index, column) in self.columns.iter().enumerate() {
            let comma = if index + 1 < self.columns.len() {
                ","
            } else {
                ""
            };
            writeln!(f, "    null::{}{comma}", column.data_type)?;
        }
        writeln!(f, ";")?;
        writeln!(f)?;
        Ok(())
    }
}

struct RegularFunctionDef {
    schema: String,
    name: String,
    args: String,
    result: String,
    language: String,
    description: String,
}

impl WriteSql for RegularFunctionDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(
            f,
            "create function {}.{}({}) returns {}",
            self.schema, self.name, self.args, self.result
        )?;
        writeln!(f, "  language {};", self.language)?;
        writeln!(f)?;
        Ok(())
    }
}

struct AggregateDef {
    schema: String,
    name: String,
    args: String,
    trans_fn: String,
    trans_type: String,
    final_fn: String,
    combine_fn: String,
    init_val: String,
    description: String,
}

impl WriteSql for AggregateDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;

        let args = if self.args.is_empty() {
            "*"
        } else {
            self.args.as_str()
        };

        writeln!(
            f,
            "create aggregate {}.{}({}) (",
            self.schema, self.name, args
        )?;
        writeln!(f, "  sfunc = {},", self.trans_fn)?;
        write!(f, "  stype = {}", self.trans_type)?;

        if self.final_fn != "-" {
            write!(f, ",\n  finalfunc = {}", self.final_fn)?;
        }

        if self.combine_fn != "-" {
            write!(f, ",\n  combinefunc = {}", self.combine_fn)?;
        }

        if !self.init_val.is_empty() {
            write!(f, ",\n  initcond = {}", self.init_val)?;
        }

        write!(f, "\n);\n\n")?;

        Ok(())
    }
}

enum FunctionDef {
    Aggregate(AggregateDef),
    Regular(RegularFunctionDef),
}

impl WriteSql for FunctionDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        match self {
            FunctionDef::Aggregate(aggregate) => aggregate.write_sql(f),
            FunctionDef::Regular(function) => function.write_sql(f),
        }
    }
}

struct OperatorDef {
    schema: String,
    name: String,
    left_type: String,
    right_type: String,
    function_schema: String,
    function_name: String,
    description: String,
}

impl WriteSql for OperatorDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create operator {}.{} (", self.schema, self.name)?;
        let args = match (self.left_type.as_str(), self.right_type.as_str()) {
            ("-", right_type) => format!("  rightarg = {right_type},"),
            (left_type, "-") => format!("  leftarg = {left_type},"),
            (left_type, right_type) => {
                format!("  leftarg = {left_type},\n  rightarg = {right_type},")
            }
        };
        writeln!(f, "{args}")?;
        writeln!(
            f,
            "  function = {}.{}",
            self.function_schema, self.function_name
        )?;
        writeln!(f, ");")?;
        writeln!(f)?;

        Ok(())
    }
}

// Module / File / Extension
//
// General either the builtins or an extension's defs
#[derive(Default)]
struct Module {
    schemas: Vec<SchemaDef>,
    types: Vec<TypeDef>,
    range_types: Vec<RangeTypeDef>,
    tables: Vec<TableDef>,
    views: Vec<ViewDef>,
    functions: Vec<FunctionDef>,
    operators: Vec<OperatorDef>,
}

impl Module {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        for schema in &self.schemas {
            schema.write_sql(f)?;
        }

        for type_def in &self.types {
            type_def.write_sql(f)?;
        }

        for range_type in &self.range_types {
            range_type.write_sql(f)?;
        }

        for table in &self.tables {
            table.write_sql(f)?;
        }

        for view in &self.views {
            view.write_sql(f)?;
        }

        for function in &self.functions {
            function.write_sql(f)?;
        }

        for operator in &self.operators {
            operator.write_sql(f)?;
        }

        Ok(())
    }
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

fn create_extensions_if_missing() -> Result<()> {
    run_sql(CREATE_EXTENSIONS_QUERY)?;
    Ok(())
}

fn header(version: &str) -> String {
    format!(
        "\
-- squawk-ignore-file
-- pg version: {version}
-- update via:
--   cargo xtask sync-builtins

"
    )
}

fn write_module(
    path: impl AsRef<Path>,
    version: &str,
    prefix: &str,
    module: &Module,
) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}{}", header(version), prefix)?;
    module.write_sql(&mut writer)?;
    writer.flush()?;

    Ok(())
}

fn query_schemas(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for line in run_sql(SCHEMAS_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let description = parts.next().context("expected schema description")?;
        let extension_name = parts.next().context("expected extension name")?;

        modules
            .entry(extension_name.to_string())
            .or_default()
            .schemas
            .push(SchemaDef {
                description: description.to_string(),
                schema: schema.to_string(),
            });
    }
    Ok(())
}

fn query_types(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for line in run_sql(TYPES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let type_name = parts.next().context("expected type name")?;
        let type_size = parts.next().context("expected type size")?;
        let type_align = parts.next().context("expected type alignment")?;
        let subtype = parts.next().context("expected subtype")?;
        let is_range = parts.next().context("expected is_range")?;
        let description = parts.next().context("expected type description")?;
        let extension_name = parts.next().context("expected extension name")?;
        if type_align.is_empty() {
            bail!("unexpected type alignment for {schema}.{type_name}");
        }

        let module = modules.entry(extension_name.to_string()).or_default();
        if is_range == "1" {
            module.range_types.push(RangeTypeDef {
                align: type_align.to_string(),
                description: description.to_string(),
                name: type_name.to_string(),
                schema: schema.to_string(),
                size: type_size.to_string(),
                subtype: subtype.to_string(),
            });
        } else {
            module.types.push(TypeDef {
                align: type_align.to_string(),
                description: description.to_string(),
                name: type_name.to_string(),
                schema: schema.to_string(),
                size: type_size.to_string(),
            });
        }
    }

    Ok(())
}

fn query_relations(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for line in run_sql(TABLES_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let rel_name = parts.next().context("expected relation name")?;
        let relkind = parts.next().context("expected relkind")?;
        let description = parts.next().context("expected description")?;
        let extension_name = parts.next().context("expected extension name")?;
        let columns = parts.next().context("expected columns")?;

        let columns: Vec<Column> =
            serde_json::from_str(columns).context("expected valid column json")?;

        let module = modules.entry(extension_name.to_string()).or_default();
        match relkind {
            "r" => module.tables.push(TableDef {
                columns,
                description: description.to_string(),
                name: rel_name.to_string(),
                schema: schema.to_string(),
            }),
            "v" => module.views.push(ViewDef {
                columns,
                description: description.to_string(),
                name: rel_name.to_string(),
                schema: schema.to_string(),
            }),
            _ => bail!("unexpected relation kind: {relkind}"),
        }
    }

    Ok(())
}

fn query_functions(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for line in run_sql(FUNCTIONS_QUERY)?
        .lines()
        .filter(|line| !line.is_empty())
    {
        let mut parts = line.split('\t');
        let schema = parts.next().context("expected schema name")?;
        let func_name = parts.next().context("expected function name")?;
        let args = parts.next().context("expected function arguments")?;
        let result = parts.next().context("expected function result")?;
        let language = parts.next().context("expected function language")?;
        let description = parts.next().context("expected function description")?;
        let prokind = parts.next().context("expected function kind")?;
        let trans_fn = parts.next().context("expected aggregate trans fn")?;
        let trans_type = parts.next().context("expected aggregate trans type")?;
        let final_fn = parts.next().context("expected aggregate final fn")?;
        let combine_fn = parts.next().context("expected aggregate combine fn")?;
        let init_val = parts.next().context("expected aggregate init value")?;
        let extension_name = parts.next().context("expected extension name")?;

        let function = if prokind == "a" {
            FunctionDef::Aggregate(AggregateDef {
                args: args.to_string(),
                combine_fn: combine_fn.to_string(),
                description: description.to_string(),
                final_fn: final_fn.to_string(),
                init_val: init_val.to_string(),
                name: func_name.to_string(),
                schema: schema.to_string(),
                trans_fn: trans_fn.to_string(),
                trans_type: trans_type.to_string(),
            })
        } else {
            FunctionDef::Regular(RegularFunctionDef {
                args: args.to_string(),
                description: description.to_string(),
                language: language.to_string(),
                name: func_name.to_string(),
                result: result.to_string(),
                schema: schema.to_string(),
            })
        };

        modules
            .entry(extension_name.to_string())
            .or_default()
            .functions
            .push(function);
    }

    Ok(())
}

fn query_operators(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for line in run_sql(OPERATORS_QUERY)?
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
        let description = parts.next().context("expected operator description")?;
        let extension_name = parts.next().context("expected extension name")?;

        modules
            .entry(extension_name.to_string())
            .or_default()
            .operators
            .push(OperatorDef {
                description: description.to_string(),
                function_name: func_name.to_string(),
                function_schema: func_schema.to_string(),
                left_type: left_type.to_string(),
                name: op_name.to_string(),
                right_type: right_type.to_string(),
                schema: schema.to_string(),
            });
    }

    Ok(())
}

pub(crate) fn sync_builtins() -> Result<()> {
    create_extensions_if_missing()?;

    let version = run_sql(PG_VERSION_QUERY)?;
    let version = version
        .split_whitespace()
        .next()
        .context("version not found")?;

    let mut modules: BTreeMap<String, Module> = BTreeMap::new();

    query_schemas(&mut modules)?;
    query_types(&mut modules)?;
    query_relations(&mut modules)?;
    query_functions(&mut modules)?;
    query_operators(&mut modules)?;

    let extensions_root = project_root().join("crates/squawk_ide/src/generated/extensions");
    let builtins_path = project_root().join("crates/squawk_ide/src/generated/builtins.sql");

    std::fs::create_dir_all(&extensions_root).context("Failed to create builtins directory")?;
    if extensions_root.exists() {
        std::fs::remove_dir_all(&extensions_root)?;
    }
    std::fs::create_dir_all(&extensions_root)?;

    let builtins = modules.remove("builtins").unwrap_or_default();
    write_module(&builtins_path, version, PG_TEMP_SCHEMA_SQL, &builtins)?;

    for (file_name, module) in &modules {
        let file_path = extensions_root.join(format!("{file_name}.sql"));
        write_module(&file_path, version, "", module)?;
    }

    Ok(())
}
