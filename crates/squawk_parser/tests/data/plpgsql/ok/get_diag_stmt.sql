do $$
declare
  get int;
  x int;
  y text;
  "table" int;
  r record;
begin
  get diagnostics x = row_count;
  get current diagnostics x = row_count;
  get diagnostics x := row_count;
  get diagnostics X = ROW_COUNT;
  get diagnostics get = row_count, x = pg_routine_oid;
  get diagnostics "table" = row_count;
  get diagnostics y = pg_context;
  r := row(1);
  get diagnostics r.f1 = row_count;
  get := 1;
exception when others then
  get stacked diagnostics
    y = returned_sqlstate,
    y = message_text,
    y = pg_exception_detail,
    y = pg_exception_hint,
    y = pg_exception_context,
    y = column_name,
    y = constraint_name,
    y = pg_datatype_name,
    y = table_name,
    y = schema_name;
end
$$;

-- `current` and `stacked` name variables here, so they can't also head the area
do $$
declare
  current int;
  stacked int;
begin
  get diagnostics current = row_count;
  get diagnostics stacked = row_count;
end
$$;

create function get_diag(a int) returns void as $$
begin
  get diagnostics $1 = row_count;
end
$$ language plpgsql;
