select * from foo;
select * from public.foo as f, bar b;
select * from foo as f (id, display_name);
select * from foo f (id int, display_name text collate "C");
select * from foo
  /* before alias */ as /* before alias name */ f /* before open paren */ (
    /* after open paren */ id /* before comma */,
    /* after comma */ display_name /* before close paren */
  );
select * from users tablesample bernoulli(10) repeatable (42);
select *
/* before from */ from
  /* before item */ only /* before relation */ public /* before dot */ . /* before table */ foo
  /* before star */ *
  /* before alias */ as /* before alias name */ f /* before item comma */,
  /* before second item */ other /* before second alias */ o;
