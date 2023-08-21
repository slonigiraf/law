// Was generated with https://github.com/slonigiraf/law-testing
use super::*;
use crate as laws;
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub const CREATOR: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];
pub const EDITOR: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];
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

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        LawModule: laws::{Pallet, Call, Storage, Event<T>, Config},
    }
);

// Helper Functions
/// Convert a raw byte array into an AccountId.
fn account_id_from_raw(bytes: [u8; 32]) -> AccountId {
    AccountId::from(Public::from_raw(bytes)).into_account()
}

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

parameter_types! {}

impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type WeightInfo = ();
}

impl<T: Config> Pallet<T> {
    /// A helper function to find out if the law exists
    fn law_exists(id: [u8; 32]) -> bool {
        <Laws<T>>::contains_key(&id)
    }
    /// A helper function to get law text
    fn get_law(id: [u8; 32]) -> Option<([u8; 32], BalanceOf<T>)> {
        <Laws<T>>::get(&id)
    }
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (account_id_from_raw(CREATOR), INITIAL_BALANCE),
            (account_id_from_raw(EDITOR), INITIAL_BALANCE),
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
fn creation_success() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Get initial balance
        let initial_balance = <pallet_balances::Pallet<Test>>::total_balance(&creator);

        // Assert law does not exist initially
        assert_eq!(LawModule::law_exists(A_LAW_ID), false);

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_TEXT,
            A_LAW_PRICE
        ));

        // Assert law now exists and the balance was deducted
        assert_eq!(LawModule::law_exists(A_LAW_ID), true);
        let (text, price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(text, A_LAW_TEXT);
        assert_eq!(price, A_LAW_PRICE);
        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&creator);
        assert_eq!(post_balance, initial_balance - A_LAW_PRICE);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(laws::Event::<Test>::LawCreated(
                A_LAW_ID,
                A_LAW_TEXT,
                A_LAW_PRICE
            ))
        );
    });
}

#[test]
fn creation_used_id() {
    new_test_ext().execute_with(|| {
        let creator = account_id_from_raw(CREATOR);
        assert_ok!(LawModule::create(
            Origin::signed(creator),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));
        assert_noop!(
            LawModule::create(Origin::signed(creator), A_LAW_ID, A_LAW_ID, A_LAW_PRICE),
            Error::<Test>::UsedId
        );
    });
}

#[test]
fn creation_balance_is_not_enough() {
    new_test_ext().execute_with(|| {
        let creator = account_id_from_raw(CREATOR);
        assert_noop!(
            LawModule::create(Origin::signed(creator), A_LAW_ID, A_LAW_ID, INITIAL_BALANCE + 1),
            Error::<Test>::BalanceIsNotEnough
        );
        assert_eq!(LawModule::law_exists(A_LAW_ID), false);
    });
}

#[test]
fn edit_success() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to edit the law
        let price_for_edit = A_LAW_PRICE;
        let pre_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_ok!(LawModule::edit(
            Origin::signed(editor.clone()),
            A_LAW_ID,
            A_LAW_ID,
            EDITED_LAW_TEXT,
            price_for_edit
        ));

        // Assert law was edited
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, EDITED_LAW_TEXT);
        assert_eq!(new_price, price_for_edit);

        // Assert the balance was deducted
        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_eq!(post_balance, pre_balance - price_for_edit);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(laws::Event::<Test>::LawEdited(
                A_LAW_ID,
                A_LAW_ID,
                EDITED_LAW_TEXT,
                price_for_edit
            ))
        );
    });
}

#[test]
fn edit_missing_id() {
    new_test_ext().execute_with(|| {
        let editor = account_id_from_raw(EDITOR);
        assert_noop!(
            LawModule::edit(
                Origin::signed(editor.clone()),
                A_LAW_ID,
                A_LAW_ID,
                EDITED_LAW_TEXT,
                A_LAW_PRICE
            ),
            Error::<Test>::MissingId
        );
    });
}

#[test]
fn edit_new_price_is_low() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to create the law
        let price_for_edit = A_LAW_PRICE - 1;
        assert_noop!(
            LawModule::edit(
                Origin::signed(editor.clone()),
                A_LAW_ID,
                A_LAW_ID,
                EDITED_LAW_TEXT,
                price_for_edit
            ),
            Error::<Test>::NewPriceIsLow
        );
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, INITIAL_LAW_TEXT);
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn edit_balance_is_not_enough() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Attempt to edit the law
        let price_for_edit = INITIAL_BALANCE + 1;
        assert_noop!(
            LawModule::edit(
                Origin::signed(editor.clone()),
                A_LAW_ID,
                A_LAW_ID,
                EDITED_LAW_TEXT,
                price_for_edit
            ),
            Error::<Test>::BalanceIsNotEnough
        );
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, INITIAL_LAW_TEXT);
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn edit_outdated_text() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Attempt to edit the law
        assert_noop!(
            LawModule::edit(
                Origin::signed(editor.clone()),
                A_LAW_ID,
                A_LAW_TEXT,
                EDITED_LAW_TEXT,
                A_LAW_PRICE
            ),
            Error::<Test>::OutdatedText
        );
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, INITIAL_LAW_TEXT);
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn upvote_success() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to upvote the law
        let upvote_price = A_LAW_PRICE;
        let pre_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_ok!(LawModule::upvote(
            Origin::signed(editor.clone()),
            A_LAW_ID,
            A_LAW_ID,
            upvote_price
        ));

        // Assert law was upvoted
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, INITIAL_LAW_TEXT);
        assert_eq!(new_price, A_LAW_PRICE + upvote_price);

        // Assert the balance was deducted
        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_eq!(post_balance, pre_balance - upvote_price);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(laws::Event::<Test>::LawUpvoted(
                A_LAW_ID,
                upvote_price
            ))
        );
    });
}

#[test]
fn upvote_missing_id() {
    new_test_ext().execute_with(|| {
        let creator = account_id_from_raw(CREATOR);
        assert_noop!(
            LawModule::upvote(Origin::signed(creator.clone()), A_LAW_ID, A_LAW_ID, A_LAW_PRICE),
            Error::<Test>::MissingId
        );
    });
}

#[test]
fn upvote_price_overflow() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Attempt to upvote the law
        let upvote_price = std::u64::MAX;

        assert_noop!(
            LawModule::upvote(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_ID,
                upvote_price
            ),
            Error::<Test>::PriceOverflow
        );

        // Assert law was not upvoted
        let (_, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn upvote_balance_is_not_enough() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Attempt to upvote the law
        let upvote_price = INITIAL_BALANCE + 1;

        assert_noop!(
            LawModule::upvote(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_ID,
                upvote_price
            ),
            Error::<Test>::BalanceIsNotEnough
        );

        // Assert law was not upvoted
        let (_, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn upvote_outdated_text() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        assert_noop!(
            LawModule::upvote(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_TEXT,
                A_LAW_PRICE
            ),
            Error::<Test>::OutdatedText
        );

        // Assert law was not upvoted
        let (_, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn downvote_success() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to downvote the law
        let downvote_price = 1;
        let pre_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_ok!(LawModule::downvote(
            Origin::signed(editor.clone()),
            A_LAW_ID,
            A_LAW_ID,
            downvote_price
        ));

        // Assert law was downvoted
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, INITIAL_LAW_TEXT);
        assert_eq!(new_price, A_LAW_PRICE - downvote_price);

        // Assert the balance was deducted
        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_eq!(post_balance, pre_balance - downvote_price);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(laws::Event::<Test>::LawDownvoted(
                A_LAW_ID,
                downvote_price
            ))
        );
    });
}

#[test]
fn downvote_missing_id() {
    new_test_ext().execute_with(|| {
        let creator = account_id_from_raw(CREATOR);
        assert_noop!(
            LawModule::downvote(Origin::signed(creator.clone()), A_LAW_ID, A_LAW_ID, A_LAW_PRICE),
            Error::<Test>::MissingId
        );
    });
}

#[test]
fn downvote_underflow() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to downvote the law
        let downvote_price = INITIAL_BALANCE;
        let pre_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_ok!(LawModule::downvote(
            Origin::signed(editor.clone()),
            A_LAW_ID,
            A_LAW_ID,
            downvote_price
        ));

        // Assert law was downvoted
        let (updated_text, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(updated_text, INITIAL_LAW_TEXT);
        assert_eq!(new_price, 0);

        // Assert the balance was deducted
        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_eq!(post_balance, pre_balance - A_LAW_PRICE);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(laws::Event::<Test>::LawDownvoted(
                A_LAW_ID,
                A_LAW_PRICE
            ))
        );
    });
}

#[test]
fn downvote_balance_is_not_enough() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

		let creation_price = INITIAL_BALANCE - 1;

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            creation_price
        ));

        // Attempt to downvote the law
        let downvote_price = creation_price;

        assert_noop!(
            LawModule::downvote(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_ID,
                downvote_price
            ),
            Error::<Test>::BalanceIsNotEnough
        );

        // Assert law was not downvoted
        let (_, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(new_price, creation_price);
    });
}

#[test]
fn downvote_outdated_text() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        assert_noop!(
            LawModule::downvote(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_TEXT,
                A_LAW_PRICE
            ),
            Error::<Test>::OutdatedText
        );

        // Assert law was not downvoted
        let (_, new_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(new_price, A_LAW_PRICE);
    });
}

#[test]
fn remove_success() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);
        let editor = account_id_from_raw(EDITOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

		assert_eq!(LawModule::law_exists(A_LAW_ID), true);

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Attempt to remove the law
        let pre_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_ok!(LawModule::remove(
            Origin::signed(editor.clone()),
            A_LAW_ID,
            A_LAW_ID
        ));

        // Assert law was removed
        assert_eq!(LawModule::law_exists(A_LAW_ID), false);

        // Assert the balance was deducted
        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&editor);
        assert_eq!(post_balance, pre_balance - A_LAW_PRICE);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].event,
            TestEvent::LawModule(laws::Event::<Test>::LawRemoved(
                A_LAW_ID,
                A_LAW_PRICE
            ))
        );
    });
}

#[test]
fn remove_missing_id() {
    new_test_ext().execute_with(|| {
        let creator = account_id_from_raw(CREATOR);
        assert_noop!(
            LawModule::remove(Origin::signed(creator.clone()), A_LAW_ID, A_LAW_ID),
            Error::<Test>::MissingId
        );
    });
}

#[test]
fn remove_balance_is_not_enough() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

		let creation_price = INITIAL_BALANCE - 1;

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            creation_price
        ));

        // Attempt to remove the law
        
        assert_noop!(
            LawModule::remove(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_ID
            ),
            Error::<Test>::BalanceIsNotEnough
        );

        // Assert law was not removed
        assert_eq!(LawModule::law_exists(A_LAW_ID), true);
    });
}

#[test]
fn remove_outdated_text() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Attempt to create the law
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_ID,
            A_LAW_PRICE
        ));

        // Attempt to remove the law
        
        assert_noop!(
            LawModule::remove(
                Origin::signed(creator.clone()),
                A_LAW_ID,
                A_LAW_TEXT
            ),
            Error::<Test>::OutdatedText
        );

        // Assert law was not removed
        assert_eq!(LawModule::law_exists(A_LAW_ID), true);
    });
}

#[test]
fn create_and_edit_success() {
    new_test_ext().execute_with(|| {
        // Extract account creation for reuse
        let creator = account_id_from_raw(CREATOR);

        // Get initial balance
        let initial_balance = <pallet_balances::Pallet<Test>>::total_balance(&creator);

        // Assert law does not exist initially
        assert_eq!(LawModule::law_exists(A_LAW_ID), false);

        // Create a law that will be edited
        assert_ok!(LawModule::create(
            Origin::signed(creator.clone()),
            ANOTHER_LAW_ID,
            ANOTHER_LAW_TEXT,
            ANOTHER_LAW_PRICE
        ));

        // Clear events
        frame_system::Pallet::<Test>::reset_events();

        // Create and edit
        let edit_price = ANOTHER_LAW_PRICE+1;
        assert_ok!(LawModule::create_and_edit(
            Origin::signed(creator.clone()),
            A_LAW_ID,
            A_LAW_TEXT,
            A_LAW_PRICE,
            ANOTHER_LAW_ID,
            ANOTHER_LAW_TEXT,
            EDITED_LAW_TEXT,
            edit_price,
        ));
        
        // Assert data was updated and the balance was deducted
        let (created_text, created_price) = LawModule::get_law(A_LAW_ID).unwrap();
        assert_eq!(created_text, A_LAW_TEXT);
        assert_eq!(created_price, A_LAW_PRICE);
    
        let (edited_text, edited_price) = LawModule::get_law(ANOTHER_LAW_ID).unwrap();
        assert_eq!(edited_text, EDITED_LAW_TEXT);
        assert_eq!(edited_price, edit_price);

        let post_balance = <pallet_balances::Pallet<Test>>::total_balance(&creator);
        assert_eq!(post_balance, initial_balance - A_LAW_PRICE - ANOTHER_LAW_PRICE - edit_price);

        // Check for emitted event
        let events = frame_system::Pallet::<Test>::events();
        assert_eq!(events.len(), 4);
        assert_eq!(
            events[2].event,
            TestEvent::LawModule(laws::Event::<Test>::LawCreated(
                A_LAW_ID,
                A_LAW_TEXT,
                A_LAW_PRICE
            ))
        );
        assert_eq!(
            events[3].event,
            TestEvent::LawModule(laws::Event::<Test>::LawEdited(
                ANOTHER_LAW_ID,
                ANOTHER_LAW_TEXT,
                EDITED_LAW_TEXT,
                edit_price
            ))
        );
    });
}