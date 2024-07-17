import {
    Connection,
    PublicKey,
    Keypair,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction,
    SystemProgram
} from '@solana/web3.js';
import * as borsh from 'borsh';
import * as fs from 'fs';
import * as dotenv from 'dotenv';

class GreetingAccount {
    counter = 0;
    constructor(fields: { counter: number } | undefined = undefined) {
        if (fields) {
            this.counter = fields.counter;
        }
    }
}
const schema = { struct: { counter: 'u32' } };
const GREETING_SIZE = borsh.serialize(
    schema,
    new GreetingAccount(),
).length;

dotenv.config();
(async () => {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    // Read the keypair from the file
    const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));
    const programId = new PublicKey(process.env.greeting_hello as string);

    const GREETING_SEED = 'hello';
    const greetedPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        GREETING_SEED,
        programId,
    );

    // Check if the greeting account has already been created
    const greetedAccount = await connection.getAccountInfo(greetedPubkey);
    if (greetedAccount === null) {
        console.log(
            'Creating account',
            greetedPubkey.toBase58(),
            'to say hello to',
        );
        const lamports = await connection.getMinimumBalanceForRentExemption(
            GREETING_SIZE,
        );

        const ix = SystemProgram.createAccountWithSeed({
            fromPubkey: payer.publicKey,
            basePubkey: payer.publicKey,
            seed: GREETING_SEED,
            newAccountPubkey: greetedPubkey,
            lamports,
            space: GREETING_SIZE,
            programId,
        });
        const transaction = new Transaction().add(ix);
        const txhash = await sendAndConfirmTransaction(connection, transaction, [payer]);
        console.info(`createAccountWithSeed txhash=${txhash}`);
    }

    await queryCounter(connection, greetedPubkey);
    const instruction = new TransactionInstruction({
        keys: [{ pubkey: payer.publicKey, isSigner: false, isWritable: true }],
        programId,
        data: Buffer.alloc(0)
    });
    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer]
    );
    console.log('counter+=1 txhash:', signature);
    await queryCounter(connection, greetedPubkey);
})();

async function queryCounter(connection: Connection, greetedPubkey: PublicKey) {
    const accountInfo = await connection.getAccountInfo(greetedPubkey);
    if (accountInfo === null) {
        throw 'Error: cannot find the greeted account';
    }
    const greeting = borsh.deserialize(
        schema,
        accountInfo.data,
    ) as GreetingAccount;
    console.info(`counter = ${greeting.counter}`);
}
