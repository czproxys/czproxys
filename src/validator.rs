use chrono::Utc; 
use reqwest::Client;
use std::time::Duration;
use crate::structer::Proxy;
use futures::stream::{self, StreamExt};
use crate::storage::{store_proxies,live_proxies_db_update};

pub fn proxy_scheme(proxy_types: &str, force_https: bool) -> String {
    let proxy_types = proxy_types.to_lowercase();
    let mut proxy_scheme = proxy_types.split(", ").collect::<Vec<_>>()[0];
    if proxy_scheme == "socks4" {proxy_scheme = "socks5"};
    if force_https && proxy_types.contains("https") {proxy_scheme = "https"};
    proxy_scheme.to_string()
}
pub async fn check_proxy_alive(proxy: &Proxy) -> bool {
    let proxy_type = proxy_scheme(&proxy.proxy_type, false);
    let proxy_url = format!("{}://{}:{}", proxy_type, proxy.ip, proxy.port);
    let client = Client::builder()
        .proxy(reqwest::Proxy::all(&proxy_url).expect(&format!("Client builder, proxy: {proxy_url}")))
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let test_url = "https://httpbin.org/ip";
    match client.get(test_url).timeout(Duration::from_secs(10)).send().await {
        Ok(response) => {
            print!("{proxy_url} {:?} status: {}", proxy, response.status());
            let ret = response.status().is_success();
            if ret {print!(" body: {}", response.text().await.unwrap_or_default());}
            println!();
            ret
        },
        Err(_e) => {
            //println!("{:?}",proxy);
            false
        },
    }
}

pub async fn process_proxies(proxies: Vec<Proxy>) -> Vec<Proxy> {
    let check_futures = stream::iter(proxies.into_iter().map(|proxy| {
        async move {
            let alive = check_proxy_alive(&proxy).await;
            let last_checked = if alive {
                Utc::now().format("%Y.%m.%d %H:%M:%S").to_string()
            } else {
                proxy.last_checked.clone()
            };
            Proxy {
                ip: proxy.ip,
                port: proxy.port,
                proxy_type: proxy.proxy_type.to_lowercase(),
                country: proxy.country.to_uppercase(),
                last_checked, 
                check_number: proxy.check_number + 1,
                live: alive,
            }
        }
    }));

    check_futures.buffered(300).collect().await
}


pub async fn store_checked_proxies(check_proxies: Vec<Proxy>) {

    println!("inserting all discovery proxy to db...");

    tokio::task::spawn_blocking(|| {
        let _ = store_proxies(check_proxies);
    }).await.expect("Failed to execute store_proxies");
    
    println!("inserting all live proxy to db...");

    tokio::task::spawn_blocking(|| {
        let cnt = live_proxies_db_update().unwrap();
        println!("Alive proxies - {cnt}");
    }).await.expect("Failed to execute live_proxies_db_update");

    println!("All db stored, bye !!!")

}

