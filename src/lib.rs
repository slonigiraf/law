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
    // TODO: remove this
    // #[pallet::metadata(
    // 	T::AccountId = "AccountId", LetterIndexOf<T> = "LetterIndex", Option<BalanceOf<T>> = "Option<Balance>", BalanceOf<T> = "Balance",
    // )]
    pub enum Event<T: Config> {
        LawCreated([u8; 32], BalanceOf<T>),
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
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Create a law functionality
        #[pallet::weight(10_000)] //TODO: change
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            id: [u8; 32],
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
            Laws::<T>::insert(id, (id, price));
            Self::deposit_event(Event::LawCreated(id, price));
            Ok(().into())
        }
        // Edit a law functionality
        #[pallet::weight(10_000)] //TODO: change
        #[transactional]
        pub fn edit(
            origin: OriginFor<T>,
            id: [u8; 32],
            new_text: [u8; 32],
            new_price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (old_text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
            ensure!(new_price >= old_price, Error::<T>::NewPriceIsLow);
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

        #[pallet::weight(10_000)] //TODO: change
        #[transactional]
        pub fn upvote(
            origin: OriginFor<T>,
            id: [u8; 32],
            price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
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

        #[pallet::weight(10_000)] //TODO: change
        #[transactional]
        pub fn downvote(
            origin: OriginFor<T>,
            id: [u8; 32],
            price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
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

        #[pallet::weight(10_000)] //TODO: change
        #[transactional]
        pub fn remove(origin: OriginFor<T>, id: [u8; 32]) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (_, price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
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
