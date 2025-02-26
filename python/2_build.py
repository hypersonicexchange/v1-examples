#!/usr/bin/env python3
"""
Hypersonic API Build Tx Example
This script demonstrates how to build a transaction using the Hypersonic API.
"""

import requests
import json

def build_transaction(quote_data):
    """
    Build a transaction using the quote data from Hypersonic API
    
    Args:
        quote_data (dict): Quote data obtained from the `/quote` endpoint
        
    Returns:
        dict: Transaction data ready to be executed
    
    Raises:
        Exception: If the API request fails
    """

    try:
        print("Building transaction...")
        response = requests.post('https://api.hypersonic.exchange/v1/build', headers={'Content-Type': 'application/json'}, json=quote_data)
        response.raise_for_status()
        
        response_data = response.json()
        if not response_data.get("success"):
            raise Exception(f"Failed to build transaction: {response_data}")
        
        return response_data["data"]["transaction"]

    except requests.exceptions.RequestException as e:
        print(f"Error building transaction: {e}")
        raise
    except Exception as e:
        print(f"Error processing transaction build: {e}")
        raise

def main():
    """Example usage of build_transaction()"""
    # Sample quote data (obtained from `/quote` endpoint)
    sample_quote_data = {
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
                                            "multiHop": False
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
    }
    
    try:
        transaction = build_transaction(sample_quote_data)
        print("--------------------------------------------------------")
        print(f"{json.dumps(transaction, indent=2)}")
    
    except Exception as e:
        print(e)

# Run the example
main()