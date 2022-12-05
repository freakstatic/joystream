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

//! Autogenerated weights for project_token
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=project_token
// --extrinsic=*
// --chain=prod-test
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=./scripts/../runtime-modules/project-token/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for project_token.
pub trait WeightInfo {
	fn transfer(_o: u32, _m: u32, ) -> Weight;
	fn dust_account() -> Weight;
	fn join_whitelist(_h: u32, ) -> Weight;
	fn purchase_tokens_on_sale() -> Weight;
	fn participate_in_split() -> Weight;
	fn exit_revenue_split() -> Weight;
	fn burn() -> Weight;
	fn buy_on_amm_with_account_creation() -> Weight;
	fn buy_on_amm_with_existing_account() -> Weight;
	fn sell_on_amm() -> Weight;
}

/// Weights for project_token using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Membership MembershipById (r:1025 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1025 w:1025)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Token BloatBond (r:1 w:0)
	// Proof: Token BloatBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	fn transfer(o: u32, m: u32, ) -> Weight {
		(7_287_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((12_789_000 as Weight).saturating_mul(o as Weight))
			// Standard Error: 154_000
			.saturating_add((1_433_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(o as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(o as Weight)))
	}
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn dust_account() -> Weight {
		(53_111_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Token BloatBond (r:1 w:0)
	// Proof: Token BloatBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `h` is `[1, 10]`.
	fn join_whitelist(h: u32, ) -> Weight {
		(64_273_000 as Weight)
			// Standard Error: 76_000
			.saturating_add((680_000 as Weight).saturating_mul(h as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token SalePlatformFee (r:1 w:0)
	// Proof: Token SalePlatformFee (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Token BloatBond (r:1 w:0)
	// Proof: Token BloatBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn purchase_tokens_on_sale() -> Weight {
		(87_433_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn participate_in_split() -> Weight {
		(57_921_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:0)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	fn exit_revenue_split() -> Weight {
		(33_145_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	fn burn() -> Weight {
		(35_257_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Storage: Token BloatBond (r:1 w:0)
	// Storage: Token AmmBuyTxFees (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Timestamp Now (r:1 w:0)
	fn buy_on_amm_with_account_creation() -> Weight {
		(74_786_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Storage: Token BloatBond (r:1 w:0)
	// Storage: Token AmmBuyTxFees (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Timestamp Now (r:1 w:0)
	fn buy_on_amm_with_existing_account() -> Weight {
		(75_190_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Token AmmSellTxFees (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn sell_on_amm() -> Weight {
		(61_192_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn transfer(o: u32, _m: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn dust_account() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn join_whitelist(h: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn purchase_tokens_on_sale() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn participate_in_split() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn exit_revenue_split() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn burn() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn buy_on_amm_with_account_creation() -> Weight {
		0
	}
	fn buy_on_amm_with_existing_account() -> Weight {
		0
	}
	fn sell_on_amm() -> Weight {
		0
	}
}
