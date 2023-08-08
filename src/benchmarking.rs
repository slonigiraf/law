use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub const REFEREE_ID: [u8; 32] = [
	212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
	76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

pub const WORKER_ID: [u8; 32] = [
	142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
	18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];
pub const EMPLOYER_ID: [u8; 32] = [
	254, 101, 113, 125, 173, 4, 71, 215, 21, 246, 96, 160, 165, 132, 17, 222, 80, 155, 66, 230,
	239, 184, 55, 95, 86, 47, 88, 165, 84, 213, 134, 14,
];

benchmarks! {
	reimburse {
		let referee_signature: [u8; 64] = [
			230, 177, 124,  45,  11,  85,  71, 101, 190, 120, 239,
			211, 218, 219, 206, 153, 133,  14, 144,  52, 254, 192,
			 75, 116, 222,  90, 104,  63, 233, 254,  44,  94, 183,
			 94,  42, 222, 150, 253, 223,  27,  94,  18, 136, 186,
			156,   7, 169, 225, 232, 146, 222, 190, 131,  61, 224,
			141,  28,   4,  46, 133, 237, 155,  57, 128,
		];
		let worker_signature: [u8; 64] = [
			102,  45, 124, 211,  15, 227, 227, 231,  97,  99,  98,
			123, 213, 172, 167,   4, 145, 194, 184,  62,  59,  62,
			140,  12,  49,  95, 197, 236,  83,  35,  56, 119, 136,
			26, 240, 141,  63,  17,  81, 157, 120,  64, 194,  80,
			140, 247,   8, 108, 183, 107,  18,  74,  42, 114, 252,
			138,  15, 232,  13,  62,  73, 235, 204, 138,
		];

		let caller = whitelisted_caller();
		let letter_id = 0 as u32;
		let ask_price: BalanceOf<T> = 0u32.into();

		let referee_id = H256::from(REFEREE_ID);
		let worker_id = H256::from(WORKER_ID);
		let employer_id = H256::from(EMPLOYER_ID);
		let referee_sign = H512::from(referee_signature);
		let worker_sign = H512::from(worker_signature);
	}: _(RawOrigin::Signed(caller), letter_id, referee_id, worker_id, employer_id, ask_price, referee_sign, worker_sign)
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test,);
