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

	pub type TypeVirtualMinerWeight = u128;


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
		VirtualMinerAlreadyExists,
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
		pub fn add_new_virtual_miner(origin: OriginFor<T>, miner_id: T::AccountId, miner_weight: TypeVirtualMinerWeight) -> DispatchResult {
			let account_id = ensure_signed(origin)?;

			ensure!(
				VirtualMinerWeightLookup::<T>::get(&miner_id).is_none(),
				Error::<T>::VirtualMinerAlreadyExists
			);

			log::info!("{:?}, {:?}, {:?}",account_id, miner_id, miner_weight);
			let weight_total_old = VirtualMinerWeightTotal::<T>::get();
			VirtualMinerWeightLookup::<T>::insert(&miner_id, &miner_weight);
			let weight_total_new = weight_total_old + miner_weight;
			<VirtualMinerWeightTotal<T>>::put(weight_total_new);

			Ok(())
		}
	}



	#[pallet::storage]
	#[pallet::getter(fn block_number_interval_distribution)]
	pub type BlockNumberIntervalDistribution<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn virtual_miner_weight_total)]
	pub type VirtualMinerWeightTotal<T: Config> = StorageValue<_, TypeVirtualMinerWeight, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn virtual_miner_weight_lookup)]
	pub type VirtualMinerWeightLookup<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, TypeVirtualMinerWeight, OptionQuery>;

	#[pallet::genesis_config]
	/// Genesis config for spf setting pallet
	pub struct GenesisConfig<T: Config> {
		pub map_virtual_miner_weight: Vec<(T::AccountId, TypeVirtualMinerWeight)>
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				map_virtual_miner_weight : vec![]
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {

			<BlockNumberIntervalDistribution<T>>::put(12);
			let mut total_weight = 0;
			for (miner_id, miner_weight) in &self.map_virtual_miner_weight {
				total_weight += miner_weight;
				VirtualMinerWeightLookup::<T>::insert(&miner_id, &miner_weight);
			}
			<VirtualMinerWeightTotal<T>>::put(total_weight);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: T::BlockNumber) {
//			log::info!("block number {:?}, {:?}", n, T::AccountId::type_info());

			let rewards_each_round:u128 = 1000000000000000000u128;
			let block_interval_distribution = BlockNumberIntervalDistribution::<T>::get().into();
			if (n % block_interval_distribution).is_zero() {
				let total_weight = VirtualMinerWeightTotal::<T>::get();
				let iter = VirtualMinerWeightLookup::<T>::iter();
				iter.for_each(|(miner_id, miner_weight)| {
					log::info!("do mining reward : {:?} , {:?}", miner_id, miner_weight);
					let amt:BalanceOf<T> = (rewards_each_round * miner_weight / total_weight).saturated_into::<BalanceOf<T>>();
//					Self::doOneMiningReward(tmp_miner_address_h160, amt);
					Self::doOneMiningRewardEx(&miner_id, amt);
				})

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

			let ret = Self::doOneMiningRewardEx(&collator_id, amt);
			ret
		}

		fn doOneMiningRewardEx(collator_id : &T::AccountId, amt : BalanceOf<T>) -> bool {
			// log::info!("collator_id {:?}", collator_id);
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
			let positive_imbalance_value = positive_imbalance.peek().saturated_into::<TypeVirtualMinerWeight>();
			if positive_imbalance_value > 0 {
//				log::info!("positive_imbalance_value is {:?}", positive_imbalance_value);
				Self::deposit_event(Event::VirtualMinerRewarded {
					account: collator_id.clone(),
					amount: positive_imbalance.peek().clone(),
				});
			}
			true
		}

		pub fn miner_weight_of(account_id: &T::AccountId) -> Option<TypeVirtualMinerWeight> {
			VirtualMinerWeightLookup::<T>::get(account_id)
		}
	}

}
