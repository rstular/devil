use crate::db::models;
use crate::db::DbPool;
use crate::handlers::*;
use actix_web::{web, web::Bytes, HttpRequest, HttpResponse, Responder};
use lazy_static::lazy_static;
use log::{debug, trace};
use regex::Regex;

lazy_static! {
    static ref REGISTERED_HANDLERS: Vec<RequestHandler> = {
        vec![
            etc_passwd::register(),
            eval_stdin::register(),
            cgi_bin::register(),
            wordpress_login::register(),
            wordpress_xmlrpc::register(),
            envfile::register(),
        ]
    };
    static ref DEFAULT_HANDLER: RequestHandler =
        RequestHandler::new("default", Regex::new("").unwrap(), default_handler);
}

pub struct HandlerResponse {
    pub http_response: HttpResponse,
    pub handler_event: Option<models::HandlerEvent>,
}

impl HandlerResponse {
    pub fn new(response_content: &'static str) -> Self {
        HandlerResponse {
            http_response: HttpResponse::Ok().body(response_content),
            handler_event: None,
        }
    }

    pub fn set_event(mut self, event: models::HandlerEvent) -> Self {
        self.handler_event = Some(event);
        self
    }
}

type RequestHandlerFunction = fn(Bytes, HttpRequest) -> HandlerResponse;

pub struct RequestHandler {
    pub name: &'static str,
    pub pattern: Regex,
    pub handler: RequestHandlerFunction,
}

impl RequestHandler {
    pub fn new(
        name: &'static str,
        pattern: Regex,
        handler: RequestHandlerFunction,
    ) -> RequestHandler {
        RequestHandler {
            name,
            pattern,
            handler,
        }
    }
}

pub fn default_handler(_bytes: Bytes, _req: HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::NotFound().body("404 - Not Found"),
        handler_event: None,
    }
}

pub fn get_header_value(req: &HttpRequest, header: &str) -> Option<String> {
    req.headers().get(header).map(|val| {
        val.to_str()
            .unwrap_or_else(|e| {
                trace!("Failed to decode header: {}", e);
                ""
            })
            .to_string()
    })
}

pub async fn request_dispatcher(
    bytes: Bytes,
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let handler: &RequestHandler = REGISTERED_HANDLERS
        .iter()
        .find(|handler| handler.pattern.is_match(&req.uri().to_string()))
        .unwrap_or(&DEFAULT_HANDLER);
    let handler_func: RequestHandlerFunction = handler.handler;

    debug!("Running handler: {}", handler.name);
    let resp = handler_func(bytes, req);

    if let Some(event) = resp.handler_event {
        models::HandlerEvent::insert(event, &pool.get().unwrap());
    }

    resp.http_response
}
