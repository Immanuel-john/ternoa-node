//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=alphanet-dev
// --steps=50
// --repeat=20
// --pallet=pallet_assets
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./output

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
        // Storage: Assets Asset (r:1 w:1)
        fn create() -> Weight {
                Weight::from_ref_time(97_543_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(1 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        fn force_create() -> Weight {
                Weight::from_ref_time(58_309_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(1 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:5002 w:5001)
        // Storage: System Account (r:5000 w:5000)
        // Storage: Assets Metadata (r:1 w:0)
        // Storage: Assets Approvals (r:501 w:500)
        /// The range of component `c` is `[0, 5000]`.
        /// The range of component `s` is `[0, 5000]`.
        /// The range of component `a` is `[0, 500]`.
        fn destroy(c: u32, s: u32, a: u32, ) -> Weight {
                Weight::from_ref_time(32_330_393_000 as u64)
                        // Standard Error: 946_000
                        .saturating_add(Weight::from_ref_time(59_072_000 as u64).saturating_mul(c as u64))
                        // Standard Error: 946_000
                        .saturating_add(Weight::from_ref_time(69_924_000 as u64).saturating_mul(s as u64))
                        .saturating_add(T::DbWeight::get().reads(5 as u64))
                        .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
                        .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
                        .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
                        .saturating_add(T::DbWeight::get().writes(2 as u64))
                        .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(c as u64)))
                        .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
                        .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(a as u64)))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:1 w:1)
        fn mint() -> Weight {
                Weight::from_ref_time(147_819_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(2 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:1 w:1)
        fn burn() -> Weight {
                Weight::from_ref_time(139_892_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(2 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:2 w:2)
        // Storage: System Account (r:1 w:1)
        fn transfer() -> Weight {
                Weight::from_ref_time(140_104_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(4 as u64))
                        .saturating_add(T::DbWeight::get().writes(4 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:2 w:2)
        // Storage: System Account (r:1 w:1)
        fn transfer_keep_alive() -> Weight {
                Weight::from_ref_time(165_162_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(4 as u64))
                        .saturating_add(T::DbWeight::get().writes(4 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:2 w:2)
        // Storage: System Account (r:1 w:1)
        fn force_transfer() -> Weight {
                Weight::from_ref_time(120_417_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(4 as u64))
                        .saturating_add(T::DbWeight::get().writes(4 as u64))
        }
        // Storage: Assets Asset (r:1 w:0)
        // Storage: Assets Account (r:1 w:1)
        fn freeze() -> Weight {
                Weight::from_ref_time(62_488_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:0)
        // Storage: Assets Account (r:1 w:1)
        fn thaw() -> Weight {
                Weight::from_ref_time(48_281_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        fn freeze_asset() -> Weight {
                Weight::from_ref_time(48_320_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(1 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        fn thaw_asset() -> Weight {
                Weight::from_ref_time(82_135_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(1 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Metadata (r:1 w:0)
        fn transfer_ownership() -> Weight {
                Weight::from_ref_time(97_653_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        fn set_team() -> Weight {
                Weight::from_ref_time(89_458_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(1 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:0)
        // Storage: Assets Metadata (r:1 w:1)
        /// The range of component `n` is `[0, 50]`.
        /// The range of component `s` is `[0, 50]`.
        fn set_metadata(_n: u32, _s: u32, ) -> Weight {
                Weight::from_ref_time(123_574_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:0)
        // Storage: Assets Metadata (r:1 w:1)
        fn clear_metadata() -> Weight {
                Weight::from_ref_time(209_255_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:0)
        // Storage: Assets Metadata (r:1 w:1)
        /// The range of component `n` is `[0, 50]`.
        /// The range of component `s` is `[0, 50]`.
        fn force_set_metadata(_n: u32, s: u32, ) -> Weight {
                Weight::from_ref_time(83_645_000 as u64)
                        // Standard Error: 50_000
                        .saturating_add(Weight::from_ref_time(103_000 as u64).saturating_mul(s as u64))
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:0)
        // Storage: Assets Metadata (r:1 w:1)
        fn force_clear_metadata() -> Weight {
                Weight::from_ref_time(188_635_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        fn force_asset_status() -> Weight {
                Weight::from_ref_time(69_219_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(1 as u64))
                        .saturating_add(T::DbWeight::get().writes(1 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Approvals (r:1 w:1)
        fn approve_transfer() -> Weight {
                Weight::from_ref_time(164_430_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(2 as u64))
        }
        // Storage: Assets Approvals (r:1 w:1)
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Account (r:2 w:2)
        // Storage: System Account (r:1 w:1)
        fn transfer_approved() -> Weight {
                Weight::from_ref_time(275_459_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(5 as u64))
                        .saturating_add(T::DbWeight::get().writes(5 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Approvals (r:1 w:1)
        fn cancel_approval() -> Weight {
                Weight::from_ref_time(177_224_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(2 as u64))
        }
        // Storage: Assets Asset (r:1 w:1)
        // Storage: Assets Approvals (r:1 w:1)
        fn force_cancel_approval() -> Weight {
                Weight::from_ref_time(207_601_000 as u64)
                        .saturating_add(T::DbWeight::get().reads(2 as u64))
                        .saturating_add(T::DbWeight::get().writes(2 as u64))
        }
}