-- Your SQL goes here
ALTER TABLE devil ADD COLUMN subhandler VARCHAR;
ALTER TABLE devil RENAME COLUMN details TO handler_data;
CREATE INDEX CONCURRENTLY idx_subhandler ON devil(subhandler);
