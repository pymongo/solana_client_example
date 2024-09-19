import fetch, { RequestInit, Response } from 'node-fetch';
import { HttpsProxyAgent } from 'https-proxy-agent';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js'
import { AccountLayout } from '@solana/spl-token'

const agent = new HttpsProxyAgent(process.env.http_proxy!, {
    // headers: {
    //     "Origin": "https://app.phoenix.trade"
    // }
});
async function fetchWithProxy(url: string, options?: RequestInit): Promise<Response> {
    // Ensure headers is defined and is an object
    options = options || {};
    options.headers = {
        ...options.headers,
        "Origin": "https://app.phoenix.trade"
    };
    const optionsWithAgent: RequestInit = {
        ...options,
        agent
    };
    return fetch(url, optionsWithAgent);
}

const connection = new Connection("https://ellipsis-main-98a6.mainnet.rpcpool.com", {
    fetch: fetchWithProxy as any,
    // httpHeaders: {
    //     "Origin": "https://app.phoenix.trade"
    // },
    // wsEndpoint: "wss://ellipsis-main-98a6.mainnet.rpcpool.com",
    httpAgent: agent,
});

export function log(...optionalParams: any[]) {
    function getCallerInfo(): string {
        const stack = new Error().stack;
        if (!stack) {
            return '';
        }
        const stackLines = stack.split('\n');
        const callerLine = stackLines[3]; // Adjust index if needed based on your environment
        // const idx = callerLine.lastIndexOf("/", callerLine.lastIndexOf("/") - 1)
        const idx = callerLine.lastIndexOf("/")
        return callerLine.substring(idx);
    }    
    console.log(getCallerInfo(), ...optionalParams);
}

(async () => {
    log((await connection.getLatestBlockhash()).blockhash)
    const pool = new PublicKey("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE")
    const wsol = new PublicKey("EUuUbDcafPrmVTD5M6qoJAoyyNbihBhugADAxRMn5he9")
    const usdc = new PublicKey("2WLWEuKDgkDUccTpbwYp1GToYktiSB1cXvreHUwiSUVP")

    connection.getTokenAccountBalance(wsol)
    // connection.onSlotUpdate(async (slot) => {
    //     log(slot.slot);
    // })
    // connection.onLogs(pool, async (accountInfo) => {
    //     log(`pool change`);
    // });
    // connection.onAccountChange(pool, async (accountInfo) => {
    //     log(`pool change`);
    // });
    const subscriptionId = connection.onAccountChange(wsol, async (accountInfo) => {
        const accountData = AccountLayout.decode(accountInfo.data);
        const amount = Number(accountData.amount) / 1e9;
        log(`wsol: ${amount}`);
    });
    connection.onAccountChange(usdc, async (accountInfo) => {
        const accountData = AccountLayout.decode(accountInfo.data);
        const amount = Number(accountData.amount) / 1e6;
        log(`usdc: ${amount}`);
    });
    log("end")
})()
