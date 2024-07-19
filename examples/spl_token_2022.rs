use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token_2022::extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensionsOwned};
use spl_token_metadata_interface::state::TokenMetadata;
/* 只能用 2022 版本去设置 metadata 低于 2022 版本要用 mpl 方式设置 metadata
root@lb1:~# spl-token --program-id TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA create-token --enable-metadata
Creating token 8jvRBRbci3mi5ArFKAFhDfirHz812xRKJrC28J4d6fmY under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
Error: Program(IncorrectProgramId)
*/
fn main() {
    let (client, payer) = solana_client_example::init();
    // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
    println!("{}", spl_token::ID);
    // TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
    println!("{}", spl_token_2022::ID);
    // 用 spl-2022 最后一个指令mint+1的时候 估计是关联账户的问题会报错 invalid account data for instruction

    let program_id = spl_token_2022::ID;
    use spl_token_2022::instruction;
    // let program_id = spl_token::ID;
    // use spl_token::instruction;

    let mut extensions = Vec::new();
    extensions.push(ExtensionType::MetadataPointer);
    let space = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extensions)
        .unwrap() as u64;
    println!("space = {space}");
    // space 设置成 234 用 spl_token mint 会报错
    // let space = 82;

    let rent_lamports = client
        .get_minimum_balance_for_rent_exemption(space as usize)
        .unwrap();
    let mint = Keypair::new();
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent_lamports,
        space,
        &program_id,
    );
    let enable_metadata_ix = spl_token_2022::extension::metadata_pointer::instruction::initialize(
        &spl_token_2022::ID,
        &mint.pubkey(),
        Some(payer.pubkey()),
        // 如果 metadata_address 存储在另一个account就写另一个account地址，否则就token自身存储metadata
        Some(mint.pubkey()),
    )
    .unwrap();
    // enable_metadata_ix 必须在 init_mint 之前执行去消耗部分存储空间 预留82byte给mint
    let decimals = 0;
    let init_mint_ix =
        instruction::initialize_mint(&program_id, &mint.pubkey(), &payer.pubkey(), None, decimals)
            .unwrap();
    let blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, enable_metadata_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("mint pubkey={} {txhash}", mint.pubkey());

    let token_metadata = TokenMetadata {
        mint: mint.pubkey(),
        name: "USDT".to_string(),
        symbol: "USDT".to_string(),
        uri:
            "https://ipfs.io/ipfs/QmPDHYbztLwZAZj53XT8aRvZyQxZkkMkZHiVArjgJGVaBX"
                .to_string(),
        additional_metadata: vec![("whitepaper".to_string(), "https://tether.to/".to_string())],
        ..Default::default()
    };
    let account = client.get_account(&mint.pubkey()).unwrap();
    let mint_state =
        StateWithExtensionsOwned::<spl_token_2022::state::Mint>::unpack(account.data).unwrap();
    let account_lamports = account.lamports;
    let new_account_len = mint_state
        .try_get_new_account_len_for_variable_len_extension::<TokenMetadata>(&token_metadata)
        .unwrap();
    let new_rent_exempt_minimum = client
        .get_minimum_balance_for_rent_exemption(new_account_len)
        .unwrap();
    let additional_lamports = new_rent_exempt_minimum.saturating_sub(account_lamports);
    let mut instructions = vec![];
    if additional_lamports > 0 {
        instructions.push(system_instruction::transfer(
            &payer.pubkey(),
            &mint.pubkey(),
            additional_lamports,
        ));
    }
    instructions.push(spl_token_metadata_interface::instruction::initialize(
        &program_id,
        &mint.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
        token_metadata.name,
        token_metadata.symbol,
        token_metadata.uri,
    ));
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("metadata {txhash}");

    let token_account = if program_id == spl_token_2022::ID {
        let mint_extensions: Vec<ExtensionType> = mint_state.get_extension_types().unwrap();
        let mut required_extensions =
            ExtensionType::get_required_init_account_extensions(&mint_extensions);
        for extension_type in extensions.into_iter() {
            if !required_extensions.contains(&extension_type) {
                required_extensions.push(extension_type);
            }
        }
        let account = Keypair::new();
        let space = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(
            &required_extensions,
        )
        .unwrap();
        let mut instructions = vec![system_instruction::create_account(
            &payer.pubkey(),
            &account.pubkey(),
            client
                .get_minimum_balance_for_rent_exemption(space)
                .unwrap(),
            space as u64,
            &program_id,
        )];

        if required_extensions.contains(&ExtensionType::ImmutableOwner) {
            instructions.push(
                instruction::initialize_immutable_owner(&program_id, &account.pubkey()).unwrap(),
            )
        }

        instructions.push(
            instruction::initialize_account(
                &program_id,
                &account.pubkey(),
                &mint.pubkey(),
                &mint.pubkey(),
            )
            .unwrap(),
        );
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer.pubkey()),
            &[&payer, &account],
            blockhash,
        );
        let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
        println!("token_account pubkey={} {txhash}", account.pubkey());
        account.pubkey()
    } else {
        let token_account = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
        let ix = spl_associated_token_account::instruction::create_associated_token_account(
            // TokenAccount 租金的 payer
            &payer.pubkey(),
            // 要给谁创建 TokenAccount
            &payer.pubkey(),
            // token mint_addr
            &mint.pubkey(),
            &program_id,
        );
        let transaction =
            Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
        let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
        println!("token_account pubkey={token_account} {txhash}");
        token_account
    };

    // mint_authority 就是 token 发布者也就是 payer 我自己
    let ix = instruction::mint_to(
        &program_id,
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
