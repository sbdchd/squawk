ALTER /* group */ GROUP /* group name */ administrators /* action */ ADD /* user */ USER /* first */ alice, /* second */ bob, /* long */ user_with_an_exceptionally_long_name;

ALTER GROUP administrators DROP /* user */ USER /* first */ alice, /* second */ bob;

ALTER GROUP administrators RENAME /* to */ TO /* name */ application_administrators_with_a_very_long_name;
