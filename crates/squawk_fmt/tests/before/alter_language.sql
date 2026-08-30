ALTER /* procedural */ PROCEDURAL /* language */ LANGUAGE /* name */ plpgsql /* rename */ RENAME /* to */ TO /* new name */ plpgsql_new_name_with_a_very_long_descriptive_suffix /* end */;

ALTER /* language */ LANGUAGE /* name */ plpgsql /* owner */ OWNER /* to */ TO /* role */ language_owner_with_a_very_long_descriptive_name;

ALTER LANGUAGE old_language RENAME TO new_language;

ALTER LANGUAGE plpgsql OWNER TO CURRENT_USER;
