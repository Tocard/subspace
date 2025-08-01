
//! Autogenerated weights for `pallet_messenger_from_consensus_extension`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.2.0
//! DATE: 2025-07-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `protocol-team-testing`, CPU: `AMD Ryzen 5 3600 6-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/subspace-node
// domain
// benchmark
// pallet
// --runtime=./target/production/wbuild/auto-id-domain-runtime/auto_id_domain_runtime.compact.compressed.wasm
// --extrinsic=*
// --wasm-execution=compiled
// --genesis-builder=none
// --steps=50
// --repeat=20
// --heap-pages=4096
// --pallet=pallet_messenger_from_consensus_extension
// --output=./domains/runtime/auto-id/src/weights/pallet_messenger_from_consensus_extension.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_messenger_from_consensus_extension`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_messenger::extensions::FromConsensusWeightInfo for WeightInfo<T> {
	/// Storage: `Messenger::Channels` (r:1 w:1)
	/// Proof: `Messenger::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SelfDomainId::SelfDomainId` (r:1 w:0)
	/// Proof: `SelfDomainId::SelfDomainId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Messenger::NextChannelId` (r:1 w:1)
	/// Proof: `Messenger::NextChannelId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Messenger::Inbox` (r:0 w:1)
	/// Proof: `Messenger::Inbox` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::EventSegments` (r:0 w:1)
	/// Proof: `System::EventSegments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn from_consensus_relay_message_channel_open() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3465`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(26_819_000, 0)
			.saturating_add(Weight::from_parts(0, 3465))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Messenger::Channels` (r:1 w:1)
	/// Proof: `Messenger::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SelfDomainId::SelfDomainId` (r:1 w:0)
	/// Proof: `SelfDomainId::SelfDomainId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Messenger::NextChannelId` (r:1 w:0)
	/// Proof: `Messenger::NextChannelId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Messenger::Inbox` (r:0 w:1)
	/// Proof: `Messenger::Inbox` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::EventSegments` (r:0 w:1)
	/// Proof: `System::EventSegments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn from_consensus_relay_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `3719`
		// Minimum execution time: 26_599_000 picoseconds.
		Weight::from_parts(26_859_000, 0)
			.saturating_add(Weight::from_parts(0, 3719))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Messenger::Channels` (r:1 w:1)
	/// Proof: `Messenger::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SelfDomainId::SelfDomainId` (r:1 w:0)
	/// Proof: `SelfDomainId::SelfDomainId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Messenger::NextChannelId` (r:1 w:0)
	/// Proof: `Messenger::NextChannelId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Messenger::OutboxResponses` (r:0 w:1)
	/// Proof: `Messenger::OutboxResponses` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::EventSegments` (r:0 w:1)
	/// Proof: `System::EventSegments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn from_consensus_relay_message_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `3703`
		// Minimum execution time: 24_419_000 picoseconds.
		Weight::from_parts(24_729_000, 0)
			.saturating_add(Weight::from_parts(0, 3703))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
