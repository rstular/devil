use super::schema::handler_events;
use super::schema::handler_events::dsl::handler_events as handler_events_dsl;
use diesel::prelude::*;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "handler_events"]
pub struct HandlerEvent {
    pub handler: String,
    pub host: Option<String>,
    pub uri: Option<String>,
    pub src_ip: Option<String>,
    pub payload: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<String>,
    pub x_forwarded_for: Option<String>,
}

impl HandlerEvent {
    pub fn new(handler: &str) -> Self {
        HandlerEvent {
            handler: handler.to_string(),
            host: None,
            uri: None,
            src_ip: None,
            payload: None,
            user_agent: None,
            details: None,
            x_forwarded_for: None,
        }
    }

    pub fn set_host(mut self, host: Option<String>) -> Self {
        self.host = host;
        self
    }

    pub fn set_uri(mut self, uri: String) -> Self {
        self.uri = Some(uri);
        self
    }

    pub fn set_src_ip(mut self, src_ip: Option<String>) -> Self {
        self.src_ip = src_ip;
        self
    }

    pub fn set_payload(mut self, payload: Option<String>) -> Self {
        self.payload = payload;
        self
    }

    pub fn set_user_agent(mut self, user_agent: Option<String>) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn set_details(mut self, details: Option<String>) -> Self {
        self.details = details;
        self
    }
    pub fn set_x_forwarded_for(mut self, x_forwarded_for: Option<String>) -> Self {
        self.x_forwarded_for = x_forwarded_for;
        self
    }

    pub fn insert(handler_event: Self, conn: &SqliteConnection) {
        if let Err(e) = diesel::insert_into(handler_events_dsl)
            .values(handler_event)
            .execute(conn)
        {
            error!("Error inserting new event: {}", e);
        }
    }
}
