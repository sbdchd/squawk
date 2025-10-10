-- missing commas
PREPARE fooplan (int  text  bool, numeric) AS
    INSERT INTO foo VALUES($1, $2, $3, $4);
