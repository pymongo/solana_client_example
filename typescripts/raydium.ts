import { Connection, ConnectionConfig, PublicKey } from '@solana/web3.js';
import { LIQUIDITY_STATE_LAYOUT_V4  } from '@raydium-io/raydium-sdk';
import { HttpsProxyAgent } from 'https-proxy-agent'; 
import fetch, { RequestInfo, RequestInit } from 'node-fetch';

const url = 'https://jupiter-fe.helius-rpc.com';

const proxyUrl = process.env.HTTP_PROXY || process.env.HTTPS_PROXY;
if (proxyUrl) {
    console.log("proxyUrl", proxyUrl)
}
const agent = proxyUrl ? new HttpsProxyAgent(proxyUrl) : undefined;

const customFetch = (input: RequestInfo, init?: RequestInit): Promise<Response> => {
    const headers = {
      ...init?.headers,
      "Origin": "https://jup.ag",
      // 'Authorization': `Bearer token`,
    };
    
    return fetch(input, {
      ...init,
      headers: headers,
      agent,
    });
};

const connectionConfig: ConnectionConfig = {
    fetch: customFetch,  // Pass the fetch function in the config
};
const connection = new Connection(url, connectionConfig);

async function fetchTokenBalance(publicKey){
    const tokenInfo = await connection.getTokenAccountBalance(publicKey);
    // console.log(tokenInfo)
    // const tokenAmount = tokenInfo.value.amount;
    // const tokenDecimals = tokenInfo.value.decimals;
    // return parseFloat(tokenAmount) / Math.pow(10, tokenDecimals);
    return tokenInfo.value.uiAmount
}

async function raydiumPoolPrice(poolAddr: PublicKey): Promise<number> {
    const info = await connection.getAccountInfo(new PublicKey(poolAddr), 'finalized');
    const poolState = LIQUIDITY_STATE_LAYOUT_V4.decode(info!.data);
    const base = await fetchTokenBalance(poolState.baseVault);
    const quote = await fetchTokenBalance(poolState.quoteVault);
    return quote! / base!
}

// tsc typescripts/raydium.ts && node typescripts/raydium.js 
(async () => {
    // const wen = new PublicKey("WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk")
    // const sol = new PublicKey("So11111111111111111111111111111111111111112")
    const wensol = new PublicKey("5WGx6mE9Xww3ocYzSenGVQMJLCVVwK7ePnYV6cXcpJtK")
    const solusdc = new PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")
    console.info("sol price", await raydiumPoolPrice(solusdc));
    console.info("wen price", await raydiumPoolPrice(wensol), "SOL");
})()
