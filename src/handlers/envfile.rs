use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use actix_web::{web::Bytes, HttpRequest};
use regex::Regex;

pub const HANDLER_NAME: &str = "envfile";

pub const RESP_CONTENT: &str = "HTTP_ADMINISTRATION_ENDPOINT = /data/xmlrpc.php
HTTP_ADMINISTRATION_ENDPOINT_SSL = /data/xmlrpc.php
HTTP_ADMINISTRATION_ENDPOINT_SSL_PORT = 443
HTTP_ADMINISTRATION_ENDPOINT_PORT = 80
HTTP_ADMINISTRATION_TOKEN = admin";

pub fn handler(_bytes: Bytes, req: HttpRequest) -> HandlerResponse {
    HandlerResponse::new(RESP_CONTENT)
        .set_event(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(&req, "Host"))
                .set_src_ip(get_header_value(&req, "X-Forwarded-For"))
                .set_uri(req.uri().to_string()),
        )
        .set_report(get_ip_address(&req).map(|ip| {
            Report::new(ip).add_categories(vec![
                Category::Hacking,
                Category::WebAppAttack,
                Category::BadWebBot,
            ])
        }))
}

pub fn register() -> RequestHandler {
    RequestHandler {
        name: HANDLER_NAME,
        pattern: Regex::new("\\.env").expect("Failed to compile regex"),
        handler,
    }
}
