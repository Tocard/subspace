
//! Autogenerated weights for `pallet_runtime_configs`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.2.0
//! DATE: 2025-07-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `protocol-team-testing`, CPU: `AMD Ryzen 5 3600 6-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/subspace-node
// benchmark
// pallet
// --runtime=./target/production/wbuild/subspace-runtime/subspace_runtime.compact.compressed.wasm
// --extrinsic=*
// --wasm-execution=compiled
// --genesis-builder=none
// --steps=50
// --repeat=20
// --heap-pages=4096
// --pallet=pallet_runtime_configs
// --output=./crates/subspace-runtime/src/weights/pallet_runtime_configs.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_runtime_configs`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_runtime_configs::WeightInfo for WeightInfo<T> {
	/// Storage: `RuntimeConfigs::EnableDomains` (r:0 w:1)
	/// Proof: `RuntimeConfigs::EnableDomains` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn set_enable_domains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_950_000 picoseconds.
		Weight::from_parts(2_030_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `RuntimeConfigs::EnableDynamicCostOfStorage` (r:0 w:1)
	/// Proof: `RuntimeConfigs::EnableDynamicCostOfStorage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn set_enable_dynamic_cost_of_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_940_000 picoseconds.
		Weight::from_parts(2_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `RuntimeConfigs::EnableBalanceTransfers` (r:0 w:1)
	/// Proof: `RuntimeConfigs::EnableBalanceTransfers` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn set_enable_balance_transfers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_900_000 picoseconds.
		Weight::from_parts(2_010_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
