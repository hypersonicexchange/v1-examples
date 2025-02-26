import axios from 'axios'

/**
 * Hypersonic API interface for quote endpoint
 */
interface QuoteRequest {
    chainId: number
    inToken: string
    outToken: string
    inAmount: string
    slippage: number
    refCode?: number
    // If you want to charge a fee, you can register a refCode by following -> https://docs.hypersonic.exchange/referral
}

/**
 * Get a swap quote from Hypersonic API
 * 
 * @param quoteRequest Quote request parameters
 * @returns Quote data with the best route for a swap
 */
export async function getQuote(quoteRequest: QuoteRequest) {
    try {
        console.log('Getting quote...')
        const res = await axios.post('https://api.hypersonic.exchange/v1/quote', quoteRequest, { headers: { 'Content-Type': 'application/json' } })
        if (!res.data.success) {
            throw new Error('Failed to get quote: ' + JSON.stringify(res.data))
        }
        return res.data.data
    } catch (e) {
        console.error('Error getting quote:', e)
        throw e
    }
}

// Example usage of getQuote()
async function main() {
    const quoteRequest: QuoteRequest = {
        chainId: 146, // Sonic
        inToken: '0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38', // wS on Sonic
        outToken: '0x29219dd400f2bf60e5a23d13be72b486d4038894', // USDC.e on Sonic
        inAmount: '1000000000000000000', // 1 wS (18 decimals)
        slippage: 1, // 1% slippage tolerance
        refCode: 0 // *Optional 
    }

    try {
        const quoteData = await getQuote(quoteRequest)
        console.log(`--------------------------------------------------------`)
        console.log(JSON.stringify(quoteData, null))
        console.log(`--------------------------------------------------------`)
        console.log(`Input Token: ${quoteData.inToken}`)
        console.log(`Output Token: ${quoteData.outToken}`)
        console.log(`Expected Output Amount: ${quoteData.outAmount}`)
        console.log(`Minimum Received: ${quoteData.minReceived}`)
    } catch (e) {
        console.log(e)
    }
}

// Run the example
main()