-- Your SQL goes here
CREATE TABLE main_table (id SERIAL PRIMARY KEY, 
    reference_time TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
    insert_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data JSONB NOT NULL,
    tags JSONB not NULL,
    bucket_name TEXT NOT NULL
);
SELECT diesel_manage_updated_at('main_table');