alter view public.orders alter column status set default 'pending';

alter view if exists public.orders alter column status drop default;

alter view public.orders rename column customer_identifier to account_identifier;

alter view public.orders owner to reporting_role;

alter view public.orders set schema reporting;

alter view public.orders rename to archived_orders;

alter view public.orders set (security_barrier = true, security_invoker = false);

alter view public.orders reset (security_barrier, security_invoker);

alter /* view keyword */ view /* if exists */ if /* exists keyword */ exists /* view name */ extraordinarily_long_schema_name.extraordinarily_long_view_name /* action */ alter /* column keyword */ column /* column name */ extraordinarily_long_column_name /* set keyword */ set /* default keyword */ default /* expression */ coalesce(current_setting('app.default_status'), 'pending') /* semicolon */;
