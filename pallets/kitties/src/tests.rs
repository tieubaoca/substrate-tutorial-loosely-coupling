use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
#[test]
fn test_create_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn test_transfer_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let _ = KittiesModule::create_kitty(Origin::signed(1));
		let kitty_hash = KittiesModule::kitties_owned(1)[0];

		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, kitty_hash));
	});
}

#[test]
fn test_buy_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let _ = KittiesModule::create_kitty(Origin::signed(1));
		let kitty_hash = KittiesModule::kitties_owned(1)[0];
		let _ = KittiesModule::set_price(Origin::signed(1), kitty_hash, Some(1_000_000u128));
		let _ = Balances::set_balance(Origin::root(), 2, 1000_000_000_000_000, 1_000_000);
		assert_eq!(Balances::free_balance(2), 1000_000_000_000_000);
		assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), kitty_hash, 1_000_000u128));
	});
}

// #[test]
// fn test_buy_kitty() {
// 	System::set_block_number(1);
// 	// Dispatch a signed extrinsic.
// 	let _ = KittiesModule::create_kitty(Origin::signed(1));
// 	let kitty_hash = KittiesModule::kitties_owned(1)[0];
// 	let _ = KittiesModule::set_price(Origin::signed(1), kitty_hash, Some(1_000_000u128));
// 	let _ = Balances::set_balance(Origin::root(), 2, 1000_000_000_000_000, 1_000_000);
// 	assert_eq!(Balances::free_balance(2), 1000_000_000_000_000);
// 	assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), kitty_hash, 1_000_000u128));
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
