use solana_sdk::{
    signature::Keypair, signature::Signer, system_instruction, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
// use mpl_token_metadata::instructions::CreateMetadataAccountV3;
fn main() {
    let (client, payer) = solana_client_example::init();
    let decimals = 0;
    let space = 82u64;
    let rent_lamports = client
        .get_minimum_balance_for_rent_exemption(space as usize)
        .unwrap();

    let mint = Keypair::new();
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent_lamports,
        space,
        &spl_token::id(),
    );
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )
    .unwrap();
    let blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("mint pubkey={} {txhash}", mint.pubkey());

    let token_account = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    let ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &token_account,
        &mint.pubkey(),
        &spl_token::ID,
    );
    let transaction =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("token_account pubkey={token_account} {txhash}");

    // mint_authority 就是 token 发布者也就是 payer 我自己
    let ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &token_account,
        &payer.pubkey(),
        &[&payer.pubkey()],
        1,
    )
    .unwrap();
    let transaction =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("mint {txhash}");
}

/*
let seed_prefix = "token001";
let get_or_create_account = |seed: &str, owner: &Pubkey, size: u64| {
    let seed = format!("{seed_prefix}_{seed}");
    let pubkey = Pubkey::create_with_seed(&payer.pubkey(), &seed, &owner).unwrap();
    let mut account_not_found = false;
    match client.get_account(&pubkey) {
        Ok(_) => {}
        Err(err) => {
            match err.kind {
                ClientErrorKind::RpcError(RpcError::ForUser(err)) => {
                    account_not_found = true;
                }
                _ => {
                    panic!("{err}");
                }
            }
        }
    }
    if !account_not_found {
        return pubkey;
    }
    let lamports = client
        .get_minimum_balance_for_rent_exemption(size as usize)
        .unwrap();
    let ix = system_instruction::create_account_with_seed(
        &payer.pubkey(),
        &pubkey,
        &payer.pubkey(),
        &seed,
        lamports,
        size as u64,
        &owner,
    );
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().unwrap(),
    );
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("seed={seed} account_pubkey={pubkey} txhash={txhash}");
    pubkey
};
*/
