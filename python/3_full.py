#!/usr/bin/env python3
"""
Hypersonic API Integration Example
This script demonstrates the full flow of integrating Hypersonic API:
1. Get a quote
2. Build a transaction
3. Execute the swap (with web3.py)
"""

import requests
import json
from web3 import Web3

"""
Hypersonic API interface for quote endpoint
"""
# Required parameters:
# - chainId: Network ID for the chain
# - inToken: Input token address
# - outToken: Output token address
# - inAmount: Input amount in wei
# - slippage: Slippage tolerance in percentage
# - refCode: Optional referral code

"""
STEP 1: Get a quote from Hypersonic API

Args:
    quote_request (dict): Quote request parameters
    
Returns:
    dict: Quote data with the best route for a swap
"""
def get_quote(quote_request):
    try:
        res = requests.post('https://api.hypersonic.exchange/v1/quote', headers={'Content-Type': 'application/json'}, json=quote_request)
        res.raise_for_status()
        
        if not res.json().get('success'):
            raise Exception('Failed to get quote: ' + json.dumps(res.json()))
        
        return res.json()['data']
    except Exception as e:
        print('Error getting quote:', e)
        raise e

"""
STEP 2: Build a transaction using the quote data from Hypersonic API

Args:
    quote_data (dict): Quote data obtained from the `/quote` endpoint
    
Returns:
    dict: Transaction data ready to be executed
"""
def build_transaction(quote_data):
    try:
        res = requests.post('https://api.hypersonic.exchange/v1/build', headers={'Content-Type': 'application/json'}, json=quote_data)
        res.raise_for_status()
        
        if not res.json().get('success'):
            raise Exception('Failed to build transaction: ' + json.dumps(res.json()))
        
        return res.json()['data']['transaction']
    except Exception as e:
        print('Error building transaction:', e)
        raise e

"""
STEP 3: Execute the swap using web3.py

Args:
    transaction (dict): Transaction object from the `/build` endpoint
    provider (Web3): Web3 provider
    wallet (dict): Wallet info with address and private_key
    
Returns:
    str: Transaction hash
"""
def execute_swap(transaction, web3, wallet):
    try:
        # Check if you need to approve the token first
        # This example assumes approval has been done already (Using native)

        # Create transaction object
        tx_params = {
            'from': wallet['address'],
            'to': transaction['to'],
            'data': transaction['data'],
            'value': int(transaction['value']) if transaction['value'] else 0,
            # Optional: specify gas parameters or leave them to be estimated
        }

        # Sign & send transaction
        signed_tx = web3.eth.account.sign_transaction(tx_params, wallet['private_key'])
        tx_hash = web3.eth.send_raw_transaction(signed_tx.rawTransaction)
        print(f"Transaction sent with hash: {web3.to_hex(tx_hash)}")

        # Wait for confirmation
        receipt = web3.eth.wait_for_transaction_receipt(tx_hash)
        print(f"Transaction confirmed in block {receipt['blockNumber']}")

        return web3.to_hex(tx_hash)
    except Exception as e:
        print('Error executing swap:', e)
        raise e

# Example usage
def main():
    # NOTICE: Uncomment and replace with your own provider/private key to execute the transaction
    """
    rpc = 'https://rpc.soniclabs.com'
    private_key = 'YOUR_PRIVATE_KEY'

    web3 = Web3(Web3.HTTPProvider(rpc))
    
    wallet = {
        'address': web3.eth.account.from_key(private_key).address,
        'private_key': private_key
    }
    """

    # STEP 1: Get a quote
    quote_request = {
        "chainId": 146,  # Sonic
        "inToken": "0x0000000000000000000000000000000000000000",  # S on Sonic
        "outToken": "0x29219dd400f2bf60e5a23d13be72b486d4038894",  # USDC.e on Sonic
        "inAmount": "1000000000000000000",  # 1 S (18 decimals)
        "slippage": 1,  # 1% slippage tolerance
        "refCode": 0  # *Optional 
    }

    try:
        # Step 1: Get quote
        print('Step 1: Getting quote...')
        quote_data = get_quote(quote_request)
        print('QuoteData ready for encoding:', quote_data)

        # Step 2: Build transaction
        print('Step 2: Building transaction...')
        transaction = build_transaction(quote_data)
        print('Transaction ready for execution:', transaction)

        # Step 3: Execute swap
        print('Step 3: Executing swap...')
        # This step is commented out to prevent accidental execution
        # NOTICE: Uncomment to execute the transaction
        """
        tx_hash = execute_swap(transaction, web3, wallet)
        print(f"Swap completed with transaction hash: {tx_hash}")
        """

    except Exception as e:
        print(e)

# Run the example
main()