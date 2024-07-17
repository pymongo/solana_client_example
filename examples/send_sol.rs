use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};
fn main() {
    dotenv::dotenv().unwrap();
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    let payer = solana_client_example::keypair();
    let to: Pubkey = std::env::var("to").unwrap().parse().unwrap();
    // 0.001 SOL
    let instruction = solana_sdk::system_instruction::transfer(&payer.pubkey(), &to, 1_000_000);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        connection.get_latest_blockhash().unwrap(),
    );
    match connection.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent with signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
}
