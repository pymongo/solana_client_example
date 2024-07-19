# spl-token 常用命令

## 发币
```
# default decimal 9
spl-token create-token
root@lb1:~# spl-token create-account 4M9LUzCsr8cSuCZD6uimjhP7jxJJcgg7P7jaY7JFNV7q
Creating account AjFPSqpEVcycMuVw71tjhHdKaWV3C7FEV51SdAPYQCYk

Signature: 3iytwHuD99C9hJN8C825RnbD8shjftpTozo49hcb6VnZKHiq4hWe891BmvBULP6yArQGL1X28PiF2eEwyFNkkbQ2

root@lb1:~# spl-token mint 4M9LUzCsr8cSuCZD6uimjhP7jxJJcgg7P7jaY7JFNV7q 1
Minting 1 tokens
  Token: 4M9LUzCsr8cSuCZD6uimjhP7jxJJcgg7P7jaY7JFNV7q
  Recipient: AjFPSqpEVcycMuVw71tjhHdKaWV3C7FEV51SdAPYQCYk

Signature: 4HRPLo6WPZrtb1LxezAn1sDHGsk9ZZJ5jcVpuZmUa9R7fUCdgSL49ngZJQmUVdw3H6EWYxU1aT8XnQFEjdojrMKY
```

源码: https://github.com/solana-labs/solana-program-library/blob/493aa06589b59c148ad5b485c50cdac47804c227/token/cli/src/command.rs#L221

> async fn command_create_token

## 查询 USDC 余额
spl-token balance 4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU

## NFT发行

```
# 必须设置成 0 decimal
spl-token create-token --decimals 0 
spl-token create-account
spl-token mint
spl-token authorize $spl-account mint --disable
```

## 

```
let ix = system_instruction::create_account(
    &payer.pubkey(),
    &token_program.pubkey(),
    rent_lamports,
    space,
    &spl_token::ID,
);
let blockhash = client.get_latest_blockhash().unwrap();
let transaction = Transaction::new_signed_with_payer(
    &[ix],
    Some(&payer.pubkey()),
    &[&payer, &token_program],
    blockhash,
);
let txhash = client.send_and_confirm_transaction(&transaction).unwrap();
println!("token_program pubkey={} {txhash}", token_program.pubkey());

let mint = Keypair::new();
// let token_program = get_or_create_account("token_program", &spl_token::ID, 82);
// let mint = get_or_create_account("mint", &token_program, mint_len);
// mint_authority 就是 token 发布者也就是 payer 我自己
let create_mint_account_ix = system_instruction::create_account(
    &payer.pubkey(),
    &mint.pubkey(),
    rent_lamports,
    space,
    &token_program.pubkey()
);
let init_mint_ix = spl_token::instruction::initialize_mint(
    &token_program.pubkey(),
    &mint.pubkey(),
    &payer.pubkey(),
    None,
    decimals,
)
.unwrap();
```

```
token_program pubkey=HifTogBnro2PNeE4kusMH9qTSLBefBYoe6qgXLqQeB7s 5QcVvdCaxWoWpK2zpfyrKio8P7QYSavfgj1eT1UNodPBTN6d1uqycdhnjqgFXKg9i2pjDpKqxYv3A8WDqrtrEEcJ
thread 'main' panicked at examples/create_token.rs:101:6:
called `Result::unwrap()` on an `Err` value: IncorrectProgramId
```
