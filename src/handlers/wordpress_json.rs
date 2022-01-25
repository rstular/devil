use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use actix_web::{web::Bytes, HttpRequest, HttpResponse};
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

const HANDLER_NAME: &str = "wp-json";
const DEFAULT_RESP: &str = "";

struct RESTEndpoint {
    pattern: Regex,
    response: &'static str,
}

lazy_static! {
    static ref ENDPOINT_LIST: Vec<RESTEndpoint> = {
        vec![RESTEndpoint {
            pattern: Regex::new("v2/users/").expect("Failed to compile regex"),
            response: ENUM_USERS_RESP,
        }]
    };
    static ref DEFAULT_ENDPOINT: RESTEndpoint = RESTEndpoint {
        pattern: Regex::new("").expect("Failed to compile regex"),
        response: DEFAULT_RESP,
    };
}

pub fn handler(bytes: Bytes, req: HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::Ok()
            .content_type("text/html;charset=UTF-8")
            .body(
                ENDPOINT_LIST
                    .iter()
                    .find(|endpoint| endpoint.pattern.is_match(req.path()))
                    .unwrap_or(&DEFAULT_ENDPOINT)
                    .response,
            ),
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
            Report::new(ip)
                .add_categories(vec![
                    Category::Hacking,
                    Category::WebAppAttack,
                    Category::BadWebBot,
                    Category::BruteForce,
                ])
                .set_comment_text(format!("{} {}", req.method().as_str(), req.uri()))
        }),
    }
}

pub fn register() -> RequestHandler {
    RequestHandler {
        name: HANDLER_NAME,
        pattern: Regex::new("wp-json").expect("Failed to compile regex"),
        handler,
    }
}

const ENUM_USERS_RESP: &str = "[{\"id\":1,\"name\":\"johnny\",\"url\":\"\",\"description\":\"\",\"link\":\"https://averygoodsite.com/author/johnny/\",\"slug\":\"johnny\",\"meta\":[],\"_links\":{\"self\":[{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users/1\"}],\"collection\":[{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users\"}]}},{\"id\":6,\"name\":\"mikel\",\"url\":\"\",\"description\":\"\",\"link\":\"https://averygoodsite.com/author/mikel/\",\"slug\":\"mikel\",\"meta\":[],\"_links\":{\"self\":[{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users/6\"}],\"collection\":[{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users\"}]}},{\"id\":2,\"name\":\"administrator2\",\"url\":\"\",\"description\":\"\",\"link\":\"https://averygoodsite.com/author/administrator2/\",\"slug\":\"administrator2\",\"meta\":[],\"_links\":{\"self\":[{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users/2\"}],\"collection\":[{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users\"}]}}]>";
