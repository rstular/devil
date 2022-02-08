-- Your SQL goes here
CREATE TABLE IF NOT EXISTS handler_events (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    handler VARCHAR(255) NOT NULL,
    host VARCHAR,
    uri VARCHAR,
    src_ip INET,
    payload VARCHAR,
    user_agent VARCHAR,
    details VARCHAR,
    x_forwarded_for VARCHAR
);