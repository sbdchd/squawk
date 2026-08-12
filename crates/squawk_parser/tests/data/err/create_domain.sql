-- missing trailing inherit
create domain d as int check (v > 0) no;

-- constraint options after a missing constraint body
alter domain d add constraint c deferrable initially deferred;
