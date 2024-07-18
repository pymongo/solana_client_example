import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as fs from 'fs';
import * as dotenv from 'dotenv';
export interface initResponse {
    client: Connection,
    payer: Keypair,
    to: PublicKey
}
export function init(): initResponse {
    dotenv.config();
    const client = new Connection('https://api.devnet.solana.com', 'confirmed');
    const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));
    const to = new PublicKey(process.env.to!);
    return {
        client,
        payer,
        to
    }
}