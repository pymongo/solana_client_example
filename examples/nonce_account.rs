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
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    let payer = solana_client_example::keypair();
    let nonce_account = solana_client_example::nonce_account();
    let nonce_account_data = client.get_account(&nonce_account.pubkey()).unwrap();
    let nonce_data = solana_rpc_client_nonce_utils::data_from_account(&nonce_account_data).unwrap();
    let block_hash = nonce_data.blockhash();

    let program_id: Pubkey = std::env::var("hello_world").unwrap().parse().unwrap();

    let advance_ix = solana_sdk::system_instruction::advance_nonce_account(
        &nonce_account.pubkey(),
        &payer.pubkey(),
    );
    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(payer.pubkey(), true)],
        data: Vec::new(),
    };
    let transaction = Transaction::new_signed_with_payer(
        &[advance_ix, instruction],
        Some(&payer.pubkey()),
        &[&payer],
        block_hash,
    );
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent with signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
}
