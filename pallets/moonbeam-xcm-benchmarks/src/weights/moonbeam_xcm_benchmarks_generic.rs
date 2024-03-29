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

//! Autogenerated weights for moonbeam_xcm_benchmarks_generic
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// moonbeam_xcm_benchmarks_generic
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
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

/// Weight functions needed for moonbeam_xcm_benchmarks_generic.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn query_holding() -> Weight;
	#[rustfmt::skip]
	fn buy_execution() -> Weight;
	#[rustfmt::skip]
	fn query_response() -> Weight;
	#[rustfmt::skip]
	fn transact() -> Weight;
	#[rustfmt::skip]
	fn refund_surplus() -> Weight;
	#[rustfmt::skip]
	fn set_error_handler() -> Weight;
	#[rustfmt::skip]
	fn set_appendix() -> Weight;
	#[rustfmt::skip]
	fn clear_error() -> Weight;
	#[rustfmt::skip]
	fn descend_origin() -> Weight;
	#[rustfmt::skip]
	fn clear_origin() -> Weight;
	#[rustfmt::skip]
	fn report_error() -> Weight;
	#[rustfmt::skip]
	fn claim_asset() -> Weight;
	#[rustfmt::skip]
	fn trap() -> Weight;
	#[rustfmt::skip]
	fn subscribe_version() -> Weight;
	#[rustfmt::skip]
	fn unsubscribe_version() -> Weight;
	#[rustfmt::skip]
	fn initiate_reserve_withdraw() -> Weight;
}

/// Weights for moonbeam_xcm_benchmarks_generic using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn query_holding() -> Weight {
		Weight::from_ref_time(392_845_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:0)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:0)
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	#[rustfmt::skip]
	fn buy_execution() -> Weight {
		Weight::from_ref_time(130_464_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
	}
	// Storage: PolkadotXcm Queries (r:1 w:0)
	#[rustfmt::skip]
	fn query_response() -> Weight {
		Weight::from_ref_time(24_677_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: MaintenanceMode MaintenanceMode (r:1 w:0)
	#[rustfmt::skip]
	fn transact() -> Weight {
		Weight::from_ref_time(31_693_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	#[rustfmt::skip]
	fn refund_surplus() -> Weight {
		Weight::from_ref_time(25_506_000 as u64)
	}
	#[rustfmt::skip]
	fn set_error_handler() -> Weight {
		Weight::from_ref_time(8_089_000 as u64)
	}
	#[rustfmt::skip]
	fn set_appendix() -> Weight {
		Weight::from_ref_time(8_110_000 as u64)
	}
	#[rustfmt::skip]
	fn clear_error() -> Weight {
		Weight::from_ref_time(8_222_000 as u64)
	}
	#[rustfmt::skip]
	fn descend_origin() -> Weight {
		Weight::from_ref_time(9_620_000 as u64)
	}
	#[rustfmt::skip]
	fn clear_origin() -> Weight {
		Weight::from_ref_time(8_268_000 as u64)
	}
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn report_error() -> Weight {
		Weight::from_ref_time(24_787_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	#[rustfmt::skip]
	fn claim_asset() -> Weight {
		Weight::from_ref_time(17_798_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	#[rustfmt::skip]
	fn trap() -> Weight {
		Weight::from_ref_time(8_424_000 as u64)
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn subscribe_version() -> Weight {
		Weight::from_ref_time(30_071_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	#[rustfmt::skip]
	fn unsubscribe_version() -> Weight {
		Weight::from_ref_time(12_915_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn initiate_reserve_withdraw() -> Weight {
		Weight::from_ref_time(465_091_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn query_holding() -> Weight {
		Weight::from_ref_time(392_845_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:0)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:0)
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	#[rustfmt::skip]
	fn buy_execution() -> Weight {
		Weight::from_ref_time(130_464_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
	}
	// Storage: PolkadotXcm Queries (r:1 w:0)
	#[rustfmt::skip]
	fn query_response() -> Weight {
		Weight::from_ref_time(24_677_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: MaintenanceMode MaintenanceMode (r:1 w:0)
	#[rustfmt::skip]
	fn transact() -> Weight {
		Weight::from_ref_time(31_693_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	#[rustfmt::skip]
	fn refund_surplus() -> Weight {
		Weight::from_ref_time(25_506_000 as u64)
	}
	#[rustfmt::skip]
	fn set_error_handler() -> Weight {
		Weight::from_ref_time(8_089_000 as u64)
	}
	#[rustfmt::skip]
	fn set_appendix() -> Weight {
		Weight::from_ref_time(8_110_000 as u64)
	}
	#[rustfmt::skip]
	fn clear_error() -> Weight {
		Weight::from_ref_time(8_222_000 as u64)
	}
	#[rustfmt::skip]
	fn descend_origin() -> Weight {
		Weight::from_ref_time(9_620_000 as u64)
	}
	#[rustfmt::skip]
	fn clear_origin() -> Weight {
		Weight::from_ref_time(8_268_000 as u64)
	}
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn report_error() -> Weight {
		Weight::from_ref_time(24_787_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	#[rustfmt::skip]
	fn claim_asset() -> Weight {
		Weight::from_ref_time(17_798_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	#[rustfmt::skip]
	fn trap() -> Weight {
		Weight::from_ref_time(8_424_000 as u64)
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn subscribe_version() -> Weight {
		Weight::from_ref_time(30_071_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	#[rustfmt::skip]
	fn unsubscribe_version() -> Weight {
		Weight::from_ref_time(12_915_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn initiate_reserve_withdraw() -> Weight {
		Weight::from_ref_time(465_091_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
