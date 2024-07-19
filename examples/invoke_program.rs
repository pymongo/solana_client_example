use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
fn main() {
    let (client, payer) = solana_client_example::init();
    let program_id: Pubkey = std::env::var("hello_world").unwrap().parse().unwrap();

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
        client.get_latest_blockhash().unwrap(),
    );
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent with signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
}
