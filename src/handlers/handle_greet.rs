use crate::interface::{constants::*, GreetingState};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};
use std::collections::HashMap;

fn get_random_greeting(seed: u64, timestamp: u64) -> String {
    let mut greetings: HashMap<u64, &str> = HashMap::new();
    let timed_seed = seed.checked_add(timestamp).unwrap_or(timestamp);
    let index = timed_seed % greetings.len() as u64;

    greetings.insert(0, DEFAULT_GREETING);
    greetings.insert(1, CHINESE_GREETING);
    greetings.insert(2, HINDI_GREETING);
    greetings.insert(3, SPANISH_GREETING);
    greetings.insert(4, FRENCH_GREETING);
    greetings.insert(5, ARABIC_GREETING);
    greetings.insert(6, RUSSIAN_GREETING);
    greetings.insert(7, PORTUGUESE_GREETING);
    greetings.insert(8, GERMAN_GREETING);
    greetings.insert(9, JAPANESE_GREETING);

    return greetings
        .get(&index)
        .unwrap_or(&DEFAULT_GREETING)
        .to_string();
}

pub fn handle_greet(program_id: &Pubkey, accounts: &[AccountInfo], seed: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let greeting_pda = next_account_info(accounts_iter)?;

    if greeting_pda.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // deserialize the account data
    let mut greeting_data_bytes = greeting_pda.data.borrow_mut();
    let mut greeting_struct = GreetingState::try_from_slice(&greeting_data_bytes)?;

    // update the greeting data
    let clock = Clock::get()?;
    greeting_struct.current = get_random_greeting(seed, clock.unix_timestamp as u64);
    greeting_struct.exp_time = clock.unix_timestamp as u64;

    // serialize the greeting data into the account's data field
    greeting_struct.serialize(&mut &mut greeting_data_bytes[..])?;

    msg!("Greeting update: {:?}", greeting_struct.current);
    Ok(())
}
