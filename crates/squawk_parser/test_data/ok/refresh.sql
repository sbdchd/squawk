-- simple
REFRESH MATERIALIZED VIEW order_summary;

REFRESH MATERIALIZED VIEW annual_statistics_basis WITH NO DATA;

refresh materialized view concurrently v with data;

