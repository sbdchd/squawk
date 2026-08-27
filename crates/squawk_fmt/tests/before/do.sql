DO 'BEGIN NULL; END';

DO $$BEGIN RAISE NOTICE 'hello'; END$$;

do language plpgsql $$begin perform refresh_materialized_view_with_an_intentionally_long_name(); end$$;

do $$begin null; end$$ language 'plpgsql';

do $body$
begin
  raise notice 'hello';
end
$body$;

/* before */ DO /* after do */ LANGUAGE /* after language */ plpgsql /* before body */ $body$BEGIN NULL; END$body$ /* before semicolon */;

DO /* before trailing body */ $body$BEGIN NULL; END$body$ /* before trailing language */ LANGUAGE /* before language literal */ 'plpgsql' /* before trailing semicolon */;
