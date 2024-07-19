use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
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

    // mpl_token_metadata 库用的 solana-sdk 版本是 1.18 太旧了 版本冲突
    // let metadata_program_id = mpl_token_metadata::ID;
    let metadata_program_id = mpl_token_metadata::ID;
    let metadata_seeds = &[
        b"metadata",
        metadata_program_id.as_ref(),
        &mint.pubkey().to_bytes(),
    ];
    let (metadata_pda, _metadata_bump) =
        Pubkey::find_program_address(metadata_seeds, &metadata_program_id);
    let metadata_args = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: mint.pubkey(),
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        // bool: is_signer
        update_authority: (payer.pubkey(), true),
        system_program: solana_sdk::system_program::ID,
        rent: None,
    };
    let ix = metadata_args.instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: "Solana Summer NFT".to_string(),
            symbol: "sol_summer".to_string(),
            /*
            {
                "name": "Solana Summer",
                "description": "Forget winter. The sun never sets on Solana.",
                "image": "https://ipfs.io/ipfs/QmW5wvF52B5yL7QqRM6MvofT3huVEe19VnLLbmC1QR4g3h",
                "external_url": "https://solanasummer.click/"
            }
            */
            uri: "https://ipfs.io/ipfs/QmPDHYbztLwZAZj53XT8aRvZyQxZkkMkZHiVArjgJGVaBX".to_string(),
            // uri "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB/logo.svg",
            // 用于定义当 NFT 被出售时需要支付的佣金费用。例如：如果 seller_fee_basis_points 设置为 500，这表示销售费用为 5%
            seller_fee_basis_points: 500,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: false,
        collection_details: None,
    });
    // let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    // transaction.sign(&[&payer, &mint], blockhash);
    // let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    let transaction =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("metadata pubkey={metadata_pda} {txhash}");

    let token_account = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    let ix = spl_associated_token_account::instruction::create_associated_token_account(
        // TokenAccount 租金的 payer
        &payer.pubkey(),
        // 要给谁创建 TokenAccount
        &payer.pubkey(),
        // token mint_addr
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
