use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use crate::utils::generate_random_string;
use actix_web::{web::Bytes, HttpRequest, HttpResponse};
use lazy_static::lazy_static;
use log::warn;
use rand::Rng;
use regex::Regex;

const HANDLER_NAME: &str = "wp-json";

struct RESTEndpoint {
    pattern: Regex,
    response: fn() -> RESTEndpointResponse,
}

struct RESTEndpointResponse {
    content: String,
    details: Option<String>,
}

lazy_static! {
    static ref ENDPOINT_LIST: Vec<RESTEndpoint> = {
        vec![RESTEndpoint {
            pattern: Regex::new("v2/users/").expect("Failed to compile regex"),
            response: get_users_response,
        }]
    };
    static ref DEFAULT_ENDPOINT: RESTEndpoint = RESTEndpoint {
        pattern: Regex::new("").expect("Failed to compile regex"),
        response: get_default_response,
    };
}

// TODO: save generated usernames into "details" column
pub fn handler(bytes: Bytes, req: &HttpRequest) -> HandlerResponse {
    let endpoint_resp = (ENDPOINT_LIST
        .iter()
        .find(|endpoint| endpoint.pattern.is_match(req.path()))
        .unwrap_or(&DEFAULT_ENDPOINT)
        .response)();
    HandlerResponse {
        http_response: HttpResponse::Ok()
            .content_type("text/html;charset=UTF-8")
            .body(endpoint_resp.content),
        handler_event: Some(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(req, "Host"))
                .set_uri(req.uri().to_string())
                .set_x_forwarded_for(get_header_value(req, "X-Forwarded-For"))
                .set_src_ip(get_ip_address(req))
                .set_user_agent(get_header_value(req, "User-Agent"))
                .set_details(endpoint_resp.details)
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
                    Category::BruteForce,
                ])
                .set_comment_text(format!("{} {}", req.method().as_str(), req.uri()))
        }),
    }
}

fn get_users_response() -> RESTEndpointResponse {
    let mut rng = rand::thread_rng();
    let username1 = generate_random_string(rng.gen_range(10..20));
    let username2 = generate_random_string(rng.gen_range(10..20));
    let username3 = generate_random_string(rng.gen_range(10..20));
    RESTEndpointResponse {
        content: format!(
            "[{{\"id\":1,\"name\":\"{0}\",\"url\":\"\",\"description\":\"\",\"link\":\"https://averygoodsite.com/author/{0}/\",\"slug\":\"{0}\",\"meta\":[],\"_links\":{{\"self\":[{{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users/1\"}}],\"collection\":[{{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users\"}}]}}}},{{\"id\":6,\"name\":\"{1}\",\"url\":\"\",\"description\":\"\",\"link\":\"https://averygoodsite.com/author/{1}/\",\"slug\":\"{1}\",\"meta\":[],\"_links\":{{\"self\":[{{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users/6\"}}],\"collection\":[{{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users\"}}]}}}},{{\"id\":2,\"name\":\"{2}\",\"url\":\"\",\"description\":\"\",\"link\":\"https://averygoodsite.com/author/{2}/\",\"slug\":\"{2}\",\"meta\":[],\"_links\":{{\"self\":[{{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users/2\"}}],\"collection\":[{{\"href\":\"https://averygoodsite.com/wp-json/wp/v2/users\"}}]}}}}]",
            username1,
            username2,
            username3
        ),
        details: Some(format!("Usernames: {}, {}, {}", username1, username2, username3)),
    }
}

fn get_default_response() -> RESTEndpointResponse {
    RESTEndpointResponse {
        content: "".to_string(),
        details: None,
    }
}

pub fn register() -> RequestHandler {
    RequestHandler {
        name: HANDLER_NAME,
        pattern: Regex::new("wp-json").expect("Failed to compile regex"),
        handler,
    }
}
