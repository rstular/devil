use actix_rt::System;
use actix_web::{middleware, web, App, HttpServer};
use config::Config;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use reporter::Report;
use std::env;
use std::net::SocketAddr;
use std::sync::{mpsc, RwLock};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod handler;
mod handlers;
mod reporter;

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
            std::process::abort();
        });
        config
            .merge(config::File::with_name(config_path))
            .unwrap_or_else(|e| {
                error!("Failed to load config file \"{}\": {}", config_path, e);
                std::process::abort();
            });
        config
            .merge(config::Environment::with_prefix("LILDEVIL"))
            .unwrap_or_else(|e| {
                error!("Failed to load environment variables: {}", e);
                std::process::abort();
            });
    }
    info!("Loaded configuration");
    trace!("{:#?}", SETTINGS.read().unwrap());

    let addr = SETTINGS
        .read()
        .unwrap_or_else(|e| {
            error!("Failed to acquire read lock on settings: {}", e);
            std::process::abort();
        })
        .get_str("http-host")
        .unwrap_or_else(|_| String::from("127.0.0.1:8080"));

    let conn_pool = db::establish_connection();
    info!("Connected to database");

    let (tx, rx) = mpsc::channel::<Report>();

    let reporter_config = reporter::ReporterConfig {
        api_key: SETTINGS
            .read()
            .expect("Could not obtain read lock on settings")
            .get_str("abuseipdb-key")
            .unwrap_or_else(|_| {
                error!("Failed to acquire read lock on settings");
                std::process::abort();
            }),
        endpoint: SETTINGS
            .read()
            .unwrap_or_else(|e| {
                error!("Failed to acquire read lock on settings: {}", e);
                std::process::abort();
            })
            .get_str("reporter-endpoint")
            .unwrap_or_else(|_| String::from("https://api.abuseipdb.com/api/v2/report")),
    };
    std::thread::spawn(move || {
        info!("Starting reporter thread");
        let mut sys = System::new("reporter");
        sys.block_on(reporter::submit_reports(reporter_config, rx));
    });

    info!("Starting HTTP server");
    let mut srv = HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .data(tx.clone())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(request_dispatcher))
    });
    let port = SETTINGS
        .read()
        .unwrap_or_else(|e| {
            error!("Failed to acquire read lock on settings: {}", e);
            std::process::abort();
        })
        .get_int("http-port")
        .unwrap_or(8080);
    srv = if format!("{}:{}", addr, port).parse::<SocketAddr>().is_ok() {
        info!("Binding to IP address {}:{}", addr, port);
        srv.bind(format!("{}:{}", addr, port))?
    } else {
        warn!("Binding to non-IP address {}", addr);
        srv.bind_uds(addr)?
    };
    let run_result = srv.run();

    run_result.await
}
