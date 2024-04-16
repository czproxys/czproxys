use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::hash::Hash;

#[derive(Debug,Clone, Eq, PartialEq, Hash)]
pub struct Proxy {
    pub ip: String,
    pub port: String,
    pub proxy_type: String,
    pub country: String,
    pub last_checked: String,
    pub check_number: u64,
    pub live: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckerProxy {
    pub id: u64,
    pub local_id: u32,
    pub report_id: String,
    pub addr: String,
    #[serde(rename = "type")]
    pub proxy_type: u8,
    pub kind: u8,
    pub timeout: u32,
    pub cookie: bool,
    pub referer: bool,
    pub post: bool,
    pub ip: String,
    pub addr_geo_iso: String,
    pub addr_geo_country: String,
    pub addr_geo_city: String,
    pub ip_geo_iso: String,
    pub ip_geo_country: String,
    pub ip_geo_city: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub skip: bool,
    pub from_cache: bool,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct GeonodeProxy {
    pub _id: String,
    pub ip: String,
    #[serde(rename = "anonymityLevel")]
    pub anonymity_level: Option<String>,
    pub asn: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    pub google: bool,
    pub isp: Option<String>,
    #[serde(rename = "lastChecked")]
    pub last_checked: u64,
    pub latency: f64,
    pub org: Option<String>,
    pub port: Option<String>,
    pub protocols: Vec<String>,
    pub region: Option<String>,
    #[serde(rename = "responseTime")]
    pub response_time: u64,
    pub speed: u64,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "workingPercent")]
    pub working_percent: Option<f64>,
    #[serde(rename = "upTime")]
    pub up_time: f64,
    #[serde(rename = "upTimeSuccessCount")]
    pub up_time_success_count: u64,
    #[serde(rename = "upTimeTryCount")]
    pub up_time_try_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeonodeProxyData {
    pub data: Vec<GeonodeProxy>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

