-- Your SQL goes here
CREATE TABLE main_table (
    -- id             SERIAL      PRIMARY KEY, 
    -- time           TIMESTAMPTZ NOT NULL,
    reference_time TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
    insert_time    TIMESTAMPTZ PRIMARY KEY NOT NULL DEFAULT NOW(),
    data           JSONB       NOT NULL,
    tags           JSONB       NOT NULL,
    bucket_name    TEXT        NOT NULL
);
SELECT diesel_manage_updated_at('main_table');