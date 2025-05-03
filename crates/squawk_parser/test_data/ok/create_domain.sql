-- simple
create domain d int;

-- full
create domain d as varchar(100)
  collate "fr_FR"
  default 'fooooo'
  constraint c check (a > b)
  not null
  null;

