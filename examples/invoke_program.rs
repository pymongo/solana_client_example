use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::Signer,
    transaction::Transaction,
    pubkey::Pubkey,
    instruction::{Instruction, AccountMeta},
};

fn main() {
    // Initialize the connection to the devnet cluster
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    let payer = solana_client_example::keypair();
    let program_id: Pubkey = "6jL67XKqEVWPBZEmMT8AuhTP4zJefpPs48BmV3JLxncR".parse().unwrap();

    // Create an instruction to call your program
    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(payer.pubkey(), true)],
        data: Vec::new(),
    };
    // Create a transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        connection.get_latest_blockhash().unwrap(),
    );
    // Sign and Send the transaction
    match connection.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent with signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
}
