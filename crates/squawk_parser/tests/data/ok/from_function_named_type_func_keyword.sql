select * from left('abc', 2);
select * from t natural join full();
merge into t using is() on true when matched then delete;
