
//! Autogenerated weights for `pallet_kitties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-18, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-kitties
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_kitties
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --json-file=raw.json
// --output
// ./pallets/kitties/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
pub trait KittiesWeightInfo {
	fn create_kitty( ) -> Weight;
	fn set_price( ) -> Weight;
	fn transfer( ) -> Weight;
}
/// Weight functions for `pallet_kitties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: SubstrateKitties KittyCnt (r:1 w:1)
	// Storage: SubstrateKitties Kitties (r:1 w:1)
	// Storage: SubstrateKitties KittiesOwned (r:1 w:1)
	fn create_kitty( ) -> Weight {
		(36_135_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: SubstrateKitties Kitties (r:1 w:1)
	fn set_price( ) -> Weight {
		(22_113_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: SubstrateKitties Kitties (r:1 w:1)
	// Storage: SubstrateKitties KittiesOwned (r:2 w:2)
	fn transfer( ) -> Weight {
		(34_049_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
