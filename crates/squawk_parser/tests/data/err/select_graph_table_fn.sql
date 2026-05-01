select * from graph_table(
  foo match - columns (a b)
--                      ^ comma missing
);
