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
    // let rpc_url = "http://192.168.50.102:8545"; // Replace with your Ethereum RPC URL
    let rpc_url = "http://localhost:8545";
    let client = Client::new();

    let mut previous_block_number = fetch_block_number(&client, rpc_url).await?;

    loop {
        sleep(Duration::from_secs(10)).await;

        match fetch_block_number(&client, rpc_url).await {
            Ok(current_block_number) => {
                if current_block_number > previous_block_number {
                    println!("New block detected: {}", current_block_number);
                    previous_block_number = current_block_number;
                } else {
                    println!("No new block yet. Current block: {}", current_block_number);
                }
            }
            Err(e) => {
                eprintln!("Error fetching block number: {}", e);
            }
        }
    }
}
