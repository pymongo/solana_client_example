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

## 设置币种名字
spl-token initialize-metadata 

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

## IncorrectProgramId

```
token_program pubkey=HifTogBnro2PNeE4kusMH9qTSLBefBYoe6qgXLqQeB7s 5QcVvdCaxWoWpK2zpfyrKio8P7QYSavfgj1eT1UNodPBTN6d1uqycdhnjqgFXKg9i2pjDpKqxYv3A8WDqrtrEEcJ
thread 'main' panicked at examples/create_token.rs:101:6:
called `Result::unwrap()` on an `Err` value: IncorrectProgramId
```

修复: token_program_id 入参只能写死 spl_token::ID

## mint_to invalid data

```
thread 'main' panicked at examples/create_token.rs:64:68:
called `Result::unwrap()` on an `Err` value: Error { request: Some(SendTransaction), kind: RpcError(RpcResponseError { code: -32002, message: "Transaction simulation failed: Error processing Instruction 0: invalid account data for instruction", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(InstructionError(0, InvalidAccountData)), logs: Some(["Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]", "Program log: Instruction: MintTo", "Program log: Error: InvalidAccountData", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1342 of 200000 compute units", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA failed: invalid account data for instruction"]), accounts: None, units_consumed: Some(1342), return_data: None, inner_instructions: None, replacement_blockhash: None }) }) }
```

修复如下

```diff
     let ix = spl_associated_token_account::instruction::create_associated_token_account(
         &payer.pubkey(),
-        &token_account,
+        &payer.pubkey(),
         &mint.pubkey(),
         &spl_token::ID,
     );
```

## metadata 里面的 solana 版本不一样

代码:

```
let metadata_program_id = mpl_token_metadata::ID;
let metadata_seeds = &[
    b"metadata",
    metadata_program_id.as_ref(),
    mint.pubkey().as_ref(),
];
let (metadata_pda, _metadata_bump) =
    solana_sdk::pubkey::Pubkey::find_program_address(metadata_seeds, &metadata_program_id);
```

报错:

```
    = note: `solana_program::pubkey::Pubkey` and `solana_sdk::pubkey::Pubkey` have similar names, but are actually distinct types
note: `solana_program::pubkey::Pubkey` is defined in crate `solana_program`
   --> /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-program-1.18.18/src/pubkey.rs:88:1
    |
88  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `solana_sdk::pubkey::Pubkey` is defined in crate `solana_program`
   --> /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-program-2.0.2/src/pubkey.rs:96:1
```

## NFT
solana summber NFT mint 成功!(测试网)

走完整个流程，要创建各种account payer,PDA,TokenAccount 等等，

代码写错一个参数就 rpc InstructionError 调半天，改solana源码一点点加日志看看哪里入参传错了

Rust代码体验好比ts代码在交易前会检查每个指令数据对不对

## 没有开 metadata 权限

```
root@lb1:~# spl-token initialize-metadata 2KHuEsYHqfjBJjZX2A22HTiRWsk194w2RheRvpcr6A7c usdt usdt "https://raw.githubusercontent.com/spothq/cryptocurrency-icons/master/svg/icon/usdt.svg"
Error: Client(Error { request: Some(SendTransaction), kind: RpcError(RpcResponseError { code: -32002, message: "Transaction simulation failed: Error processing Instruction 1: custom program error: 0xc", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(InstructionError(1, Custom(12))), logs: Some(["Program 11111111111111111111111111111111 invoke [1]", "Program 11111111111111111111111111111111 success", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]", "Program log: Error: Invalid instruction", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 863 of 399850 compute units", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA failed: custom program error: 0xc"]), accounts: None, units_consumed: Some(1013), return_data: None }) }) })
```

```
spl-2022: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
spl     : TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb


root@lb1:~# spl-token create-token
Creating token 2KHuEsYHqfjBJjZX2A22HTiRWsk194w2RheRvpcr6A7c under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA


---
root@lb1:~# spl-token create-token --enable-metadata --program-id TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
Creating token Am9xYd17dcMTGFLFtdPxCU12JhiD3vMndux6jgewk6wq under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
Error: Program(IncorrectProgramId)
root@lb1:~# spl-token create-token --enable-metadata --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
Creating token CW9b3SRUM8BSkLAQ23HxQeEm9JaUoHeiSY2xQ1mWu3GS under program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
To initialize metadata inside the mint, please run `spl-token initialize-metadata CW9b3SRUM8BSkLAQ23HxQeEm9JaUoHeiSY2xQ1mWu3GS <YOUR_TOKEN_NAME> <YOUR_TOKEN_SYMBOL> <YOUR_TOKEN_URI>`, and sign with the mint authority.

Address:  CW9b3SRUM8BSkLAQ23HxQeEm9JaUoHeiSY2xQ1mWu3GS
Decimals:  9

Signature: 4ahpGH71tNyFwjXEk4HdjBNBmuoQTxUmKcGErq6VNXpRsEdVLpSna9L6oTUxyAo7P47K6fynAj6V5NQN7qMNzmvW

root@lb1:~# spl-token create-token --enable-metadata
Creating token 98yf76RutMa71X1yVq1mEdJaF4HchVRo3L5HWDC2SWYj under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
Error: Program(IncorrectProgramId)
---

root@lb1:~# spl-token initialize-metadata CW9b3SRUM8BSkLAQ23HxQeEm9JaUoHeiSY2xQ1mWu3GS  usdt usdt "https://raw.githubusercontent.com/spothq/cryptocurrency-icons/master/svg/icon/usdt.svg"

Signature: jKXFqQmVRMVsqrBJJpuLLvb6UC9x9HXqciuNQjVY4fuYGDQvrkuZwYVVZgB9zVwVYwaQVSbvhp1QvAbCD4y1Xkw
```


