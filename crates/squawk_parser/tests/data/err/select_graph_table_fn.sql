select * from graph_table(
  foo match - columns (a b)
--                      ^ comma missing
);

-- graph pattern qualifiers require unsigned integer bounds
select * from graph_table(
  g match - {1.5} columns (a)
);

select * from graph_table(
  g match - {n} columns (a)
);

select * from graph_table(
  g match - {1,n} columns (a)
);

select * from graph_table(
  g match - {,n} columns (a)
);

select * from graph_table(
  g match - {} columns (a)
);

select * from graph_table(
  g match - {,} columns (a)
);

-- path pattern lists cannot end with a comma
select * from graph_table (g match (a), columns (a));

select * from graph_table (g match (a), where true columns (a));
