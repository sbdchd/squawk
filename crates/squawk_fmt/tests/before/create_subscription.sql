create subscription local_sub connection 'host=localhost port=5432 dbname=publisher user=replicator password=very_long_password' publication all_changes, selected_tables with (copy_data = true, enabled = false, streaming = parallel);

create subscription server_sub server publisher_server publication all_changes;

create /* after create */ subscription /* before name */ commented_sub connection /* before connection */ 'host=localhost' publication /* before publication */ all_changes, /* after comma */ selected_tables with /* before params */ (enabled = true) /* before semicolon */;
