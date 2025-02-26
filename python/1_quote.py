#!/usr/bin/env python3
"""
Hypersonic API Quote Example
This script demonstrates how to get a quote from Hypersonic API.
"""

import requests
import json

def get_quote(quote_request):
    """
    Get a swap quote from Hypersonic API
    
    Args:
        quote_request (dict): Quote request parameters
        
    Returns:
        dict: Quote data with the best route for a swap
    
    Raises:
        Exception: If the API request fails
    """
    
    try:
        print("Getting quote...")
        response = requests.post('https://api.hypersonic.exchange/v1/quote', headers={'Content-Type': 'application/json'}, json=quote_request)
        response.raise_for_status()
        
        response_data = response.json()
        if not response_data.get("success"):
            raise Exception(f"Failed to get quote: {response_data}")
        
        return response_data["data"]
    
    except requests.exceptions.RequestException as e:
        print(f"Error getting quote: {e}")
        raise
    except Exception as e:
        print(f"Error processing quote: {e}")
        raise

def main():
    """Example usage of get_quote()"""

    quote_request = {
        "chainId": 146,  # Sonic
        "inToken": "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38",  # wS on Sonic
        "outToken": "0x29219dd400f2bf60e5a23d13be72b486d4038894",  # USDC.e on Sonic
        "inAmount": "1000000000000000000",  # 1 wS (18 decimals)
        "slippage": 1,  # 1% slippage tolerance
        # "refCode": 0  # *Optional 
        # If you want to charge a fee, you can register a refCode by following -> https://docs.hypersonic.exchange/referral
    }
    
    try:
        quote_data = get_quote(quote_request)
        print("--------------------------------------------------------")
        print(f"{json.dumps(quote_data)}")
        print("--------------------------------------------------------")
        print(f"Input Token: {quote_data['inToken']}")
        print(f"Output Token: {quote_data['outToken']}")
        print(f"Expected Output Amount: {quote_data['outAmount']}")
        print(f"Minimum Received: {quote_data['minReceived']}")
    
    except Exception as e:
        print(e)

# Run the example
main()