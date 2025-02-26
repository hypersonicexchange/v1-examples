<h1 align="left" style="display: flex; align-items: center;">
  <img src="https://hypersonic.exchange/brand/logo.svg" alt="Hypersonic" height="28px">
  &nbsp;<span>Hypersonic V1 Examples</span>
</h1>

[![Hypersonic](https://img.shields.io/static/v1?label=&message=Hypersonic&color=grey&logo=ethereum&logoColor=white)](https://hypersonic.exchange)
[![Docs](https://img.shields.io/badge/Docs-%F0%9F%93%84-blue)](https://docs.hypersonic.exchange)
[![API](https://img.shields.io/badge/API-%F0%9F%93%84-green)](https://docs.hypersonic.exchange/api-reference)

Integrate Hypersonic Smart Order Routing API not only enables to offer your users the best swap route, you can also turn this into a stream of income. For every swap made via your referral code, you can charge a fee on the positive slippage instantly paid in the output token. [Learn more about integrator fees](https://docs.hypersonic.exchange/referral).

## Get started with Hypersonic API

- [Node](https://docs.hypersonic.exchange/integrate/node/introduction)
- [Python](https://docs.hypersonic.exchange/integrate/python/introduction)
- [Rust](https://docs.hypersonic.exchange/integrate/rust/introduction)

## Examples

| Lang | Code snippet | Description |
|--------|-------|-------------|
| Node |  [`node/src/quote.ts`](./node/src/1_quote.ts) | Get a `quoteData` including the best route for a swap. |
| Node |  [`node/src/build.ts`](./node/src/2_build.ts) | Get an encoded tx to be executed from a `quoteData`. |
| Node |  [`node/src/full.ts`](./node/src/3_full.ts) | Full example demonstrating a complete usage w/ Node. |
| Python |  [`python/src/quote.py`](./python/src/1_quote.py) | Get a `quoteData` including the best route for a swap. |
| Python |  [`python/src/build.py`](./python/src/2_build.py) | Get an encoded tx to be executed from a `quoteData`. |
| Python |  [`python/src/full.py`](./python/src/3_full.py) | Full example demonstrating a complete usage w/ Python. |
| Rust |  [`rust/src/quote.rs`](./rust/src/1_quote.rs) | Get a `quoteData` including the best route for a swap. |
| Rust |  [`rust/src/build.rs`](./rust/src/2_build.rs) | Get an encoded tx to be executed from a `quoteData`. |
| Rust |  [`rust/src/full.rs`](./rust/src/3_full.rs) | Full example demonstrating a complete usage w/ Rust. |

## Integration Flow

- 1/ **Quote**: Call `/quote` endpoint to get the best route for your swap
- 2/ **Build**: Pass the quote data to the `/build` endpoint to get an encoded tx
- 3/ **Execute**: Send the tx to the blockchain using your preferred Web3 library

### Simple example using curl

**1. Get a Quote**

```
curl -X POST https://api.hypersonic.exchange/v1/quote \
-H "Content-Type: application/json" \
-d '{
  "chainId": 146, // SONIC CHAINID
  "inToken": "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38", // wS
  "outToken": "0x29219dd400f2bf60e5a23d13be72b486d4038894", // USDC.e
  "inAmount": "1000000000000000000", // 1 wS
  "slippage": 1, // 1%
  "refCode": 0 // *Optional 
}'
```

**2. Build a Transaction**

```
curl -X POST https://api.hypersonic.exchange/v1/build \
-H "Content-Type: application/json" \
-d '<quote_response_data>'
```

## Need help?

- Refer to [API reference](https://docs.hypersonic.exchange/api-reference) for detailed documentation.
- Reach out on [Telegram](https://t.me/hypersonicexchange) for any help or specific code snippets.
