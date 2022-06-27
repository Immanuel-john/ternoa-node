
//! Autogenerated weights for `ternoa_marketplace`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// D:\TernoaCode\chain\target\release\ternoa.exe
// benchmark
// --chain
// alphanet-dev
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=ternoa_marketplace

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `ternoa_marketplace`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_marketplace::WeightInfo for WeightInfo<T> {
	// Storage: Marketplace MarketplaceMintFee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Marketplace NextMarketplaceId (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:0 w:1)
	fn create_marketplace() -> Weight {
		(47_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_marketplace_owner() -> Weight {
		(16_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_marketplace_kind() -> Weight {
		(16_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_marketplace_configuration() -> Weight {
		(41_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace MarketplaceMintFee (r:0 w:1)
	fn set_marketplace_mint_fee() -> Weight {
		(11_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: Marketplace ListedNfts (r:0 w:1)
	fn list_nft() -> Weight {
		(26_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: Marketplace ListedNfts (r:1 w:1)
	fn unlist_nft() -> Weight {
		(22_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: Marketplace ListedNfts (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn buy_nft() -> Weight {
		(73_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}