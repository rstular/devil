-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "handler_events" (
    "id" INTEGER UNIQUE,
    "timestamp" DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "handler" TEXT NOT NULL,
    "host" TEXT,
    "uri" TEXT,
    "src_ip" TEXT,
    "info" TEXT,
    PRIMARY KEY("id" AUTOINCREMENT)
);