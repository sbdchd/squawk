-- PL/pgSQL keywords live in their own namespace and stay plain identifiers in
-- SQL. See postgres's pl_reserved_kwlist.h / pl_unreserved_kwlist.h.
create table t (
    alias text,
    assert text,
    constant text,
    datatype text,
    detail text,
    diagnostics text,
    elsif text,
    exception text,
    hint text,
    info text,
    log text,
    loop text,
    message text,
    notice text,
    perform text,
    query text,
    raise text,
    reverse text,
    rowtype text,
    slice text,
    sqlstate text,
    stacked text,
    warning text,
    while text
);

select message, detail, hint from t where query = 'x';

create index on t (slice, reverse);
