-- simple
create user mapping for u server s;

-- full
create user mapping if not exists for current_user server some_server_name options (user 'bob', password 'secret');
