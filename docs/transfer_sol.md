```
/root/solana_client_example/node_modules/@solana/web3.js/src/connection.ts:6006
      throw new SendTransactionError({
            ^
SendTransactionError: Simulation failed.
Message: Transaction simulation failed: Error processing Instruction 0: instruction spent from the balance of an account it does not own.
Logs:
[
  "Program GXScYf8mQbRUPYzjgthe3idqqvfqPoYKbBe3vHmdT3dS invoke [1]",
  "Program log: programs/transfer_sol/src/lib.rs:66 transfer_sol_with_program accounts.len()=2 amount=10000000",
  "Program GXScYf8mQbRUPYzjgthe3idqqvfqPoYKbBe3vHmdT3dS consumed 1739 of 200000 compute units",
  "Program GXScYf8mQbRUPYzjgthe3idqqvfqPoYKbBe3vHmdT3dS failed: instruction spent from the balance of an account it does not own"
].
Catch the `SendTransactionError` and call `getLogs()` on it for full details.
    at Connection.sendEncodedTransaction (/root/solana_client_example/node_modules/@solana/web3.js/src/connection.ts:6006:13)
    at processTicksAndRejections (node:internal/process/task_queues:95:5)
    at async Connection.sendRawTransaction (/root/solana_client_example/node_modules/@solana/web3.js/src/connection.ts:5962:20)
    at async Connection.sendTransaction (/root/solana_client_example/node_modules/@solana/web3.js/src/connection.ts:5950:12)
    at async sendAndConfirmTransaction (/root/solana_client_example/node_modules/@solana/web3.js/src/utils/send-and-confirm-transaction.ts:36:21)
    at async /root/solana_client_example/typescripts/transfer_sol.ts:52:23 {
  signature: '',
  transactionMessage: 'Transaction simulation failed: Error processing Instruction 0: instruction spent from the balance of an account it does not own',
  transactionLogs: [
    'Program GXScYf8mQbRUPYzjgthe3idqqvfqPoYKbBe3vHmdT3dS invoke [1]',
    'Program log: programs/transfer_sol/src/lib.rs:66 transfer_sol_with_program accounts.len()=2 amount=10000000',
    'Program GXScYf8mQbRUPYzjgthe3idqqvfqPoYKbBe3vHmdT3dS consumed 1739 of 200000 compute units',
    'Program GXScYf8mQbRUPYzjgthe3idqqvfqPoYKbBe3vHmdT3dS failed: instruction spent from the balance of an account it does not own'
  ]
}
```

ETH/SOL 智能合约内都不能直接进行ETH/SOL余额加减进行转账，必须调用内置函数或者SystemProgram例如transfer/send/call才能进行 ETH/SOL 交易

ERC20 Token的开发者确实有能力通过智能合约逻辑来直接修改账户的Token余额。因此，如果发现Token被盗，开发者可以编写和部署新的智能合约或更新现有合约来回滚被盗的Token，Solana Token发行方无法直接修改账户余额

```
root@lb1:~/solana_client_example# solana account 8xubajzX923ZXpUzbyTcXuxy9QcMbrUCosm4H6ZRtTtk

Public Key: 8xubajzX923ZXpUzbyTcXuxy9QcMbrUCosm4H6ZRtTtk
Balance: 0.00114144 SOL
Owner: BPFLoaderUpgradeab1e11111111111111111111111
```

所以所有 solana account owner 都是 11111 这个 SystemProgram
