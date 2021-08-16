#![cfg(feature = "test-bpf")]

mod helpers;

use helpers::*;
use assert_matches::*;
use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::{Transaction, TransactionError},
};
use spl_token::{
    instruction::approve,
    state::{Account as Token, AccountState, Mint},
};
use poc_program::{
    processor::process_instruction,
    instruction::deposit_reserve_liquidity
};
use std::str::FromStr;

#[tokio::test]
async fn test_success() {
    let program_id = Pubkey::from_str("Sysvar1111111111111111111111111111111111111").unwrap();
    let liquidity_amount = 100 * FRACTIONAL_TO_USDC;
    let mut test = ProgramTest::new(
        "poc_program",
        program_id,
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_bpf_compute_max_units(27_000);

    let user_accounts_owner = Keypair::new();
    let lending_market = add_lending_market(&mut test);

    let usdc_mint = add_usdc_mint(&mut test);
    let usdc_oracle = add_usdc_oracle(&mut test);
    let usdc_test_reserve = add_reserve(
        &mut test,
        &lending_market,
        &usdc_oracle,
        &user_accounts_owner,
        AddReserveArgs {
            user_liquidity_amount: 100 * FRACTIONAL_TO_USDC,
            liquidity_amount: 10_000 * FRACTIONAL_TO_USDC,
            liquidity_mint_decimals: usdc_mint.decimals,
            liquidity_mint_pubkey: usdc_mint.pubkey,
            config: TEST_RESERVE_CONFIG,
            mark_fresh: true,
            ..AddReserveArgs::default()
        },
    );

    let (mut banks_client, payer, _recent_blockhash) = test.start().await;

    let user_transfer_authority = Keypair::new();
    let mut transaction = Transaction::new_with_payer(
        &[
            approve(
                &spl_token::id(),
                &usdc_test_reserve.user_liquidity_pubkey,
                &user_transfer_authority.pubkey(),
                &user_accounts_owner.pubkey(),
                &[],
                liquidity_amount,
            )
            .unwrap(),
            deposit_reserve_liquidity(
                program_id,
                liquidity_amount,
                usdc_test_reserve.user_liquidity_pubkey,
                usdc_test_reserve.user_collateral_pubkey,
                usdc_test_reserve.pubkey,
                usdc_test_reserve.liquidity_supply_pubkey,
                usdc_test_reserve.collateral_mint_pubkey,
                lending_market.pubkey,
                user_transfer_authority.pubkey(),
            ),
        ],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = banks_client.get_recent_blockhash().await.unwrap();
    transaction.sign(
        &[&payer, &user_accounts_owner, &user_transfer_authority],
        recent_blockhash,
    );

    assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
}
