use sp_arithmetic::traits::{One, Zero};
use sp_runtime::traits::Hash;
use sp_runtime::Perquintill;
use sp_std::collections::btree_map::BTreeMap;

use crate::tests::mock::*;
use crate::types::{
    AccountData, AccountDataOf, BlockRate, MerkleProof, MerkleSide, PatronageData, Payment,
    TokenAllocation, TokenIssuanceParameters, TokenSaleId, TokenSaleOf, TransferPolicy,
    TransferPolicyOf, Transfers,
};
use crate::{balance, GenesisConfig};

pub struct TokenDataBuilder {
    pub(crate) total_supply: <Test as crate::Trait>::Balance,
    pub(crate) tokens_issued: <Test as crate::Trait>::Balance,
    pub(crate) sale: Option<TokenSaleOf<Test>>,
    pub(crate) next_sale_id: TokenSaleId,
    pub(crate) transfer_policy: TransferPolicyOf<Test>,
    pub(crate) patronage_info:
        PatronageData<<Test as crate::Trait>::Balance, <Test as frame_system::Trait>::BlockNumber>,
    pub(crate) symbol: <Test as frame_system::Trait>::Hash,
}

impl TokenDataBuilder {
    pub fn build(self) -> crate::types::TokenDataOf<Test> {
        crate::types::TokenDataOf::<Test> {
            total_supply: self.total_supply,
            tokens_issued: self.tokens_issued,
            sale: self.sale,
            next_sale_id: self.next_sale_id,
            transfer_policy: self.transfer_policy,
            patronage_info: self.patronage_info,
            symbol: self.symbol,
            accounts_number: 0u64,
        }
    }

    pub fn with_symbol(self, symbol: <Test as frame_system::Trait>::Hash) -> Self {
        Self { symbol, ..self }
    }

    pub fn with_supply(self, supply: Balance) -> Self {
        Self {
            total_supply: supply,
            tokens_issued: supply,
            ..self
        }
    }

    pub fn with_transfer_policy(self, transfer_policy: TransferPolicyOf<Test>) -> Self {
        Self {
            transfer_policy,
            ..self
        }
    }

    pub fn with_patronage_rate(self, rate: BlockRate) -> Self {
        Self {
            patronage_info: PatronageData::<_, _> {
                unclaimed_patronage_tally_amount: Balance::zero(),
                rate,
                last_unclaimed_patronage_tally_block: BlockNumber::one(),
            },
            ..self
        }
    }

    pub fn new_empty() -> Self {
        Self {
            tokens_issued: Balance::zero(),
            total_supply: Balance::zero(),
            sale: None,
            next_sale_id: 0,
            transfer_policy: TransferPolicy::Permissionless,
            patronage_info: PatronageData::<Balance, BlockNumber> {
                rate: BlockRate(Perquintill::zero()),
                unclaimed_patronage_tally_amount: Balance::zero(),
                last_unclaimed_patronage_tally_block: BlockNumber::one(),
            },
            // hash of "default"
            symbol: <Test as frame_system::Trait>::Hash::default(),
        }
    }
}

impl GenesisConfigBuilder {
    pub fn new_empty() -> Self {
        Self {
            token_info_by_id: vec![],
            account_info_by_token_and_member: vec![],
            next_token_id: TokenId::one(),
            symbol_used: vec![],
            bloat_bond: JoyBalance::zero(),
            min_sale_duration: BlockNumber::zero(),
        }
    }

    // add token with given params & zero supply
    pub fn with_token(mut self, token_id: TokenId, token_info: TokenData) -> Self {
        self.symbol_used = vec![(token_info.symbol.clone(), ())];
        self.token_info_by_id.push((token_id, token_info));
        self.next_token_id = self.next_token_id.saturating_add(TokenId::one());
        self
    }

    // add token and owner: useful for tests
    pub fn with_token_and_owner(
        self,
        token_id: TokenId,
        token_info: TokenData,
        owner: MemberId,
        initial_supply: Balance,
    ) -> Self {
        self.with_token(token_id, token_info)
            .with_account(owner, AccountData::new_with_amount(initial_supply))
    }

    pub fn with_bloat_bond(self, bloat_bond: JoyBalance) -> Self {
        Self { bloat_bond, ..self }
    }

    pub fn with_min_sale_duration(self, min_sale_duration: BlockNumber) -> Self {
        Self {
            min_sale_duration,
            ..self
        }
    }

    // add account & updates token supply
    pub fn with_account(mut self, member_id: MemberId, account_data: AccountDataOf<Test>) -> Self {
        let id = self.next_token_id.saturating_sub(TokenId::one());

        self.token_info_by_id
            .last_mut()
            .unwrap()
            .1
            .increase_supply_by(Balance::from(account_data.amount));

        self.account_info_by_token_and_member
            .push((id, member_id, account_data));

        self.token_info_by_id.last_mut().unwrap().1.accounts_number += 1u64;
        self
    }

    pub fn build(self) -> GenesisConfig<Test> {
        GenesisConfig::<Test> {
            account_info_by_token_and_member: self.account_info_by_token_and_member,
            token_info_by_id: self.token_info_by_id,
            next_token_id: self.next_token_id,
            symbol_used: self.symbol_used,
            bloat_bond: self.bloat_bond,
            min_sale_duration: self.min_sale_duration,
        }
    }
}

impl<VestingSchedule, Balance: Zero, StakingStatus, ReserveBalance: Zero>
    AccountData<VestingSchedule, Balance, StakingStatus, ReserveBalance>
{
    pub fn new_with_amount(amount: Balance) -> Self {
        Self {
            amount,
            ..Self::default()
        }
    }
}

impl<Hasher: Hash> MerkleProof<Hasher> {
    pub fn new(v: Vec<(Hasher::Output, MerkleSide)>) -> Self {
        MerkleProof::<Hasher>(v)
    }
}

impl<Balance, Account: Ord> Transfers<Account, Balance> {
    pub fn new(v: Vec<(Account, Balance)>) -> Self {
        Transfers::<_, _>(
            v.into_iter()
                .map(|(acc, amount)| {
                    (
                        acc,
                        Payment::<Balance> {
                            remark: vec![],
                            amount,
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<Hash, Balance, VestingScheduleParams, TransferPolicyParams, MemberId>
    TokenIssuanceParameters<
        Hash,
        TokenAllocation<Balance, VestingScheduleParams>,
        TransferPolicyParams,
        MemberId,
    >
where
    MemberId: Ord + Clone,
    Balance: Clone,
    VestingScheduleParams: Clone,
{
    pub fn with_allocation(
        self,
        member_id: &MemberId,
        amount: Balance,
        vesting_schedule_params: Option<VestingScheduleParams>,
    ) -> Self {
        let mut initial_allocation = self.initial_allocation.clone();
        initial_allocation.insert(
            member_id.clone(),
            TokenAllocation {
                amount,
                vesting_schedule_params,
            },
        );
        Self {
            initial_allocation,
            ..self
        }
    }
}

#[cfg(test)]
#[test]
fn with_token_assigns_correct_token_id() {
    let token_id: TokenId = 1;
    let token_params = TokenDataBuilder::new_empty().build();

    let builder = GenesisConfigBuilder::new_empty().with_token(token_id, token_params);

    let id = builder.token_info_by_id.last().unwrap().0;
    assert_eq!(id, token_id);
}

#[test]
fn with_supply_adds_supply_to_token() {
    let token_params = TokenDataBuilder::new_empty().with_supply(5).build();

    let builder = GenesisConfigBuilder::new_empty().with_token(1, token_params);

    let token = &builder.token_info_by_id.last().unwrap().1;
    assert_eq!(token.tokens_issued, 5);
    assert_eq!(token.total_supply, 5);
}

#[test]
fn adding_account_with_tokens_also_adds_supply() {
    let token_params = TokenDataBuilder::new_empty().with_supply(5).build();
    let mut builder = GenesisConfigBuilder::new_empty().with_token(1, token_params);
    builder = builder.with_account(1, AccountData::new_with_amount(balance!(5)));

    let token = &builder.token_info_by_id.last().unwrap().1;
    assert_eq!(token.tokens_issued, 10);
    assert_eq!(token.total_supply, 10);
}
