use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use actix_web::{web::Bytes, HttpRequest};
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

// Return a fake robots.txt, which blacklists a specific endpoint
// Catch everyone trying to access the endpoint

pub const HANDLER_NAME: &str = "robots-bait";

pub const ROBOTS_CONTENT: &str = "User-Agent: *
Disallow: /bb.php";
pub const ENDPOINT_CONTENT: &str = "400: Bad request";

lazy_static! {
    static ref ROBOTS_PATTERN: Regex =
        Regex::new("^/robots\\.txt").expect("Failed to compile robots.txt pattern regex");
}

pub fn handler(bytes: Bytes, req: &HttpRequest) -> HandlerResponse {
    if ROBOTS_PATTERN.is_match(&req.uri().to_string()) {
        return HandlerResponse::new(ROBOTS_CONTENT);
    }

    HandlerResponse::new(ENDPOINT_CONTENT)
        .set_event(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(req, "Host"))
                .set_x_forwarded_for(get_header_value(req, "X-Forwarded-For"))
                .set_src_ip(get_ip_address(req))
                .set_user_agent(get_header_value(req, "User-Agent"))
                .set_uri(req.uri().to_string())
                .set_payload(
                    match (req.method().as_str(), String::from_utf8(bytes.to_vec())) {
                        ("POST", Ok(text)) => Some(text),
                        ("PUT", Ok(text)) => Some(text),
                        (_, Err(e)) => {
                            warn!("Failed to decode POST payload: {}", e);
                            None
                        }
                        _ => None,
                    },
                ),
        )
        .set_report(get_ip_address(req).map(|ip| {
            Report::new(ip)
                .add_categories(vec![Category::Hacking, Category::BadWebBot])
                .set_comment_text(format!("{} {}", req.method().as_str(), req.uri()))
        }))
}

pub fn register() -> RequestHandler {
    RequestHandler {
        name: HANDLER_NAME,
        pattern: Regex::new("^/robots\\.txt|bb\\.php").expect("Failed to compile regex"),
        handler,
    }
}
