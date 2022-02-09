-- Your SQL goes here
ALTER TABLE handler_events ADD COLUMN subhandler VARCHAR;
ALTER TABLE handler_events RENAME COLUMN details TO handler_data;
CREATE INDEX CONCURRENTLY idx_subhandler ON handler_events(subhandler);
