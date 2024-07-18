> solana program deploy target/deploy/transfer_sol.s

``
To resume a deploy, pass the recovered keypair as the
[BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
==========================================================================
Error: Deploying program failed: RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: account data too small for instruction [3 log messages]
```

> p solana program extend GmCUexGFRgPmqYzGjvRwaP5YKLzEE1dGj7NSCcxzfZ3E 655536

Extended Program Id GmCUexGFRgPmqYzGjvRwaP5YKLzEE1dGj7NSCcxzfZ3E by 655536 bytes

不小心多打了一个 0 收了 4.5 SOL，只好关掉去退钱

> solana program close GmCUexGFRgPmqYzGjvRwaP5YKLzEE1dGj7NSCcxzfZ3E --bypass-warning

```
w@w:~/solana_client_example$ p solana program deploy target/deploy/transfer_sol.so 
ProxyChains-3.1 (http://proxychains.sf.net)
Error: Program GmCUexGFRgPmqYzGjvRwaP5YKLzEE1dGj7NSCcxzfZ3E has been closed, use a new Program Id

rm target/deploy/transfer_sol-keypair.json
```
