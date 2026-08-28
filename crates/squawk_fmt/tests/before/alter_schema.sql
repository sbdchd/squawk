alter schema accounting rename to finance;

alter schema reporting owner to reporting_admin;

alter schema schema_with_a_very_long_descriptive_name rename to schema_with_an_even_longer_descriptive_replacement_name;

alter /* before schema */ schema /* before name */ commented_schema /* before rename */ rename /* before to */ to /* before new name */ renamed_commented_schema /* before semicolon */;

alter schema another_commented_schema /* before owner */ owner /* before to */ to /* before role */ current_user;
