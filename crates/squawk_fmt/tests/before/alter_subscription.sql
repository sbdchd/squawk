alter subscription local_sub connection 'host=otherhost dbname=publisher';

alter subscription local_sub server publisher_server;

alter subscription local_sub set (slot_name = new_slot, synchronous_commit = local);

alter subscription regress_testsub4 set (origin = any, max_retention_duration = -1);

alter subscription regress_testsub4 set (/* before origin */ origin /* before equals */ = /* before any */ any /* after any */, /* before retention */ max_retention_duration = /* before minus */ - /* before one */ 1 /* after one */);

alter subscription local_sub add publication another_publication, third_publication with (copy_data = false);

alter subscription local_sub set publication all_changes with (refresh = true);

alter subscription local_sub drop publication selected_tables with (refresh = false);

alter subscription local_sub refresh publication with (copy_data = true);

alter subscription local_sub enable;

alter subscription local_sub disable;

alter subscription local_sub skip (lsn = '0/16B6C50');

alter subscription local_sub owner to replication_admin;

alter subscription local_sub rename to renamed_sub;

alter /* after alter */ subscription /* after subscription */ renamed_sub add /* after add */ publication /* before publication name */ commented_pub, /* after publication comma */ selected_tables with /* before params */ (copy_data = true) /* before semicolon */;
