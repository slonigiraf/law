// Was generated with https://github.com/slonigiraf/law-testing
use super::*;

use crate as letters;
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		LettersModule: letters::{Pallet, Call, Storage, Event<T>, Config},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

parameter_types! {
	pub static MockRandom: H256 = Default::default();
}

impl Randomness<H256, u64> for MockRandom {
	fn random(_subject: &[u8]) -> (H256, u64) {
		(MockRandom::get(), 0)
	}
}

parameter_types! {
	pub const MaxClassMetadata: u32 = 0;
	pub const MaxTokenMetadata: u32 = 0;
}

parameter_types! {
	pub const DefaultDifficulty: u32 = 3;
	pub const LettersPerChunk: u32 = 1000;
    pub const TheParaId: u32 = 1;
}

impl Config for Test {
	type Event = Event;
	type Randomness = MockRandom;
	type Currency = Balances;
	type WeightInfo = ();
	type DefaultDifficulty = DefaultDifficulty;
	type LettersPerChunk = LettersPerChunk;
    type TheParaId = TheParaId;
}


pub const REFEREE_ID: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
pub const WORKER_ID: [u8; 32] = [142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72];
pub const EMPLOYER_ID: [u8; 32] = [254,101,113,125,173,4,71,215,21,246,96,160,165,132,17,222,80,155,66,230,239,184,55,95,86,47,88,165,84,213,134,14];
pub const MALICIOUS_ID: [u8; 32] = [6,166,233,218,61,113,34,154,171,136,61,55,234,9,139,102,146,228,207,22,127,51,101,144,61,162,221,109,87,172,54,120];
pub const INITIAL_BALANCE: u64 = 1000;
pub const REFEREE_STAKE: u64 = 10;
pub const PARA_ID: u32 = 1;
pub const LETTER_ID: u32 = 1;
pub const BEFORE_VALID_BLOCK_NUMBER: u64 = 99;
pub const LAST_VALID_BLOCK_NUMBER: u64 = 100;
pub const AFTER_VALID_BLOCK_NUMBER: u64 = 101;


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(
				AccountId::from(Public::from_raw(REFEREE_ID)).into_account(),
				INITIAL_BALANCE,
			),
			(
				AccountId::from(Public::from_raw(WORKER_ID)).into_account(),
				INITIAL_BALANCE,
			),
			(
				AccountId::from(Public::from_raw(EMPLOYER_ID)).into_account(),
				INITIAL_BALANCE,
			),
			(
				AccountId::from(Public::from_raw(MALICIOUS_ID)).into_account(),
				INITIAL_BALANCE,
			),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	<crate::GenesisConfig as GenesisBuild<Test>>::assimilate_storage(
		&crate::GenesisConfig::default(),
		&mut t,
	)
	.unwrap();

	let mut t: sp_io::TestExternalities = t.into();

	t.execute_with(|| System::set_block_number(1));
	t
}

#[test]
fn coordinates_from_letter_index() {
	new_test_ext().execute_with(|| {
		let coordinates = LettersModule::coordinates_from_letter_index(0);
		assert_eq!(coordinates.chunk, 0);
		assert_eq!(coordinates.index, 0);
		//
		let coordinates = LettersModule::coordinates_from_letter_index(1);
		assert_eq!(coordinates.chunk, 0);
		assert_eq!(coordinates.index, 1);
		let coordinates = LettersModule::coordinates_from_letter_index(1001);
		assert_eq!(coordinates.chunk, 1);
		assert_eq!(coordinates.index, 1);
	});
}

#[test]
fn letter_index_from_coordinates() {
	new_test_ext().execute_with(|| {
		let number =
			LettersModule::letter_index_from_coordinates(LetterCoordinates { chunk: 0, index: 0 });
		assert_eq!(number, 0);
		//
		let number =
			LettersModule::letter_index_from_coordinates(LetterCoordinates { chunk: 0, index: 1 });
		assert_eq!(number, 1);

		let number =
			LettersModule::letter_index_from_coordinates(LetterCoordinates { chunk: 1, index: 1 });
		assert_eq!(number, 1001);
	});
}

#[test]
fn mint_chunk() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);
		let chunk = 1;
		assert_ok!(LettersModule::mint_chunk(referee_hash.clone(), chunk));
		assert_noop!(
			LettersModule::mint_chunk(referee_hash.clone(), chunk),
			"Letter already contains_key"
		);

		assert_eq!(
			LettersModule::chunk_exists(referee_hash.clone(), chunk),
			true
		);
		assert_eq!(LettersModule::chunk_exists(referee_hash.clone(), 0), false);
		assert_eq!(LettersModule::chunk_exists(referee_hash.clone(), 2), false);
	});
}

#[test]
fn was_letter_canceled() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);
		let number = 1;
		let coordinates = LettersModule::coordinates_from_letter_index(number);
		//Assert fresh letters are unused
		assert_ok!(LettersModule::mint_chunk(
			referee_hash.clone(),
			coordinates.chunk
		));
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			false
		);
		//Use letters
		assert_ok!(LettersModule::mark_letter_as_fraud(
			referee_hash.clone(),
			number
		));
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			true
		);
		//Assert letters in other chunks are unused
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), 1001),
			false
		);
	});
}

#[test]
fn mark_letter_as_fraud() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);
		let number = 1;
		assert_ok!(LettersModule::mark_letter_as_fraud(
			referee_hash.clone(),
			number
		));
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			true
		);
	});
}


#[test]
fn signature_is_valid() {
    new_test_ext().execute_with(|| {
        let data_bytes: [u8; 96] = [0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,100,212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125,142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10];
        let signer_bytes: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
        let sign_bytes: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let mut data = Vec::new();
        data.extend_from_slice(&data_bytes);
        assert_eq!(
            LettersModule::signature_is_valid(
                H512::from(sign_bytes),
                data,
                H256::from(signer_bytes)
            ),
            true
        );
    });
}

#[test]
fn expired() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let worker_signature: [u8; 64] = [168,174,81,192,173,33,161,63,219,108,119,65,205,98,248,17,248,180,216,88,217,168,129,79,11,151,82,9,19,250,17,95,145,12,117,145,145,6,96,37,240,65,79,8,179,109,8,110,110,215,221,35,100,45,219,34,170,196,28,17,68,102,111,135];
        frame_system::Pallet::<Test>::set_block_number(AFTER_VALID_BLOCK_NUMBER);
        
        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                PARA_ID,
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::Expired
        );
    });
}

#[test]
fn wrong_para_id() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let worker_signature: [u8; 64] = [168,174,81,192,173,33,161,63,219,108,119,65,205,98,248,17,248,180,216,88,217,168,129,79,11,151,82,9,19,250,17,95,145,12,117,145,145,6,96,37,240,65,79,8,179,109,8,110,110,215,221,35,100,45,219,34,170,196,28,17,68,102,111,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);
        
        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                2,
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::WrongParaId
        );
    });
}

#[test]
fn successful_reimburse() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let worker_signature: [u8; 64] = [168,174,81,192,173,33,161,63,219,108,119,65,205,98,248,17,248,180,216,88,217,168,129,79,11,151,82,9,19,250,17,95,145,12,117,145,145,6,96,37,240,65,79,8,179,109,8,110,110,215,221,35,100,45,219,34,170,196,28,17,68,102,111,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);
        
        assert_eq!(
            LettersModule::was_letter_canceled(referee_hash.clone(), LETTER_ID as usize),
            false
        );

        assert_ok!(LettersModule::reimburse(
            Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
            PARA_ID,
            LETTER_ID,
            LAST_VALID_BLOCK_NUMBER,
            H256::from(REFEREE_ID),
            H256::from(WORKER_ID),
            H256::from(EMPLOYER_ID),
            REFEREE_STAKE,
            H512::from(referee_signature),
            H512::from(worker_signature)
        ));

        assert_eq!(
            LettersModule::was_letter_canceled(referee_hash.clone(), LETTER_ID as usize),
            true
        );

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                PARA_ID,
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::LetterWasMarkedAsFraudBefore
        );
    });
}

#[test]
fn wrong_referee_sign() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [166,127,31,226,56,160,42,192,56,13,126,67,192,10,247,37,245,9,73,221,153,161,57,117,4,169,105,177,143,37,190,1,65,231,177,138,81,182,30,28,162,228,35,231,102,80,253,159,109,49,192,176,235,74,80,28,28,193,104,129,252,239,165,134];
        let worker_signature: [u8; 64] = [168,174,81,192,173,33,161,63,219,108,119,65,205,98,248,17,248,180,216,88,217,168,129,79,11,151,82,9,19,250,17,95,145,12,117,145,145,6,96,37,240,65,79,8,179,109,8,110,110,215,221,35,100,45,219,34,170,196,28,17,68,102,111,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                PARA_ID,
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::InvalidRefereeSign
        );
    });
}

#[test]
fn referee_has_not_enough_balance() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let worker_signature: [u8; 64] = [168,174,81,192,173,33,161,63,219,108,119,65,205,98,248,17,248,180,216,88,217,168,129,79,11,151,82,9,19,250,17,95,145,12,117,145,145,6,96,37,240,65,79,8,179,109,8,110,110,215,221,35,100,45,219,34,170,196,28,17,68,102,111,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

        Balances::make_free_balance_be(
            &AccountId::from(Public::from_raw(REFEREE_ID)).into_account(),
            9,
        );

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                PARA_ID,
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::RefereeBalanceIsNotEnough
        );
    });
}

#[test]
fn wrong_worker_sign() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let worker_signature: [u8; 64] = [134,31,20,121,100,127,87,97,20,190,58,39,104,98,202,213,188,156,18,254,236,23,136,109,171,158,130,3,97,177,115,69,190,64,117,254,29,156,145,107,248,165,219,190,122,41,92,121,227,54,41,245,214,11,73,1,66,221,62,85,170,57,16,138];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                PARA_ID,
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::InvalidWorkerSign
        );
    });
}