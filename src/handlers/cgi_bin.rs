use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use actix_web::{web::Bytes, HttpRequest, HttpResponse};
use log::warn;
use regex::Regex;

const HANDLER_NAME: &str = "cgi-bin";

pub fn handler(bytes: Bytes, req: HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::Ok()
            .content_type("text/plain;charset=UTF-8")
            .body(""),
        handler_event: Some(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(&req, "Host"))
                .set_uri(req.uri().to_string())
                .set_src_ip(get_header_value(&req, "X-Forwarded-For"))
                .set_info(
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
        ),
        report: get_ip_address(&req).map(|ip| {
            Report::new(ip).add_categories(vec![
                Category::Hacking,
                Category::WebAppAttack,
                Category::BadWebBot,
            ])
        }),
    }
}

pub fn register() -> RequestHandler {
    RequestHandler {
        name: HANDLER_NAME,
        pattern: Regex::new("cgi-bin").expect("Failed to compile regex"),
        handler,
    }
}
