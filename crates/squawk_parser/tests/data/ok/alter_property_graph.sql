-- rename to
alter property graph if exists foo.bar
  rename to r;

-- set owner
alter property graph if exists foo.bar
  owner to o;

-- set schema
alter property graph if exists foo.bar
  set schema s;

-- add vertex/edge tables
alter property graph if exists foo.bar
  add vertex tables (
    a key (c, k),
    d key (l, u) 
      no properties
  )
  add edge tables (
    a key (x, y)
      source key (s) references k (id)
      destination key (d) references k (id)
      label q properties (o, f * 10 as p)
      label q properties (i as x)
  );

-- add vertex/edge tables part 2
alter property graph if exists g
  add vertex tables (
    d key (l) 
      properties all columns
  )
  add edge tables (
    a key (x, y)
      source key (s) references k (id)
      destination key (d) references k (id)
  );

-- alter element tables
alter property graph g alter vertex table v add label l;
alter property graph g alter edge table e drop label l;
alter property graph g alter vertex table v alter label l add properties (id);
alter property graph g alter relationship table e alter label l drop properties (id);

-- drop element tables
alter property graph g drop vertex tables (v1, v2);
alter property graph g drop relationship tables (e1, e2);
