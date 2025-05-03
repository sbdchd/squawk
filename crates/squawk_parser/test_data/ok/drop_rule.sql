-- simple
drop rule s on t;;

-- full
drop rule if exists r on foo.t_name cascade;
drop rule if exists r on a restrict;

