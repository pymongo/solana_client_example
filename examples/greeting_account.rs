use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::client_error::ClientErrorKind;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcError;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    system_instruction::create_account_with_seed,
    transaction::Transaction,
};
/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}
fn main() {
    dotenv::dotenv().unwrap();
    std::env::set_var("RUST_BACKTRACE", "1");
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    let payer = solana_client_example::keypair();
    let program_id: Pubkey = std::env::var("greeting_hello").unwrap().parse().unwrap();
    let seed = "hello_rust";
    let program_data_pubkey = Pubkey::create_with_seed(&payer.pubkey(), seed, &program_id).unwrap();

    match client.get_account(&program_data_pubkey) {
        Ok(_acc) => {}
        Err(err) => {
            match err.kind {
                ClientErrorKind::RpcError(err) => {
                    match err {
                        RpcError::ForUser(err) => {
                            // AccountNotFound: pubkey=65hH53vj34oG6DGdFePftj7mMeiM3uQtvPLQNPUnSAkV
                            eprintln!("{err}");
                            let data = GreetingAccount { counter: 0 };
                            let mut buffer = Vec::new();
                            data.serialize(&mut buffer).unwrap();
                            let size = buffer.len();
                            let lamports =
                                client.get_minimum_balance_for_rent_exemption(size).unwrap();
                            let ix = create_account_with_seed(
                                &payer.pubkey(),
                                &program_data_pubkey,
                                &payer.pubkey(),
                                seed,
                                lamports,
                                size as u64,
                                &program_id,
                            );
                            let transaction = Transaction::new_signed_with_payer(
                                &[ix],
                                Some(&payer.pubkey()),
                                &[&payer],
                                client.get_latest_blockhash().unwrap(),
                            );
                            let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
                            println!("create_account_with_seed {txhash}");
                        }
                        _ => panic!("{err}"),
                    }
                }
                _ => panic!("{err}"),
            }
        }
    }

    let query_account = || {
        let data = client.get_account_data(&program_data_pubkey).unwrap();
        let data = GreetingAccount::deserialize(&mut data.as_ref()).unwrap();
        println!("counter = {}", data.counter);
    };
    query_account();
    let instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(program_data_pubkey, false)],
        data: Vec::new(),
    };
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().unwrap(),
    );
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent with signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
    query_account();
}
