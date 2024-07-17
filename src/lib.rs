use solana_sdk::signature::Keypair;

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
