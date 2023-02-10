#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;



#[pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{
		tokens::WithdrawReasons, Currency, Get, Imbalance, LockIdentifier, LockableCurrency,
		ReservableCurrency,
	};
	use frame_system::pallet_prelude::*;
//	use nimbus_primitives::{AccountLookup, NimbusId};
//	use session_keys_primitives::KeysLookup;
	use sp_std::{mem::size_of, vec::Vec};


	//pub type BalanceOf<T> = <<T as Config>::DepositCurrency as Currency<<T as frame_system::Config>::AccountId, >>::Balance;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug, scale_info::TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct RegistrationInfo<T: Config> {
		pub(crate) account: T::AccountId,
//		pub(crate) deposit: BalanceOf<T>
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Overarching event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Currency in which the security deposit will be taken.
		//type DepositCurrency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId>;

		/// The amount that should be taken as a security deposit when registering a NimbusId.
//		type DepositAmount: Get<<Self::DepositCurrency as Currency<Self::AccountId>>::Balance>;
		type DepositAmount: Get<<Self::Currency as Currency<Self::AccountId>>::Balance>;
	}

	/// An error that can occur while executing the mapping pallet's logic.
	#[pallet::error]
	pub enum Error<T> {
		/// The association can't be cleared because it is not found.
		AssociationNotFound,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A NimbusId has been registered and mapped to an AccountId.
		KeysRegistered {
			account_id: T::AccountId,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn set_keys(origin: OriginFor<T>, keys: Vec<u8>) -> DispatchResult {
			let account_id = ensure_signed(origin)?;
			Ok(())
		}
	}

	#[pallet::genesis_config]
	/// Genesis config for author mapping pallet
	pub struct GenesisConfig<T: Config> {
		/// The associations that should exist at chain genesis
		pub mappings: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { mappings: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for account_id in &self.mappings {

			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: T::BlockNumber) {
			log::info!("block number {:?}, {:?}", n, T::AccountId::type_info());

			let mut h160_raw_data = [0u8; 20];
			hex::decode_to_slice("976f8456e4e2034179b284a23c0e0c8f6d3da50c", &mut h160_raw_data, ).expect("example data is 20 bytes of valid hex");

			/*
			let collator_id = T::AccountId::from(h160_raw_data);
//			let collator_id = create_funded_collator::<T>("collator", seed.take(), 0u32.into(), true, 1, )?;
			let amt = 10000000000000000u32.into();
			if let Ok(amount_transferred) = T::Currency::deposit_into_existing(&collator_id, amt) {
//				Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amount_transferred.peek(), });
			}
			*/

		}
	}

}
