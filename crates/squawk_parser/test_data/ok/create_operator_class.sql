-- simple
create operator class c for type t
  using i as storage s;

-- full
create operator class foo.c for type varchar(100)
  using i family foo.f as 
    storage s,
    operator 1 &&,
    operator 100 <<<< (text, foo.bigint) for search,
    operator 100 # (varchar(100), varchar(10)) for order by foo.bar,
    function 1010 foo,
    function 1010 (bigint) foo.bar(in b text, out a text, text),
    function 1010 (bigint, smallint) foo.bar(text, text);

