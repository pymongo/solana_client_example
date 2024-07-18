import {
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction
} from '@solana/web3.js';
import { init } from './init';
(async () => {
    const { client, payer } = init();
    const programId = new PublicKey(process.env.hello_world!);
    const instruction = new TransactionInstruction({
        keys: [], // our program only logs hello world and doesn't need any accounts.
        programId,
        data: Buffer.alloc(0)
    });
    const signature = await sendAndConfirmTransaction(
        client,
        new Transaction().add(instruction),
        [payer]
    );
    console.log('signature/txhash:', signature);
})();