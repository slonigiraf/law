use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub const LAW_PRICE: u64 = 10;
pub const INITIAL_LAW_ID: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];
pub const INITIAL_LAW_TEXT: [u8; 32] = INITIAL_LAW_ID;
pub const EDITED_LAW_TEXT: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];

benchmarks! {
	create {
		let caller = whitelisted_caller();
		let ask_price: BalanceOf<T> = LAW_PRICE.into();
	}: _(RawOrigin::Signed(caller), INITIAL_LAW_ID, LAW_PRICE)
	edit {
		let caller = whitelisted_caller();
		let ask_price: BalanceOf<T> = LAW_PRICE.into();
	}: _(RawOrigin::Signed(caller), INITIAL_LAW_ID, EDITED_LAW_TEXT, LAW_PRICE)
	upvote {
		let caller = whitelisted_caller();
		let ask_price: BalanceOf<T> = LAW_PRICE.into();
	}: _(RawOrigin::Signed(caller), INITIAL_LAW_ID, LAW_PRICE)
	downvote {
		let caller = whitelisted_caller();
		let ask_price: BalanceOf<T> = LAW_PRICE.into();
	}: _(RawOrigin::Signed(caller), INITIAL_LAW_ID, LAW_PRICE)
	remove {
		let caller = whitelisted_caller();
		let ask_price: BalanceOf<T> = LAW_PRICE.into();
	}: _(RawOrigin::Signed(caller), INITIAL_LAW_ID)
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test,);
