-- pg_docs
DECLARE cursor_name CURSOR FOR SELECT * FROM t;

-- full
declare c binary insensitive no scroll cursor without hold for select 1;

declare c 
binary 
asensitive 
scroll 
cursor 
    with hold 
    for select 2;

declare c cursor for select 1;

