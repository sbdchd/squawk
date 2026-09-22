do $$ begin null; end $$;
do language plpgsql $$ begin null; end $$;
do $$ anything $$ language custom_language;
do language "SQL" $$ anything $$;
do language 'SQL' $$ anything $$;
