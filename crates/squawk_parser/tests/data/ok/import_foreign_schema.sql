-- simple
import foreign schema s
  from server a
  into b;

-- except
import foreign schema some_schema_name
    EXCEPT (t1, t2)
    FROM SERVER server_name
    INTO local_schema;

-- options
import foreign schema a
  from server b
  into c
  options (foo 'bar');

-- options_multi
import foreign schema a
  from server b
  into c
  options (foo 'bar', a 'b');

-- doc_example_1
IMPORT FOREIGN SCHEMA foreign_films
    FROM SERVER film_server INTO films;

-- doc_example_2
IMPORT FOREIGN SCHEMA foreign_films LIMIT TO (actors, directors)
    FROM SERVER film_server INTO films;

