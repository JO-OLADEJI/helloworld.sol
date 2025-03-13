use crate::interface::{
    constants::{DEFAULT_GREETING, MAX_GREETING_LENGTH, U64_SIZE_BYTES},
    state::*,
};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
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

    let account_size = MAX_GREETING_LENGTH + U64_SIZE_BYTES;

    let rent = Rent::get()?;
    let space_allocation_rent = rent.minimum_balance(account_size);

    // TODO: confirm the account hasn't been initialized
    if greeting_pda.data.borrow().len() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

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

    let clock = Clock::get()?;
    let account_data = GreetingState {
        current: String::from(DEFAULT_GREETING),
        exp_time: clock.unix_timestamp as u64,
    };
    account_data.serialize(&mut &mut greeting_pda.data.borrow_mut()[..])?;

    Ok(())
}
