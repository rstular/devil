-- This file should undo anything in `up.sql`
ALTER TABLE handler_events DROP COLUMN subhandler;
ALTER TABLE handler_events RENAME COLUMN handler_data TO details;
DROP INDEX CONCURRENTLY idx_subhandler;
