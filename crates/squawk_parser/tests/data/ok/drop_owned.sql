-- simple
drop owned by s;

-- full
drop owned by a, current_role, c cascade;
drop owned by session_user, current_user restrict;

