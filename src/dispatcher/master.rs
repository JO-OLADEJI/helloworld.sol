use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::handlers::handle_greet::*;
use crate::handlers::init_greet::*;
use crate::interface::instructions::*;

pub fn dispatch_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    calldata: &[u8],
) -> ProgramResult {
    let instruction = HelloWorldInstruction::dispatch(calldata)?;

    match instruction {
        HelloWorldInstruction::Initialize => {
            return init_greet(program_id, accounts);
        }
        HelloWorldInstruction::Greet { seed } => {
            return handle_greet(program_id, accounts, seed);
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }
}
