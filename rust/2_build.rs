use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Response struct for Hypersonic build API
#[derive(Debug, Deserialize)]
struct BuildResponse {
    success: bool,
    data: BuildData,
    #[allow(dead_code)]
    timestamp: String,
}

/// Transaction data returned by Hypersonic API
#[derive(Debug, Deserialize)]
struct BuildData {
    transaction: Transaction,
}

/// Transaction struct
#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    to: String,
    data: String,
    value: String,
}

/// Build a transaction using quote data from Hypersonic API
///
/// # Arguments
///
/// * `quote_data` - Quote data obtained from `/quote` endpoint
///
/// # Returns
///
/// Transaction data ready to be executed
async fn build_transaction(quote_data: serde_json::Value) -> Result<Transaction, Box<dyn Error>> {

    println!("Building transaction...");

    let client = Client::new();
    
    let response = client
        .post("https://api.hypersonic.exchange/v1/build")
        .header("Content-Type", "application/json")
        .json(&quote_data)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }
    
    let build_response: BuildResponse = response.json().await?;
    
    if !build_response.success {
        return Err(format!("Failed to build transaction: {:?}", build_response).into());
    }
    
    Ok(build_response.data.transaction)
}

/// Example usage of build_transaction()
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Sample quote data (obtained from `/quote` endpoint)
    let sample_quote_data = serde_json::json!({
        "chainId": 146,
        "inToken": "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38",
        "inDecimals": 18,
        "inAmount": "1000000000000000000",
        "outToken": "0x29219dd400f2bf60e5a23d13be72b486d4038894",
        "outDecimals": 6,
        "outAmount": "703174",
        "slippage": 1,
        "refCode": 0,
        "minReceived": "696212",
        "bestRoute": [
            {
                "percent": 100,
                "swaps": [
                    [
                        {
                            "inToken": "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38",
                            "inDecimals": 18,
                            "outToken": "0x29219dd400f2bf60e5a23d13be72b486d4038894",
                            "outDecimals": 6,
                            "swapExchanges": [
                                [
                                    {
                                        "exchange": "WagmiV3",
                                        "inAmount": "1000000000000000000",
                                        "outAmount": "703244",
                                        "percent": 100,
                                        "data": {
                                            "path": [
                                                "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38",
                                                "0x29219dd400f2bf60e5a23d13be72b486d4038894"
                                            ],
                                            "fee": "500",
                                            "multiHop": false
                                        }
                                    }
                                ]
                            ]
                        }
                    ]
                ]
            }
        ],
        "contractAddress": "0x5045E3E6F8a07690390dE1240C5Bb8ab2184500a",
        "contractMethod": "swap_wagmi_v3",
        "blockNumber": 10351341
    });
    
    match build_transaction(sample_quote_data).await {
        Ok(transaction) => {
            println!("--------------------------------------------------------");
            println!("{}", serde_json::to_string_pretty(&transaction)?);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    Ok(())
}