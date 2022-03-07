use crate::db::models::HandlerEvent;
use crate::handler::HandlerResponse;
use crate::handler::{get_header_value, get_ip_address};
use actix_web::{web::Bytes, HttpRequest, HttpResponse};
use log::warn;

pub const HANDLER_NAME: &str = "default";

pub fn handler(bytes: Bytes, req: &HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::NotFound().body("404 - Not Found"),
        handler_event: Some(
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
        ),
        report: None,
    }
}
