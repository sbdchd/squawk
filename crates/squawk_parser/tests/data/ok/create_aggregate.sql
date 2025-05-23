-- simple_old_syntax
create aggregate a (
  basetype = t,
  sfunc = f,
  stype = u
);

-- full_old_syntax
create or replace aggregate foo.my_aggregate (
  basetype = foo.input_type,
  sfunc = foo.state_function,
  stype = foo.state_type,
  sspace = 1024,
  finalfunc = foo.final_function,
  finalfunc_extra = true,
  finalfunc_modify = read_only,
  combinefunc = foo.combine_function,
  serialfunc = foo.serial_function,
  deserialfunc = foo.deserial_function,
  initcond = '0',
  msfunc = foo.moving_state_function,
  minvfunc = foo.moving_inverse_function,
  mstype = foo.moving_state_type,
  msspace = 2048,
  mfinalfunc = foo.moving_final_function,
  mfinalfunc_extra = true,
  mfinalfunc_modify = shareable,
  minitcond = '{"initial":"value"}',
  sortop = <
);

-- simple
create aggregate a(t) (
  sfunc = f,
  stype = t
);

-- full
create or replace aggregate a(in a p.bar, text, smallint) (
  sfunc = f,
  stype = t,
  sspace = 1024,
  finalfunc = ff,
  finalfunc_extra,
  finalfunc_modify = read_only,
  combinefunc = cf,
  serialfunc = sf,
  deserialfunc = df,
  initcond = '0',
  msfunc = msf,
  minvfunc = mif,
  mstype = mt,
  msspace = 2048,
  mfinalfunc = mff,
  mfinalfunc_extra,
  mfinalfunc_modify = shareable,
  minitcond = '{"initial":"value"}',
  sortop = <,
  parallel = safe
);

-- ordered_aggregate
create or replace aggregate percentile_disc(
  in p1 float8,
--   in p2 text ORDER BY in value1 anyelement,
  in value2 timestamp ORDER BY a,
  result numeric
) (
  sfunc = percentile_disc_transition,
  stype = internal,
  sspace = 1024,
  finalfunc = percentile_disc_final,
  finalfunc_extra,
  finalfunc_modify = read_only,
  initcond = '0.5',
  parallel = safe,
  hypothetical
);

-- doc_example_1
CREATE AGGREGATE array_accum (anycompatible)
(
    sfunc = array_append,
    stype = anycompatiblearray,
    initcond = '{}'
);

-- doc_example_2
CREATE AGGREGATE array_agg (anynonarray)
(
    sfunc = array_agg_transfn,
    stype = internal,
    finalfunc = array_agg_finalfn,
    finalfunc_extra
);

-- doc_example_3
CREATE AGGREGATE percentile_disc (float8 ORDER BY anyelement)
(
    sfunc = ordered_set_transition,
    stype = internal,
    finalfunc = percentile_disc_final,
    finalfunc_extra
);

-- doc_example_4
CREATE AGGREGATE sum (complex)
(
    sfunc = complex_add,
    stype = complex,
    initcond = '(0,0)',
    msfunc = complex_add,
    minvfunc = complex_sub,
    mstype = complex,
    minitcond = '(0,0)'
);

-- doc_example_5
CREATE AGGREGATE unsafe_sum (float8)
(
    stype = float8,
    sfunc = float8pl,
    mstype = float8,
    msfunc = float8pl,
    minvfunc = float8mi
);

