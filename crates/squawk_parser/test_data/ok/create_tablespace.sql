-- simple
create tablespace t location '';

-- full
create tablespace t 
  owner current_role
  location ''
  with (
    seq_page_cost = 10,
    random_page_cost = 1,
    effective_io_concurrency = 0,
    maintenance_io_concurrency = 4
  );

