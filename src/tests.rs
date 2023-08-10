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
		LawModule: letters::{Pallet, Call, Storage, Event<T>, Config},
	}
);

pub type TestEvent = Event;

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
}

impl Config for Test {
	type Event = Event;
	type Currency = Balances;
	type WeightInfo = ();
}


pub const REFEREE_ID: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
pub const WORKER_ID: [u8; 32] = [142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72];
pub const INITIAL_BALANCE: u64 = 1000;
pub const REFEREE_STAKE: u64 = 10;
pub const PARA_ID: u32 = 1;
pub const LETTER_ID: u32 = 1;
pub const BEFORE_VALID_BLOCK_NUMBER: u64 = 99;
pub const LAST_VALID_BLOCK_NUMBER: u64 = 100;
pub const AFTER_VALID_BLOCK_NUMBER: u64 = 101;

// --------------
pub const INITIAL_LAW_ID: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
pub const EDITED_LAW_ID: [u8; 32] = [142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72];


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
fn successful_creation() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let referee_account = AccountId::from(Public::from_raw(REFEREE_ID)).into_account();
        
        // Get initial balance
		let initial_balance = <pallet_balances::Pallet<Test>>::total_balance(&referee_account);

		// Assert law does not exist initially
		assert_eq!(LawModule::law_exists(INITIAL_LAW_ID), false);

		// Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(referee_account.clone()),
            INITIAL_LAW_ID,
            REFEREE_STAKE
        ));

        // Assert law now exists and the balance was deducted
		assert_eq!(LawModule::law_exists(INITIAL_LAW_ID), true);
		let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&referee_account);
        assert_eq!(post_balance, initial_balance - REFEREE_STAKE);

		
		// Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(letters::Event::<Test>::LawCreated(INITIAL_LAW_ID, REFEREE_STAKE))
        );
    });
}


#[test]
fn prohibit_creation_with_existing_id() {
    new_test_ext().execute_with(|| {
        assert_ok!(LawModule::create(
            Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
            INITIAL_LAW_ID,
            REFEREE_STAKE
        ));
		assert_noop!(
            LawModule::create(
				Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
				INITIAL_LAW_ID,
				REFEREE_STAKE
			),
            Error::<Test>::UsedId
        );
    });
}

#[test]
fn successful_edit() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let referee_account = AccountId::from(Public::from_raw(REFEREE_ID)).into_account();
        
        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(referee_account.clone()),
            INITIAL_LAW_ID,
            REFEREE_STAKE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

		// Attempt to edit the law
		let price_for_edit = REFEREE_STAKE;
		let pre_balance = <pallet_balances::Pallet<Test>>::total_balance(&referee_account);
        assert_ok!(LawModule::edit(
            Origin::signed(referee_account.clone()),
            INITIAL_LAW_ID,
			EDITED_LAW_ID,
            price_for_edit
        ));

		// Assert law was edited
		let (updated_text, new_price) = LawModule::get_law(INITIAL_LAW_ID).unwrap();
		assert_eq!(updated_text, EDITED_LAW_ID);
		assert_eq!(new_price, price_for_edit);
		
		// Assert the balance was deducted
		let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&referee_account);
        assert_eq!(post_balance, pre_balance - price_for_edit);

		// Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(letters::Event::<Test>::LawEdited(INITIAL_LAW_ID, INITIAL_LAW_ID, EDITED_LAW_ID, price_for_edit))
        );
    });
}

#[test]
fn edit_balance_is_not_enough() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let referee_account = AccountId::from(Public::from_raw(REFEREE_ID)).into_account();
        
        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(referee_account.clone()),
            INITIAL_LAW_ID,
            REFEREE_STAKE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

		// Attempt to create the law
		let price_for_edit = REFEREE_STAKE-1;
		assert_noop!(
            LawModule::edit(
				Origin::signed(referee_account.clone()),
				INITIAL_LAW_ID,
				EDITED_LAW_ID,
				price_for_edit
			),
            Error::<Test>::NewPriceIsLow
        );
    });
}