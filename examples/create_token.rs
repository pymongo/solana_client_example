use solana_sdk::{
    signature::Keypair, signature::Signer, system_instruction, transaction::Transaction,
};
fn main() {
    let (client, payer) = solana_client_example::init();
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
    let decimals = 0;
    // let token_program = Keypair::new();
    // let extension_types = Vec::new();
    // let space = ExtensionType::try_calculate_account_len::<Mint>(&extension_types)?;
    let space = 82u64;
    let rent_lamports = client
        .get_minimum_balance_for_rent_exemption(space as usize)
        .unwrap();
    // let ix = system_instruction::create_account(
    //     &payer.pubkey(),
    //     &token_program.pubkey(),
    //     rent_lamports,
    //     space,
    //     &spl_token::ID,
    // );
    // let blockhash = client.get_latest_blockhash().unwrap();
    // let transaction = Transaction::new_signed_with_payer(
    //     &[ix],
    //     Some(&payer.pubkey()),
    //     &[&payer, &token_program],
    //     blockhash,
    // );
    // let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    // println!("token_program pubkey={} {txhash}", token_program.pubkey());

    let mint = Keypair::new();
    // let token_program = get_or_create_account("token_program", &spl_token::ID, 82);
    // let mint = get_or_create_account("mint", &token_program, mint_len);
    // mint_authority 就是 token 发布者也就是 payer 我自己
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
    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        client.get_latest_blockhash().unwrap(),
    );
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("mint pubkey={} {txhash}", mint.pubkey());
}
/*
use solana_sdk::{
    client::SyncClient,
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use spl_token::{instruction as token_instruction, state::Mint};
use metaplex_token_metadata::{instruction as metadata_instruction, state::Metadata};

fn main() {
    // Setup RPC client and keypairs
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();
    let mint = Keypair::new();

    // Create mint account
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();
    let create_mint_account_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    // Initialize mint
    let initialize_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        0, // 0 decimal places
    )
    .unwrap();

    // Create transaction and send
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;
    let tx = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );
    client.send_and_confirm_transaction(&tx).unwrap();

    // Create metadata account
    let metadata_seeds = &[
        b"metadata",
        &metadata_instruction::id().to_bytes(),
        &mint.pubkey().to_bytes(),
    ];
    let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &metadata_instruction::id());

    let create_metadata_account_ix = metadata_instruction::create_metadata_accounts(
        metadata_instruction::id(),
        metadata_pubkey,
        mint.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        "PEPE".to_string(),
        "https://example.com".to_string(),
        "".to_string(),
        None,
        1,
        true,
        true,
    );

    // Send transaction to create metadata
    let tx = Transaction::new_signed_with_payer(
        &[create_metadata_account_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    client.send_and_confirm_transaction(&tx).unwrap();

    println!("Token created with mint address: {}", mint.pubkey());
}
*/
