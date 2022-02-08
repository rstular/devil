use config::Config;
use lazy_static::lazy_static;
use log::error;
use std::sync::{RwLock, RwLockReadGuard};

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::default());
}

pub fn get_config_reader() -> RwLockReadGuard<'static, Config> {
    CONFIG.read().unwrap_or_else(|e| {
        error!("Failed to acquire read lock on config file: {}", e);
        std::process::abort();
    })
}

pub fn get_settings_reader() -> RwLockReadGuard<'static, Settings> {
    SETTINGS.read().unwrap_or_else(|e| {
        error!("Failed to acquire read lock on settings: {}", e);
        std::process::abort();
    })
}

pub fn load_configuration() {
    let settings = get_config_reader();
    let parsed_config = Settings {
        host: settings.get_str("http.host").unwrap_or_else(|_| {
            error!("Failed to get HTTP host from config");
            std::process::abort();
        }),
        port: settings.get_int("http.port").ok(),
        workers: settings.get_int("http.workers").ok(),
        reporting_enabled: settings.get_bool("reporting.enabled").unwrap_or(false),
        abuseipdb_key: settings.get_str("reporting.abuseipdb-key").ok(),
        report_endpoint: settings
            .get("report-endpoint")
            .unwrap_or_else(|_| String::from("https://api.abuseipdb.com/api/v2/report")),
        db_path: settings
            .get_str("db.path")
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

#[derive(Debug)]
pub struct Settings {
    pub host: String,
    pub port: Option<i64>,
    pub workers: Option<i64>,
    pub reporting_enabled: bool,
    pub abuseipdb_key: Option<String>,
    pub report_endpoint: String,
    pub db_path: String,
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            host: String::from("127.0.0.1"),
            port: Some(8080),
            workers: Some(2),
            reporting_enabled: false,
            abuseipdb_key: None,
            report_endpoint: String::from("https://api.abuseipdb.com/api/v2/report"),
            db_path: String::from("storage.db"),
        }
    }
}
