use std::error::Error;
use crate::storage::initialize_db;
use scheduler::all_enginer_running;
use tokio::time::{sleep, Duration};
use async_recursion::async_recursion;

/**
 page => 
    https://advanced.name/freeproxy?page=1
    https://github.com/TheSpeedX/PROXY-List
 */
mod spider;
mod structer;
mod extractor;
mod scheduler;
mod validator;
mod storage;


#[async_recursion]
async fn run_periodically() -> Result<(),Box<dyn Error>> {
    // 运行任务
    match all_enginer_running().await {
        Ok(_) => println!("Task completed successfully. wait 10 minutes"),
        Err(e) => eprintln!("Task failed: {}", e),
    }
    sleep(Duration::from_secs(60 * 10)).await;
    run_periodically().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initialize_db()?;
    run_periodically().await

}
