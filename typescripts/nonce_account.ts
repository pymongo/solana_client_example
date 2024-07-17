import {
    Connection,
    PublicKey,
    Keypair,
    NonceAccount,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction,
    SystemProgram
} from '@solana/web3.js';
import * as fs from 'fs';
import * as dotenv from 'dotenv';
dotenv.config();
function readKeypair(file: string): Keypair {
    const keypairPath = `${process.env.HOME}/.config/solana/${file}`;
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    return Keypair.fromSecretKey(new Uint8Array(keypairData));
}
(async () => {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    const payer = readKeypair("id.json");
    const nonceAccountKey = readKeypair("nonce-account.json")
    const accountInfo = await connection.getAccountInfo(nonceAccountKey.publicKey);
    if (accountInfo === null) throw new Error("")
    const nonceAccount = NonceAccount.fromAccountData(accountInfo.data);
    const programId = new PublicKey(process.env.program_id as string);

    // make a nonce advance instruction
    const advanceIX = SystemProgram.nonceAdvance({
        authorizedPubkey: payer.publicKey,
        noncePubkey: nonceAccountKey.publicKey,
    });
    const ix = new TransactionInstruction({
        // our program only logs hello world and doesn't need any accounts.
        keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: true}],
        programId,
        data: Buffer.alloc(0)
    });
    const tx = new Transaction();
    tx.add(advanceIX);
    tx.add(ix);
    tx.recentBlockhash = nonceAccount.nonce;
    tx.feePayer = payer.publicKey;

    const signature = await sendAndConfirmTransaction(
        connection,
        tx,
        [payer]
    );
    console.log('signature/txhash:', signature);
})();
