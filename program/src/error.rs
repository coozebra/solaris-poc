use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum PoCError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("The account is not currently owned by the program")]
    IncorrectProgramId,
    #[error("The amount is invalid")]
    InvalidAmount,
}

impl From<PoCError> for ProgramError {
    fn from(e: PoCError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
