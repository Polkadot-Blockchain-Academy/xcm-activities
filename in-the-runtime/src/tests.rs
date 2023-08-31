// Copyright Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(test)]

use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::AccountId32;
use test_log::test;
use xcm::prelude::*;

type Location = MultiLocation;
type Assets = MultiAssets;

#[test]
fn trap_assets_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(System::block_number() + 1);
		let account: AccountId32 = [0; 32].into();
		let amount = 10;
		let assets: Assets = vec![(Here, amount).into()].into();
		assert_ok!(ActivityPallet::trap_assets(RuntimeOrigin::signed(account.clone()), assets));
		assert!(assets_have_been_trapped());
		assert_eq!(Balances::free_balance(account), INITIAL_BALANCE - amount);
	});
}

#[test]
fn claim_assets_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(System::block_number() + 1);
		let account: AccountId32 = [0; 32].into();
		let amount = 10;
		let assets: Assets = vec![(Here, amount).into()].into();
		assert_ok!(ActivityPallet::trap_assets(
			RuntimeOrigin::signed(account.clone()),
			assets.clone()
		));

		let ticket: Location = Here.into();
		assert_ok!(ActivityPallet::claim_assets(
			RuntimeOrigin::signed(account.clone()),
			assets,
			ticket
		));

		assert!(assets_have_been_claimed());
		assert_eq!(Balances::free_balance(&account), INITIAL_BALANCE);
	});
}
