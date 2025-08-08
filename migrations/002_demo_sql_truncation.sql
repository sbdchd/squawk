-- Demo Migration 002: SQL Truncation Test
-- This migration has 60+ lines to test the SQL truncation feature
-- Expected: SQL content truncated at 50 lines with truncation notice

BEGIN;

-- Adding NOT NULL field (violation at the beginning)
ALTER TABLE users ADD COLUMN phone varchar(20) NOT NULL;

-- Generate enough content to trigger truncation
CREATE TABLE demo_table_1 (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE TABLE demo_table_2 (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE TABLE demo_table_3 (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE TABLE demo_table_4 (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE TABLE demo_table_5 (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE TABLE demo_table_6 (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

-- Line 51: This should be truncated
-- Line 52: This should be truncated
-- Line 53: This should be truncated
-- Line 54: This should be truncated
-- Line 55: This should be truncated
-- Line 56: This should be truncated
-- Line 57: This should be truncated
-- Line 58: This should be truncated
-- Line 59: This should be truncated
-- Line 60: This should be truncated

-- Adding another violation at the end (should still be detected)
ALTER TABLE products ALTER COLUMN description TYPE text;

COMMIT;
