use actix_rt::System;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use handler::request_dispatcher;
use log::{debug, error, info, trace, warn};
use reporter::Report;
use std::env;
use std::net::SocketAddr;
use std::sync::mpsc;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod configuration;
mod db;
mod handler;
mod handlers;
mod reporter;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args: Vec<String> = env::args().collect();
    let default_config_file: String = String::from("Config.toml");
    let config_path = args.get(1).unwrap_or(&default_config_file);

    debug!("Loading configuration from {}", config_path);
    {
        let mut config = configuration::CONFIG.write().unwrap_or_else(|e| {
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

    configuration::load_configuration();
    info!("Loaded configuration");

    let settings = configuration::get_settings_reader();
    trace!("{:#?}", settings);

    let conn_pool = db::establish_connection();
    info!("Connected to database");

    let (tx, rx) = mpsc::channel::<Report>();
    if settings.reporting_enabled {
        let reporter_config = reporter::ReporterConfig {
            api_key: settings.abuseipdb_key.clone().unwrap_or_else(|| {
                error!("Failed to get abuseipdb-key from config");
                std::process::abort();
            }),
            endpoint: settings.report_endpoint.clone(),
        };
        std::thread::spawn(move || {
            info!("Starting reporter thread");
            let mut sys = System::new("reporter");
            sys.block_on(reporter::submit_reports(reporter_config, rx));
        });
    } else {
        warn!("AbuseIPDB reporting is disabled");
    }

    info!("Starting HTTP server");
    let mut srv = HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .data(tx.clone())
            .default_service(web::route().to(request_dispatcher))
    })
    .workers(settings.workers.unwrap_or(2).try_into().unwrap_or(2));
    srv = if settings.port.is_some() {
        let port = settings.port.expect("Could not get port from settings");
        let addr_obj = match format!("{}:{}", settings.host, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to parse HTTP host and HTTP port: {}", e);
                std::process::abort();
            }
        };
        info!("Binding to IP address {}:{}", settings.host, port);
        srv.bind(addr_obj)?
    } else {
        warn!("Binding to UNIX socket \"{}\"", settings.host);
        srv.bind_uds(&settings.host)?
    };
    let run_result = srv.run();

    run_result.await
}
