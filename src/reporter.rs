use actix_web::client::ClientBuilder;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct ReporterConfig {
    pub api_key: String,
    pub endpoint: String,
}

pub struct Report {
    pub ip: String,
    pub categories: HashSet<Category>,
    pub comment: Option<String>,
}

#[allow(dead_code)]
impl Report {
    pub fn new(ip: String) -> Report {
        Report {
            ip,
            categories: HashSet::new(),
            comment: None,
        }
    }

    pub fn set_comment(mut self, comment: Option<String>) -> Self {
        self.comment = comment;
        self
    }

    pub fn add_category(mut self, category: Category) -> Self {
        self.categories.insert(category);
        self
    }

    pub fn add_categories(mut self, categories: impl IntoIterator<Item = Category>) -> Self {
        self.categories.extend(categories);
        self
    }

    pub fn remove_category(mut self, category: Category) -> Self {
        self.categories.remove(&category);
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Category {
    DNSCompromise = 1,
    DNSPoisoning,
    FraudOrders,
    DDoSAttack,
    FTPBruteForce,
    PingOfDeath,
    Phishing,
    FraudVoIP,
    OpenProxy,
    WebSpam,
    EmailSpam,
    BlogSpam,
    VPNAddress,
    PortScan,
    Hacking,
    SQLInjection,
    Spoofing,
    BruteForce,
    BadWebBot,
    ExploitedHost,
    WebAppAttack,
    SSH,
    IoTTargeted,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReportHttpBody {
    ip: String,
    categories: String,
    comment: Option<String>,
}

pub async fn submit_reports(config: ReporterConfig, receiver: mpsc::Receiver<Report>) {
    debug!("Submitting reports");

    let mut report_timestamps: HashMap<String, Instant> = HashMap::new();

    let endpoint_url = config.endpoint.as_str();
    let client = ClientBuilder::default()
        .header("Accept", "application/json")
        .header("Key", config.api_key)
        .finish();

    while let Ok(msg) = receiver.recv() {
        if report_timestamps.contains_key(&msg.ip)
            && Instant::now().duration_since(report_timestamps.get(&msg.ip).unwrap().to_owned())
                < Duration::from_secs(15 * 60)
        {
            debug!("Skipping report for {} - rate limit", msg.ip);
            continue;
        }
        report_timestamps.insert(msg.ip.clone(), Instant::now());

        let http_report = ReportHttpBody {
            ip: msg.ip,
            categories: msg
                .categories
                .into_iter()
                .map(|c| (c as i32).to_string())
                .collect::<Vec<String>>()
                .join(","),
            comment: msg.comment,
        };

        match client.post(endpoint_url).send_json(&http_report).await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully submitted report for {}", http_report.ip);
                } else {
                    error!(
                        "Failed to submit report for {}: {} ({})",
                        http_report.ip,
                        response.status(),
                        response.status().canonical_reason().unwrap_or("Unknown")
                    );
                }
            }
            Err(e) => {
                error!("Failed to submit report for {}: {}", http_report.ip, e);
            }
        };
    }
    println!("Exiting");
}
