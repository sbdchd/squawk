INSERT INTO foo (id, name) VALUES (1, 'one') RETURNING id;

WITH inserted AS (INSERT INTO foo DEFAULT VALUES RETURNING id) SELECT * FROM inserted;

WITH inserted AS (INSERT INTO foo AS f (id, name) OVERRIDING SYSTEM VALUE SELECT id, name FROM incoming ON CONFLICT ON CONSTRAINT foo_pkey DO NOTHING RETURNING id) SELECT * FROM inserted;

WITH inserted AS (INSERT INTO a_very_long_schema_name.a_very_long_table_name (organization_identifier, extremely_long_descriptive_column_name) VALUES (123456789, 'an extremely long value that forces the insert statement to wrap across lines') ON CONFLICT (organization_identifier) DO UPDATE SET extremely_long_descriptive_column_name = excluded.extremely_long_descriptive_column_name WHERE a_very_long_table_name.organization_identifier = excluded.organization_identifier RETURNING organization_identifier, extremely_long_descriptive_column_name) SELECT organization_identifier, extremely_long_descriptive_column_name FROM inserted;

/*before*/ WITH /*a*/ inserted /*b*/ (/*c*/ result_id /*d*/) /*e*/ AS /*f*/ NOT /*g*/ MATERIALIZED /*h*/ (/*i*/ INSERT /*j*/ INTO /*k*/ public /*l*/ . /*m*/ foo /*n*/ AS /*o*/ f /*p*/ (/*q*/ id /*r*/, /*s*/ payload /*t*/) /*u*/ OVERRIDING /*v*/ USER /*w*/ VALUE /*x*/ VALUES /*y*/ (/*z*/ 1 /*aa*/, /*ab*/ 'new' /*ac*/) /*ad*/ ON /*ae*/ CONFLICT /*af*/ (/*ag*/ id /*ah*/ COLLATE /*ai*/ "C" /*aj*/ text_ops /*ak*/) /*al*/ WHERE /*am*/ id > 0 /*an*/ DO /*ao*/ UPDATE /*ap*/ SET /*aq*/ payload /*ar*/ = /*as*/ excluded.payload /*at*/ WHERE /*au*/ foo.id = excluded.id /*av*/ RETURNING /*aw*/ WITH /*ax*/ (/*ay*/ OLD /*az*/ AS /*ba*/ old_row /*bb*/, /*bc*/ NEW /*bd*/ AS /*be*/ new_row /*bf*/) /*bg*/ new_row.id /*bh*/) /*bi*/ SELECT /*bj*/ result_id /*bk*/ FROM /*bl*/ inserted /*bm*/;
