use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use actix_web::{web::Bytes, HttpRequest, HttpResponse};
use log::warn;
use regex::Regex;

const HANDLER_NAME: &str = "cgi-bin";

pub fn handler(bytes: Bytes, req: &HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::Ok()
            .content_type("text/plain;charset=UTF-8")
            .body(""),
        handler_event: Some(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(req, "Host"))
                .set_uri(req.uri().to_string())
                .set_x_forwarded_for(get_header_value(req, "X-Forwarded-For"))
                .set_src_ip(get_ip_address(req))
                .set_user_agent(get_header_value(req, "User-Agent"))
                .set_payload(
                    match (req.method().as_str(), String::from_utf8(bytes.to_vec())) {
                        ("POST" | "PUT", Ok(text)) => Some(text),
                        (_, Err(e)) => {
                            warn!("Failed to decode POST payload: {}", e);
                            None
                        }
                        _ => None,
                    },
                ),
        ),
        report: get_ip_address(req).map(|ip| {
            Report::new(ip)
                .add_categories(vec![
                    Category::Hacking,
                    Category::WebAppAttack,
                    Category::BadWebBot,
                ])
                .set_comment_text(format!("{} {}", req.method().as_str(), req.uri()))
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
