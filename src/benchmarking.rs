use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub const INITIAL_BALANCE: u64 = 1000;
pub const A_LAW_PRICE: u64 = 10;
pub const ANOTHER_LAW_PRICE: u64 = 11;

pub const A_LAW_ID: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];
pub const ANOTHER_LAW_ID: [u8; 32] = [
    10, 10, 10, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 10, 10, 10,
];
pub const ANOTHER_LAW_TEXT: [u8; 32] = [
    20, 20, 20, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 20, 20, 20,
];
pub const A_LAW_TEXT: [u8; 32] = [
    100, 50, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 200,
];
pub const INITIAL_LAW_TEXT: [u8; 32] = A_LAW_ID;
pub const EDITED_LAW_TEXT: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];

benchmarks! {
	create {
		let caller: T::AccountId = whitelisted_caller();
		let balance_to_add: BalanceOf<T> = INITIAL_BALANCE.into();
		<T as Config>::Currency::deposit_creating(&caller, balance_to_add);
		let law_price: BalanceOf<T> = LAW_PRICE.into();
	}: _(RawOrigin::Signed(caller), A_LAW_ID, A_LAW_TEXT, law_price)
	edit {
		let caller: T::AccountId = whitelisted_caller();
		let balance_to_add: BalanceOf<T> = INITIAL_BALANCE.into();
		<T as Config>::Currency::deposit_creating(&caller, balance_to_add);
		let law_price: BalanceOf<T> = LAW_PRICE.into();
		Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), A_LAW_ID, A_LAW_TEXT, law_price)?;
	}: _(RawOrigin::Signed(caller), A_LAW_ID, A_LAW_TEXT, EDITED_LAW_TEXT, law_price)
	create_and_edit {
		let caller: T::AccountId = whitelisted_caller();
		let balance_to_add: BalanceOf<T> = INITIAL_BALANCE.into();
		<T as Config>::Currency::deposit_creating(&caller, balance_to_add);
		let law_price: BalanceOf<T> = LAW_PRICE.into();
		Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), ANOTHER_LAW_ID, ANOTHER_LAW_TEXT, law_price)?;
	}: _(RawOrigin::Signed(caller), A_LAW_ID, A_LAW_TEXT, law_price, ANOTHER_LAW_ID, ANOTHER_LAW_TEXT, EDITED_LAW_TEXT, law_price)
	upvote {
		let caller: T::AccountId = whitelisted_caller();
		let balance_to_add: BalanceOf<T> = INITIAL_BALANCE.into();
		<T as Config>::Currency::deposit_creating(&caller, balance_to_add);
		let law_price: BalanceOf<T> = LAW_PRICE.into();
		Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), A_LAW_ID, A_LAW_TEXT, law_price)?;
	}: _(RawOrigin::Signed(caller), A_LAW_ID, A_LAW_TEXT, law_price)
	downvote {
		let caller: T::AccountId = whitelisted_caller();
		let balance_to_add: BalanceOf<T> = INITIAL_BALANCE.into();
		<T as Config>::Currency::deposit_creating(&caller, balance_to_add);
		let law_price: BalanceOf<T> = LAW_PRICE.into();
		Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), A_LAW_ID, A_LAW_TEXT, law_price)?;
	}: _(RawOrigin::Signed(caller), A_LAW_ID, A_LAW_TEXT, law_price)
	remove {
		let caller: T::AccountId = whitelisted_caller();
		let balance_to_add: BalanceOf<T> = INITIAL_BALANCE.into();
		<T as Config>::Currency::deposit_creating(&caller, balance_to_add);
		let law_price: BalanceOf<T> = LAW_PRICE.into();
		Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into(), A_LAW_ID, A_LAW_TEXT, law_price)?;
	}: _(RawOrigin::Signed(caller), A_LAW_ID, A_LAW_TEXT)
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test,);
