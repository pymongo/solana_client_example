import {
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
    LAMPORTS_PER_SOL,
    SystemProgram
} from '@solana/web3.js';
import { init } from './init';
(async () => {
    const { client, payer } = init();
    const to_addr = new PublicKey(process.env.to!);
    const transfer = SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: to_addr,
        lamports: 0.01 * LAMPORTS_PER_SOL,
    });
    // const transferTransaction = new Transaction().add(transfer);
    // transferTransaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    const txhash = await sendAndConfirmTransaction(
        client,
        new Transaction().add(transfer),
        [payer]
    );
    console.info(txhash);
})();