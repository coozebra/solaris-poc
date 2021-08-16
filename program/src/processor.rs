use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack:: Pack,
    pubkey::Pubkey,
};

use spl_token_lending::{
    state::{LendingMarket, Reserve},
    error::LendingError,
};

use crate::{
    error::{PoCError},
    instruction::PoCInstruction
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PoCInstruction::unpack(instruction_data)?;

    match instruction {
        PoCInstruction::DepositReserveLiquidity { liquidity_amount } => {
            msg!("Instruction: Deposit Reserve Liquidity");
            process_deposit_reserve_liquidity(program_id, liquidity_amount, accounts)
        }
    }
}

fn process_deposit_reserve_liquidity(
    program_id: &Pubkey,
    liquidity_amount: u64,
    accounts: &[AccountInfo],
) -> ProgramResult {
    if liquidity_amount == 0 {
        msg!("Amount provided cannot be zero");
        return Err(PoCError::InvalidAmount.into());
    }

    let account_info_iter = &mut accounts.iter();
    let source_liquidity_info = next_account_info(account_info_iter)?;
    let destination_collateral_info = next_account_info(account_info_iter)?;
    let reserve_info = next_account_info(account_info_iter)?;

    let reserve_liquidity_supply_info = next_account_info(account_info_iter)?;
    let reserve_collateral_mint_info = next_account_info(account_info_iter)?;

    let lending_market_info = next_account_info(account_info_iter)?;
    let user_transfer_authority_info = next_account_info(account_info_iter)?;
    let lending_program_id = next_account_info(account_info_iter)?;

    let lending_market = LendingMarket::unpack(&lending_market_info.data.borrow())?;
    if lending_market_info.owner != lending_program_id.key {
        msg!("Lending market provided is not owned by the lending program");
        return Err(LendingError::InvalidAccountOwner.into());
    }
    let reserve = Reserve::unpack(&reserve_info.data.borrow())?;
    if reserve_info.owner != lending_program_id.key {
        msg!("Reserve provided is not owned by the lending program");
        return Err(LendingError::InvalidAccountOwner.into());
    }
    if &reserve.lending_market != lending_market_info.key {
        msg!("Reserve lending market does not match the lending market provided");
        return Err(LendingError::InvalidAccountInput.into());
    }

    let deposit_reserve_liquidity_ix = spl_token_lending::instruction::deposit_reserve_liquidity(
        *lending_program_id.key,
        liquidity_amount,
        *source_liquidity_info.key,
        *destination_collateral_info.key,
        *reserve_info.key,
        *reserve_liquidity_supply_info.key,
        *reserve_collateral_mint_info.key,
        *lending_market_info.key,
        *user_transfer_authority_info.key,
    );

    Ok(())
}
