import {
    Connection,
    PublicKey,
    Keypair,
    Transaction,
    sendAndConfirmTransaction,
    LAMPORTS_PER_SOL,
    SystemProgram
} from '@solana/web3.js';
import * as fs from 'fs';
import * as dotenv from 'dotenv';
dotenv.config();
(async () => {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    // Read the keypair from the file
    const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));

    const fromKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));
    const to_addr = new PublicKey(process.env.to as string);

    const lamportsToSend = 1_000_000;
    const transferTransaction = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: fromKeypair.publicKey,
            toPubkey: to_addr,
            lamports: lamportsToSend,
        }),
    );
   
    const txhash = await sendAndConfirmTransaction(connection, transferTransaction, [
        fromKeypair,
    ]);
    console.info(txhash);
})();
