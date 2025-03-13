use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub fn handle_greet(_program_id: &Pubkey, _accounts: &[AccountInfo], _seed: u64) -> ProgramResult {
    Ok(())
}
