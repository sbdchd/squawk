select * from t for read only;
create rule r as on insert to t do (table x) order by x;
