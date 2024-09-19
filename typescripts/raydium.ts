import { Connection, ConnectionConfig, PublicKey } from '@solana/web3.js';
import { LIQUIDITY_STATE_LAYOUT_V4, ClmmPoolInfo } from '@raydium-io/raydium-sdk';
import {
    CurrencyAmount,
    Liquidity,
    Token,
    Percent,
    jsonInfo2PoolKeys,
    LiquidityPoolKeys,
    TokenAmount,
    Currency,
    ApiClmmPoolsItem,
  } from "@raydium-io/raydium-sdk";
import { HttpsProxyAgent } from 'https-proxy-agent'; 
import { Agent } from 'https'
import assert from "assert";
// import fetch from 'node-fetch';

// import fetch, { RequestInfo, RequestInit } from 'node-fetch';
// export function solana_connection(): Connection {
//     const url = 'https://jupiter-fe.helius-rpc.com';
//     const proxyUrl = process.env.HTTP_PROXY || process.env.HTTPS_PROXY;
//     if (proxyUrl) {
//         console.log("proxyUrl", proxyUrl)
//     }
//     const agent = proxyUrl ? new HttpsProxyAgent(proxyUrl) : undefined;
    
//     const customFetch = (input: RequestInfo, init?: RequestInit): Promise<Response> => {
//         const headers = {
//           ...init?.headers,
//           "Origin": "https://jup.ag",
//           // 'Authorization': `Bearer token`,
//         };
        
//         return fetch(input, {
//           ...init,
//           headers: headers,
//           agent,
//         });
//     };
    
//     const connectionConfig: ConnectionConfig = {
//         fetch: customFetch,  // Pass the fetch function in the config
//     };
//     return new Connection(url, connectionConfig)
// }

export function solana_connection(): Connection {
    const proxyUrl = process.env.HTTPS_PROXY || process.env.HTTP_PROXY;
    console.log("proxyUrl", proxyUrl)
    const agent = new HttpsProxyAgent(proxyUrl!);
    const connectionConfig: ConnectionConfig = {
        httpHeaders: {
            "Origin": "https://app.phoenix.trade"
        },
        wsEndpoint: "wss://ellipsis-main-98a6.mainnet.rpcpool.com",
        httpAgent: agent as Agent,
        // fetch: async (input, options) => {
        //     const processedInput =
        //     typeof input === 'string' && input.slice(0, 2) === '//'
        //       ? 'https:' + input
        //       : input;    
        
        //     const result = await fetch(processedInput, {
        //       ...options,
        //       agent: agent,
        //     });
        //     return result;
        // },
    }
    return new Connection("https://ellipsis-main-98a6.mainnet.rpcpool.com", connectionConfig)
}

const connection = solana_connection()
async function fetchTokenBalance(publicKey: PublicKey){
    const tokenInfo = await connection.getTokenAccountBalance(publicKey);
    // console.log(tokenInfo)
    // const tokenAmount = tokenInfo.value.amount;
    // const tokenDecimals = tokenInfo.value.decimals;
    // return parseFloat(tokenAmount) / Math.pow(10, tokenDecimals);
    return tokenInfo.value.uiAmount
}

async function raydiumV4PoolPrice(poolAddr: PublicKey): Promise<number> {
    const info = await connection.getAccountInfo(new PublicKey(poolAddr), 'finalized');
    // https://github.com/raydium-io/raydium-clmm/blob/master/programs/amm/src/states/pool.rs#L58
    const poolState = LIQUIDITY_STATE_LAYOUT_V4.decode(info!.data);
    console.info(poolState.baseVault)
    const base = await fetchTokenBalance(poolState.baseVault);
    console.info(poolState.quoteVault)
    const quote = await fetchTokenBalance(poolState.quoteVault);
    return quote! / base!
}

// tsc typescripts/raydium.ts && node typescripts/raydium.js 
(async () => {
    // const wen = new PublicKey("WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk")
    // const sol = new PublicKey("So11111111111111111111111111111111111111112")
    const wensol = new PublicKey("5WGx6mE9Xww3ocYzSenGVQMJLCVVwK7ePnYV6cXcpJtK")
    // raydium内流动性最好openbook共享流动性, TVL=5m (最大池子orca的SOL/USDC TVL=15m)
    const solusdc_fee_25bp = new PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")
    // fee 1bp 的交易量比 上述 25bp 的高很多 Raydium Concentrated Liquidity
    const solusdc_fee_1bp = new PublicKey("8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj")
    console.info("sol price", await raydiumV4PoolPrice(solusdc_fee_25bp));
    console.info("sol price", await raydiumV4PoolPrice(solusdc_fee_1bp));
    console.info("wen price", await raydiumV4PoolPrice(wensol), "SOL");

    // const targetPool = await formatAmmKeysById(input.targetPool);
    // assert(targetPool, "Invalid targetPool");
    // const poolKeys = jsonInfo2PoolKeys(targetPool) as LiquidityPoolKeys;
})
