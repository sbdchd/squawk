use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

use crate::path::project_root;

fn deserialize_json_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}

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
  left join pg_description d on
    d.objoid = n.oid and d.classoid = 'pg_namespace'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_namespace'::regclass
      and dep.objid = n.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where n.nspname not like 'pg_temp%' and n.nspname not like 'pg_toast_temp_%'
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
            .args(["--csv", "--quiet", "--command", Self::QUERY])
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
    input: String,
    output: String,
    element: String,
    subscript: String,
    subscript_result: String,
    slice_result: String,
    category: String,
    is_preferred: i32,
    description: String,
    extension_name: String,
}

impl Query for TypeQuery {
    const QUERY: &'static str = r"
create function pg_temp.subscript_result(ty regtype, index text) returns text
  language plpgsql
  as $$
declare
  result regtype;
begin
  execute format('select pg_typeof((null::%s)[%s])', ty, index) into result;
  return case
    when e.oid is not null then format('%I.%I[]', en.nspname, e.typname)
    else format('%I.%I', n.nspname, t.typname)
  end
  from pg_type t
    join pg_namespace n on n.oid = t.typnamespace
    left join pg_type e on e.typarray = t.oid
    left join pg_namespace en on en.oid = e.typnamespace
  where t.oid = result;
exception when others then
  return '';
end $$;
with recursive type_depths as (
  select oid, 0 as depth from pg_type where typelem = 0
  union all
  select t.oid, d.depth + 1
  from pg_type t join type_depths d on d.oid = t.typelem
)
select
  case when ext.extname is null then n.nspname || '.' else '' end as schema,
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
  case
    when t.typtype in ('b', 'p') then
      coalesce(
        (
          select quote_ident(proname)
          from pg_proc
          where oid = t.typinput and pronamespace = ext.extnamespace
        ),
        t.typinput::text
      )
    else ''
  end as input,
  case
    when t.typtype in ('b', 'p') then
      coalesce(
        (
          select quote_ident(proname)
          from pg_proc
          where oid = t.typoutput and pronamespace = ext.extnamespace
        ),
        t.typoutput::text
      )
    else ''
  end as output,
  case
    when t.typtype = 'b' and t.typelem <> 0 then format_type(t.typelem, null)
    else ''
  end as element,
  case
    when t.typtype = 'b' and t.typsubscript <> 0 then
      coalesce(
        (
          select quote_ident(proname)
          from pg_proc
          where oid = t.typsubscript and pronamespace = ext.extnamespace
        ),
        t.typsubscript::text
      )
    else ''
  end as subscript,
  case
    when t.typtype = 'b' and t.typsubscript <> 0 then
      pg_temp.subscript_result(t.oid, $$'1'$$)
    else ''
  end as subscript_result,
  case
    when t.typtype = 'b' and t.typsubscript <> 0 then
      pg_temp.subscript_result(t.oid, $$'1':'1'$$)
    else ''
  end as slice_result,
  case
    when t.typtype in ('b', 'p') and t.typcategory <> 'U' then
      quote_literal(t.typcategory::text)
    else ''
  end as category,
  case
    when t.typtype in ('b', 'p') and t.typispreferred then 1
    else 0
  end as is_preferred,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_type t
  join type_depths td on td.oid = t.oid
  join pg_namespace n on n.oid = t.typnamespace
  left join pg_range r on r.rngtypid = t.oid
  left join pg_description d on
    d.objoid = t.oid and d.classoid = 'pg_type'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_type'::regclass
      and dep.objid = t.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and (
    r.rngtypid is not null
      or (t.typtype in ('b', 'p', 'd', 'e') and t.typname not like '\_%')
  )
order by td.depth, n.nspname, t.typname, t.oid;
";
}

#[derive(Deserialize)]
struct RelationQuery {
    schema: String,
    name: String,
    relkind: String,
    description: String,
    extension_name: String,
    #[serde(deserialize_with = "deserialize_json_string")]
    columns: Vec<Column>,
}

impl Query for RelationQuery {
    const QUERY: &'static str = r"
select
  case when ext.extname is null then n.nspname || '.' else '' end as schema,
  c.relname as name,
  c.relkind as relkind,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name,
  json_agg(
    json_build_object(
      'name', a.attname,
      'data_type', format_type(a.atttypid, a.atttypmod)
    ) order by a.attnum
  ) as columns
from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  join pg_attribute a on a.attrelid = c.oid
  left join pg_description d on
    d.objoid = c.oid and d.classoid = 'pg_class'::regclass and d.objsubid = 0
  left join pg_depend dep on
    dep.classid = 'pg_class'::regclass
      and dep.objid = c.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and c.relkind in ('r', 'v')
  and a.attnum > 0
  and not a.attisdropped
group by c.oid, n.nspname, schema, name, relkind, description, extension_name
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
    is_final_extra: i32,
    combine_fn: String,
    init_val: String,
    sort_op: String,
    moving_trans_fn: String,
    moving_inv_trans_fn: String,
    moving_trans_type: String,
    moving_final_fn: String,
    is_moving_final_extra: i32,
    moving_init_val: String,
    serial_fn: String,
    deserial_fn: String,
    is_hypothetical: i32,
    extension_name: String,
}

impl Query for FunctionQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null or ext.extname = 'plpgsql' then n.nspname || '.'
    else ''
  end as schema,
  p.proname as name,
  pg_get_function_arguments(p.oid) as args,
  pg_get_function_result(p.oid) as result,
  l.lanname as language,
  coalesce(d.description, '') as description,
  p.prokind as prokind,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggtransfn and pronamespace = ext.extnamespace
    ),
    a.aggtransfn::regproc::text
  ) as trans_fn,
  format_type(a.aggtranstype, null) as trans_type,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggfinalfn and pronamespace = ext.extnamespace
    ),
    a.aggfinalfn::regproc::text
  ) as final_fn,
  case when a.aggfinalextra then 1 else 0 end as is_final_extra,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggcombinefn and pronamespace = ext.extnamespace
    ),
    a.aggcombinefn::regproc::text
  ) as combine_fn,
  coalesce(quote_literal(a.agginitval), '') as init_val,
  coalesce(
    'OPERATOR('
      || case
        when son.oid = ext.extnamespace then ''
        else son.nspname || '.'
      end
      || so.oprname
      || ')',
    ''
  ) as sort_op,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggmtransfn and pronamespace = ext.extnamespace
    ),
    a.aggmtransfn::regproc::text
  ) as moving_trans_fn,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggminvtransfn and pronamespace = ext.extnamespace
    ),
    a.aggminvtransfn::regproc::text
  ) as moving_inv_trans_fn,
  case
    when a.aggmtranstype = 0 then ''
    else format_type(a.aggmtranstype, null)
  end as moving_trans_type,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggmfinalfn and pronamespace = ext.extnamespace
    ),
    a.aggmfinalfn::regproc::text
  ) as moving_final_fn,
  case when a.aggmfinalextra then 1 else 0 end as is_moving_final_extra,
  coalesce(quote_literal(a.aggminitval), '') as moving_init_val,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggserialfn and pronamespace = ext.extnamespace
    ),
    a.aggserialfn::regproc::text
  ) as serial_fn,
  coalesce(
    (
      select quote_ident(proname)
      from pg_proc
      where oid = a.aggdeserialfn and pronamespace = ext.extnamespace
    ),
    a.aggdeserialfn::regproc::text
  ) as deserial_fn,
  case when a.aggkind = 'h' then 1 else 0 end as is_hypothetical,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_proc p
  join pg_namespace n on n.oid = p.pronamespace
  join pg_language l on l.oid = p.prolang
  left join pg_description d on
    d.objoid = p.oid and d.classoid = 'pg_proc'::regclass
  left join pg_aggregate a on a.aggfnoid = p.oid
  left join pg_operator so on so.oid = a.aggsortop
  left join pg_namespace son on son.oid = so.oprnamespace
  left join pg_depend dep on
    dep.classid = 'pg_proc'::regclass
      and dep.objid = p.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by
  n.nspname,
  p.proname,
  pg_get_function_arguments(p.oid),
  pg_get_function_result(p.oid),
  p.oid;
";
}

#[derive(Deserialize)]
struct CompositeTypeQuery {
    schema: String,
    name: String,
    description: String,
    extension_name: String,
    #[serde(deserialize_with = "deserialize_json_string")]
    columns: Vec<Column>,
}

impl Query for CompositeTypeQuery {
    const QUERY: &'static str = r"
select
  case when ext.extname is null then n.nspname || '.' else '' end as schema,
  t.typname as name,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name,
  json_agg(
    json_build_object(
      'name', a.attname,
      'data_type', format_type(a.atttypid, a.atttypmod)
    ) order by a.attnum
  ) as columns
from pg_type t
  join pg_namespace n on n.oid = t.typnamespace
  join pg_class c on c.oid = t.typrelid
  join pg_attribute a on a.attrelid = c.oid
  left join pg_description d on
    d.objoid = t.oid and d.classoid = 'pg_type'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_type'::regclass
      and dep.objid = t.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and t.typtype = 'c'
  and c.relkind = 'c'
  and a.attnum > 0
  and not a.attisdropped
group by t.oid, n.nspname, t.typname, d.description, ext.extname
order by n.nspname, t.typname, t.oid;
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
  case when ext.extname is null then n.nspname || '.' else '' end as schema,
  o.oprname as name,
  format_type(o.oprleft, null) as left_type,
  format_type(o.oprright, null) as right_type,
  case
    when pn.oid = ext.extnamespace then ''
    else pn.nspname || '.'
  end as func_schema,
  p.proname as func_name,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_operator o
  join pg_namespace n on n.oid = o.oprnamespace
  join pg_proc p on p.oid = o.oprcode
  join pg_namespace pn on pn.oid = p.pronamespace
  left join pg_description d on
    d.objoid = o.oid and d.classoid = 'pg_operator'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_operator'::regclass
      and dep.objid = o.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by
  n.nspname,
  o.oprname,
  format_type(o.oprleft, null),
  format_type(o.oprright, null),
  pn.nspname,
  p.proname,
  o.oid;
";
}

#[derive(Deserialize)]
struct CastQuery {
    source: String,
    target: String,
    context: String,
    method: String,
    function: String,
    description: String,
    extension_name: String,
}

impl Query for CastQuery {
    const QUERY: &'static str = r"
select
  format_type(c.castsource, null) as source,
  format_type(c.casttarget, null) as target,
  c.castcontext as context,
  c.castmethod as method,
  coalesce(
    case
        when pn.oid = ext.extnamespace then ''
        else pn.nspname || '.'
      end || quote_ident(p.proname)
      || '(' || oidvectortypes(p.proargtypes)
      || ')',
    ''
  ) as function,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_cast c
  left join pg_proc p on p.oid = c.castfunc
  left join pg_namespace pn on pn.oid = p.pronamespace
  left join pg_description d on
    d.objoid = c.oid and d.classoid = 'pg_cast'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_cast'::regclass
      and dep.objid = c.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
order by
  format_type(c.castsource, null),
  format_type(c.casttarget, null),
  c.oid;
";
}

#[derive(Deserialize)]
struct CollationQuery {
    schema: String,
    name: String,
    provider: String,
    locale: String,
    is_deterministic: i32,
    description: String,
    extension_name: String,
}

impl Query for CollationQuery {
    const QUERY: &'static str = r"
select
  case when ext.extname is null then n.nspname || '.' else '' end as schema,
  quote_ident(c.collname) as name,
  c.collprovider as provider,
  coalesce(c.colllocale, c.collcollate, '') as locale,
  case when c.collisdeterministic then 1 else 0 end as is_deterministic,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_collation c
  join pg_namespace n on n.oid = c.collnamespace
  left join pg_description d on
    d.objoid = c.oid and d.classoid = 'pg_collation'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_collation'::regclass
      and dep.objid = c.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
  and (c.oid < 10000 or ext.extname is not null)
order by n.nspname, c.collname, c.oid;
";
}

#[derive(Deserialize)]
struct LanguageQuery {
    name: String,
    handler: String,
    inline: String,
    validator: String,
    description: String,
    extension_name: String,
}

impl Query for LanguageQuery {
    const QUERY: &'static str = r"
select
  quote_ident(l.lanname) as name,
  coalesce(
    case
      when hn.oid = ext.extnamespace and ext.extname != 'plpgsql' then ''
      else quote_ident(hn.nspname) || '.'
    end || quote_ident(h.proname),
    ''
  ) as handler,
  coalesce(
    case
      when inn.oid = ext.extnamespace and ext.extname != 'plpgsql' then ''
      else quote_ident(inn.nspname) || '.'
    end || quote_ident(i.proname),
    ''
  ) as inline,
  coalesce(
    case
      when vn.oid = ext.extnamespace and ext.extname != 'plpgsql' then ''
      else quote_ident(vn.nspname) || '.'
    end || quote_ident(v.proname),
    ''
  ) as validator,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_language l
  left join pg_proc h on h.oid = l.lanplcallfoid
  left join pg_namespace hn on hn.oid = h.pronamespace
  left join pg_proc i on i.oid = l.laninline
  left join pg_namespace inn on inn.oid = i.pronamespace
  left join pg_proc v on v.oid = l.lanvalidator
  left join pg_namespace vn on vn.oid = v.pronamespace
  left join pg_description d on
    d.objoid = l.oid and d.classoid = 'pg_language'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_language'::regclass
      and dep.objid = l.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
order by l.lanname, l.oid;
";
}

#[derive(Deserialize)]
struct ForeignDataWrapperQuery {
    name: String,
    handler: String,
    validator: String,
    description: String,
    extension_name: String,
}

impl Query for ForeignDataWrapperQuery {
    const QUERY: &'static str = r"
select
  quote_ident(w.fdwname) as name,
  coalesce(
    case
      when hn.oid = ext.extnamespace then ''
      else quote_ident(hn.nspname) || '.'
    end || quote_ident(h.proname),
    ''
  ) as handler,
  coalesce(
    case
      when vn.oid = ext.extnamespace then ''
      else quote_ident(vn.nspname) || '.'
    end || quote_ident(v.proname),
    ''
  ) as validator,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_foreign_data_wrapper w
  left join pg_proc h on h.oid = w.fdwhandler
  left join pg_namespace hn on hn.oid = h.pronamespace
  left join pg_proc v on v.oid = w.fdwvalidator
  left join pg_namespace vn on vn.oid = v.pronamespace
  left join pg_description d on
    d.objoid = w.oid and d.classoid = 'pg_foreign_data_wrapper'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_foreign_data_wrapper'::regclass
      and dep.objid = w.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
order by w.fdwname, w.oid;
";
}

#[derive(Deserialize)]
struct ConversionQuery {
    schema: String,
    name: String,
    source_encoding: String,
    target_encoding: String,
    function: String,
    is_default: i32,
    description: String,
    extension_name: String,
}

impl Query for ConversionQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(c.conname) as name,
  quote_literal(pg_encoding_to_char(c.conforencoding)) as source_encoding,
  quote_literal(pg_encoding_to_char(c.contoencoding)) as target_encoding,
  case
    when pn.oid = ext.extnamespace then ''
    else quote_ident(pn.nspname) || '.'
  end || quote_ident(p.proname) as function,
  case when c.condefault then 1 else 0 end as is_default,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_conversion c
  join pg_namespace n on n.oid = c.connamespace
  join pg_proc p on p.oid = c.conproc
  join pg_namespace pn on pn.oid = p.pronamespace
  left join pg_description d on
    d.objoid = c.oid and d.classoid = 'pg_conversion'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_conversion'::regclass
      and dep.objid = c.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, c.conname, c.oid;
";
}

#[derive(Deserialize)]
struct OperatorFamilyQuery {
    schema: String,
    name: String,
    access_method: String,
    description: String,
    extension_name: String,
}

impl Query for OperatorFamilyQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(f.opfname) as name,
  quote_ident(a.amname) as access_method,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_opfamily f
  join pg_namespace n on n.oid = f.opfnamespace
  join pg_am a on a.oid = f.opfmethod
  left join pg_description d on
    d.objoid = f.oid and d.classoid = 'pg_opfamily'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_opfamily'::regclass
      and dep.objid = f.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, f.opfname, a.amname, f.oid;
";
}

#[derive(Deserialize)]
struct OperatorClassQuery {
    schema: String,
    name: String,
    is_default: i32,
    data_type: String,
    access_method: String,
    family: String,
    storage_type: String,
    description: String,
    extension_name: String,
}

impl Query for OperatorClassQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(c.opcname) as name,
  case when c.opcdefault then 1 else 0 end as is_default,
  format_type(c.opcintype, null) as data_type,
  quote_ident(a.amname) as access_method,
  case
    when fn.oid = ext.extnamespace then ''
    else quote_ident(fn.nspname) || '.'
  end || quote_ident(f.opfname) as family,
  format_type(
    coalesce(nullif(c.opckeytype, 0), c.opcintype),
    null
  ) as storage_type,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_opclass c
  join pg_namespace n on n.oid = c.opcnamespace
  join pg_am a on a.oid = c.opcmethod
  join pg_opfamily f on f.oid = c.opcfamily
  join pg_namespace fn on fn.oid = f.opfnamespace
  left join pg_description d on
    d.objoid = c.oid and d.classoid = 'pg_opclass'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_opclass'::regclass
      and dep.objid = c.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, c.opcname, a.amname, c.oid;
";
}

#[derive(Deserialize)]
struct TextSearchParserQuery {
    schema: String,
    name: String,
    start: String,
    gettoken: String,
    end: String,
    lextypes: String,
    headline: String,
    description: String,
    extension_name: String,
}

impl Query for TextSearchParserQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(p.prsname) as name,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = p.prsstart
    ),
    ''
  ) as start,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = p.prstoken
    ),
    ''
  ) as gettoken,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = p.prsend
    ),
    ''
  ) as end,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = p.prslextype
    ),
    ''
  ) as lextypes,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = p.prsheadline
    ),
    ''
  ) as headline,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_ts_parser p
  join pg_namespace n on n.oid = p.prsnamespace
  left join pg_description d on
    d.objoid = p.oid and d.classoid = 'pg_ts_parser'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_ts_parser'::regclass
      and dep.objid = p.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, p.prsname, p.oid;
";
}

#[derive(Deserialize)]
struct TextSearchTemplateQuery {
    schema: String,
    name: String,
    init: String,
    lexize: String,
    description: String,
    extension_name: String,
}

impl Query for TextSearchTemplateQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(t.tmplname) as name,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = t.tmplinit
    ),
    ''
  ) as init,
  coalesce(
    (
      select case
        when pn.oid = ext.extnamespace then ''
        else quote_ident(pn.nspname) || '.'
      end || quote_ident(pr.proname)
      from pg_proc pr join pg_namespace pn on pn.oid = pr.pronamespace
      where pr.oid = t.tmpllexize
    ),
    ''
  ) as lexize,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_ts_template t
  join pg_namespace n on n.oid = t.tmplnamespace
  left join pg_description d on
    d.objoid = t.oid and d.classoid = 'pg_ts_template'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_ts_template'::regclass
      and dep.objid = t.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, t.tmplname, t.oid;
";
}

#[derive(Deserialize)]
struct TextSearchDictionaryQuery {
    schema: String,
    name: String,
    template: String,
    options: String,
    description: String,
    extension_name: String,
}

impl Query for TextSearchDictionaryQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(dc.dictname) as name,
  case
    when tn.oid = ext.extnamespace then ''
    else quote_ident(tn.nspname) || '.'
  end || quote_ident(t.tmplname) as template,
  coalesce(dc.dictinitoption, '') as options,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_ts_dict dc
  join pg_namespace n on n.oid = dc.dictnamespace
  join pg_ts_template t on t.oid = dc.dicttemplate
  join pg_namespace tn on tn.oid = t.tmplnamespace
  left join pg_description d on
    d.objoid = dc.oid and d.classoid = 'pg_ts_dict'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_ts_dict'::regclass
      and dep.objid = dc.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, dc.dictname, dc.oid;
";
}

#[derive(Deserialize)]
struct TextSearchConfigurationQuery {
    schema: String,
    name: String,
    parser: String,
    description: String,
    extension_name: String,
}

impl Query for TextSearchConfigurationQuery {
    const QUERY: &'static str = r"
select
  case
    when ext.extname is null then quote_ident(n.nspname) || '.'
    else ''
  end as schema,
  quote_ident(c.cfgname) as name,
  case
    when pn.oid = ext.extnamespace then ''
    else quote_ident(pn.nspname) || '.'
  end || quote_ident(p.prsname) as parser,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_ts_config c
  join pg_namespace n on n.oid = c.cfgnamespace
  join pg_ts_parser p on p.oid = c.cfgparser
  join pg_namespace pn on pn.oid = p.prsnamespace
  left join pg_description d on
    d.objoid = c.oid and d.classoid = 'pg_ts_config'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_ts_config'::regclass
      and dep.objid = c.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
where
  n.nspname not like 'pg_temp%'
  and n.nspname not like 'pg_toast%'
  and (n.nspname != 'public' or ext.extname is not null)
order by n.nspname, c.cfgname, c.oid;
";
}

#[derive(Deserialize)]
struct AccessMethodQuery {
    name: String,
    kind: String,
    handler: String,
    description: String,
    extension_name: String,
}

impl Query for AccessMethodQuery {
    const QUERY: &'static str = r"
select
  quote_ident(a.amname) as name,
  case a.amtype when 't' then 'table' when 'i' then 'index' end as kind,
  case
    when n.oid = ext.extnamespace then ''
    else quote_ident(n.nspname) || '.'
  end || quote_ident(p.proname) as handler,
  coalesce(d.description, '') as description,
  coalesce(ext.extname, 'builtins') as extension_name
from pg_am a
  join pg_proc p on p.oid = a.amhandler
  join pg_namespace n on n.oid = p.pronamespace
  left join pg_description d on
    d.objoid = a.oid and d.classoid = 'pg_am'::regclass
  left join pg_depend dep on
    dep.classid = 'pg_am'::regclass
      and dep.objid = a.oid
      and dep.objsubid = 0
      and dep.deptype = 'e'
  left join pg_extension ext on ext.oid = dep.refobjid
order by a.amname, a.oid;
";
}

#[derive(Deserialize)]
struct RoleQuery {
    name: String,
    description: String,
}

impl Query for RoleQuery {
    const QUERY: &'static str = r"
select
  quote_ident(r.rolname) as name,
  coalesce(d.description, '') as description
from pg_roles r
  left join pg_shdescription d on
    d.objoid = r.oid and d.classoid = 'pg_authid'::regclass
where r.rolname like 'pg\_%'
order by r.rolname, r.oid;
";
}

#[derive(Deserialize)]
struct TablespaceQuery {
    name: String,
    location: String,
    description: String,
}

impl Query for TablespaceQuery {
    const QUERY: &'static str = r"
select
  quote_ident(t.spcname) as name,
  quote_literal(pg_tablespace_location(t.oid)) as location,
  coalesce(d.description, '') as description
from pg_tablespace t
  left join pg_shdescription d on
    d.objoid = t.oid and d.classoid = 'pg_tablespace'::regclass
order by t.spcname, t.oid;
";
}

#[derive(Deserialize)]
struct DatabaseQuery {
    name: String,
    description: String,
}

impl Query for DatabaseQuery {
    const QUERY: &'static str = r"
select
  quote_ident(d.datname) as name,
  coalesce(s.description, '') as description
from pg_database d
  left join pg_shdescription s on
    s.objoid = d.oid and s.classoid = 'pg_database'::regclass
where d.datname in ('postgres', 'template0', 'template1')
order by d.datname, d.oid;
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
    'btree_gin',
    'btree_gist',
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
    'tablefunc',
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

struct LanguageDef {
    name: String,
    handler: String,
    inline: String,
    validator: String,
    description: String,
}

impl WriteSql for LanguageDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(f, "create language {}", self.name)?;
        if !self.handler.is_empty() {
            write!(f, " handler {}", self.handler)?;
            if !self.inline.is_empty() {
                write!(f, " inline {}", self.inline)?;
            }
            if !self.validator.is_empty() {
                write!(f, " validator {}", self.validator)?;
            }
        }
        writeln!(f, ";")?;
        writeln!(f)?;
        Ok(())
    }
}

struct ForeignDataWrapperDef {
    name: String,
    handler: String,
    validator: String,
    description: String,
}

impl WriteSql for ForeignDataWrapperDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(f, "create foreign data wrapper {}", self.name)?;
        if !self.handler.is_empty() {
            write!(f, " handler {}", self.handler)?;
        }
        if !self.validator.is_empty() {
            write!(f, " validator {}", self.validator)?;
        }
        writeln!(f, ";")?;
        writeln!(f)?;
        Ok(())
    }
}

struct ConversionDef {
    schema: String,
    name: String,
    source_encoding: String,
    target_encoding: String,
    function: String,
    default: bool,
    description: String,
}

impl WriteSql for ConversionDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(f, "create ")?;
        if self.default {
            write!(f, "default ")?;
        }
        writeln!(
            f,
            "conversion {}{} for {} to {} from {};",
            self.schema, self.name, self.source_encoding, self.target_encoding, self.function
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct OperatorFamilyDef {
    schema: String,
    name: String,
    access_method: String,
    description: String,
}

impl WriteSql for OperatorFamilyDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(
            f,
            "create operator family {}{} using {};",
            self.schema, self.name, self.access_method
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct OperatorClassDef {
    schema: String,
    name: String,
    default: bool,
    data_type: String,
    access_method: String,
    family: String,
    storage_type: String,
    description: String,
}

impl WriteSql for OperatorClassDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(f, "create operator class {}{} ", self.schema, self.name)?;
        if self.default {
            write!(f, "default ")?;
        }
        // TODO: emit the class's OPERATOR and FUNCTION members (pg_amop/pg_amproc), and the
        // loose family members as `alter operator family ... add`, split using pg_depend.
        writeln!(
            f,
            "for type {} using {} family {} as storage {};",
            self.data_type, self.access_method, self.family, self.storage_type
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct TextSearchParserDef {
    schema: String,
    name: String,
    start: String,
    gettoken: String,
    end: String,
    lextypes: String,
    headline: String,
    description: String,
}

impl WriteSql for TextSearchParserDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(
            f,
            "create text search parser {}{} (start = {}, gettoken = {}, end = {}, lextypes = {}",
            self.schema, self.name, self.start, self.gettoken, self.end, self.lextypes
        )?;
        if !self.headline.is_empty() {
            write!(f, ", headline = {}", self.headline)?;
        }
        writeln!(f, ");")?;
        writeln!(f)?;
        Ok(())
    }
}

struct TextSearchTemplateDef {
    schema: String,
    name: String,
    init: String,
    lexize: String,
    description: String,
}

impl WriteSql for TextSearchTemplateDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(
            f,
            "create text search template {}{} (",
            self.schema, self.name
        )?;
        if !self.init.is_empty() {
            write!(f, "init = {}, ", self.init)?;
        }
        writeln!(f, "lexize = {});", self.lexize)?;
        writeln!(f)?;
        Ok(())
    }
}

struct TextSearchDictionaryDef {
    schema: String,
    name: String,
    template: String,
    options: String,
    description: String,
}

impl WriteSql for TextSearchDictionaryDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        write!(
            f,
            "create text search dictionary {}{} (template = {}",
            self.schema, self.name, self.template
        )?;
        if !self.options.is_empty() {
            write!(f, ", {}", self.options)?;
        }
        writeln!(f, ");")?;
        writeln!(f)?;
        Ok(())
    }
}

struct TextSearchConfigurationDef {
    schema: String,
    name: String,
    parser: String,
    description: String,
}

impl WriteSql for TextSearchConfigurationDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        // TODO: emit the token type to dictionary mappings (pg_ts_config_map) as
        // `alter text search configuration ... add mapping for ... with ...`.
        writeln!(
            f,
            "create text search configuration {}{} (parser = {});",
            self.schema, self.name, self.parser
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct AccessMethodDef {
    name: String,
    kind: String,
    handler: String,
    description: String,
}

impl WriteSql for AccessMethodDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(
            f,
            "create access method {} type {} handler {};",
            self.name, self.kind, self.handler
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct RoleDef {
    name: String,
    description: String,
}

impl WriteSql for RoleDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create role {};", self.name)?;
        writeln!(f)?;
        Ok(())
    }
}

struct TablespaceDef {
    name: String,
    location: String,
    description: String,
}

impl WriteSql for TablespaceDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(
            f,
            "create tablespace {} location {};",
            self.name, self.location
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct DatabaseDef {
    name: String,
    description: String,
}

impl WriteSql for DatabaseDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create database {};", self.name)?;
        writeln!(f)?;
        Ok(())
    }
}

struct TypeDef {
    schema: String,
    name: String,
    size: String,
    align: String,
    input: String,
    output: String,
    element: String,
    subscript: String,
    subscript_result: String,
    slice_result: String,
    category: String,
    preferred: bool,
    description: String,
}

impl WriteSql for TypeDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "-- size: {}, align: {}", self.size, self.align)?;
        writeln!(f, "create type {}{};", self.schema, self.name)?;
        writeln!(f)?;
        Ok(())
    }
}

// Base types start as shells so their I/O functions can reference them, then
// get filled in once the functions exist, matching the order Postgres needs.
struct BaseTypeDef<'a>(&'a TypeDef);

impl WriteSql for BaseTypeDef<'_> {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        let ty = self.0;
        let mut options = vec![
            format!("input = {}", ty.input),
            format!("output = {}", ty.output),
        ];
        if !ty.element.is_empty() {
            options.push(format!("element = {}", ty.element));
        }
        if !ty.subscript.is_empty() {
            options.push(format!("subscript = {}", ty.subscript));
        }
        if !ty.category.is_empty() {
            options.push(format!("category = {}", ty.category));
        }
        if ty.preferred {
            options.push("preferred = true".to_string());
        }
        if !ty.subscript_result.is_empty() {
            writeln!(f, "-- subscript: {}", ty.subscript_result)?;
        }
        if !ty.slice_result.is_empty() {
            writeln!(f, "-- subscript slice: {}", ty.slice_result)?;
        }
        writeln!(
            f,
            "create type {}{} ({});",
            ty.schema,
            ty.name,
            options.join(", ")
        )?;
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
            "create type {}{} as range (subtype = {});",
            self.schema, self.name, self.subtype
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct CompositeTypeDef {
    schema: String,
    name: String,
    description: String,
    columns: Vec<Column>,
}

impl WriteSql for CompositeTypeDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create type {}{} as (", self.schema, self.name)?;
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

struct TableDef {
    schema: String,
    name: String,
    description: String,
    columns: Vec<Column>,
}

impl WriteSql for TableDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        writeln!(f, "create table {}{} (", self.schema, self.name)?;
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
            "create view {}{}({}) as",
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
            "create function {}{}({}) returns {}",
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
    final_extra: bool,
    combine_fn: String,
    init_val: String,
    sort_op: String,
    moving_trans_fn: String,
    moving_inv_trans_fn: String,
    moving_trans_type: String,
    moving_final_fn: String,
    moving_final_extra: bool,
    moving_init_val: String,
    serial_fn: String,
    deserial_fn: String,
    hypothetical: bool,
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
            "create aggregate {}{}({}) (",
            self.schema, self.name, args
        )?;
        writeln!(f, "  sfunc = {},", self.trans_fn)?;
        write!(f, "  stype = {}", self.trans_type)?;

        if self.final_fn != "-" {
            write!(f, ",\n  finalfunc = {}", self.final_fn)?;
        }

        if self.final_extra {
            write!(f, ",\n  finalfunc_extra")?;
        }

        if self.combine_fn != "-" {
            write!(f, ",\n  combinefunc = {}", self.combine_fn)?;
        }

        if !self.init_val.is_empty() {
            write!(f, ",\n  initcond = {}", self.init_val)?;
        }

        if self.serial_fn != "-" {
            write!(f, ",\n  serialfunc = {}", self.serial_fn)?;
        }

        if self.deserial_fn != "-" {
            write!(f, ",\n  deserialfunc = {}", self.deserial_fn)?;
        }

        if self.moving_trans_fn != "-" {
            write!(f, ",\n  msfunc = {}", self.moving_trans_fn)?;
        }

        if self.moving_inv_trans_fn != "-" {
            write!(f, ",\n  minvfunc = {}", self.moving_inv_trans_fn)?;
        }

        if !self.moving_trans_type.is_empty() {
            write!(f, ",\n  mstype = {}", self.moving_trans_type)?;
        }

        if self.moving_final_fn != "-" {
            write!(f, ",\n  mfinalfunc = {}", self.moving_final_fn)?;
        }

        if self.moving_final_extra {
            write!(f, ",\n  mfinalfunc_extra")?;
        }

        if !self.moving_init_val.is_empty() {
            write!(f, ",\n  minitcond = {}", self.moving_init_val)?;
        }

        if !self.sort_op.is_empty() {
            write!(f, ",\n  sortop = {}", self.sort_op)?;
        }

        if self.hypothetical {
            write!(f, ",\n  hypothetical")?;
        }

        write!(f, "\n);\n\n")?;

        Ok(())
    }
}

enum FunctionDef {
    Aggregate(Box<AggregateDef>),
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
        writeln!(f, "create operator {}{} (", self.schema, self.name)?;
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
            "  function = {}{}",
            self.function_schema, self.function_name
        )?;
        writeln!(f, ");")?;
        writeln!(f)?;

        Ok(())
    }
}

struct CastDef {
    source: String,
    target: String,
    context: String,
    method: String,
    function: String,
    description: String,
}

impl WriteSql for CastDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        let method = match self.method.as_str() {
            "f" => format!("with function {}", self.function),
            "b" => "without function".to_string(),
            "i" => "with inout".to_string(),
            method => {
                return Err(io::Error::other(format!(
                    "unexpected cast method: {method}"
                )));
            }
        };
        let context = match self.context.as_str() {
            "i" => " as implicit",
            "a" => " as assignment",
            "e" => "",
            context => {
                return Err(io::Error::other(format!(
                    "unexpected cast context: {context}"
                )));
            }
        };
        writeln!(
            f,
            "create cast ({} as {}) {method}{context};",
            self.source, self.target
        )?;
        writeln!(f)?;
        Ok(())
    }
}

struct CollationDef {
    schema: String,
    name: String,
    provider: String,
    locale: String,
    deterministic: bool,
    description: String,
}

impl WriteSql for CollationDef {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        write_description(f, &self.description)?;
        let mut options = vec![];
        match self.provider.as_str() {
            "d" => options.push("provider = default".to_string()),
            "c" => options.push("provider = libc".to_string()),
            "i" => options.push("provider = icu".to_string()),
            "b" => options.push("provider = builtin".to_string()),
            provider => {
                return Err(io::Error::other(format!(
                    "unexpected collation provider: {provider}"
                )));
            }
        }
        if !self.locale.is_empty() {
            options.push(format!("locale = '{}'", self.locale.replace('\'', "''")));
        }
        if !self.deterministic {
            options.push("deterministic = false".to_string());
        }
        writeln!(
            f,
            "create collation {}{} ({});",
            self.schema,
            self.name,
            options.join(", ")
        )?;
        writeln!(f)?;
        Ok(())
    }
}

// Module / File / Extension
//
// General either the builtins or an extension's defs
#[derive(Default)]
struct Module {
    roles: Vec<RoleDef>,
    tablespaces: Vec<TablespaceDef>,
    databases: Vec<DatabaseDef>,
    schemas: Vec<SchemaDef>,
    types: Vec<TypeDef>,
    range_types: Vec<RangeTypeDef>,
    composite_types: Vec<CompositeTypeDef>,
    tables: Vec<TableDef>,
    views: Vec<ViewDef>,
    functions: Vec<FunctionDef>,
    operators: Vec<OperatorDef>,
    casts: Vec<CastDef>,
    collations: Vec<CollationDef>,
    access_methods: Vec<AccessMethodDef>,
    languages: Vec<LanguageDef>,
    foreign_data_wrappers: Vec<ForeignDataWrapperDef>,
    conversions: Vec<ConversionDef>,
    operator_families: Vec<OperatorFamilyDef>,
    operator_classes: Vec<OperatorClassDef>,
    text_search_parsers: Vec<TextSearchParserDef>,
    text_search_templates: Vec<TextSearchTemplateDef>,
    text_search_dictionaries: Vec<TextSearchDictionaryDef>,
    text_search_configurations: Vec<TextSearchConfigurationDef>,
}

impl Module {
    fn write_sql<W: Write>(&self, f: &mut W) -> io::Result<()> {
        for role in &self.roles {
            role.write_sql(f)?;
        }

        for tablespace in &self.tablespaces {
            tablespace.write_sql(f)?;
        }

        for database in &self.databases {
            database.write_sql(f)?;
        }

        for schema in &self.schemas {
            schema.write_sql(f)?;
        }

        for type_def in &self.types {
            type_def.write_sql(f)?;
        }

        for range_type in &self.range_types {
            range_type.write_sql(f)?;
        }

        for composite_type in &self.composite_types {
            composite_type.write_sql(f)?;
        }

        for table in &self.tables {
            table.write_sql(f)?;
        }

        for view in &self.views {
            view.write_sql(f)?;
        }

        for language in self.languages.iter().filter(|l| l.handler.is_empty()) {
            language.write_sql(f)?;
        }

        for function in self
            .functions
            .iter()
            .filter(|function| matches!(function, FunctionDef::Regular(_)))
        {
            function.write_sql(f)?;
        }

        for language in self.languages.iter().filter(|l| !l.handler.is_empty()) {
            language.write_sql(f)?;
        }

        for type_def in self
            .types
            .iter()
            .filter(|type_def| !type_def.input.is_empty())
        {
            BaseTypeDef(type_def).write_sql(f)?;
        }

        for operator in &self.operators {
            operator.write_sql(f)?;
        }

        for function in self
            .functions
            .iter()
            .filter(|function| matches!(function, FunctionDef::Aggregate(_)))
        {
            function.write_sql(f)?;
        }

        for cast in &self.casts {
            cast.write_sql(f)?;
        }

        for collation in &self.collations {
            collation.write_sql(f)?;
        }

        for access_method in &self.access_methods {
            access_method.write_sql(f)?;
        }

        for foreign_data_wrapper in &self.foreign_data_wrappers {
            foreign_data_wrapper.write_sql(f)?;
        }

        for conversion in &self.conversions {
            conversion.write_sql(f)?;
        }

        for operator_family in &self.operator_families {
            operator_family.write_sql(f)?;
        }

        for operator_class in &self.operator_classes {
            operator_class.write_sql(f)?;
        }

        for parser in &self.text_search_parsers {
            parser.write_sql(f)?;
        }

        for template in &self.text_search_templates {
            template.write_sql(f)?;
        }

        for dictionary in &self.text_search_dictionaries {
            dictionary.write_sql(f)?;
        }

        for configuration in &self.text_search_configurations {
            configuration.write_sql(f)?;
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

fn query_roles(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    let module = modules.entry("builtins".to_string()).or_default();
    for row in RoleQuery::run()? {
        module.roles.push(RoleDef {
            name: row.name,
            description: row.description,
        });
    }
    Ok(())
}

fn query_tablespaces(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    let module = modules.entry("builtins".to_string()).or_default();
    for row in TablespaceQuery::run()? {
        module.tablespaces.push(TablespaceDef {
            name: row.name,
            location: row.location,
            description: row.description,
        });
    }
    Ok(())
}

fn query_databases(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    let module = modules.entry("builtins".to_string()).or_default();
    for row in DatabaseQuery::run()? {
        module.databases.push(DatabaseDef {
            name: row.name,
            description: row.description,
        });
    }
    Ok(())
}

fn query_schemas(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in SchemaQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
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
            bail!("unexpected type alignment for {}{}", row.schema, row.name);
        }

        let module = modules.entry(module_name(row.extension_name)).or_default();
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
                element: row.element,
                input: row.input,
                name: row.name,
                output: row.output,
                schema: row.schema,
                size: row.size,
                subscript: row.subscript,
                subscript_result: row.subscript_result,
                slice_result: row.slice_result,
                category: row.category,
                preferred: row.is_preferred == 1,
            });
        }
    }

    Ok(())
}

fn query_composite_types(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in CompositeTypeQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .composite_types
            .push(CompositeTypeDef {
                columns: row.columns,
                description: row.description,
                name: row.name,
                schema: row.schema,
            });
    }

    Ok(())
}

fn query_relations(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in RelationQuery::run()? {
        let module = modules.entry(module_name(row.extension_name)).or_default();
        match row.relkind.as_str() {
            "r" => module.tables.push(TableDef {
                columns: row.columns,
                description: row.description,
                name: row.name,
                schema: row.schema,
            }),
            "v" => module.views.push(ViewDef {
                columns: row.columns,
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
            FunctionDef::Aggregate(Box::new(AggregateDef {
                args: row.args,
                combine_fn: row.combine_fn,
                description: row.description,
                final_fn: row.final_fn,
                final_extra: row.is_final_extra == 1,
                init_val: row.init_val,
                name: row.name,
                schema: row.schema,
                trans_fn: row.trans_fn,
                trans_type: row.trans_type,
                sort_op: row.sort_op,
                moving_trans_fn: row.moving_trans_fn,
                moving_inv_trans_fn: row.moving_inv_trans_fn,
                moving_trans_type: row.moving_trans_type,
                moving_final_fn: row.moving_final_fn,
                moving_final_extra: row.is_moving_final_extra == 1,
                moving_init_val: row.moving_init_val,
                serial_fn: row.serial_fn,
                deserial_fn: row.deserial_fn,
                hypothetical: row.is_hypothetical == 1,
            }))
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
            .entry(module_name(row.extension_name))
            .or_default()
            .functions
            .push(function);
    }

    Ok(())
}

fn query_operators(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in OperatorQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
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

fn query_casts(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in CastQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .casts
            .push(CastDef {
                context: row.context,
                description: row.description,
                function: row.function,
                method: row.method,
                source: row.source,
                target: row.target,
            });
    }

    Ok(())
}

fn query_collations(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in CollationQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .collations
            .push(CollationDef {
                deterministic: row.is_deterministic == 1,
                description: row.description,
                locale: row.locale,
                name: row.name,
                provider: row.provider,
                schema: row.schema,
            });
    }

    Ok(())
}

fn query_languages(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in LanguageQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .languages
            .push(LanguageDef {
                name: row.name,
                handler: row.handler,
                inline: row.inline,
                validator: row.validator,
                description: row.description,
            });
    }
    Ok(())
}

fn query_foreign_data_wrappers(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in ForeignDataWrapperQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .foreign_data_wrappers
            .push(ForeignDataWrapperDef {
                name: row.name,
                handler: row.handler,
                validator: row.validator,
                description: row.description,
            });
    }
    Ok(())
}

fn query_conversions(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in ConversionQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .conversions
            .push(ConversionDef {
                schema: row.schema,
                name: row.name,
                source_encoding: row.source_encoding,
                target_encoding: row.target_encoding,
                function: row.function,
                default: row.is_default == 1,
                description: row.description,
            });
    }
    Ok(())
}

fn query_operator_families(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in OperatorFamilyQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .operator_families
            .push(OperatorFamilyDef {
                schema: row.schema,
                name: row.name,
                access_method: row.access_method,
                description: row.description,
            });
    }
    Ok(())
}

fn query_operator_classes(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in OperatorClassQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .operator_classes
            .push(OperatorClassDef {
                schema: row.schema,
                name: row.name,
                default: row.is_default == 1,
                data_type: row.data_type,
                access_method: row.access_method,
                family: row.family,
                storage_type: row.storage_type,
                description: row.description,
            });
    }
    Ok(())
}

fn query_text_search(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in TextSearchParserQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .text_search_parsers
            .push(TextSearchParserDef {
                schema: row.schema,
                name: row.name,
                start: row.start,
                gettoken: row.gettoken,
                end: row.end,
                lextypes: row.lextypes,
                headline: row.headline,
                description: row.description,
            });
    }
    for row in TextSearchTemplateQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .text_search_templates
            .push(TextSearchTemplateDef {
                schema: row.schema,
                name: row.name,
                init: row.init,
                lexize: row.lexize,
                description: row.description,
            });
    }
    for row in TextSearchDictionaryQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .text_search_dictionaries
            .push(TextSearchDictionaryDef {
                schema: row.schema,
                name: row.name,
                template: row.template,
                options: row.options,
                description: row.description,
            });
    }
    for row in TextSearchConfigurationQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .text_search_configurations
            .push(TextSearchConfigurationDef {
                schema: row.schema,
                name: row.name,
                parser: row.parser,
                description: row.description,
            });
    }
    Ok(())
}

fn module_name(extension_name: String) -> String {
    if extension_name == "plpgsql" {
        "builtins".to_string()
    } else {
        extension_name
    }
}

fn query_access_methods(modules: &mut BTreeMap<String, Module>) -> Result<()> {
    for row in AccessMethodQuery::run()? {
        modules
            .entry(module_name(row.extension_name))
            .or_default()
            .access_methods
            .push(AccessMethodDef {
                name: row.name,
                kind: row.kind,
                handler: row.handler,
                description: row.description,
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

    query_roles(&mut modules)?;
    query_tablespaces(&mut modules)?;
    query_databases(&mut modules)?;
    query_schemas(&mut modules)?;
    query_types(&mut modules)?;
    query_composite_types(&mut modules)?;
    query_relations(&mut modules)?;
    query_functions(&mut modules)?;
    query_operators(&mut modules)?;
    query_casts(&mut modules)?;
    query_collations(&mut modules)?;
    query_access_methods(&mut modules)?;
    query_languages(&mut modules)?;
    query_foreign_data_wrappers(&mut modules)?;
    query_conversions(&mut modules)?;
    query_operator_families(&mut modules)?;
    query_operator_classes(&mut modules)?;
    query_text_search(&mut modules)?;

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
