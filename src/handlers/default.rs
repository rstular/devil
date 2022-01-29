use crate::handler::HandlerResponse;
use actix_web::{web::Bytes, HttpRequest, HttpResponse};

pub fn handler(_bytes: Bytes, _req: &HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::NotFound().body("404 - Not Found"),
        handler_event: None,
        report: None,
    }
}
