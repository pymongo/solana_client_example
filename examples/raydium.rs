use solana_client::client_error::reqwest;

fn main() {
    let rpc_url = "https://jupiter-fe.helius-rpc.com";
    let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_static("Bearer xxx"));
    headers.insert(
        reqwest::header::ORIGIN,
        reqwest::header::HeaderValue::from_static("https://jup.ag"),
    );
    let reqwest_client = solana_client::client_error::reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap();
    let http_client =
        solana_rpc_client::http_sender::HttpSender::new_with_client(rpc_url, reqwest_client);
    let client = solana_client::rpc_client::RpcClient::new_sender(
        http_client,
        solana_rpc_client::rpc_client::RpcClientConfig::default(),
    );
    let raydiumv4_addr = solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
    
    let (_sub, mut rx) = solana_pubsub_client::pubsub_client::PubsubClient::program_subscribe("wss://zeta-zeta-61e4.mainnet.rpcpool.com/whirligig", &raydiumv4_addr, None).unwrap();
    loop {
        let msg = rx.recv().unwrap();
        dbg!(msg);
    }
}
