#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, WithdrawReasons, Randomness},
	transactional,
};
use frame_system::pallet_prelude::*;
use sp_std::{convert::TryInto, prelude::*};

use sp_core::sr25519::{Public, Signature};
use sp_core::{H256, H512};

use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::AccountId32;
use sp_runtime::traits::SaturatedConversion;

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;
pub mod weights;

pub use weights::WeightInfo;

/// Struct for holding recommendation letter information.
pub struct LetterCoordinates {
	chunk: usize,
	index: usize,
}

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
		type WeightInfo: WeightInfo;
		#[pallet::constant]
		type DefaultDifficulty: Get<u32>;
		type LettersPerChunk: Get<u32>;
		type TheParaId: Get<u32>;
	}
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// A storage for recommendation letters
	/// Keeps track of what accounts issued which letters
	#[pallet::storage]
	#[pallet::getter(fn was_letter_used)]
	pub(super) type OwnedLetersArray<T: Config> =
		StorageMap<_, Twox64Concat, (H256, u64), BoundedVec<bool, T::LettersPerChunk>, ValueQuery>;

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
	// #[pallet::metadata(
	// 	T::AccountId = "AccountId", LetterIndexOf<T> = "LetterIndex", Option<BalanceOf<T>> = "Option<Balance>", BalanceOf<T> = "Balance",
	// )]
	pub enum Event<T: Config> {
		ReimbursementHappened(H256, u64),
		LawCreated([u8; 32], BalanceOf<T>),
		LawEdited([u8; 32], [u8; 32], [u8; 32], BalanceOf<T>),
		LawUpvoted([u8; 32], BalanceOf<T>),
		LawDownvoted([u8; 32], BalanceOf<T>),
		LawRemoved([u8; 32], BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidRefereeSign,
		InvalidWorkerSign,
		InvalidLetterAmount,
		RefereeBalanceIsNotEnough,
		LetterWasMarkedAsFraudBefore,
		Expired,
		WrongParaId,
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
			ensure!( ! Laws::<T>::contains_key(&id), Error::<T>::UsedId);
			<T as Config>::Currency::withdraw(&sender, price, WithdrawReasons::TRANSFER.into(), ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::BalanceIsNotEnough)?;
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
			<T as Config>::Currency::withdraw(&sender, new_price, WithdrawReasons::TRANSFER.into(), ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::BalanceIsNotEnough)?;
			Laws::<T>::insert(id, (new_text, new_price));
			Self::deposit_event(Event::LawEdited(id, old_text, new_text, new_price));
			Ok(().into())
		}

		#[pallet::weight(10_000)] //TODO: change
        pub fn upvote(
			origin: OriginFor<T>,
			id: [u8; 32],
			price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
			let (text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
			let new_price = old_price + price;
			ensure!(new_price > old_price, Error::<T>::PriceOverflow);
			<T as Config>::Currency::withdraw(&sender, price, WithdrawReasons::TRANSFER.into(), ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::BalanceIsNotEnough)?;
			Laws::<T>::insert(id, (text, new_price));
			Self::deposit_event(Event::LawUpvoted(id, price));
			Ok(().into())
        }

		#[pallet::weight(10_000)] //TODO: change
		pub fn downvote(
			origin: OriginFor<T>,
			id: [u8; 32],
			price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
			let (text, old_price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
			let mut new_price = old_price;
			let mut payment  = price;
			if price < old_price {
				new_price = old_price - price;
			} else {
				new_price = old_price - old_price;
				payment = old_price;
			}

			<T as Config>::Currency::withdraw(&sender, payment, WithdrawReasons::TRANSFER.into(), ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::BalanceIsNotEnough)?;
			Laws::<T>::insert(id, (text, new_price));
			Self::deposit_event(Event::LawDownvoted(id, payment));
			Ok(().into())
        }

		#[pallet::weight(10_000)] //TODO: change
        pub fn remove(
			origin: OriginFor<T>,
			id: [u8; 32],
		) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
			let (_, price) = Laws::<T>::get(&id).ok_or(Error::<T>::MissingId)?;
			<T as Config>::Currency::withdraw(&sender, price, WithdrawReasons::TRANSFER.into(), ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::BalanceIsNotEnough)?;
			Laws::<T>::remove(id);
			Self::deposit_event(Event::LawRemoved(id, price));
			Ok(().into())
        }
						

		// A reimbursement functionality. A referee should should pay initially defined Balance sum if employer thinks that the letter is wrong.
		#[pallet::weight(T::WeightInfo::reimburse())]
		#[transactional]
		pub fn reimburse(
			origin: OriginFor<T>,
			para_id: u32,
			letter_id: u32,
			block_number: u64,
			referee_id: H256,
			worker_id: H256,
			employer_id: H256,
			ask_price: BalanceOf<T>,
			referee_sign: H512,
			worker_sign: H512,
		) -> DispatchResultWithPostInfo {
			let _sender = ensure_signed(origin)?;

			ensure!(
				T::TheParaId::get() == para_id,
				Error::<T>::WrongParaId
			);

			ensure!(
				frame_system::Pallet::<T>::block_number().saturated_into::<u64>() <= block_number,
				Error::<T>::Expired
			);

			// 1 , referee_id, worker_id, 10 - see below
			// [0, 0, 0, 1],
			// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
			// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
			// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
			// or in line:

			let para_id_bytes = &para_id.to_be_bytes();
			let letter_id_bytes = &letter_id.to_be_bytes();
			let block_number_bytes = &block_number.to_be_bytes();
			let referee_id_bytes = referee_id.as_bytes();
			let employer_id_bytes = employer_id.as_bytes();
			let worker_id_bytes = worker_id.as_bytes();

			let ask_price_u128 = TryInto::<u128>::try_into(ask_price)
				.map_err(|_| Error::<T>::InvalidLetterAmount)?;
			let ask_price_bytes = &ask_price_u128.to_be_bytes();

			let mut skill_receipt_data = Vec::new();
			skill_receipt_data.extend_from_slice(para_id_bytes);
			skill_receipt_data.extend_from_slice(letter_id_bytes);
			skill_receipt_data.extend_from_slice(block_number_bytes);
			skill_receipt_data.extend_from_slice(referee_id_bytes);
			skill_receipt_data.extend_from_slice(worker_id_bytes);
			skill_receipt_data.extend_from_slice(ask_price_bytes);

			ensure!(
				Self::signature_is_valid(
					referee_sign.clone(),
					skill_receipt_data.clone(),
					referee_id.clone()
				),
				Error::<T>::InvalidRefereeSign
			);

			let mut skill_letter_data = skill_receipt_data;
			skill_letter_data.extend_from_slice(referee_sign.as_bytes());
			skill_letter_data.extend_from_slice(employer_id.as_bytes());

			ensure!(
				Self::signature_is_valid(worker_sign, skill_letter_data, worker_id.clone()),
				Error::<T>::InvalidWorkerSign
			);

			ensure!(
				!Self::was_letter_canceled(referee_id, letter_id as usize),
				Error::<T>::LetterWasMarkedAsFraudBefore
			);

			T::Currency::transfer(
				&Self::account_id_from(referee_id_bytes),
				&Self::account_id_from(employer_id_bytes),
				ask_price,
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|_| Error::<T>::RefereeBalanceIsNotEnough)?;

			Self::mark_letter_as_fraud(referee_id, letter_id as usize)?;

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {}

const INSURANCE_PER_CHUNK: usize = 1000;
impl<T: Config> Pallet<T> {
	/// A wrapper function to provide AccountId
	fn account_id_from(account_bytes: &[u8]) -> T::AccountId {
		//
		let referee_bytes_array: [u8; 32] = Self::slice_to_array(account_bytes);
		let referee: AccountId32 = AccountId32::new(referee_bytes_array);
		let mut referee_init_account32 = AccountId32::as_ref(&referee);
		T::AccountId::decode(&mut referee_init_account32).unwrap()
	}
	/// A wrapper function to validate signatures
	fn signature_is_valid(signature: H512, message: Vec<u8>, pubkey: H256) -> bool {
		let mut data_signed_by_extension = Vec::new();
		data_signed_by_extension.extend_from_slice(b"<Bytes>");
		data_signed_by_extension.extend_from_slice(&message);
		data_signed_by_extension.extend_from_slice(b"</Bytes>");

		sp_io::crypto::sr25519_verify(
			&Signature::from_raw(*signature.as_fixed_bytes()),
			&data_signed_by_extension,
			&Public::from_h256(pubkey),
		)
	}
	fn slice_to_array(barry: &[u8]) -> [u8; 32] {
		let mut array = [0u8; 32];
		for (&x, p) in barry.iter().zip(array.iter_mut()) {
			*p = x;
		}
		array
	}

	/// A helper function to mark recommendation letter as used.
	fn mint_chunk(to: H256, chunk: usize) -> DispatchResult {
		ensure!(
			!<OwnedLetersArray<T>>::contains_key((to.clone(), chunk as u64)),
			"Letter already contains_key"
		);
		let data: BoundedVec<bool, T::LettersPerChunk> = (vec![true; INSURANCE_PER_CHUNK]).try_into().unwrap();
		// Write Letter counting information to storage.
		<OwnedLetersArray<T>>::insert((to.clone(), chunk as u64), data);
		Ok(())
	}

	/// A helper function to find out if the storage contains a chunk
	fn law_exists(id: [u8; 32]) -> bool {
		<Laws<T>>::contains_key(id.clone())
	}

	/// A helper function to find out if the storage contains a chunk
	fn chunk_exists(to: H256, chunk: usize) -> bool {
		<OwnedLetersArray<T>>::contains_key((to.clone(), chunk as u64))
	}
	/// Convert a letter id to coordinates to be used at the storage
	fn coordinates_from_letter_index(number: usize) -> LetterCoordinates {
		let chunk = number / INSURANCE_PER_CHUNK;
		let index = number % INSURANCE_PER_CHUNK;
		LetterCoordinates { chunk, index }
	}
	/// Convert coordinates of letter at storage to a letter id
	#[allow(dead_code)]
	fn letter_index_from_coordinates(coordinates: LetterCoordinates) -> usize {
		coordinates.chunk * INSURANCE_PER_CHUNK + coordinates.index
	}
	/// Shows if letter was used for referee penalization to exclude multiple penalizaions for the same letter
	/// Used letters are marked as false
	fn was_letter_canceled(referee: H256, number: usize) -> bool {
		let coordinates = Self::coordinates_from_letter_index(number);
		match Self::chunk_exists(referee, coordinates.chunk) {
			false => false,
			true => {
				let data = <OwnedLetersArray<T>>::get((referee.clone(), coordinates.chunk as u64));
				!data[coordinates.index] //used letters are marked as false
			}
		}
	}
	/// Mark a recommendation letter as fraud
	fn mark_letter_as_fraud(referee: H256, letter_number: usize) -> DispatchResult {
		let coordinates = Self::coordinates_from_letter_index(letter_number);
		if !Self::chunk_exists(referee, coordinates.chunk) {
			Self::mint_chunk(referee, coordinates.chunk)?;
		}
		let mut data = <OwnedLetersArray<T>>::get((referee.clone(), coordinates.chunk as u64));
		data[coordinates.index] = false;
		<OwnedLetersArray<T>>::remove((referee.clone(), coordinates.chunk as u64));
		<OwnedLetersArray<T>>::insert((referee.clone(), coordinates.chunk as u64), data);
		// Write `mint` event
		Self::deposit_event(Event::ReimbursementHappened(
			referee,
			coordinates.chunk as u64,
		));
		Ok(())
	}
}
