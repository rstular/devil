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

    pub fn set_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn set_port(mut self, port: i64) -> Self {
        self.port = Some(port);
        self
    }

    pub fn set_workers(mut self, workers: i64) -> Self {
        self.workers = Some(workers);
        self
    }

    pub fn set_db_path(mut self, db_path: String) -> Self {
        self.db_path = db_path;
        self
    }

    pub fn set_reporting_enabled(mut self, reporting_enabled: bool) -> Self {
        self.reporting_enabled = reporting_enabled;
        self
    }

    pub fn set_abuseipdb_key(mut self, abuseipdb_key: String) -> Self {
        self.abuseipdb_key = Some(abuseipdb_key);
        self
    }

    pub fn set_report_endpoint(mut self, report_endpoint: String) -> Self {
        self.report_endpoint = report_endpoint;
        self
    }

    pub fn clear_port(&mut self) {
        self.port = None;
    }

    pub fn clear_abuseipdb_key(&mut self) {
        self.abuseipdb_key = None;
    }
}
