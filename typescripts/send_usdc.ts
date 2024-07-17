import {
    Connection,
    Keypair,
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
    getOrCreateAssociatedTokenAccount,
    createTransferInstruction,
} from '@solana/spl-token';
import * as fs from 'fs';
import * as dotenv from 'dotenv';
dotenv.config();

(async () => {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    
    const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    const fromKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));
    
    const toAddr = new PublicKey(process.env.to as string);
    const usdcMint = new PublicKey(process.env.usdc_mint as string);
    const lamportsToSend = 1_000_000; // Amount to send (1 USDC in this case, as USDC has 6 decimals)

    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        fromKeypair,
        usdcMint,
        fromKeypair.publicKey
    );
    const toTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        fromKeypair,
        usdcMint,
        toAddr
    );

    const transfer = createTransferInstruction(
        fromTokenAccount.address,
        toTokenAccount.address,
        fromKeypair.publicKey,
        lamportsToSend
    );
    const transferTransaction = new Transaction().add(transfer);
    const txhash = await sendAndConfirmTransaction(connection, transferTransaction, [
        fromKeypair,
    ]);

    console.info(txhash);
})();
