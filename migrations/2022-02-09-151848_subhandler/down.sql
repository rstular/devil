-- This file should undo anything in `up.sql`
ALTER TABLE devil DROP COLUMN subhandler;
ALTER TABLE devil RENAME COLUMN handler_data TO details;
DROP INDEX CONCURRENTLY idx_subhandler;
