#![cfg(feature = "test-bpf")]

mod helpers;

use helpers::*;
use paychains_program_test::*;
use paychains_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::instruction::approve;
use spl_token_lending::{
    instruction::deposit_obligation_collateral, processor::process_instruction,
    state::INITIAL_COLLATERAL_RATIO,
};

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new(
        "spl_token_lending",
        spl_token_lending::id(),
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_compute_max_units(38_000);

    const PAY_DEPOSIT_AMOUNT_LAMPORTS: u64 = 10 * LAMPORTS_TO_PAY * INITIAL_COLLATERAL_RATIO;
    const PAY_RESERVE_COLLATERAL_LAMPORTS: u64 = 2 * PAY_DEPOSIT_AMOUNT_LAMPORTS;

    let user_accounts_owner = Keypair::new();
    let user_transfer_authority = Keypair::new();

    let lending_market = add_lending_market(&mut test);

    let pay_oracle = add_pay_oracle(&mut test);
    let pay_test_reserve = add_reserve(
        &mut test,
        &lending_market,
        &pay_oracle,
        &user_accounts_owner,
        AddReserveArgs {
            user_liquidity_amount: PAY_RESERVE_COLLATERAL_LAMPORTS,
            liquidity_amount: PAY_RESERVE_COLLATERAL_LAMPORTS,
            liquidity_mint_decimals: 9,
            liquidity_mint_pubkey: spl_token::native_mint::id(),
            config: TEST_RESERVE_CONFIG,
            mark_fresh: true,
            ..AddReserveArgs::default()
        },
    );

    let test_obligation = add_obligation(
        &mut test,
        &lending_market,
        &user_accounts_owner,
        AddObligationArgs::default(),
    );

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    test_obligation.validate_state(&mut banks_client).await;

    let initial_collateral_supply_balance =
        get_token_balance(&mut banks_client, pay_test_reserve.collateral_supply_pubkey).await;
    let initial_user_collateral_balance =
        get_token_balance(&mut banks_client, pay_test_reserve.user_collateral_pubkey).await;

    let mut transaction = Transaction::new_with_payer(
        &[
            approve(
                &spl_token::id(),
                &pay_test_reserve.user_collateral_pubkey,
                &user_transfer_authority.pubkey(),
                &user_accounts_owner.pubkey(),
                &[],
                PAY_DEPOSIT_AMOUNT_LAMPORTS,
            )
            .unwrap(),
            deposit_obligation_collateral(
                spl_token_lending::id(),
                PAY_DEPOSIT_AMOUNT_LAMPORTS,
                pay_test_reserve.user_collateral_pubkey,
                pay_test_reserve.collateral_supply_pubkey,
                pay_test_reserve.pubkey,
                test_obligation.pubkey,
                lending_market.pubkey,
                test_obligation.owner,
                user_transfer_authority.pubkey(),
            ),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(
        &vec![&payer, &user_accounts_owner, &user_transfer_authority],
        recent_blockhash,
    );
    assert!(banks_client.process_transaction(transaction).await.is_ok());

    // check that collateral tokens were transferred
    let collateral_supply_balance =
        get_token_balance(&mut banks_client, pay_test_reserve.collateral_supply_pubkey).await;
    assert_eq!(
        collateral_supply_balance,
        initial_collateral_supply_balance + PAY_DEPOSIT_AMOUNT_LAMPORTS
    );
    let user_collateral_balance =
        get_token_balance(&mut banks_client, pay_test_reserve.user_collateral_pubkey).await;
    assert_eq!(
        user_collateral_balance,
        initial_user_collateral_balance - PAY_DEPOSIT_AMOUNT_LAMPORTS
    );
}
