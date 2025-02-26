import axios from 'axios'
import { ethers } from 'ethers'

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
 * STEP 1: Get a quote from Hypersonic API
 * 
 * @param quoteRequest Quote request parameters
 * @returns Quote data with the best route for a swap
 */
async function getQuote(quoteRequest: QuoteRequest) {
    try {
        const res = await axios.post('https://api.hypersonic.exchange/v1/quote', quoteRequest, { headers: { 'Content-Type': 'application/json' } })
        if (!res.data.success) throw new Error('Failed to get quote: ' + JSON.stringify(res.data))
        return res.data.data
    } catch (e) {
        console.error('Error getting quote:', e)
        throw e
    }
}

/**
 * STEP 2: Build a transaction using the quote data from Hypersonic API
 * 
 * @param quoteData Quote data obtained from the `/quote` endpoint
 * @returns Transaction data ready to be executed
 */
export async function buildTransaction(quoteData: any) {
    try {
        const res = await axios.post('https://api.hypersonic.exchange/v1/build', quoteData, { headers: { 'Content-Type': 'application/json' } })
        if (!res.data.success) throw new Error('Failed to build transaction: ' + JSON.stringify(res.data))
        return res.data.data.transaction
    } catch (e) {
        console.log('Error building transaction:', e)
        throw e
    }
}

/**
 * STEP 3: Execute the swap using ethers.js
 * 
 * @param transaction Transaction object from the `/build` endpoint
 * @param provider An ethers provider
 * @param wallet An ethers signer with funds
 * @returns Transaction hash
 */
async function executeSwap(transaction: any, provider: ethers.Provider, wallet: ethers.Signer) {
    try {
        // Check if you need to approve the token first
        // This example assumes approval has been done already (Using native)

        // Create transaction object
        const txObject = {
            to: transaction.to,
            data: transaction.data,
            value: transaction.value,
            // Optional: you can specify gas parameters or leave them to be estimated
        }

        // Send the transaction
        const tx = await wallet.sendTransaction(txObject)
        console.log(`Transaction sent with hash: ${tx.hash}`)

        // Wait for confirmation
        const receipt = await tx.wait()
        if (receipt != null) console.log(`Transaction confirmed in block ${receipt.blockNumber}`)

        return tx.hash
    } catch (e) {
        console.error('Error executing swap:', e)
        throw e
    }
}

// Example usage
async function main() {
    // NOTICE: Uncomment and replace with your own provider/private key to execute the transaction
    /*
    const rpc = 'https://rpc.soniclabs.com'
    const privateKey = 'YOUR_PRIVATE_KEY'

    const provider = new ethers.JsonRpcProvider(rpc)
    const wallet = new ethers.Wallet(privateKey, provider)
    */

    // STEP 1: Get a quote
    const quoteRequest: QuoteRequest = {
        chainId: 146, // Sonic
        inToken: '0x0000000000000000000000000000000000000000', // S on Sonic
        outToken: '0x29219dd400f2bf60e5a23d13be72b486d4038894', // USDC.e on Sonic
        inAmount: '1000000000000000000', // 1 S (18 decimals)
        slippage: 1, // 1% slippage tolerance
        refCode: 0 // *Optional 
    }

    try {
        // Step 1: Get quote
        console.log('Step 1: Getting quote...')
        const quoteData = await getQuote(quoteRequest)
        console.log('QuoteData ready for encoding:', quoteData)

        // Step 2: Build transaction
        console.log('Step 2: Building transaction...')
        const transaction = await buildTransaction(quoteData)
        console.log('Transaction ready for execution:', transaction)

        // Step 3: Execute swap
        console.log('Step 3: Executing swap...')
        // This step is commented out to prevent accidental execution
        // NOTICE: Uncomment to execute the transaction
        /*
        const tx_hash = await executeSwap(transaction, provider, wallet)
        console.log(`Swap completed with transaction hash: ${tx_hash}`)
        */

    } catch (e) {
        console.log(e)
    }
}

// Run the example
main()