#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;
pub use pallet::*;

#[pallet]
pub mod pallet {
	use frame_support::sp_runtime::SaturatedConversion;
	use frame_support::sp_runtime::traits::Zero;
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

	/// An error that can occur while executing the spf setting pallet's logic.
	#[pallet::error]
	pub enum Error<T> {
		VirtualMinerNotFound,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VirtualMinerRewarded {
			account: T::AccountId,
			amount: BalanceOf<T>,
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

	#[pallet::storage]
	#[pallet::getter(fn Virtual_miner_weight_lookup)]
	pub type VirtualMinerWeightLookup<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, OptionQuery>;

	#[pallet::genesis_config]
	/// Genesis config for spf setting pallet
	pub struct GenesisConfig<T: Config> {
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
//			log::info!("block number {:?}, {:?}", n, T::AccountId::type_info());
			let blockIntervalDistribution = 12u32.into();
			if (n % blockIntervalDistribution).is_zero() {
				/*
				let allVirtualMiners = vec![
					"0x8358Cc1d77F700E7D401239ccf5106afE7332bDe",
					"0x9A405e3218c84D029c4aEF1b99E57216c9D17F0b",
					"6FFC840Fe25202e59ED54055d48362A9F1cbb194",
				];
				*/
				let mut allVirtualMiners = Vec::new();
				allVirtualMiners.push("0x8358Cc1d77F700E7D401239ccf5106afE7332bDe");
				allVirtualMiners.push("0x9A405e3218c84D029c4aEF1b99E57216c9D17F0b");
				allVirtualMiners.push("0x6FFC840Fe25202e59ED54055d48362A9F1cbb194");

				for tmp_miner_address_h160 in allVirtualMiners {
					let amt:BalanceOf<T> = 1000000000000000000u128.saturated_into::<BalanceOf<T>>(); //why not u128 ?
					Self::doOneMiningReward(tmp_miner_address_h160, amt);
				}

			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn doOneMiningReward(miner_address_h160 : &str, amt : BalanceOf<T>) -> bool {
			//log::info!("type info : {:?}", T::AccountId::type_info());
//			log::info!("miner_address_h160 {}", miner_address_h160);
			log::info!("{}", miner_address_h160.starts_with("0x"));
			let miner_address_h160_without_prefix =
				if miner_address_h160.starts_with("0x") {
					&miner_address_h160[2..]
				} else {
					miner_address_h160
				};

			let mut h160_raw_data = [0u8; 20];
			hex::decode_to_slice(miner_address_h160_without_prefix, &mut h160_raw_data, ).expect("example data is 20 bytes of valid hex");
			let collator_id = T::AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::new(&h160_raw_data)).unwrap();
//			log::info!("collator_id {:?}", collator_id);

			/*
				let retResult = T::Currency::deposit_into_existing(&collator_id, amt);
				if let Ok(amount_transferred) = retResult {
				//				Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amount_transferred.peek(), });
					log::info!("amount_transferred {:?}", amount_transferred.peek());
				} else if let Err(e) = retResult {
					log::info!("not right {:?}", e);
				}
			*/
			let positive_imbalance = T::Currency::deposit_creating(&collator_id, amt);
			let positive_imbalance_value = positive_imbalance.peek().saturated_into::<u128>();
			if positive_imbalance_value > 0 {
//				log::info!("positive_imbalance_value is {:?}", positive_imbalance_value);
				Self::deposit_event(Event::VirtualMinerRewarded {
					account: collator_id.clone(),
					amount: positive_imbalance.peek().clone(),
				});
			}

			true
		}
	}

}
