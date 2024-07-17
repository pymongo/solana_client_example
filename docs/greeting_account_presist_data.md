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
