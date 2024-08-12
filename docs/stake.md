```
solana-keygen new --outfile stake-account-keypair.json
solana create-stake-account stake-account-keypair.json 1 SOL
solana delegate-stake stake-account-keypair.json <VALIDATOR_VOTE_ACCOUNT_ADDRESS>
solana stake-account stake-account-keypair.json
```

关于 `solana delegate-stake` 我的理解是，创建了质押账户并充值了1个SOL之后，我是没法获得质押利息收入的，
我需要将质押账户投票权益委托给验证者节点账户，这样验证者的出块奖励会按照我的质押委托比例发放给我
类似于JUP社区质押了JUP但不参与投票/不行使投票权的话，是没有奖励的。大佬们看看我的理解有没有问题吗？
