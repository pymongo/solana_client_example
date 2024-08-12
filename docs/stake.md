```
solana-keygen new --outfile stake-account-keypair.json
solana create-stake-account stake-account-keypair.json 1 SOL
solana delegate-stake stake-account-keypair.json <VALIDATOR_VOTE_ACCOUNT_ADDRESS>
solana stake-account stake-account-keypair.json
```

