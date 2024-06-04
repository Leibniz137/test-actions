use tokio::time::{sleep, Duration};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct BlockNumberResponse {
    result: String,
}

async fn fetch_block_number(client: &Client, rpc_url: &str) -> Result<u64, Box<dyn Error>> {
    let response = client.post(rpc_url)
        .json(&serde_json::json!({"jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": 1}))
        .send()
        .await?
        .json::<BlockNumberResponse>()
        .await?;

    // Convert the hex string result to a u64 number
    let block_number = u64::from_str_radix(&response.result.trim_start_matches("0x"), 16)?;
    Ok(block_number)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "http://localhost:8543";
    let client = Client::new();

    let mut previous_block_number = match fetch_block_number(&client, rpc_url).await {
        Ok(number) => number,
        Err(e) => {
            eprintln!("Error fetching block number: {}", e);
            0
        }
    };
    let mut failed_attempts = 0;
    let mut successful_attempts = 0;
    let max_failed_attempts = 100;
    let max_successful_attempts = 10;

    loop {
        sleep(Duration::from_secs(10)).await;

        match fetch_block_number(&client, rpc_url).await {
            Ok(current_block_number) => {
                if current_block_number > previous_block_number {
                    println!("New block detected: {}", current_block_number);
                    previous_block_number = current_block_number;
                    successful_attempts += 1; // Reset attempts on successful block detection
                } else {
                    println!("No new block yet. Current block: {}", current_block_number);
                    failed_attempts += 1;
                }
            }
            Err(e) => {
                eprintln!("Error fetching block number: {}", e);
                failed_attempts += 1;
            }
        }

        if failed_attempts >= max_failed_attempts {
            eprintln!("Failed to fetch new block or no new block produced after {} attempts.", max_failed_attempts);
            std::process::exit(1);
        }
        if successful_attempts >= max_successful_attempts {
            eprintln!("we're making blocks!");
            std::process::exit(0);
        }
    }
}
