//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::tokens::currency::Currency;
use frame_system::RawOrigin;
// /// Grab a funded user.
// pub fn create_funded_user<T: Config>(
// 	string: &'static str,
// 	n: u32,
// 	balance_factor: u32,
// ) -> T::AccountId {
// 	let user = account(string, n, 1);
// 	let balance = T::Currency::minimum_balance() * balance_factor.into();
// 	let _ = T::Currency::make_free_balance_be(&user, balance);
// 	user
// }
pub fn create_funded_user<T: pallet::Config>(
	string: &'static str,
	n: u32,
	balance_factor: u32,
) -> T::AccountId {
	let user = account(string, n, 1);
	let balance =
		<T::Currency as Currency<T::AccountId>>::minimum_balance() * balance_factor.into();
	let _ = <T::Currency as Currency<T::AccountId>>::make_free_balance_be(&user, balance);
	user
}
benchmarks! {

	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

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
	} : _(RawOrigin::Signed(caller), to, kitty_hashes[0])

	buy_kitty{
		let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
		let _ = Kitties::<T>::set_price(RawOrigin::Signed(caller).into(), kitty_hashes[0], Some(s.into()));
		let buyer = create_funded_user::<T>("caller", 1, s.into());
	}: _(RawOrigin::Signed(buyer), kitty_hashes[0], s.into())
	breed_kitty{
		let s in 0 .. 1000;
		let caller: T::AccountId = whitelisted_caller();
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		frame_system::pallet::Pallet::<T>::set_block_number(2u32.into());
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());

	}: _(RawOrigin::Signed(caller), kitty_hashes[0], kitty_hashes[1])

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
