-- simple
drop access method s;

-- full
drop access method if exists a cascade;
drop access method if exists a restrict;

drop access method if exists a, b cascade;
