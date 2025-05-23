-- simple
create access method m
  type table
  handler f;

-- full
create access method m
  type index
  handler bar.f;

