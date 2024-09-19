"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.log = log;
const node_fetch_1 = __importDefault(require("node-fetch"));
const https_proxy_agent_1 = require("https-proxy-agent");
const web3_js_1 = require("@solana/web3.js");
const spl_token_1 = require("@solana/spl-token");
const agent = new https_proxy_agent_1.HttpsProxyAgent(process.env.http_proxy, {
// headers: {
//     "Origin": "https://app.phoenix.trade"
// }
});
function fetchWithProxy(url, options) {
    return __awaiter(this, void 0, void 0, function* () {
        // Ensure headers is defined and is an object
        options = options || {};
        options.headers = Object.assign(Object.assign({}, options.headers), { "Origin": "https://app.phoenix.trade" });
        const optionsWithAgent = Object.assign(Object.assign({}, options), { agent });
        return (0, node_fetch_1.default)(url, optionsWithAgent);
    });
}
const connection = new web3_js_1.Connection("https://ellipsis-main-98a6.mainnet.rpcpool.com", {
    fetch: fetchWithProxy,
    // httpHeaders: {
    //     "Origin": "https://app.phoenix.trade"
    // },
    // wsEndpoint: "wss://ellipsis-main-98a6.mainnet.rpcpool.com",
    wsEndpoint: "wss://gayleen-v43l6p-fast-mainnet.helius-rpc.com",
    httpAgent: agent,
});
function log(...optionalParams) {
    function getCallerInfo() {
        const stack = new Error().stack;
        if (!stack) {
            return '';
        }
        const stackLines = stack.split('\n');
        const callerLine = stackLines[3]; // Adjust index if needed based on your environment
        // const idx = callerLine.lastIndexOf("/", callerLine.lastIndexOf("/") - 1)
        const idx = callerLine.lastIndexOf("/");
        return callerLine.substring(idx);
    }
    console.log(getCallerInfo(), ...optionalParams);
}
(() => __awaiter(void 0, void 0, void 0, function* () {
    log((yield connection.getLatestBlockhash()).blockhash);
    const pool = new web3_js_1.PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
    const wsol = new web3_js_1.PublicKey("DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz");
    const usdc = new web3_js_1.PublicKey("HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz");
    // connection.onSlotUpdate(async (slot) => {
    //     log(slot.slot);
    // })
    // connection.onLogs(pool, async (accountInfo) => {
    //     log(`pool change`);
    // });
    // connection.onAccountChange(pool, async (accountInfo) => {
    //     log(`pool change`);
    // });
    const subscriptionId = connection.onAccountChange(wsol, (accountInfo) => __awaiter(void 0, void 0, void 0, function* () {
        const accountData = spl_token_1.AccountLayout.decode(accountInfo.data);
        const amount = Number(accountData.amount) / 1e9;
        log(`wsol: ${amount}`);
    }));
    connection.onAccountChange(usdc, (accountInfo) => __awaiter(void 0, void 0, void 0, function* () {
        const accountData = spl_token_1.AccountLayout.decode(accountInfo.data);
        const amount = Number(accountData.amount) / 1e6;
        log(`usdc: ${amount}`);
    }));
    log("end");
}))();
