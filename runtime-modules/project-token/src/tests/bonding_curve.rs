#![cfg(test)]

use crate::tests::fixtures::*;
use crate::tests::mock::*;
use crate::types::BondingCurve;
use crate::{member, token, Error};
use frame_support::{assert_err, assert_ok};
use sp_runtime::{DispatchError, Permill};

// --------------------- BONDING -------------------------------

#[test]
fn bonding_order_noop_ok_with_zero_requested_amount() {
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();
        let state_pre = sp_io::storage::root(sp_storage::StateVersion::V1);

        let result = BondFixture::default()
            .with_amount(0u32.into())
            .execute_call();

        let state_post = sp_io::storage::root(sp_storage::StateVersion::V1);
        assert_ok!(result);
        assert_eq!(state_pre, state_post);
    })
}

#[test]
fn bonding_order_fails_with_invalid_token_specified() {
    let config = GenesisConfigBuilder::new_empty().build();
    let token_id = token!(2);
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default()
            .with_token_id(token_id)
            .execute_call();

        assert_err!(result, Error::<Test>::TokenDoesNotExist);
    })
}

#[test]
fn bonding_order_fails_with_invalid_member_id() {
    let config = GenesisConfigBuilder::new_empty().build();
    let (member_id, _) = member!(2);
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default()
            .with_member_id(member_id)
            .execute_call();

        assert_err!(
            result,
            DispatchError::Other("origin signer not a member controller account")
        );
    })
}

#[test]
fn bonding_order_fails_with_invalid_origin() {
    let config = GenesisConfigBuilder::new_empty().build();
    let (_, sender) = member!(2);
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default().with_sender(sender).execute_call();

        assert_err!(
            result,
            DispatchError::Other("origin signer not a member controller account")
        );
    })
}

#[test]
fn bonding_fails_with_invalid_account_data_for_token() {
    let config = GenesisConfigBuilder::new_empty().build();
    let (member_id, sender) = member!(2);
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default()
            .with_sender(sender)
            .with_member_id(member_id)
            .execute_call();

        assert_err!(result, Error::<Test>::AccountInformationDoesNotExist);
    })
}

#[test]
fn bonding_order_fails_with_token_not_in_amm_state() {
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();

        let result = BondFixture::default().execute_call();

        assert_err!(result, Error::<Test>::NotInAmmState);
    })
}

#[test]
fn bonding_order_fails_with_past_timestamp() {
    let timestamp = 0u64;
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default().execute_call();

        assert_err!(result, Error::<Test>::InvalidTimestampSpecified);
    })
}

#[test]
fn bonding_order_fails_with_price_point_exceeding_numeric_boundaries() {
    let amount = Balance::max_value();
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default().with_amount(amount).execute_call();

        assert_err!(result, Error::<Test>::ArithmeticError);
    })
}

#[test]
fn creator_reward_correctly_accounted_during_bonding() {
    let (creator_member_id, _) = member!(1);
    let token_id = token!(1);
    let creator_reward = Permill::from_percent(10);
    let creator_amount = creator_reward.mul_floor(DEFAULT_BONDING_AMOUNT);

    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default()
            .with_creator_reward(creator_reward)
            .execute_call()
            .unwrap();
        let creator_reserve_pre =
            Token::account_info_by_token_and_member(token_id, creator_member_id).amount;

        BondFixture::default()
            .with_amount(DEFAULT_BONDING_AMOUNT)
            .execute_call()
            .unwrap();

        let creator_reserve_post =
            Token::account_info_by_token_and_member(token_id, creator_member_id).amount;
        assert_eq!(creator_reserve_post, creator_reserve_pre + creator_amount);
    })
}

#[test]
fn crt_issuance_increased_by_amount_during_bonding() {
    let config = GenesisConfigBuilder::new_empty().build();
    let token_id = token!(1);
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();
        let supply_pre = Token::token_info_by_id(token_id).total_supply;

        BondFixture::default()
            .with_amount(DEFAULT_BONDING_AMOUNT)
            .execute_call()
            .unwrap();

        let supply_post = Token::token_info_by_id(token_id).total_supply;
        assert_eq!(supply_post, supply_pre + DEFAULT_BONDING_AMOUNT);
    })
}

#[test]
fn amm_treasury_balance_correctly_increased_during_bonding() {
    let token_id = token!(1);
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();
        let amm_reserve_account = Token::module_bonding_curve_reserve_account(token_id);
        let amm_reserve_pre = Balances::usable_balance(amm_reserve_account);

        BondFixture::default()
            .with_amount(DEFAULT_BONDING_AMOUNT)
            .execute_call()
            .unwrap();

        let amm_reserve_post = Balances::usable_balance(amm_reserve_account);
        let correctly_computed_joy_amount = 0; // TODO: fix this
        assert_eq!(
            amm_reserve_pre,
            amm_reserve_post + correctly_computed_joy_amount
        );
    })
}

#[test]
fn bonding_fails_with_user_not_having_sufficient_usable_joy_required() {
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = BondFixture::default().execute_call();

        assert_err!(result, Error::<Test>::InsufficientJoyBalance);
    })
}

#[test]
fn user_joy_balance_correctly_decreased_during_bonding() {
    let (_, user_account) = member!(2);
    let config = GenesisConfigBuilder::new_empty().build();

    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();
        let user_reserve_pre = Balances::usable_balance(user_account);

        BondFixture::default()
            .with_amount(DEFAULT_BONDING_AMOUNT)
            .execute_call()
            .unwrap();

        let user_reserve_post = Balances::usable_balance(user_account);
        let correctly_computed_joy_amount = 0; // TODO: fix this
        assert_eq!(
            user_reserve_pre,
            user_reserve_post + correctly_computed_joy_amount
        );
    })
}

#[test]
fn crt_correctly_minted_to_user_during_bonding() {
    let (user_member_id, _) = member!(2);
    let token_id = token!(1);
    let creator_reward = Permill::from_percent(10);
    let creator_amount = creator_reward.mul_floor(DEFAULT_BONDING_AMOUNT);

    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        ActivateAmmFixture::default()
            .with_creator_reward(creator_reward)
            .execute_call()
            .unwrap();
        let user_crt_pre = Token::account_info_by_token_and_member(token_id, user_member_id).amount;

        BondFixture::default()
            .with_amount(DEFAULT_BONDING_AMOUNT)
            .execute_call()
            .unwrap();

        let user_crt_post =
            Token::account_info_by_token_and_member(token_id, user_member_id).amount;
        assert_eq!(
            user_crt_post - user_crt_pre,
            DEFAULT_BONDING_AMOUNT - creator_amount
        );
    })
}

// --------------- ACTIVATION ----------------------------------

#[test]
fn amm_activation_fails_with_invalid_account_id() {
    let (_, user_account_id) = member!(2);
    let config = GenesisConfigBuilder::new_empty().build();

    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        let result = ActivateAmmFixture::default()
            .with_sender(user_account_id)
            .execute_call();

        assert_err!(
            result,
            DispatchError::Other("origin signer not a member controller account")
        );
    })
}

#[test]
fn amm_activation_fails_with_invalid_member_id() {
    let (user_member_id, _) = member!(2);
    let config = GenesisConfigBuilder::new_empty().build();

    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        let result = ActivateAmmFixture::default()
            .with_member_id(user_member_id)
            .execute_call();

        assert_err!(
            result,
            DispatchError::Other("origin signer not a member controller account")
        );
    })
}

#[test]
fn amm_activation_fails_with_invalid_creator() {
    let (creator_member_id, creator_account_id) = member!(2);
    let config = GenesisConfigBuilder::new_empty().build();

    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        let result = ActivateAmmFixture::default()
            .with_sender(creator_account_id)
            .with_member_id(creator_member_id)
            .execute_call();

        assert_err!(result, Error::<Test>::AccountInformationDoesNotExist);
    })
}

#[test]
fn amm_activation_fails_with_invalid_token_id() {
    let token_id = token!(2);
    let config = GenesisConfigBuilder::new_empty().build();

    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        let result = ActivateAmmFixture::default()
            .with_token_id(token_id)
            .execute_call();

        assert_err!(result, Error::<Test>::TokenDoesNotExist);
    })
}

#[test]
fn activation_fails_when_status_is_not_idle() {
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();
        InitTokenSaleFixture::default().call_and_assert(Ok(()));

        let result = ActivateAmmFixture::default().execute_call();

        assert_err!(result, Error::<Test>::TokenIssuanceNotInIdleState);
    })
}

#[test]
fn activation_fails_when_amm_status_already_active() {
    let config = GenesisConfigBuilder::new_empty().build();
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default()
            .with_empty_allocation()
            .execute_call()
            .unwrap();
        ActivateAmmFixture::default().execute_call().unwrap();

        let result = ActivateAmmFixture::default().execute_call();

        assert_err!(result, Error::<Test>::AlreadyInAmmState);
    })
}

#[test]
fn amm_activation_successful() {
    let slope = Permill::from_perthousand(1);
    let intercept = Permill::from_perthousand(1);
    let config = GenesisConfigBuilder::new_empty().build();
    let creator_reward = Permill::from_percent(10);
    build_test_externalities(config).execute_with(|| {
        IssueTokenFixture::default().execute_call().unwrap();

        ActivateAmmFixture::default()
            .with_linear_function_params(slope, intercept)
            .execute_call()
            .unwrap();

        let token = Token::token_info_by_id(1);
        assert_eq!(
            IssuanceState::of::<Test>(&token),
            IssuanceState::BondingCurve(BondingCurve {
                slope,
                intercept,
                creator_reward
            })
        );
    })
}
// --------------------- UNBONDING -------------------------------

#[test]
fn unbonding_order_noop_ok_with_zero_requested_amount() {}

#[test]
fn unbonding_order_fails_with_requested_amount_exceeding_boundaries() {}

#[test]
fn unbonding_order_fails_with_past_timestamp() {}

#[test]
fn unbonding_order_fails_with_zero_price_point_and_non_zero_requested_amount() {}

#[test]
fn unbonding_order_fails_with_price_point_exceeding_boundaries() {}

#[test]
fn unbonding_pricing_computation_works_with_zero_request() {}

#[test]
fn unbonding_pricing_computation_works_with_max_request_amount() {}

#[test]
fn unbonding_pricing_computation_works_with_selected_random_points() {}

// TODO: provide easy to use correct pairs (x, y = F(x)) for testing
#[test]
fn slippage_tolerance_respected_during_unbonding() {}

#[test]
fn deadline_time_limit_respected_during_unbonding() {}

#[test]
fn tx_fees_correctly_accounted_during_unbonding() {}

#[test]
fn crt_issuance_decreased_by_amount_during_unbonding() {}

#[test]
fn amm_treasury_balance_decreased_during_unbonding() {}

#[test]
fn unbonding_fails_with_user_not_having_sufficient_usable_crt_required() {}

#[test]
fn user_joy_balance_correctly_increased_during_unbonding() {}

#[test]
fn crt_correctly_burned_to_user_during_unbonding() {}
