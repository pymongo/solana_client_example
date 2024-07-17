import {
    Connection,
    PublicKey,
    Keypair,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction
} from '@solana/web3.js';
import * as fs from 'fs';
import * as dotenv from 'dotenv';
dotenv.config();
(async () => {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    // Read the keypair from the file
    const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));
    const programId = new PublicKey(process.env.program_id as string);

    // Create an instruction to call your program
    const instruction = new TransactionInstruction({
        // our program only logs hello world and doesn't need any accounts.
        keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: true}],
        programId,
        data: Buffer.alloc(0)
    });
    // Create a transaction
    const transaction = new Transaction().add(instruction);
    // Sign and Send the transaction
    console.log("before send tx");
    const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer]
    );
    console.log('signature/txhash:', signature);
})();
