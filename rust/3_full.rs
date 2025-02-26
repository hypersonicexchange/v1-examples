use ethers::{ prelude::*, types::{transaction::eip2718::TypedTransaction, Bytes, U256} };
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

/// Hypersonic client
struct HypersonicClient {
    client: Client,
}

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

/// Transaction data returned by Hypersonic build endpoint
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

impl HypersonicClient {
    /// Create Hypersonic client
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    
    /// STEP 1: Get a quote from Hypersonic API
    ///
    /// # Arguments
    ///
    /// * `request` - Quote request parameters
    ///
    /// # Returns
    ///
    /// Quote data from Hypersonic API
    async fn get_quote(&self, request: QuoteRequest) -> Result<serde_json::Value, Box<dyn Error>> {

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
        
        let response = self.client
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
    
    /// STEP 2: Build a transaction using quote data from Hypersonic API
    ///
    /// # Arguments
    ///
    /// * `quote_data` - Quote data obtained from `/quote` endpoint
    ///
    /// # Returns
    ///
    /// Transaction data ready to be executed
    async fn build_transaction(&self, quote_data: &serde_json::Value) -> Result<Transaction, Box<dyn Error>> {
        let response = self.client
            .post("https://api.hypersonic.exchange/v1/build")
            .header("Content-Type", "application/json")
            .json(quote_data)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }
        
        let build_response: ApiResponse<BuildData> = response.json().await?;
        
        if !build_response.success {
            return Err(format!("Failed to build transaction: {:?}", build_response).into());
        }
        
        Ok(build_response.data.transaction)
    }
    
    /// STEP 3: Execute the swap transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction object from the `/build` endpoint
    /// * `provider` - Ethers provider
    /// * `wallet` - Ethers wallet/signer
    ///
    /// # Returns
    ///
    /// The transaction hash
    async fn execute_swap<M: Middleware + 'static>(&self, transaction: &Transaction, provider: Arc<M>, wallet: LocalWallet) -> Result<TxHash, Box<dyn Error>> {
        let client = SignerMiddleware::new(provider, wallet);
        
        // Parse tx data
        let to = transaction.to.parse::<Address>()?;
        let data = Bytes::from(hex::decode(&transaction.data[2..])?); // Remove "0x"
        let value = if transaction.value.is_empty() || transaction.value == "0" {
            U256::zero()
        } else {
            U256::from_dec_str(&transaction.value)?
        };
        
        // Build the tx request
        let tx_request = Eip1559TransactionRequest::new().to(to).data(data).value(value);
        let typed_tx: TypedTransaction = tx_request.into();
        
        // Send tx
        println!("Sending transaction...");
        let pending_tx = client.send_transaction(typed_tx, None).await?;
        
        println!("Transaction sent with hash: {}", pending_tx.tx_hash());
        
        // Wait for confirmation
        let receipt = pending_tx.await?;
        
        if let Some(r) = receipt {
            println!("Transaction confirmed in block {}", r.block_number.unwrap_or_default());
            Ok(r.transaction_hash)
        } else {
            Err("Receipt not available".into())
        }
    }
}

/// Example usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = HypersonicClient::new();
    
    // NOTICE: Uncomment and replace with your own provider/private key to execute the transaction
    /*
    let provider_url = "https://rpc.soniclabs.com";
    let provider = Provider::<Http>::try_from(provider_url)?;
    let provider = Arc::new(provider);
    let private_key = "0x0000000000000000000000000000000000000000000000000000000000000000"; // Replace with your private key
    let wallet = private_key.parse::<LocalWallet>()?;
    */

    // STEP 1: Get a quote
    let quote_request = QuoteRequest {
        chain_id: 146, // Sonic
        in_token: "0x0000000000000000000000000000000000000000".to_string(), // S on Sonic
        out_token: "0x29219dd400f2bf60e5a23d13be72b486d4038894".to_string(), // USDC.e on Sonic
        in_amount: "1000000000000000000".to_string(), // 1 S (18 decimals)
        slippage: 1, // 1% slippage tolerance
        referral_code: Some(0), // Optional
    };
    
    println!("Step 1: Getting quote...");
    let quote_data = match client.get_quote(quote_request).await {
        Ok(data) => {
            let _in_decimals = data["inDecimals"].as_i64().unwrap_or(18);
            let out_decimals = data["outDecimals"].as_i64().unwrap_or(18);
            let out_amount = data["outAmount"].as_str().unwrap_or("0");
            let _amount_f64 = out_amount.parse::<f64>().unwrap_or(0.0) / 10_f64.powi(out_decimals as i32);
            println!("QuoteData ready for encoding: {}", data);
            data
        },
        Err(e) => {
            eprintln!("Failed to get quote: {}", e);
            return Err(e);
        }
    };
    
    // STEP 2: Build transaction
    println!("Step 2: Building transaction...");
    let transaction = match client.build_transaction(&quote_data).await {
        Ok(tx) => {
            println!("Transaction ready for execution: {}", serde_json::to_string_pretty(&tx)?);
            tx
        },
        Err(e) => {
            eprintln!("Failed to build transaction: {}", e);
            return Err(e);
        }
    };
    
    // STEP 3: Execute swap
    println!("Step 3: Executing swap...");
    
    // NOTICE: Uncomment to execute the transaction
    /*
    match client.execute_swap(&transaction, provider, wallet).await {
        Ok(tx_hash) => {
            println!("Swap completed with transaction hash: {:?}", tx_hash);
        },
        Err(e) => {
            eprintln!("Failed to execute swap: {}", e);
            return Err(e);
        }
    }
    */
    
    Ok(())
}