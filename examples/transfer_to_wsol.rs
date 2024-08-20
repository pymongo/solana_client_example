use solana_client::rpc_client::RpcClient;
use solana_sdk::{hash::Hash, program_pack::Pack, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

const WSOL: Pubkey = solana_sdk::pubkey!("So11111111111111111111111111111111111111112");
fn main() {
    let (client, payer) = solana_client_example::init();
    let token_account =
        spl_associated_token_account::get_associated_token_address(&payer.pubkey(), &WSOL);
    let latest_block = client.get_latest_blockhash().unwrap();
    // println!("wsol token_account = {}", token_account);
    get_wsol_balance(&client, &token_account, &payer, latest_block);

    let amount = 1_000_000;
    let instruction = solana_sdk::system_instruction::transfer(&payer.pubkey(), &token_account, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        latest_block,
    );
    client.send_and_confirm_transaction(&transaction).unwrap();
    println!("after transfer {amount}");
    get_wsol_balance(&client, &token_account, &payer, latest_block);

    let ix = spl_token::instruction::sync_native(&spl_token::ID, &token_account).unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_block,
    );
    client.send_and_confirm_transaction(&transaction).unwrap();
    println!("after sync_native {amount}");
    get_wsol_balance(&client, &token_account, &payer, latest_block);
    println!();

    // code: -32002, message: "Transaction simulation failed: Error processing Instruction 0: custom program error: 0xa", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(InstructionError(0, Custom(10))),
    // logs: Some(["Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]", "Program log: Instruction: MintTo", "Program log: Error: Instruction does not support native tokens"
    // let ix = spl_token::instruction::mint_to(&spl_token::ID, &WSOL, &token_account, &payer.pubkey(), &[], amount).unwrap();
    // let transaction = Transaction::new_signed_with_payer(
    //     &[ix],
    //     Some(&payer.pubkey()),
    //     &[&payer],
    //     latest_block,
    // );
    // client.send_and_confirm_transaction(&transaction).unwrap();
    // println!("after mint_to(deposit) {amount}");
    // get_wsol_balance(&client, &token_account, &payer, latest_block);


    // solana源码中关于burn指令的文档:
    // Burns tokens by removing them from an account. `Burn` does not support accounts associated with the native mint, use `CloseAccount` instead.
    let ix = spl_token::instruction::burn(&spl_token::ID, &token_account, &WSOL, &payer.pubkey(), &[], amount).unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_block,
    );
    client.send_and_confirm_transaction(&transaction).unwrap();
    println!("after burn(deposit) {amount}");
    get_wsol_balance(&client, &token_account, &payer, latest_block);    
}

fn get_wsol_balance(client: &RpcClient, token_account: &Pubkey, payer: &Keypair, latest_block: Hash)  {
    let account = match client.get_account(&token_account) {
        Ok(acc) => acc,
        Err(err) => match err.kind {
            solana_client::client_error::ClientErrorKind::RpcError(
                solana_client::rpc_request::RpcError::ForUser(err),
            ) => {
                println!("{}: {err}", line!());
                let create_account_instr =
                    spl_associated_token_account::instruction::create_associated_token_account(
                        &payer.pubkey(),
                        &token_account,
                        &WSOL,
                        &spl_token::ID,
                    );
                let mut transaction =
                    Transaction::new_with_payer(&[create_account_instr], Some(&payer.pubkey()));
                transaction.sign(&[&payer], latest_block);
                client.send_and_confirm_transaction(&transaction).unwrap();
                client.get_account(&token_account).unwrap()
            }
            _ => {
                panic!("{err}");
            }
        }  
    };
    let account_state = spl_token::state::Account::unpack(&account.data).unwrap();
    let sol = account_state.amount as f64 / 1e9;
    println!("wsol = {sol}, {} lamports", account_state.amount);
}
