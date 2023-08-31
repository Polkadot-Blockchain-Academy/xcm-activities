// Copyright 2021 Parity Technologies (UK) Ltd.
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

#[cfg(test)]
mod tests {
	use codec::Encode;
	use frame_support::assert_ok;
	use xcm::{latest::prelude::*, VersionedXcm};
	use xcm_simulator_pba::{
		parachain, parachain_xcm_executed_successfully, MockNet, ParaA, ParachainPalletBalances,
		ParachainPalletXcm, TestExt, ALICE, BOB, INITIAL_BALANCE,
	};

	#[test]
	fn test_non_free_execution_from_relay() {
		MockNet::reset();
		// Insert here the appropriate code to test the configuration change
	}

	#[test]
	fn test_20_byte_account_xcm_deposit_withdraw() {
		MockNet::reset();
		// Insert here the appropriate code to test the configuration change
	}

	#[test]
	fn test_teleporting_from_relay_to_para() {
		MockNet::reset();
		// Insert here the appropriate code to test the configuration change
	}

	#[test]
	fn test_minting_token_relay_in_pallet_assets() {
		MockNet::reset();
		// Insert here the appropriate code to test the configuration change
	}
}
