use crate::db::models::HandlerEvent;
use crate::handler::{get_header_value, HandlerResponse, RequestHandler};
use actix_web::{web::Bytes, HttpRequest};
use regex::Regex;

pub const HANDLER_NAME: &str = "etc-passwd";

pub const RESP_CONTENT: &str =
    "root:$1$eaO8rFkv$Sp0ViHLtUYu4KiBdM6uBb0:0:0:root:/root:/sbin/nologin
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
bin:x:2:2:bin:/bin:/usr/sbin/nologin
sys:x:3:3:sys:/dev:/usr/sbin/nologin
sync:x:4:65534:sync:/bin:/bin/sync
games:x:5:60:games:/usr/games:/usr/sbin/nologin
man:x:6:12:man:/var/cache/man:/usr/sbin/nologin
lp:x:7:7:lp:/var/spool/lpd:/usr/sbin/nologin
mail:x:8:8:mail:/var/mail:/usr/sbin/nologin
news:x:9:9:news:/var/spool/news:/usr/sbin/nologin
uucp:x:10:10:uucp:/var/spool/uucp:/usr/sbin/nologin
proxy:x:13:13:proxy:/bin:/usr/sbin/nologin
www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin
backup:x:34:34:backup:/var/backups:/usr/sbin/nologin
list:x:38:38:Mailing List Manager:/var/list:/usr/sbin/nologin
irc:x:39:39:ircd:/run/ircd:/usr/sbin/nologin
gnats:x:41:41:Gnats Bug-Reporting System (admin):/var/lib/gnats:/usr/sbin/nologin
nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin
_apt:x:100:65534::/nonexistent:/usr/sbin/nologin
systemd-timesync:x:101:101:systemd Time Synchronization,,,:/run/systemd:/usr/sbin/nologin
systemd-network:x:102:103:systemd Network Management,,,:/run/systemd:/usr/sbin/nologin
systemd-resolve:x:103:104:systemd Resolver,,,:/run/systemd:/usr/sbin/nologin
messagebus:x:104:110::/nonexistent:/usr/sbin/nologin
sshd:x:105:65534::/run/sshd:/usr/sbin/nologin
systemd-coredump:x:999:999:systemd Core Dumper:/:/usr/sbin/nologin
srvusr:$1$QO0MLhd/$oFscmkyswsIHrKrZmD2LS0:1000:1000::/home/srvusr:/bin/bash
testuser:$1$3yYB8D5V$m5a9g1SEx9mp/EEH0.C74/:1001:1001::/home/testuser:/bin/bash
postgres:x:106:113:PostgreSQL administrator,,,:/var/lib/postgresql:/bin/bash
mysql:x:107:114:MySQL Server,,,:/nonexistent:/bin/false";

pub fn handler(_bytes: Bytes, req: HttpRequest) -> HandlerResponse {
    HandlerResponse::new(RESP_CONTENT).set_event(
        HandlerEvent::new(HANDLER_NAME)
            .set_host(get_header_value(&req, "Host"))
            .set_src_ip(get_header_value(&req, "X-Forwarded-For"))
            .set_uri(req.uri().to_string()),
    )
}

pub fn register() -> RequestHandler {
    RequestHandler {
        name: HANDLER_NAME,
        pattern: Regex::new(".*etc.*passwd").unwrap(),
        handler,
    }
}
