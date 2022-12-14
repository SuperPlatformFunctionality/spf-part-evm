// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_randomness
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_randomness
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --record-proof
// --json-file
// raw.json
// --output
// weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_randomness.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn request_randomness() -> Weight;
	#[rustfmt::skip]
	fn prepare_fulfillment() -> Weight;
	#[rustfmt::skip]
	fn finish_fulfillment() -> Weight;
	#[rustfmt::skip]
	fn increase_fee() -> Weight;
	#[rustfmt::skip]
	fn execute_request_expiration() -> Weight;
}

/// Weights for pallet_randomness using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(pub PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Randomness RequestCount (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Randomness RandomnessResults (r:1 w:1)
	// Storage: Randomness Requests (r:0 w:1)
	#[rustfmt::skip]
	fn request_randomness() -> Weight {
		(58_126_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Randomness Requests (r:1 w:0)
	// Storage: Randomness RandomnessResults (r:1 w:0)
	#[rustfmt::skip]
	fn prepare_fulfillment() -> Weight {
		(42_051_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Randomness RandomnessResults (r:1 w:1)
	// Storage: Randomness Requests (r:0 w:1)
	#[rustfmt::skip]
	fn finish_fulfillment() -> Weight {
		(46_585_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Randomness Requests (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	#[rustfmt::skip]
	fn increase_fee() -> Weight {
		(42_952_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Randomness Requests (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Randomness RandomnessResults (r:1 w:1)
	#[rustfmt::skip]
	fn execute_request_expiration() -> Weight {
		(49_749_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Randomness RequestCount (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Randomness RandomnessResults (r:1 w:1)
	// Storage: Randomness Requests (r:0 w:1)
	#[rustfmt::skip]
	fn request_randomness() -> Weight {
		(58_126_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Randomness Requests (r:1 w:0)
	// Storage: Randomness RandomnessResults (r:1 w:0)
	#[rustfmt::skip]
	fn prepare_fulfillment() -> Weight {
		(42_051_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Randomness RandomnessResults (r:1 w:1)
	// Storage: Randomness Requests (r:0 w:1)
	#[rustfmt::skip]
	fn finish_fulfillment() -> Weight {
		(46_585_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Randomness Requests (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	#[rustfmt::skip]
	fn increase_fee() -> Weight {
		(42_952_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Randomness Requests (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Randomness RandomnessResults (r:1 w:1)
	#[rustfmt::skip]
	fn execute_request_expiration() -> Weight {
		(49_749_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}
