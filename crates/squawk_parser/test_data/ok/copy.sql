-- simple_copy_from
copy copytest from '/tmp/copy.data';

-- copy_to
COPY country TO STDOUT (DELIMITER '|');

-- copy_from
COPY country FROM '/usr1/proj/bray/sql/country_data';

-- copy_to_file
COPY (SELECT * FROM country WHERE country_name LIKE 'A%') TO '/usr1/proj/bray/sql/a_list_countries.copy';

-- copy_to_compress_filed
COPY country TO PROGRAM 'gzip > /usr1/proj/bray/sql/country_data.gz';

-- log_setting
copy x (i, y) from '/tmp/input.file' ( on_error ignore, log_verbosity verbose );

-- on_error
copy copytest from '/tmp/copy.data' ( on_error ignore );

-- all_the_options
copy t from 'foo' (
  format csv,
  freeze,
  freeze true,
  freeze false,
  delimiter ',',
  null '\n',
  default 'foo',
  header,
  header true,
  header false,
  header match,
  quote 'foo',
  escape 'bar',
  force_quote *,
  force_quote (a, b, c, d),
  force_not_null *,
  force_not_null (a),
  force_null *,
  force_null (a, b),
  on_error stop,
  on_error ignore,
  encoding 'utf8',
  log_verbosity verbose
);

