create table simple_constraints (
  id bigint,
  parent_id bigint,
  name text,
  PRIMARY KEY (id),
  UNIQUE NULLS NOT DISTINCT (name),
  CHECK (id > 0),
  FOREIGN KEY (parent_id) REFERENCES parents(id)
);

create table named_constraints (
  id bigint,
  valid_at tstzrange,
  CONSTRAINT pk PRIMARY KEY (id) DEFERRABLE INITIALLY DEFERRED,
  CONSTRAINT name_unique UNIQUE (id) INCLUDE (valid_at) WITH (fillfactor = 70) USING INDEX TABLESPACE fast,
  CONSTRAINT id_check CHECK (id > 0) NOT VALID NO INHERIT,
  CONSTRAINT parent_fk FOREIGN KEY (id) REFERENCES public.parents(id) MATCH FULL ON DELETE SET NULL (id) ON UPDATE NO ACTION NOT DEFERRABLE,
  CONSTRAINT no_overlap EXCLUDE USING gist (id WITH =, valid_at WITH &&) INCLUDE (id) WITH (fillfactor = 80) USING INDEX TABLESPACE fast WHERE (id > 0) DEFERRABLE
);

create table using_indexes (
  id bigint,
  UNIQUE USING INDEX existing_unique,
  PRIMARY KEY USING INDEX existing_primary
);

create table commented_constraints (
  id bigint,
  parent_id bigint,
  valid_at tstzrange,
  /* before constraint */ CONSTRAINT /* before constraint name */ "named_pk" /* before primary */ PRIMARY /* before key */ KEY /* before column opening paren */ ( /* before column */ id /* before column closing paren */ ) /* before deferrable */ DEFERRABLE,
  CONSTRAINT named_check /* before check */ CHECK /* before check opening paren */ ( /* before check expression */ id > 0 /* before check closing paren */ ) /* before not */ NOT /* before valid */ VALID,
  CONSTRAINT named_fk /* before foreign */ FOREIGN /* before key */ KEY /* before from opening paren */ ( /* before from column */ parent_id /* before from closing paren */ ) /* before references */ REFERENCES /* before table */ public /* before dot */ . /* before table name */ parents /* before to opening paren */ ( /* before to column */ id /* before to closing paren */ ) /* before match */ MATCH /* before simple */ SIMPLE /* before on delete */ ON /* before delete */ DELETE /* before set */ SET /* before null */ NULL /* before set columns */ (parent_id) /* before on update */ ON /* before update */ UPDATE /* before cascade */ CASCADE /* before enforced */ ENFORCED,
  CONSTRAINT named_exclude /* before exclude */ EXCLUDE /* before using */ USING /* before method */ gist /* before exclusion opening paren */ ( /* before exclusion expression */ id /* before exclusion with */ WITH /* before exclusion op */ = /* before exclusion comma */, /* before second exclusion */ valid_at WITH /* before operator */ OPERATOR /* before operator opening paren */ ( /* before operator name */ public /* before operator dot */ . /* before operator op */ && /* before operator closing paren */ ) /* before exclusion closing paren */ ) /* before include */ INCLUDE /* before include opening paren */ (id /* before include closing paren */ ) /* before with params */ WITH /* before params opening paren */ ( /* before param */ fillfactor /* before equals */ = /* before value */ 80 /* before params closing paren */ ) /* before tablespace using */ USING /* before index */ INDEX /* before tablespace */ TABLESPACE /* before tablespace name */ fast /* before where */ WHERE /* before where opening paren */ ( /* before where expression */ id > 0 /* before where closing paren */ ) /* before initially */ INITIALLY /* before immediate */ IMMEDIATE
);
