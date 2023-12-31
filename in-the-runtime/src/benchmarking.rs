// TODO license_header
#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benches {
	use super::*;

	/// Benchmarks the slowest path of `change_value`.
	#[benchmark]
	fn change_value() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();

		// You can mock the storage here:
		DummyValue::<T>::put(1);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), 9);

		Ok(())
	}

	// Implements a test for each benchmark. Execute with:
	// `cargo test -p pallet-activityRuntimeXcm --features runtime-benchmarks`.
	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
