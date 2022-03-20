//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::tokens::fungible::Unbalanced;
use frame_system::RawOrigin;
use pallet_balances::Pallet as Balances;
benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	// verify {
	// 	assert_eq!(KittiesOwned::<T>::get(), Some(s));
	// }
	set_price{ let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
	}: _(RawOrigin::Signed(caller), kitty_hashes[0], Some(s.into()))

	transfer{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let to: T::AccountId = account("to", 2u32, 2u32);
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
		Balances::<T>::set_balance( RawOrigin::Root.into(), to.clone(), s.into(), 0);
	} : _(RawOrigin::Signed(caller), to, kitty_hashes[0])
	// buy_kitty{
	// 	let s in 0 .. 10000000;
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
	// 	let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
	// 	let _ = Kitties::<T>::set_price(RawOrigin::Signed(caller).into(), kitty_hashes[0], Some(s.into()));
	// 	let buyer: T::AccountId = account("buyer", 2u32, 2u32);
	// 	let _ = <pallet_balances::Pallet<T as frame_system::Config> as Unbalanced<T::AccountId>>::set_balance( &buyer, 1000_000_000_000_000);
	// }: _(RawOrigin::Signed(caller), kitty_hashes[0], s.into())
	// breed_kitty{}: _(RawOrigin::Signed(caller), kitty_ha)

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
	// impl_benchmark_test_suite!(Balances, crate::mock::new_test_ext(), crate::mock::Test);
}
