-- create_index
-- simple
create index on t (c);

-- more options
create unique index concurrently 
  if not exists i 
  on only t 
  using bar (c);

-- column options
create index on t (c collate "fr_FR" asc nulls last);
create index on t (c desc nulls first);

-- opclass
create index on t (
  a text_pattern_ops (strength = primary, variant = default),
  b text_pattern_ops (strength = primary)
);

-- column expr
create index on t ((f(c) + 2), b);

-- trailing options
create index on t (c)
  include (a, b)
  nulls not distinct
  with (foo = bar, buzz)
  tablespace space
  where x > 10;

create index on t (c)
  nulls distinct
  where x is not null;

-- schema
create index on public.t (c);

-- doc_examples
CREATE UNIQUE INDEX title_idx ON films (title);

CREATE UNIQUE INDEX title_idx ON films (title) INCLUDE (director, rating);

CREATE INDEX title_idx ON films (title) WITH (deduplicate_items = off);

CREATE INDEX ON films ((lower(title)));

CREATE INDEX title_idx_german ON films (title COLLATE "de_DE");

CREATE INDEX title_idx_nulls_low ON films (title NULLS FIRST);

CREATE UNIQUE INDEX title_idx ON films (title) WITH (fillfactor = 70);

CREATE INDEX gin_idx ON documents_table USING GIN (locations) WITH (fastupdate = off);

CREATE INDEX code_idx ON films (code) TABLESPACE indexspace;

CREATE INDEX pointloc
    ON points USING gist (box(location,location));

CREATE INDEX CONCURRENTLY sales_quantity_index ON sales_table (quantity);

