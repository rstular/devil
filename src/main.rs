use actix_rt::System;
use actix_web::{middleware, web, App, HttpServer};
use config::Config;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use reporter::Report;
use std::env;
use std::net::SocketAddr;
use std::sync::{mpsc, RwLock, RwLockReadGuard};

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

use handler::request_dispatcher;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
    pub static ref SETTINGS: RwLock<configuration::Settings> =
        RwLock::new(configuration::Settings::default());
}

pub fn get_config_reader() -> RwLockReadGuard<'static, Config> {
    CONFIG.read().unwrap_or_else(|e| {
        error!("Failed to acquire read lock on config file: {}", e);
        std::process::abort();
    })
}

pub fn get_settings_reader() -> RwLockReadGuard<'static, configuration::Settings> {
    SETTINGS.read().unwrap_or_else(|e| {
        error!("Failed to acquire read lock on settings: {}", e);
        std::process::abort();
    })
}

pub fn load_configuration() {
    let settings = get_config_reader();
    let parsed_config = configuration::Settings {
        host: settings.get_str("http-host").unwrap_or_else(|_| {
            error!("Failed to get http-host from config");
            std::process::abort();
        }),
        port: settings.get_int("http-port").ok(),
        reporting_enabled: settings.get_bool("enable-reporting").unwrap_or(false),
        abuseipdb_key: settings.get_str("abuseipdb-key").ok(),
        report_endpoint: settings
            .get("report-endpoint")
            .unwrap_or_else(|_| String::from("https://api.abuseipdb.com/api/v2/report")),
        db_path: settings
            .get_str("db-path")
            .unwrap_or_else(|_| String::from("storage.db")),
    };
    drop(settings);
    let mut settings_guard = SETTINGS.write().unwrap_or_else(|e| {
        error!("Failed to acquire write lock on settings: {}", e);
        std::process::abort();
    });
    *settings_guard = parsed_config;
    drop(settings_guard);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args: Vec<String> = env::args().collect();
    let default_config_file: String = String::from("Config.toml");
    let config_path = args.get(1).unwrap_or(&default_config_file);

    debug!("Loading configuration from {}", config_path);
    {
        let mut config = CONFIG.write().unwrap_or_else(|e| {
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

    load_configuration();
    info!("Loaded configuration");

    let settings = get_settings_reader();
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
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(request_dispatcher))
    });
    srv = if settings.port.is_some() {
        let port = settings.port.expect("Could not get port from settings");
        let addr_obj = match format!("{}:{}", settings.host, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to parse http-host and http-port: {}", e);
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
