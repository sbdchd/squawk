alter /* procedure */ procedure /* signature */ public.recalculate_account_totals(/* arg */ integer, text) /* action */ rename /* to */ to /* target */ recalculate_totals /* end */;

alter procedure process_customer_transactions_with_an_exceptionally_long_name(integer, text) security definer set work_mem = '256MB' reset all restrict;

alter procedure p no depends on extension /* extension */ auditing_tools;
