-- Your SQL goes here
-- CREATE INDEX ON main_table (data,tags, insert_time DESC);
CREATE INDEX idxgindata ON main_table USING GIN (data);
CREATE INDEX idxgintags ON main_table USING GIN (tags);