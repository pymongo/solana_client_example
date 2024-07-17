use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
fn main() {
    dotenv::dotenv().unwrap();
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    let payer = solana_client_example::keypair();
    let program_id: Pubkey = std::env::var("PROGRAM_ID").unwrap().parse().unwrap();

    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(payer.pubkey(), true)],
        data: Vec::new(),
    };
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        // get_latest_blockhash expired in 2min
        connection.get_latest_blockhash().unwrap(),
    );
    match connection.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent with signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
}
