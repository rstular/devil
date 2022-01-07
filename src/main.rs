use actix_web::{middleware, web, App, HttpServer};
use config::Config;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use std::env;
use std::net::SocketAddr;
use std::sync::RwLock;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod handler;
mod handlers;

use handler::request_dispatcher;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args: Vec<String> = env::args().collect();
    let default_config_file: String = String::from("Config.toml");
    let config_path = args.get(1).unwrap_or(&default_config_file);

    debug!("Loading configuration from {}", config_path);
    {
        let mut config = SETTINGS.write().unwrap_or_else(|e| {
            error!("Failed to acquire write lock on settings: {}", e);
            std::process::exit(1);
        });
        config
            .merge(config::File::with_name(config_path))
            .unwrap_or_else(|e| {
                error!("Failed to load config file \"{}\": {}", config_path, e);
                std::process::exit(1);
            });
        config
            .merge(config::Environment::with_prefix("LILDEVIL"))
            .unwrap_or_else(|e| {
                error!("Failed to load environment variables: {}", e);
                std::process::exit(1);
            });
    }
    info!("Loaded configuration");
    trace!("{:#?}", SETTINGS.read().unwrap());

    let addr = SETTINGS
        .read()
        .unwrap_or_else(|e| {
            error!("Failed to acquire read lock on settings: {}", e);
            std::process::exit(1);
        })
        .get_str("http-host")
        .unwrap_or_else(|_| String::from("127.0.0.1:8080"));

    let conn_pool = db::establish_connection();
    info!("Connected to database");

    info!("Starting HTTP server");

    let mut srv = HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(request_dispatcher))
    });

    let port = SETTINGS
        .read()
        .unwrap_or_else(|e| {
            error!("Failed to acquire read lock on settings: {}", e);
            std::process::exit(1);
        })
        .get_int("http-port")
        .unwrap_or_else(|_| 8080);
    srv = if let Ok(_) = format!("{}:{}", addr, port).parse::<SocketAddr>() {
        info!("Binding to IP address {}:{}", addr, port);
        srv.bind(format!("{}:{}", addr, port))?
    } else {
        warn!("Binding to non-IP address {}", addr);
        srv.bind_uds(addr)?
    };

    srv.run().await
}
