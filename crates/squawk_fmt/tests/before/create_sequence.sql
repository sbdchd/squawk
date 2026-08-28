create sequence invoice_number_seq;

create temporary sequence if not exists extraordinarily_long_schema_name.extraordinarily_long_invoice_number_sequence as bigint increment by 5 minvalue 100 maxvalue 999999999 start with 100 cache 50 cycle owned by extraordinarily_long_schema_name.extraordinarily_long_table_name.extraordinarily_long_column_name;

-- comments in every position
create /* temporary */ temporary /* sequence */ sequence /* if */ if /* not */ not /* exists */ exists /* name */ app.order_seq /* as */ as /* type */ bigint /* increment */ increment /* by */ by /* increment value */ 2 /* no */ no /* minvalue */ minvalue /* maxvalue */ maxvalue /* max */ 1000 /* start */ start /* with */ with /* start value */ 10 /* cache */ cache /* cache value */ 20 /* no cycle */ no /* cycle */ cycle /* owned */ owned /* by */ by /* owner */ app.orders.id /* end */;
