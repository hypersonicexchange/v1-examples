use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Hypersonic API quote request parameters
#[derive(Debug, Serialize)]
struct QuoteRequest {
    chain_id: u64,
    in_token: String,
    out_token: String,
    in_amount: String,
    slippage: u64,
    referral_code: Option<u64>,
    // If you want to charge a fee, you can register a refCode by following -> https://docs.hypersonic.exchange/referral
}

/// Response struct for Hypersonic API responses
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    #[allow(dead_code)]
    timestamp: String,
}

/// Get a swap quote from Hypersonic API
///
/// # Arguments
///
/// * `request` - Quote request parameters
///
/// # Returns
///
/// Quote data from Hypersonic API
async fn get_quote(request: QuoteRequest) -> Result<serde_json::Value, Box<dyn Error>> {
    println!("Getting quote...");
    
    let client = Client::new();
    
    let mut json_body = serde_json::json!({
        "chainId": request.chain_id,
        "inToken": request.in_token,
        "outToken": request.out_token,
        "inAmount": request.in_amount,
        "slippage": request.slippage,
    });
    
    if let Some(ref code) = request.referral_code {
        json_body["refCode"] = serde_json::Value::Number((*code).into());
    }
    
    let response = client
        .post("https://api.hypersonic.exchange/v1/quote")
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }
    
    let quote_response: ApiResponse<serde_json::Value> = response.json().await?;
    
    if !quote_response.success {
        return Err(format!("Failed to get quote: {:?}", quote_response).into());
    }
    
    Ok(quote_response.data)
}

/// Example usage of get_quote()
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let quote_request = QuoteRequest {
        chain_id: 146, // Sonic
        in_token: "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38".to_string(), // wS on Sonic
        out_token: "0x29219dd400f2bf60e5a23d13be72b486d4038894".to_string(), // USDC.e on Sonic
        in_amount: "1000000000000000000".to_string(), // 1 wS (18 decimals)
        slippage: 1, // 1% slippage tolerance
        referral_code: Some(0), // Optional
    };
    
    match get_quote(quote_request).await {
        Ok(quote_data) => {
            println!("--------------------------------------------------------");
            println!("{}", &quote_data);
            println!("--------------------------------------------------------");
            println!("Input Token: {}", quote_data["inToken"].as_str().unwrap_or_default());
            println!("Output Token: {}", quote_data["outToken"].as_str().unwrap_or_default());
            println!("Expected Output Amount: {}", quote_data["outAmount"].as_str().unwrap_or_default());
            println!("Minimum Received: {}", quote_data["minReceived"].as_str().unwrap_or_default());
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    Ok(())
}