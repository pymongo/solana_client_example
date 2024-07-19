## bank state
The result of interpreting all programs on the ledger at a given tick height. It includes at least the set of all accounts holding nonzero native tokens.

应该类似于 ETH tx 里面的 stateRoot 也就是 Merkle Tree 根节点哈希

## rpc connection 的 Commitment 入参
- processed: 某个节点处理并加入区块链的 fork 中
- confirmed：交易已经被网络中的一个 Supernode 确认
- finalized: 所有 Supernode 确认

## term
- data plane: A multicast network used to efficiently validate entries and gain consensus.
- warmup and cooldown: staking/unstaking期间质押的权益逐渐增加/减少 防止网络中出现突然的大量解质押变动
- drone: An off-chain service that acts as a custodian for a user's private key. It typically serves to validate and sign transactions.

## 共识层相关
- entry: POH共识相关 An entry on the ledger either a tick or a transaction's entry.
- epoch: number of slots, slot_id
- inflation: An increase in token supply over time used to fund rewards for validation and to fund continued development of Solana.
- ledger schedule: The role of a validator when it is appending entries to the ledger.
- ledger: A list of entries containing transactions signed by clients. Conceptually, this can be traced back to the genesis block, but an actual validator's ledger may have only newer blocks to reduce storage, as older ones are not needed for validation of future blocks by design.
- thin client: 不参与验证

## micro-lamports

1SOL=1e3 micro-lamports = 1e9 lamports, micro-lamports is unit of priority_fee
