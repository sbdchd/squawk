-- zone_value is a single value, not a list
set time zone 'a', 'b';

-- zone_value doesn't allow arbitrary expressions
set time zone now();

-- zone_value doesn't accept NULL
set time zone null;

-- configuration value lists cannot have a trailing comma
set s='',F,'''',;

set c='',;

set m=2,
