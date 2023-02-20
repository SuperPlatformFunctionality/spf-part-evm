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

	pub type TypeVirtualNodeWeight = u128;


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
		SpfFoundationAccountNotFound,
		SpfFoundationAccountAlreadyExist,
		VirtualMinerNotFound,
		VirtualMinerAlreadyExists,
		VirtualNodeNotFound,
		VirtualNodeAlreadyExists,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SpfFoundationRewarded {
			account: T::AccountId,
			amount: BalanceOf<T>,
		},
		VirtualMinerRewarded {
			account: T::AccountId,
			amount: BalanceOf<T>,
		},
		VirtualNodeRewarded {
			account: T::AccountId,
			amount: BalanceOf<T>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn add_new_virtual_node(origin: OriginFor<T>, node_id: T::AccountId, node_weight: TypeVirtualNodeWeight) -> DispatchResult {
			let account_id = ensure_signed(origin)?;

			ensure!(
				VirtualNodeWeightLookup::<T>::get(&node_id).is_none(),
				Error::<T>::VirtualNodeAlreadyExists
			);

			log::info!("{:?}, {:?}, {:?}",account_id, node_id, node_weight);
			let weight_total_old = VirtualNodeWeightTotal::<T>::get();
			VirtualNodeWeightLookup::<T>::insert(&node_id, &node_weight);
			let weight_total_new = weight_total_old + node_weight;
			<VirtualNodeWeightTotal<T>>::put(weight_total_new);

			Ok(())
		}
	}

	//block interval to distribute rewards
	#[pallet::storage]
	#[pallet::getter(fn block_number_interval_distribution)]
	pub type BlockNumberIntervalDistribution<T: Config> = StorageValue<_, u32, ValueQuery>;

	//SPF Foundation
	#[pallet::storage]
	#[pallet::getter(fn spf_foundation_account)]
	pub type SpfFoundationAccounts<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	//virtual miner
	#[pallet::storage]
	#[pallet::getter(fn virtual_miners)]
	pub type VirtualMiners<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	//virtual node
	#[pallet::storage]
	#[pallet::getter(fn virtual_node_weight_total)]
	pub type VirtualNodeWeightTotal<T: Config> = StorageValue<_, TypeVirtualNodeWeight, ValueQuery>;
	#[pallet::storage]
	#[pallet::getter(fn virtual_node_weight_lookup)]
	pub type VirtualNodeWeightLookup<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, TypeVirtualNodeWeight, OptionQuery>;

	#[pallet::genesis_config]
	/// Genesis config for spf setting pallet
	pub struct GenesisConfig<T: Config> {
		pub spf_foundation_accounts: Vec<T::AccountId>,
		pub vec_virtual_miners: Vec<T::AccountId>,
		pub map_virtual_node_weight: Vec<(T::AccountId, TypeVirtualNodeWeight)>
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				spf_foundation_accounts : vec![],
				vec_virtual_miners : vec![],
				map_virtual_node_weight : vec![]
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			//block interval to distribute reward
			<BlockNumberIntervalDistribution<T>>::put(12);

			//spf foundation
			<SpfFoundationAccounts<T>>::put(&self.spf_foundation_accounts);

			//virtual miners
			<VirtualMiners<T>>::put(&self.vec_virtual_miners);

			//virtual nodes
			let mut total_weight = 0;
			for (node_id, node_weight) in &self.map_virtual_node_weight {
				total_weight += node_weight;
				VirtualNodeWeightLookup::<T>::insert(&node_id, &node_weight);
			}
			<VirtualNodeWeightTotal<T>>::put(total_weight);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/*
		fn on_initialize(_: T::BlockNumber) -> Weight {
			let mut weight = T::WeightInfo::base_on_initialize();
			//what is the weight?
			weight;
		}
		*/

		fn on_finalize(n: T::BlockNumber) {
//			log::info!("block number {:?}, {:?}", n, T::AccountId::type_info());
			let block_interval_distribution = BlockNumberIntervalDistribution::<T>::get();
			let rewards_everyday:u128 = 13698000000000000000000u128;
			let rewards_each_round:u128 = rewards_everyday / (((3600*24 as u32)/6u32 * block_interval_distribution) as u128);
			log::info!("rewards_each_round {:?}", rewards_each_round);
			if (n % block_interval_distribution.into()).is_zero() {
				//distribute to spf foundation
				{
					let rewards_each_round_to_foundation = rewards_each_round * 5u128 / 100u128;
					let foundation_accounts = <SpfFoundationAccounts<T>>::get();
					let reward_each_foundation:BalanceOf<T> =
						(rewards_each_round_to_foundation / (foundation_accounts.len() as u128)).saturated_into::<BalanceOf<T>>();

					for tmp_account in &foundation_accounts {
						log::info!("do foundation reward {:?},{:?}", tmp_account, reward_each_foundation);
						Self::send_one_reward_by_account_id(&tmp_account, reward_each_foundation);
						Self::deposit_event(Event::SpfFoundationRewarded {
							account: tmp_account.clone(),
							amount: reward_each_foundation.clone(),
						});
					}
				}

				//distribution to virtual miners
				{
					let rewards_each_round_to_miners = rewards_each_round * 25u128 / 100u128;
					let v_miners = <VirtualMiners<T>>::get();
					let reward_each_miner:BalanceOf<T> =
						(rewards_each_round_to_miners / (v_miners.len() as u128)).saturated_into::<BalanceOf<T>>();

					for tmp_account in &v_miners {
						log::info!("do miner reward {:?},{:?}", tmp_account, reward_each_miner);
						Self::send_one_reward_by_account_id(&tmp_account, reward_each_miner);
						Self::deposit_event(Event::VirtualMinerRewarded {
							account: tmp_account.clone(),
							amount: reward_each_miner.clone(),
						});
					}
				}

				//distribution to virtual node
				{
					let rewards_each_round_to_nodes = rewards_each_round * 75u128 / 100u128;
					let total_weight = VirtualNodeWeightTotal::<T>::get();
					let iter = VirtualNodeWeightLookup::<T>::iter();
					iter.for_each(|(node_id, node_weight)| {
						log::info!("do node reward : {:?} , {:?}", node_id, node_weight);
						let amt:BalanceOf<T> = (rewards_each_round_to_nodes * node_weight / total_weight).saturated_into::<BalanceOf<T>>();
						Self::send_one_reward_by_account_id(&node_id, amt);
						Self::deposit_event(Event::VirtualNodeRewarded {
							account: node_id.clone(),
							amount: amt.clone(),
						});
					})
				}
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn getAccountIdByH160HexStr(node_address_h160 : &str) -> T::AccountId {
			let node_address_h160_without_prefix =
				if node_address_h160.starts_with("0x") {
					&node_address_h160[2..]
				} else {
					node_address_h160
				};

			let mut h160_raw_data = [0u8; 20];
			hex::decode_to_slice(node_address_h160_without_prefix, &mut h160_raw_data, ).expect("example data is 20 bytes of valid hex");
			let account_id = T::AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::new(&h160_raw_data)).unwrap();
			account_id
		}

		fn send_one_reward_by_h160_hex_address(node_address_h160 : &str, amt : BalanceOf<T>) -> bool {
			log::info!("node_address_h160 : {}", node_address_h160);
			let collator_id = Self::getAccountIdByH160HexStr(node_address_h160);
			let ret = Self::send_one_reward_by_account_id(&collator_id, amt);
			ret
		}

		fn send_one_reward_by_account_id(collator_id : &T::AccountId, amt : BalanceOf<T>) -> bool {
			// log::info!("collator_id {:?}", collator_id);
			/*
				let retResult = T::Currency::deposit_into_existing(&collator_id, amt);
				if let Ok(amount_transferred) = retResult {
					log::info!("amount_transferred {:?}", amount_transferred.peek());
				} else if let Err(e) = retResult {
					log::info!("not right {:?}", e);
				}
			*/
			let positive_imbalance = T::Currency::deposit_creating(&collator_id, amt);
			let positive_imbalance_value = positive_imbalance.peek().saturated_into::<TypeVirtualNodeWeight>();
			if positive_imbalance_value > 0 {
//				log::info!("positive_imbalance_value is {:?}", positive_imbalance_value);
				/*
				Self::deposit_event(Event::VirtualNodeRewarded {
					account: collator_id.clone(),
					amount: positive_imbalance.peek().clone(),
				});
				*/
			}
			true
		}

		pub fn virtual_node_weight_of(account_id: &T::AccountId) -> Option<TypeVirtualNodeWeight> {
			VirtualNodeWeightLookup::<T>::get(account_id)
		}
	}

}
