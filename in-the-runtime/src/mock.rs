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

use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, Everything, Nothing},
};
use frame_system::EnsureRoot;
use sp_runtime::{
	traits::{IdentityLookup, TryConvert},
	AccountId32, BuildStorage,
};
use xcm::prelude::*;
use xcm_builder::SignedToAccountId32;

type Location = MultiLocation;

type AccountId = AccountId32;
type Balance = u128;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		ActivityPallet: crate,
		Balances: pallet_balances,
		PalletXcm: pallet_xcm,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<AccountId>;
	type Block = Block;
	type BlockHashCount = ConstU64<10>;
	type BaseCallFilter = frame_support::traits::Everything;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type OnSetCode = ();
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<0>;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ConstU32<0>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
	type RuntimeHoldReason = RuntimeHoldReason;
}

parameter_types! {
	pub const TokenLocation: MultiLocation = Here.into_location();
	pub const AnyNetwork: Option<NetworkId> = None;
	pub UniversalLocation: InteriorMultiLocation = Here;
	pub UnitWeightCost: u64 = 1_000;
	pub const BaseXcmWeight: Weight = Weight::from_parts(1_000, 1_000);
	pub CurrencyPerSecondPerByte: (AssetId, u128, u128) = (Concrete(TokenLocation::get()), 1, 1);
	pub TrustedAssets: (MultiAssetFilter, MultiLocation) = (All.into(), Here.into());
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, AnyNetwork>;

impl pallet_xcm::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = (); // We don't handle sending messages
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = ();
	type XcmExecutor = ();
	type XcmTeleportFilter = ();
	type XcmReserveTransferFilter = ();
	type Weigher = xcm_builder::FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = ();
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = ();
	type MaxLockers = ConstU32<8>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
	type WeightInfo = pallet_xcm::TestWeightInfo;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type AdminOrigin = EnsureRoot<AccountId>;
}

/// Custom type for converting a FRAME Origin into an XCM Location
pub struct CustomOriginConverter;
impl TryConvert<RuntimeOrigin, Location> for CustomOriginConverter {
	fn try_convert(_origin: RuntimeOrigin) -> Result<Location, RuntimeOrigin> {
		unimplemented!()
	}
}

parameter_types! {
	pub const ThisNetwork: Option<NetworkId> = None;
}

pub type SovereignAccountOf = xcm_builder::AccountId32Aliases<AnyNetwork, AccountId>;

pub type LocalAssetTransactor = xcm_builder::CurrencyAdapter<
	Balances,
	xcm_builder::IsConcrete<TokenLocation>,
	SovereignAccountOf,
	AccountId,
	(),
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = (); // We only execute messages locally
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = ();
	type IsReserve = ();
	type IsTeleporter = ();
	type UniversalLocation = UniversalLocation;
	type Barrier = xcm_builder::AllowExplicitUnpaidExecutionFrom<Everything>;
	type Weigher = xcm_builder::FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = xcm_builder::FixedRateOfFungible<CurrencyPerSecondPerByte, ()>;
	type ResponseHandler = PalletXcm;
	type AssetTrap = PalletXcm;
	type AssetLocker = ();
	type AssetExchanger = ();
	type AssetClaims = PalletXcm;
	type SubscriptionService = ();
	type PalletInstancesInfo = ();
	type FeeManager = ();
	type MaxAssetsIntoHolding = ConstU32<64>;
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = ();
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
	type LocationConverter = SignedToAccountId32<RuntimeOrigin, AccountId, ThisNetwork>;
}

pub const INITIAL_BALANCE: Balance = 100;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> { balances: vec![([0; 32].into(), INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	t.into()
}

pub fn assets_have_been_trapped() -> bool {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.find_map(|event| match event {
			RuntimeEvent::PalletXcm(pallet_xcm::Event::AssetsTrapped { .. }) => Some(()),
			_ => None,
		})
		.is_some()
}

pub fn assets_have_been_claimed() -> bool {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.find_map(|event| match event {
			RuntimeEvent::PalletXcm(pallet_xcm::Event::AssetsClaimed { .. }) => Some(()),
			_ => None,
		})
		.is_some()
}
