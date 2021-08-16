use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::{convert::TryInto, mem::size_of};

use crate::error::PoCError::InvalidInstruction;

pub enum PoCInstruction {
    /// Deposit liquidity into a reserve in exchange for collateral. Collateral represents a share
    /// of the reserve liquidity pool.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account.
    ///   2. `[writable]` Reserve account.
    ///   3. `[]` Lending market account.
    ///   4. `[signer]` User transfer authority ($authority).
    ///   5. `[]` Token program id.
    DepositReserveLiquidity {
        /// Amount of liquidity to deposit in exchange for collateral tokens
        liquidity_amount: u64,
    },
}

impl PoCInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::DepositReserveLiquidity {
                liquidity_amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match *self {
            Self::DepositReserveLiquidity { liquidity_amount } => {
                buf.push(0);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
        }
        buf
    }
}

/// Creates a 'DepositReserveLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn deposit_reserve_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token_lending::id(), false),
        ],
        data: PoCInstruction::DepositReserveLiquidity { liquidity_amount }.pack(),
    }
}
