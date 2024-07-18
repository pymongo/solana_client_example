流程:
1. 部署了 program_id 8xubajzX923ZXpUzbyTcXuxy9QcMbrUCosm4H6ZRtTtk
2. createAccountWithSeed(payer,program_id,seed) seed其实就是相当于链上存储数据的key类似于move的key概念

## 报错
> "Program log: Greeted account does not have the correct program id"

```diff
     const instruction = new TransactionInstruction({
-        keys: [{ pubkey: payer.publicKey, isSigner: false, isWritable: true }],
+        keys: [{ pubkey: greetedPubkey, isSigner: false, isWritable: true }],
         programId,
         data: Buffer.alloc(0)
     });
```

## signer 问题

```
AccountNotFound: pubkey=65hH53vj34oG6DGdFePftj7mMeiM3uQtvPLQNPUnSAkV
thread 'main' panicked at /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-sdk-2.0.2/src/transaction/mod.rs:733:13:
Transaction::sign failed with error NotEnoughSigners
```

full backtrace

```
AccountNotFound: pubkey=65hH53vj34oG6DGdFePftj7mMeiM3uQtvPLQNPUnSAkV
thread 'main' panicked at /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-sdk-2.0.2/src/transaction/mod.rs:733:13:
Transaction::sign failed with error NotEnoughSigners
stack backtrace:
   0: rust_begin_unwind
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14
   2: solana_sdk::transaction::Transaction::sign
             at /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-sdk-2.0.2/src/transaction/mod.rs:733:13
   3: solana_sdk::transaction::Transaction::new
             at /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-sdk-2.0.2/src/transaction/mod.rs:378:9
   4: solana_sdk::transaction::Transaction::new_signed_with_payer
             at /home/w/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/solana-sdk-2.0.2/src/transaction/mod.rs:534:9
   5: greeting_account::main
             at ./examples/greeting_account.rs:55:47
   6: core::ops::function::FnOnce::call_once
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

```diff
                             let ix = create_account_with_seed(
-                                &payer.pubkey(),
                                 &payer.pubkey(),
                                 &program_data_pubkey,
+                                &payer.pubkey(),
```

create_account_with_seed的参数

- from_pubkey: payer
- to_pubkey: program_data_pubkey
- base: payer
- owner: program_id
