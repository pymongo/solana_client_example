use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer_checked;
use std::env;
/// ## how to get USDC
/// https://faucet.circle.com/ only support solana devnet
/// https://devnet.jup.ag/swap/SOL-USDC use rpc url https://devnet.jup.ag/swap/SOL-USDC
/// ## USDC tx
/// https://solscan.io/tx/5Y6vDu6JJpBThSGc6QuZHZCnZyJqbHfHnjAuR7qsrZJEoChMdtQkUhu2SfWkUR6hwZCt9Vwxf4nogTKUruGSiyZG?cluster=devnet
/// https://explorer.solana.com/tx/5Y6vDu6JJpBThSGc6QuZHZCnZyJqbHfHnjAuR7qsrZJEoChMdtQkUhu2SfWkUR6hwZCt9Vwxf4nogTKUruGSiyZG?cluster=devnet
/// USDC addr 4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
fn main() {
    dotenv::dotenv().unwrap();
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    let from_keypair: Keypair = solana_client_example::keypair();

    let to_addr: Pubkey = env::var("TO").unwrap().parse().unwrap();
    let usdc_mint: Pubkey = env::var("USDC_MINT").unwrap().parse().unwrap();

    let lamports_to_send = 1_000_000; // Amount to send (1 USDC in this case, as USDC has 6 decimals)

    let from_token_account = get_associated_token_address(&from_keypair.pubkey(), &usdc_mint);
    let to_token_account = get_associated_token_address(&to_addr, &usdc_mint);

    let latest_block = client.get_latest_blockhash().unwrap();

    // assert from_token_account exist
    // Create the associated token account for the receiver if it doesn't exist
    if client.get_account(&to_token_account).is_err() {
        let create_account_instr =
            spl_associated_token_account::instruction::create_associated_token_account(
                &from_keypair.pubkey(),
                &to_addr,
                &usdc_mint,
                &spl_token::ID,
            );
        let mut transaction =
            Transaction::new_with_payer(&[create_account_instr], Some(&from_keypair.pubkey()));
        transaction.sign(&[&from_keypair], latest_block);
        client.send_and_confirm_transaction(&transaction).unwrap();
    }

    // Create transfer instruction
    let transfer_instr = transfer_checked(
        &spl_token::ID,
        &from_token_account,
        &usdc_mint,
        &to_token_account,
        &from_keypair.pubkey(),
        &[],
        lamports_to_send,
        6, // USDC has 6 decimals
    )
    .unwrap();

    let mut transaction =
        Transaction::new_with_payer(&[transfer_instr], Some(&from_keypair.pubkey()));
    transaction.sign(&[&from_keypair], latest_block);

    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction hash: {}", txhash);
}
