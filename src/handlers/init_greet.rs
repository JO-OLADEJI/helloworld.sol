use crate::interface::state::*;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn init_greet(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let signer = next_account_info(accounts_iter)?;
    let greeting_pda = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // calculate size
    const MAX_GREETING_LENGTH: usize = 32 + 4;
    let account_size = MAX_GREETING_LENGTH + 8;

    // calculate rent based on size
    let rent = Rent::get()?;
    let space_allocation_rent = rent.minimum_balance(account_size);

    // create account
    invoke(
        &system_instruction::create_account(
            signer.key,
            greeting_pda.key,
            space_allocation_rent,
            account_size as u64,
            program_id,
        ),
        &[signer.clone(), greeting_pda.clone(), system_program.clone()],
    )?;

    // store data in it.
    let clock = Clock::get()?;
    let account_data = GreetingState {
        current: String::from("Hello, Solana!"),
        exp_time: clock.unix_timestamp as u64,
    };
    account_data.serialize(&mut &mut greeting_pda.data.borrow_mut()[..])?;

    Ok(())
}
