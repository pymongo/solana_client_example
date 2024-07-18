import * as borsh from 'borsh';
import {
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction,
    LAMPORTS_PER_SOL
} from '@solana/web3.js';
import { init } from './init';

enum InstructionType {
    CpiTransfer = 0,
    ProgramTransfer = 1,
}
class Instruction {
    instruction: InstructionType;
    amount: number;

    constructor(props: {
        instruction: InstructionType;
        amount: number;
    }) {
        this.instruction = props.instruction;
        this.amount = props.amount;
    }

    // first u8 is enum discriminant
    static schema = { struct: { instruction: 'u8', amount: 'u64' } };

    toBuffer(): Buffer {
        return Buffer.from(borsh.serialize(Instruction.schema, this));
    }

    static fromBuffer(buffer: Buffer): Instruction {
        return borsh.deserialize(Instruction.schema, buffer) as Instruction;
    }
}

(async () => {
    const { client, payer } = init();
    const programId = new PublicKey(process.env.transfer_sol!);
    const to = new PublicKey(process.env.to!);
    const data = new Instruction({ instruction: InstructionType.ProgramTransfer, amount: 0.01 * LAMPORTS_PER_SOL });
    const instruction = new TransactionInstruction({
        keys: [
            { pubkey: payer.publicKey, isSigner: true, isWritable: true },
            { pubkey: to, isSigner: false, isWritable: true }
        ],
        programId,
        data: data.toBuffer()
    });
    const signature = await sendAndConfirmTransaction(
        client,
        new Transaction().add(instruction),
        [payer]
    );
    console.log('signature/txhash:', signature);
})();
