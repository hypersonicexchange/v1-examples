import axios from 'axios'

/**
 * Build a transaction using the quote data from Hypersonic API
 * 
 * @param quoteData Quote data obtained from the `/quote` endpoint
 * @returns Transaction data ready to be executed
 */
export async function buildTransaction(quoteData: any) {
    try {
        console.log('Building transaction...')
        const res = await axios.post('https://api.hypersonic.exchange/v1/build', quoteData, { headers: { 'Content-Type': 'application/json' } })
        if (!res.data.success) {
            throw new Error('Failed to build transaction: ' + JSON.stringify(res.data))
        }
        return res.data.data.transaction
    } catch (e) {
        console.log('Error building transaction:', e)
        throw e
    }
}

// Example usage of buildTransaction()
async function main() {
    // Sample quote data (obtained from `/quote` endpoint)
    const sampleQuoteData = {
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
    }

    try {
        const transaction = await buildTransaction(sampleQuoteData)
        console.log(`--------------------------------------------------------`)
        console.log(JSON.stringify(transaction, null, 2))
    } catch (e) {
        console.log(e)
    }
}

// Run the example
main()