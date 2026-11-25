alter publication p set all tables except (table t), all sequences;
alter subscription s refresh sequences;
