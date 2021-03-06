use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, get_ip_address, HandlerResponse, RequestHandler};
use crate::reporter::{Category, Report};
use actix_web::{web::Bytes, HttpRequest, HttpResponse};
use regex::Regex;

pub const HANDLER_NAME: &str = "wp-wlwmanifest";

const RESP_CONTENT: &str = "<?xml version=\"1.0\" encoding=\"utf-8\" ?>

<manifest xmlns=\"http://schemas.microsoft.com/wlw/manifest/weblog\">

  <options>
    <clientType>WordPress</clientType>
	<supportsKeywords>Yes</supportsKeywords>
	<supportsGetTags>Yes</supportsGetTags>
  </options>

  <weblog>
    <serviceName>WordPress</serviceName>
    <imageUrl>images/wlw/wp-icon.png</imageUrl>
    <watermarkImageUrl>images/wlw/wp-watermark.png</watermarkImageUrl>
    <homepageLinkText>View site</homepageLinkText>
    <adminLinkText>Dashboard</adminLinkText>
    <adminUrl>
      <![CDATA[
			{blog-postapi-url}/../wp-admin/
		]]>
    </adminUrl>
    <postEditingUrl>
      <![CDATA[
			{blog-postapi-url}/../wp-admin/post.php?action=edit&post={post-id}
		]]>
    </postEditingUrl>
  </weblog>

  <buttons>
    <button>
      <id>0</id>
      <text>Manage Comments</text>
      <imageUrl>images/wlw/wp-comments.png</imageUrl>
      <clickUrl>
        <![CDATA[
				{blog-postapi-url}/../wp-admin/edit-comments.php
			]]>
      </clickUrl>
    </button>

  </buttons>

</manifest>
";

pub fn handler(_bytes: Bytes, req: &HttpRequest) -> HandlerResponse {
    HandlerResponse {
        http_response: HttpResponse::Ok()
            .content_type("application/xml;charset=UTF-8")
            .body(RESP_CONTENT),
        handler_event: Some(
            HandlerEvent::new(HANDLER_NAME)
                .set_host(get_header_value(req, "Host"))
                .set_uri(req.uri().to_string())
                .set_x_forwarded_for(get_header_value(req, "X-Forwarded-For"))
                .set_src_ip(get_ip_address(req))
                .set_user_agent(get_header_value(req, "User-Agent")),
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
        pattern: Regex::new("wp-includes/wlwmanifest\\.xml").expect("Failed to compile regex"),
        handler,
    }
}
