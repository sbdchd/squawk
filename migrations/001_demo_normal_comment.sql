-- Demo Migration 001: Normal GitHub Comment Generation
-- This migration demonstrates typical violations that should generate
-- a normal GitHub comment with full SQL content

BEGIN;

-- Adding NOT NULL field without default (violation)
ALTER TABLE users ADD COLUMN email varchar(255) NOT NULL;

-- Adding foreign key constraint (violation)  
ALTER TABLE posts ADD CONSTRAINT fk_posts_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id);

-- Changing column type (violation)
ALTER TABLE products ALTER COLUMN price TYPE decimal(10,2);

-- Adding field with default value (violation)
ALTER TABLE users ADD COLUMN status integer DEFAULT 1;

COMMIT;
