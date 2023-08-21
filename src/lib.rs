#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, WithdrawReasons},
    transactional,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_core::sr25519::{Public, Signature};
use sp_runtime::traits::{IdentifyAccount, Verify, CheckedAdd};
use sp_std::prelude::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;
pub mod weights;

pub use weights::WeightInfo;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type WeightInfo: WeightInfo;
    }
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// A storage for laws
    #[pallet::storage]
    #[pallet::getter(fn id_to_law)]
    pub(super) type Laws<T: Config> =
        StorageMap<_, Blake2_128Concat, [u8; 32], ([u8; 32], BalanceOf<T>), OptionQuery>;

    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig;

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            // May be in future we need to do some configuration here
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]

    pub enum Event<T: Config> {
        LawCreated([u8; 32], [u8; 32], BalanceOf<T>),
        LawEdited([u8; 32], [u8; 32], [u8; 32], BalanceOf<T>),
        LawUpvoted([u8; 32], BalanceOf<T>),
        LawDownvoted([u8; 32], BalanceOf<T>),
        LawRemoved([u8; 32], BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        UsedId,
        BalanceIsNotEnough,
        MissingId,
        NewPriceIsLow,
        PriceOverflow,
        OutdatedText,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Create a law functionality
        #[pallet::weight(T::WeightInfo::create())]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            id: [u8; 32],
            text: [u8; 32],
            price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(!Laws::<T>::contains_key(&id), Error::<T>::UsedId);
            <T as Config>::Currency::withdraw(
                &sender,
                price,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            Laws::<T>::insert(id, (text, price));
            Self::deposit_event(Event::LawCreated(id, text, price));
            Ok(().into())
        }
        // Edit a law functionality
        #[pallet::weight(T::WeightInfo::edit())]
        #[transactional]
        pub fn edit(
            origin: OriginFor<T>,
            id: [u8; 32],
            current_text: [u8; 32],
            new_text: [u8; 32],
            new_price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (old_text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
            ensure!(new_price >= old_price, Error::<T>::NewPriceIsLow);
            ensure!(old_text == current_text, Error::<T>::OutdatedText);
            <T as Config>::Currency::withdraw(
                &sender,
                new_price,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            Laws::<T>::insert(id, (new_text, new_price));
            Self::deposit_event(Event::LawEdited(id, old_text, new_text, new_price));
            Ok(().into())
        }

        // Create and edit
        #[pallet::weight(T::WeightInfo::create())]//TODO provide actual weight
        #[transactional]
        pub fn create_and_edit(
            origin: OriginFor<T>,
            create_id: [u8; 32],
            create_text: [u8; 32],
            create_price: BalanceOf<T>,
            edit_id: [u8; 32],
            edit_current_text: [u8; 32],
            edit_new_text: [u8; 32],
            edit_new_price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            // Check the data
            ensure!(!Laws::<T>::contains_key(&create_id), Error::<T>::UsedId);
            <T as Config>::Currency::withdraw(
                &sender,
                create_price,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            let (old_text, old_price) = Laws::<T>::get(&edit_id).ok_or(Error::<T>::MissingId)?;
            ensure!(edit_new_price >= old_price, Error::<T>::NewPriceIsLow);
            ensure!(old_text == edit_current_text, Error::<T>::OutdatedText);
            <T as Config>::Currency::withdraw(
                &sender,
                edit_new_price,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            //Storage operations
            Laws::<T>::insert(create_id, (create_text, create_price));
            Self::deposit_event(Event::LawCreated(create_id, create_text, create_price));
            Laws::<T>::insert(edit_id, (edit_new_text, edit_new_price));
            Self::deposit_event(Event::LawEdited(edit_id, old_text, edit_new_text, edit_new_price));
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::upvote())]
        #[transactional]
        pub fn upvote(
            origin: OriginFor<T>,
            id: [u8; 32],
            current_text: [u8; 32],
            price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
            ensure!(text == current_text, Error::<T>::OutdatedText);
            let new_price = old_price.checked_add(&price).ok_or(Error::<T>::PriceOverflow)?;
            <T as Config>::Currency::withdraw(
                &sender,
                price,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            Laws::<T>::insert(id, (text, new_price));
            Self::deposit_event(Event::LawUpvoted(id, price));
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::downvote())]
        #[transactional]
        pub fn downvote(
            origin: OriginFor<T>,
            id: [u8; 32],
            current_text: [u8; 32],
            price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
            ensure!(text == current_text, Error::<T>::OutdatedText);
            let mut new_price = old_price;
            let mut payment = price;
            if price < old_price {
                new_price = old_price - price;
            } else {
                new_price = old_price - old_price;
                payment = old_price;
            }

            <T as Config>::Currency::withdraw(
                &sender,
                payment,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            Laws::<T>::insert(id, (text, new_price));
            Self::deposit_event(Event::LawDownvoted(id, payment));
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::remove())]
        #[transactional]
        pub fn remove(origin: OriginFor<T>, id: [u8; 32], current_text: [u8; 32]) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (text, price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
            ensure!(text == current_text, Error::<T>::OutdatedText);
            <T as Config>::Currency::withdraw(
                &sender,
                price,
                WithdrawReasons::TRANSFER.into(),
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;
            Laws::<T>::remove(id);
            Self::deposit_event(Event::LawRemoved(id, price));
            Ok(().into())
        }
    }
}
