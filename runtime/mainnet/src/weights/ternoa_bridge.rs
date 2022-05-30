
//! Autogenerated weights for `ternoa_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// --chain
// mainnet-dev
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=ternoa_bridge

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `ternoa_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_bridge::WeightInfo for WeightInfo<T> {
	// Storage: Bridge RelayerVoteThreshold (r:0 w:1)
	fn set_threshold() -> Weight {
		(41_728_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bridge ChainNonces (r:1 w:1)
	fn add_chain() -> Weight {
		(46_558_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bridge Relayers (r:0 w:1)
	fn set_relayers() -> Weight {
		(35_016_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bridge Relayers (r:1 w:0)
	// Storage: Bridge ChainNonces (r:1 w:0)
	// Storage: Bridge RelayerVoteThreshold (r:1 w:0)
	// Storage: Bridge Votes (r:1 w:1)
	fn vote_for_proposal() -> Weight {
		(71_576_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bridge BridgeFee (r:1 w:0)
	// Storage: Bridge ChainNonces (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		(160_683_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bridge BridgeFee (r:0 w:1)
	fn set_bridge_fee() -> Weight {
		(31_098_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bridge ChainNonces (r:1 w:1)
	fn set_deposit_nonce() -> Weight {
		(36_509_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}