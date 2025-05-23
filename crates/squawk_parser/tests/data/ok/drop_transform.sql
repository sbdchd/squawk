-- simple
drop transform for text language foo;

-- full
drop transform if exists for t language l cascade;

-- type_args
drop transform for varchar(100) language l restrict;

