use scraper::{Html, Selector};
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use base64::{Engine as _, engine::general_purpose};
use std::vec;
use regex::Regex;
use std::str::FromStr;
use std::{error::Error, str};
use serde_json;
use crate::structer::{GeonodeProxy,GeonodeProxyData,Proxy};
use crate::structer::CheckerProxy;

pub struct Extractor;

impl Extractor {
    
    pub fn new() -> Self {
        Extractor
    }

    pub fn extract_proxies_advanced(&self, html: &str) -> Result<Vec<Proxy>, Box<dyn Error>> {
        let mut proxies: Vec<Proxy> = Vec::new();
        let document = Html::parse_document(html);
        let tbody_selector = Selector::parse("tbody").unwrap();
        //let mut proxies = Vec::new();
        for tbody in document.select(&tbody_selector) {
            let tr_selector = Selector::parse("tr").unwrap();
            for tr in tbody.select(&tr_selector) {
                let ip_base64 = tr.select(&Selector::parse("td[data-ip]").unwrap()).next().unwrap().value().attr("data-ip").unwrap();
                let port_base64 = tr.select(&Selector::parse("td[data-port]").unwrap()).next().unwrap().value().attr("data-port").unwrap();
    
                let ip_bytes = general_purpose::STANDARD
                    .decode(ip_base64).unwrap();
                let port_bytes = general_purpose::STANDARD
                    .decode(port_base64).unwrap();

                let proxy_type_selector = Selector::parse("td a.label").unwrap();
                let proxy_types: Vec<String> = tr.select(&proxy_type_selector).map(|a| a.inner_html()).collect();
                let proxy_type = proxy_types.join(", "); 

                // 直接从<a>标签的文本内容中获取国家代码
                let country_selector = Selector::parse("td a[href*='country=']").unwrap();
                let country = tr.select(&country_selector).next().map_or_else(|| "".to_string(), |n| n.text().collect()); // 将字节向量转换为字符串

                let last_checked_selector = Selector::parse("td").unwrap();
                let last_checked = tr.select(&last_checked_selector).last().map(|e| e.inner_html()).unwrap_or_default();
                let last_checked = format_time_advanced(&last_checked);

                let ip = str::from_utf8(&ip_bytes).unwrap();
                let port = str::from_utf8(&port_bytes).unwrap();
                //println!("IP => {:#?}  PORT => {:#?} PROXY TYPE => {:#?} COUNTRY => {:#?} LAST_CHECKED => {:#?}", ip,port,proxy_type,country,last_checked);
                let proxy = Proxy {
                    ip: ip.to_string(),
                    port: port.to_string(),
                    proxy_type: proxy_type.to_string(),
                    country: country.to_string(),
                    last_checked: last_checked.to_string(),
                    check_number: 0,
                    live: false
                };
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    pub fn extract_proxies_geonode(&self, html: &str) -> Result<Vec<Proxy>, Box<dyn Error>> {
        let mut proxies: Vec<Proxy> = vec![];
        let proxies_geonode_data: GeonodeProxyData = serde_json::from_str::<GeonodeProxyData>(html)?;
        let proxies_geonode: Vec<GeonodeProxy> = proxies_geonode_data.data;
        for proxy in proxies_geonode {
            let ip = proxy.ip;
            let port = proxy.port;
            let port = port.unwrap_or_else(|| "-".to_string());
            let proxy_type = proxy.protocols.join(",");
            let country = proxy.country;
            let country = country.unwrap_or_else(|| "-".to_string());
            let last_checked = proxy.updated_at;
            let last_checked = last_checked.unwrap_or_else(|| "-".to_string());
            let proxy = Proxy {
                ip,
                port,
                proxy_type,
                country,
                last_checked,
                check_number: 0,
                live: false
            };
            proxies.push(proxy);
        }

        Ok(proxies)
    }

}



fn format_time_advanced(input: &str) -> String {
    // 首先，移除"<small>(...)</small>"部分
    let clean_input = input.split("<small>").next().unwrap_or(input).trim();
    // 然后，调整时间格式
    if let Some((time, date)) = clean_input.split_once(' ') {
        // 假设日期格式是 dd.mm.yyyy，我们需要将其转换为 yyyy.mm.dd
        let date_parts: Vec<&str> = date.split('.').collect();
        if date_parts.len() == 3 {
            return format!("{}.{}.{} {}", date_parts[2], date_parts[1], date_parts[0], time);
        }
    }
    input.to_string()
}
