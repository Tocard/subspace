
//! Autogenerated weights for pallet_rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-09-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Ubuntu-2404-noble-amd64-base`, CPU: `Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/subspace-node
// benchmark
// pallet
// --runtime=./target/release/wbuild/subspace-runtime/subspace_runtime.compact.compressed.wasm
// --genesis-builder=runtime
// --steps=50
// --repeat=20
// --pallet=pallet_rewards
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet-rewards.rs
// --template=./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::ParityDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_rewards.
pub trait WeightInfo {
	fn update_issuance_params(p: u32, v: u32, ) -> Weight;
}

/// Weights for pallet_rewards using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Rewards::ProposerSubsidyPoints` (r:0 w:1)
	/// Proof: `Rewards::ProposerSubsidyPoints` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `Rewards::VoterSubsidyPoints` (r:0 w:1)
	/// Proof: `Rewards::VoterSubsidyPoints` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 20]`.
	/// The range of component `v` is `[0, 20]`.
	fn update_issuance_params(p: u32, v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_580_000 picoseconds.
		Weight::from_parts(4_871_200, 0)
			// Standard Error: 581
			.saturating_add(Weight::from_parts(6_370, 0).saturating_mul(p.into()))
			// Standard Error: 581
			.saturating_add(Weight::from_parts(9_390, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Rewards::ProposerSubsidyPoints` (r:0 w:1)
	/// Proof: `Rewards::ProposerSubsidyPoints` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `Rewards::VoterSubsidyPoints` (r:0 w:1)
	/// Proof: `Rewards::VoterSubsidyPoints` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 20]`.
	/// The range of component `v` is `[0, 20]`.
	fn update_issuance_params(p: u32, v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_580_000 picoseconds.
		Weight::from_parts(4_871_200, 0)
			// Standard Error: 581
			.saturating_add(Weight::from_parts(6_370, 0).saturating_mul(p.into()))
			// Standard Error: 581
			.saturating_add(Weight::from_parts(9_390, 0).saturating_mul(v.into()))
			.saturating_add(ParityDbWeight::get().writes(2_u64))
	}
}
