-- simple
create event trigger t on e
  execute function f();

-- full
create event trigger t on e
  when x in ('foo', 'bar', 'buzz')
    and real in ('a')
  execute function foo.f();

-- doc_example_1
CREATE EVENT TRIGGER abort_ddl ON ddl_command_start
   EXECUTE FUNCTION abort_any_command();

