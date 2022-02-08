use crate::db::models::HandlerEvent;
use crate::handler::HandlerResponse;
use crate::handler::{get_header_value, get_ip_address};
use actix_web::{web::Bytes, HttpRequest, HttpResponse};

pub const HANDLER_NAME: &str = "default";

pub fn handler(_bytes: Bytes, req: &HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::NotFound().body("404 - Not Found"),
        handler_event: Some(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(req, "Host"))
                .set_x_forwarded_for(get_header_value(req, "X-Forwarded-For"))
                .set_src_ip(get_ip_address(req))
                .set_user_agent(get_header_value(req, "User-Agent"))
                .set_uri(req.uri().to_string()),
        ),
        report: None,
    }
}
