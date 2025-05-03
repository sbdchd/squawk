-- simple
create database d;

-- full
create database d
  with owner 'foo'
  template = 'foo'
  encoding = 'utf-8'
  strategy = 10
  locale = 'fr_FR'
  lc_collate = 'fr_FR'
  lc_ctyep = 10
  builtin_locale = 'en'
  icu_locale = 'en'
  icu_rules = ''
  locale_provider = './foo/bar'
  collation_version = 10
  tablespace = full
  allow_connections = false
  connection limit = 1000
  is_template = false
  oid 1010;

