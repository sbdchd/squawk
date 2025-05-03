-- simple
drop operator class n using i;

-- full
drop operator class if exists foo.f using i cascade;
drop operator class if exists bar.b using i restrict;

