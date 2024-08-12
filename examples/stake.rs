use solana_sdk::stake;
use solana_sdk::transaction::Transaction;
use solana_sdk::{signature::Keypair, signer::Signer};

fn main() {
    let (client, payer) = solana_client_example::init();
    let blockhash = client.get_latest_blockhash().unwrap();

    let staker = if let Ok(json) = std::fs::read_to_string("temp/stake_account_keypair.json") {
        let bytes: Vec<_> = serde_json::from_str(&json).unwrap();
        Keypair::from_bytes(&bytes).unwrap()
    } else {
        let staker = Keypair::new();
        let json = serde_json::to_string(&staker.to_bytes().to_vec()).unwrap();
        std::fs::write("temp/stake_account_keypair.json", json).unwrap();
        let authorized = stake::state::Authorized {
            staker: staker.pubkey(),
            withdrawer: staker.pubkey(),
        };
        let lockup = stake::state::Lockup {
            unix_timestamp: unsafe { libc::time(std::ptr::null_mut()) },
            epoch: 0,
            custodian: staker.pubkey(),
        };
        let lamports = 10u64.pow(9);
        stake::program::id();
        let ixs = stake::instruction::create_account(
            &payer.pubkey(),
            &staker.pubkey(),
            &authorized,
            &lockup,
            lamports,
        );
        let transaction =
            Transaction::new_signed_with_payer(&ixs, Some(&payer.pubkey()), &[&staker], blockhash);
        let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
        println!("{txhash}");
        staker
    };
    println!("staker pubkey {}", staker.pubkey());

    /*
    Identity                                      Vote Account                            Commission  Last Vote        Root Slot     Skip Rate  Credits  Version            Active Stake
    DDnNKBaqswMQhTR5saBkUdfg4qFBVLqony7c7vfYoUCz  F2UsSsRHezY1U4h8FWMmWHkgyVd8r5hVVPNXViod9ZnJ  100%  317577293 ( -1)  317577262 ( -1)    -      912484  1.18.21         3.997717120 SOL (0.00%)
    */
    mod validators {
        solana_sdk::declare_id!("F2UsSsRHezY1U4h8FWMmWHkgyVd8r5hVVPNXViod9ZnJ");
    }
    // KeypairPubkeyMismatch
    let ix = stake::instruction::delegate_stake(
        &staker.pubkey(),
        &staker.pubkey(),
        &validators::ID,
    );
    let transaction = Transaction::new_signed_with_payer(
        &vec![ix],
        Some(&payer.pubkey()),
        &[&payer, &staker],
        blockhash,
    );
    let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("{txhash}");
}
