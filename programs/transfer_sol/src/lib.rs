use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use borsh::BorshDeserialize;

macro_rules! log {
    ($msg:expr) => {
        msg!(&format!("{}:{} {}", file!(), line!(), $msg))
    };
    ($($arg:tt)*) => {
        msg!(&format!("{}:{} {}", file!(), line!(), format!($($arg)*)))
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum Instruction {
    /// CPI=Cross Program Interface 跨合约调用，invoke 系统的 transfer 指令
    CpiTransfer(u64),
    /// 智能合约代码实现转账(一边余额加，一边余额减)
    ProgramTransfer(u64),
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    // u64::from_le_bytes()
    let instruction = Instruction::try_from_slice(instruction_data)?;
    match instruction {
        Instruction::CpiTransfer(args) => transfer_sol_with_cpi(accounts, args),
        Instruction::ProgramTransfer(args) => {
            transfer_sol_with_program(program_id, accounts, args)
        }
    }
}

fn transfer_sol_with_cpi(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    log!("transfer_sol_with_cpi accounts.len()={} amount={amount}", accounts.len());
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(payer.key, recipient.key, amount),
        accounts
        // &[payer.clone(), recipient.clone(), system_program.clone()],
    )?;

    Ok(())
}

fn transfer_sol_with_program(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    log!("transfer_sol_with_program accounts.len()={} amount={amount}", accounts.len());
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;

    **payer.try_borrow_mut_lamports()? -= amount;
    **recipient.try_borrow_mut_lamports()? += amount;

    Ok(())
}
