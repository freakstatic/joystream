// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for proposals_codex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=proposals_codex
// --extrinsic=*
// --chain=prod-test
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=./scripts/../runtime-modules/proposals/codex/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for proposals_codex.
pub trait WeightInfo {
	fn create_proposal_signal(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_runtime_upgrade(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_funding_request(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_max_validator_count(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_veto_proposal(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_create_working_group_lead_opening(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_fill_working_group_lead_opening(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_update_working_group_budget(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_decrease_working_group_lead_stake(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_slash_working_group_lead(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_working_group_lead_reward(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_terminate_working_group_lead(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_amend_constitution(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_cancel_working_group_lead_opening(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_membership_price(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_council_budget_increment(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_councilor_reward(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_initial_invitation_balance(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_initial_invitation_count(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_membership_lead_invitation_quota(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_referral_cut(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_update_global_nft_limit(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_update_channel_payouts(_t: u32, _d: u32, _i: u32, ) -> Weight;
}

/// Weights for proposals_codex using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_signal(i: u32, _t: u32, _d: u32, ) -> Weight {
		(177_081_000 as Weight)
			// Standard Error: 71_000
			.saturating_add((2_578_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_runtime_upgrade(i: u32, t: u32, _d: u32, ) -> Weight {
		(88_060_000 as Weight)
			// Standard Error: 58_000
			.saturating_add((2_398_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 459_000
			.saturating_add((3_702_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_funding_request(i: u32, _t: u32, d: u32, ) -> Weight {
		(173_995_000 as Weight)
			// Standard Error: 535_000
			.saturating_add((1_465_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 535_000
			.saturating_add((3_025_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_max_validator_count(t: u32, d: u32, ) -> Weight {
		(138_842_000 as Weight)
			// Standard Error: 385_000
			.saturating_add((958_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 385_000
			.saturating_add((1_770_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: ProposalEngine Proposals (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_veto_proposal(t: u32, d: u32, ) -> Weight {
		(138_944_000 as Weight)
			// Standard Error: 71_000
			.saturating_add((877_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 71_000
			.saturating_add((1_409_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_create_working_group_lead_opening(i: u32, t: u32, d: u32, ) -> Weight {
		(118_905_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((2_055_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 211_000
			.saturating_add((1_484_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 211_000
			.saturating_add((1_386_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup OpeningById (r:1 w:0)
	// Storage: Instance1WorkingGroup ApplicationById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_fill_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		(135_561_000 as Weight)
			// Standard Error: 40_000
			.saturating_add((1_335_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 40_000
			.saturating_add((1_551_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_update_working_group_budget(t: u32, d: u32, ) -> Weight {
		(111_798_000 as Weight)
			// Standard Error: 247_000
			.saturating_add((1_888_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 247_000
			.saturating_add((1_962_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_decrease_working_group_lead_stake(_t: u32, d: u32, ) -> Weight {
		(186_324_000 as Weight)
			// Standard Error: 449_000
			.saturating_add((3_971_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_slash_working_group_lead(t: u32, d: u32, ) -> Weight {
		(142_959_000 as Weight)
			// Standard Error: 80_000
			.saturating_add((440_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 80_000
			.saturating_add((1_427_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_working_group_lead_reward(t: u32, d: u32, ) -> Weight {
		(100_103_000 as Weight)
			// Standard Error: 250_000
			.saturating_add((2_745_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 250_000
			.saturating_add((2_772_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_terminate_working_group_lead(t: u32, d: u32, ) -> Weight {
		(152_540_000 as Weight)
			// Standard Error: 165_000
			.saturating_add((325_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 165_000
			.saturating_add((1_157_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_amend_constitution(i: u32, t: u32, d: u32, ) -> Weight {
		(136_524_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((1_861_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 62_000
			.saturating_add((873_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 62_000
			.saturating_add((969_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup OpeningById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_cancel_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		(126_233_000 as Weight)
			// Standard Error: 34_000
			.saturating_add((1_318_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 34_000
			.saturating_add((1_391_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_membership_price(t: u32, d: u32, ) -> Weight {
		(117_381_000 as Weight)
			// Standard Error: 36_000
			.saturating_add((1_260_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 36_000
			.saturating_add((1_477_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_council_budget_increment(t: u32, d: u32, ) -> Weight {
		(122_309_000 as Weight)
			// Standard Error: 187_000
			.saturating_add((1_487_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 187_000
			.saturating_add((1_108_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_councilor_reward(t: u32, d: u32, ) -> Weight {
		(115_634_000 as Weight)
			// Standard Error: 105_000
			.saturating_add((1_380_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 105_000
			.saturating_add((1_517_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_initial_invitation_balance(t: u32, d: u32, ) -> Weight {
		(112_279_000 as Weight)
			// Standard Error: 99_000
			.saturating_add((1_458_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 99_000
			.saturating_add((1_654_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_initial_invitation_count(t: u32, d: u32, ) -> Weight {
		(116_818_000 as Weight)
			// Standard Error: 34_000
			.saturating_add((1_286_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 34_000
			.saturating_add((1_450_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_membership_lead_invitation_quota(_t: u32, d: u32, ) -> Weight {
		(175_230_000 as Weight)
			// Standard Error: 344_000
			.saturating_add((1_175_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_referral_cut(t: u32, d: u32, ) -> Weight {
		(117_238_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((1_242_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 28_000
			.saturating_add((1_455_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_update_global_nft_limit(t: u32, d: u32, ) -> Weight {
		(115_819_000 as Weight)
			// Standard Error: 37_000
			.saturating_add((1_384_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 37_000
			.saturating_add((1_492_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_update_channel_payouts(t: u32, d: u32, i: u32, ) -> Weight {
		(116_918_000 as Weight)
			// Standard Error: 64_000
			.saturating_add((986_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 64_000
			.saturating_add((1_054_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 9_000
			.saturating_add((1_867_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_proposal_signal(i: u32, _t: u32, _d: u32, ) -> Weight {
		0
	}
	fn create_proposal_runtime_upgrade(i: u32, t: u32, _d: u32, ) -> Weight {
		0
	}
	fn create_proposal_funding_request(i: u32, _t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_max_validator_count(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_veto_proposal(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_create_working_group_lead_opening(i: u32, t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_fill_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_update_working_group_budget(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_decrease_working_group_lead_stake(_t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_slash_working_group_lead(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_working_group_lead_reward(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_terminate_working_group_lead(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_amend_constitution(i: u32, t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_cancel_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_membership_price(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_council_budget_increment(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_councilor_reward(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_initial_invitation_balance(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_initial_invitation_count(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_membership_lead_invitation_quota(_t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_referral_cut(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_update_global_nft_limit(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_update_channel_payouts(t: u32, d: u32, i: u32, ) -> Weight {
		0
	}
}
