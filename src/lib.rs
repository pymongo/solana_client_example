use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};

pub fn init() -> (RpcClient, Keypair) {
    dotenv::dotenv().unwrap();
    std::env::set_var("RUST_BACKTRACE", "1");
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    (connection, keypair())
}

pub fn keypair() -> Keypair {
    let home = std::env::var("HOME").unwrap();
    let key_str = std::fs::read_to_string(format!("{home}/.config/solana/id.json")).unwrap();
    let key: Vec<u8> = serde_json::from_str(&key_str).unwrap();
    Keypair::from_bytes(&key).unwrap()
}

pub fn nonce_account() -> Keypair {
    let home = std::env::var("HOME").unwrap();
    let key_str =
        std::fs::read_to_string(format!("{home}/.config/solana/nonce-account.json")).unwrap();
    let key: Vec<u8> = serde_json::from_str(&key_str).unwrap();
    Keypair::from_bytes(&key).unwrap()
}
