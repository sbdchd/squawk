-- simple
drop operator family n using i;

-- full
drop operator family if exists foo.f using i cascade;
drop operator family if exists bar.b using i restrict;

