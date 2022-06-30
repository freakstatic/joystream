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

//! Autogenerated weights for proposals_discussion
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-27, STEPS: `1`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=proposals_discussion
// --extrinsic=*
// --chain=dev
// --steps=1
// --repeat=1
// --execution=wasm
// --no-verify
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for proposals_discussion.
pub trait WeightInfo {
	fn add_post(i: u32, j: u32, ) -> Weight;
	fn update_post(j: u32, ) -> Weight;
	fn delete_post() -> Weight;
	fn change_thread_mode(i: u32, ) -> Weight;
}

/// Weights for proposals_discussion using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f0086dfeb47efcb121b0e718a654f15a6806f] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc03c4161dd5c06ffffe50605fefac36ad8] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f0086f7504996b40cd07b280a4167f76f4e57] (r:1 w:1)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f008649ccaabec42a2f78caeb903b78f5296e] (r:0 w:1)
	fn add_post(_i: u32, j: u32, ) -> Weight {
		(74_579_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f0086dfeb47efcb121b0e718a654f15a6806f] (r:1 w:0)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f008649ccaabec42a2f78caeb903b78f5296e] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn update_post(_j: u32, ) -> Weight {
		(38_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f008649ccaabec42a2f78caeb903b78f5296e] (r:1 w:1)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f0086dfeb47efcb121b0e718a654f15a6806f] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn delete_post() -> Weight {
		(43_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xde7bf41b08da4e09ac0e7c35311f0086dfeb47efcb121b0e718a654f15a6806f] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc03c4161dd5c06ffffe50605fefac36ad8] (r:1 w:0)
	fn change_thread_mode(_i: u32, ) -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn add_post(_i: u32, _j: u32, ) -> Weight {
		0
	}
	fn update_post(_j: u32, ) -> Weight {
		0
	}
	fn delete_post() -> Weight {
		0
	}
	fn change_thread_mode(_i: u32, ) -> Weight {
		0
	}
}
