BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "alpha" varchar(100) NOT NULL
);

CREATE INDEX "field_name_idx" ON "table_name" ("field_name");

ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);

ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;

COMMIT;
