use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum HelloWorldInstruction {
    Initialize,
    Greet { seed: u64 },
}

impl HelloWorldInstruction {
    pub fn dispatch(serialized_calldata: &[u8]) -> Result<Self, ProgramError> {
        let (&func_id, param) = serialized_calldata
            .split_first()
            .ok_or_else(|| ProgramError::InvalidInstructionData)?;

        match func_id {
            0 => {
                return Ok(Self::Initialize);
            }
            1 => {
                let seed = u64::from_le_bytes(
                    param
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                return Ok(Self::Greet { seed });
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    }
}
