use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::path::project_root;

const PG_TEMP_SCHEMA_SQL: &str = "create schema pg_temp;\n\n";

#[derive(Deserialize)]
struct SchemaQuery {
    schema: String,
    description: String,
    extension_name: String,
}

impl Query for SchemaQuery {
    const QUERY: &'static str = r"
select
  n.nspname as schema,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_namespace n
  left join pg_description d on d.objoid = n.oid and d.classoid = 'pg_namespace'::regclass
  left join pg_depend dep on dep.classid = 'pg_namespace'::regclass and dep.objid = n.oid and dep.objsubid = 0 and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
order by n.nspname, ext.extname, n.oid;
";
}

trait Query {
    const QUERY: &'static str;

    fn run() -> Result<Vec<Self>>
    where
        Self: serde::de::DeserializeOwned,
    {
        let output = Command::new("psql")
            .args(["--csv", "--command", Self::QUERY])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to run psql.")?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).context("Invalid utf8")?;
            bail!("psql failed: {}", stderr);
        }

        let mut reader = csv::Reader::from_reader(output.stdout.as_slice());
        let mut rows = vec![];
        for result in reader.deserialize() {
            rows.push(result.context("failed to parse csv record")?);
        }
        Ok(rows)
    }

    fn execute() -> Result<()> {
        let output = Command::new("psql")
            .args(["--command", Self::QUERY])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to run psql.")?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).context("Invalid utf8")?;
            bail!("psql failed: {}", stderr);
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct TypeQuery {
    schema: String,
    name: String,
    size: String,
    align: String,
    subtype: String,
    is_range: i32,
    description: String,
    extension_name: String,
}

impl Query for TypeQuery {
    const QUERY: &'static str = r"
select
  n.nspname as schema,
  t.typname as name,
  t.typlen as size,
  case t.typalign
    when 'c' then 1
    when 's' then 2
    when 'i' then 4
    when 'd' then 8
  end as align,
  coalesce(format_type(r.rngsubtype, null), '') as subtype,
  case when r.rngtypid is null then 0 else 1 end as is_range,
  coalesce(d.description, '') as description,
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
}

#[derive(Deserialize)]
struct RelationQuery {
    schema: String,
    name: String,
    relkind: String,
    description: String,
    extension_name: String,
    columns: String,
}

impl Query for RelationQuery {
    const QUERY: &'static str = r"
select
  n.nspname as schema,
  c.relname as name,
  c.relkind as relkind,
  coalesce(d.description, '') as description,
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
group by c.oid, schema, name, relkind, description, extension_name
order by n.nspname, c.relname, c.oid;
";
}

#[derive(Deserialize)]
struct FunctionQuery {
    schema: String,
    name: String,
    args: String,
    result: String,
    language: String,
    description: String,
    prokind: String,
    trans_fn: String,
    trans_type: String,
    final_fn: String,
    combine_fn: String,
    init_val: String,
    extension_name: String,
}

impl Query for FunctionQuery {
    const QUERY: &'static str = r"
select
  n.nspname as schema,
  p.proname as name,
  pg_get_function_arguments(p.oid) as args,
  pg_get_function_result(p.oid) as result,
  l.lanname as language,
  coalesce(d.description, '') as description,
  p.prokind as prokind,
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
}

#[derive(Deserialize)]
struct OperatorQuery {
    schema: String,
    name: String,
    left_type: String,
    right_type: String,
    func_schema: String,
    func_name: String,
    description: String,
    extension_name: String,
}

impl Query for OperatorQuery {
    const QUERY: &'static str = r"
select
  n.nspname as schema,
  o.oprname as name,
  format_type(o.oprleft, null) as left_type,
  format_type(o.oprright, null) as right_type,
  pn.nspname as func_schema,
  p.proname as func_name,
  coalesce(d.description, '') as description,
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
}

#[derive(Deserialize)]
struct VersionQuery {
    server_version: String,
}

impl Query for VersionQuery {
    const QUERY: &'static str = "show server_version;";
}

struct CreateExtensionsQuery;

impl Query for CreateExtensionsQuery {
    const QUERY: &'static str = r"
do $$
declare
  extension_name text;
begin
  foreach extension_name in array array[
    'bloom',
    'citext',
    'cube',
    'h3',
    'hll',
    'hstore',
    'isn',
    'ltree',
    'pg_stat_statements',
    'pg_trgm',
    'pg_walinspect',
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
}

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
    for row in SchemaQuery::run()? {
        modules
            .entry(row.extension_name)
            .or_default()
            .schemas
            .push(SchemaDef {
                description: row.description,
                schema: row.schema,
            });
    }
    Ok(())
}

fn query_types(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in TypeQuery::run()? {
        if row.align.is_empty() {
            bail!("unexpected type alignment for {}.{}", row.schema, row.name);
        }

        let module = modules.entry(row.extension_name).or_default();
        if row.is_range == 1 {
            module.range_types.push(RangeTypeDef {
                align: row.align,
                description: row.description,
                name: row.name,
                schema: row.schema,
                size: row.size,
                subtype: row.subtype,
            });
        } else {
            module.types.push(TypeDef {
                align: row.align,
                description: row.description,
                name: row.name,
                schema: row.schema,
                size: row.size,
            });
        }
    }

    Ok(())
}

fn query_relations(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in RelationQuery::run()? {
        let columns: Vec<Column> =
            serde_json::from_str(&row.columns).context("expected valid column json")?;

        let module = modules.entry(row.extension_name).or_default();
        match row.relkind.as_str() {
            "r" => module.tables.push(TableDef {
                columns,
                description: row.description,
                name: row.name,
                schema: row.schema,
            }),
            "v" => module.views.push(ViewDef {
                columns,
                description: row.description,
                name: row.name,
                schema: row.schema,
            }),
            _ => bail!("unexpected relation kind: {}", row.relkind),
        }
    }

    Ok(())
}

fn query_functions(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in FunctionQuery::run()? {
        let function = if row.prokind == "a" {
            FunctionDef::Aggregate(AggregateDef {
                args: row.args,
                combine_fn: row.combine_fn,
                description: row.description,
                final_fn: row.final_fn,
                init_val: row.init_val,
                name: row.name,
                schema: row.schema,
                trans_fn: row.trans_fn,
                trans_type: row.trans_type,
            })
        } else {
            FunctionDef::Regular(RegularFunctionDef {
                args: row.args,
                description: row.description,
                language: row.language,
                name: row.name,
                result: row.result,
                schema: row.schema,
            })
        };

        modules
            .entry(row.extension_name)
            .or_default()
            .functions
            .push(function);
    }

    Ok(())
}

fn query_operators(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in OperatorQuery::run()? {
        modules
            .entry(row.extension_name)
            .or_default()
            .operators
            .push(OperatorDef {
                description: row.description,
                function_name: row.func_name,
                function_schema: row.func_schema,
                left_type: row.left_type,
                name: row.name,
                right_type: row.right_type,
                schema: row.schema,
            });
    }

    Ok(())
}

pub(crate) fn sync_builtins() -> Result<()> {
    CreateExtensionsQuery::execute()?;

    let version_rows = VersionQuery::run()?;
    let version_row = version_rows.first().context("version not found")?;
    let version = version_row
        .server_version
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
