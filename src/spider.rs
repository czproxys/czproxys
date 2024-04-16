use std::time::Duration;
use tokio_retry::{strategy::FixedInterval, Retry};
use reqwest::{Client, Error};
use md5;


pub struct Spider {
    pub urls: Vec<String>
}

impl Spider {

    pub async fn new(urls: Vec<String>) -> Self {
        Spider { urls }
    }

    pub async fn fetch(&self) -> Result<Vec<String>, Error> {
        let client = create_client().await?;
        let mut bodies = Vec::new();
        for url in &self.urls {
            match fetch_with_retry(&client, url).await {
                Ok(body) => bodies.push(body),
                Err(e) => eprintln!("Failed after retries for URL {}: {}", url, e),
            }
        }
        Ok(bodies)
    }
}

async fn fetch_with_retry(client: &Client, url: &str) -> Result<String, Error> {    
    let url_md5 = format!("{:x}", md5::compute(url.as_bytes()));
    println!("spider core downloader => {}",url_md5);
    let strategy = FixedInterval::from_millis(1000).take(3); // 每2秒重试，最多重试3次
    let action = || async {
        let response = client.get(url).send().await?;
        response.text().await
    };

    Retry::spawn(strategy, action).await
}

async fn create_client() -> Result<Client,Error> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .http1_only()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()?;
    Ok(client)
}

