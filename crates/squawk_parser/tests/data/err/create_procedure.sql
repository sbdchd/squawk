-- transform list cannot end with a comma before a semicolon
create procedure p()
language sql
as ''
transform for type int,;

-- transform list cannot end with a comma at EOF
create procedure p() language sql as '' transform for type int,
