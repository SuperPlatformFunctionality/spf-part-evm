// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! The Moonbeam Runtime.
//!
//! Primary features of this runtime include:
//! * Ethereum compatibility
//! * Moonbeam tokenomics

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use account::AccountId20;
use cumulus_pallet_parachain_system::{RelayChainStateProof, RelaychainBlockNumberProvider};
use cumulus_primitives_core::relay_chain;
use fp_rpc::TransactionStatus;

// Re-export required by get! macro.
use cumulus_primitives_core::{relay_chain::BlockNumber as RelayBlockNumber, DmpMessageHandler};
#[cfg(feature = "std")]
pub use fp_evm::GenesisAccount;
pub use frame_support::traits::Get;
use frame_support::{
	construct_runtime,
	pallet_prelude::DispatchResult,
	parameter_types,
	traits::{
		ConstBool, ConstU128, ConstU16, ConstU32, ConstU64, ConstU8, Contains,
		Currency as CurrencyT, EitherOfDiverse, EqualPrivilegeOnly, Imbalance, InstanceFilter,
		OffchainWorker, OnFinalize, OnIdle, OnInitialize, OnRuntimeUpgrade, OnUnbalanced,
        KeyOwnerProofSystem,
	},
	weights::{
		constants::{RocksDbWeight, WEIGHT_PER_SECOND},
		ConstantMultiplier, DispatchClass, GetDispatchInfo, Weight, WeightToFeeCoefficient,
		WeightToFeeCoefficients, WeightToFeePolynomial,
	},
	PalletId, ConsensusEngineId,
};
use frame_system::{EnsureRoot, EnsureSigned};
pub use moonbeam_core_primitives::{
	AccountId, AccountIndex, Address, AssetId, Balance, BlockNumber, DigestItem, Hash, Header,
	Index, Signature,
};
use moonbeam_rpc_primitives_txpool::TxPoolResponse;
pub use pallet_author_slot_filter::EligibilityValue;
use pallet_balances::NegativeImbalance;
use pallet_ethereum::Call::transact;
use pallet_ethereum::Transaction as EthereumTransaction;
use pallet_evm::{
	Account as EVMAccount, EVMCurrencyAdapter, EnsureAddressNever, EnsureAddressRoot,
	FeeCalculator, GasWeightMapping, OnChargeEVMTransaction as OnChargeEVMTransactionT, Runner,
    EnsureAddressTruncated, HashedAddressMapping
};
use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
pub use pallet_parachain_staking::{InflationInfo, Range};
use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{
    crypto::{ByteArray, KeyTypeId},
    OpaqueMetadata, H160, H256, U256
};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		BlakeTwo256, Block as BlockT, DispatchInfoOf, Dispatchable, IdentityLookup,
		PostDispatchInfoOf, UniqueSaturatedInto, NumberFor,
	},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, TransactionValidityError,
	},
	ApplyExtrinsicResult, FixedPointNumber, Perbill, Permill, Perquintill, SaturatedConversion,
};
use sp_std::{convert::TryFrom, prelude::*};

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use nimbus_primitives::CanAuthor;

mod precompiles;
use precompiles::FrontierPrecompiles;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;


/// GLMR, the native token, uses 18 decimals of precision.
pub mod currency {
	use super::Balance;

	// Provide a common factor between runtimes based on a supply of 10_000_000 tokens.
	pub const SUPPLY_FACTOR: Balance = 100;

	pub const WEI: Balance = 1;
	pub const KILOWEI: Balance = 1_000;
	pub const MEGAWEI: Balance = 1_000_000;
	pub const GIGAWEI: Balance = 1_000_000_000;
	pub const MICROGLMR: Balance = 1_000_000_000_000;
	pub const MILLIGLMR: Balance = 1_000_000_000_000_000;
	pub const GLMR: Balance = 1_000_000_000_000_000_000;
	pub const KILOGLMR: Balance = 1_000_000_000_000_000_000_000;

	pub const TRANSACTION_BYTE_FEE: Balance = 1 * GIGAWEI * SUPPLY_FACTOR;
	pub const STORAGE_BYTE_FEE: Balance = 100 * MICROGLMR * SUPPLY_FACTOR;
	pub const WEIGHT_FEE: Balance = 50 * KILOWEI * SUPPLY_FACTOR;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 100 * MILLIGLMR * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
	}
}

/// Maximum weight per block
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_div(2);

pub const MILLISECS_PER_BLOCK: u64 = 12000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const WEEKS: BlockNumber = DAYS * 7;
pub const MONTHS: BlockNumber = DAYS * 30;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;

	impl_opaque_keys! {
		pub struct SessionKeys {
            pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

/// This runtime version.
/// The spec_version is composed of 2x2 digits. The first 2 digits represent major changes
/// that can't be skipped, such as data migration upgrades. The last 2 digits represent minor
/// changes which can be skipped.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("moonbeam"),
	impl_name: create_runtime_str!("moonbeam"),
	authoring_version: 3,
	spec_version: 2000,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 2,
	state_version: 0,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const NORMAL_WEIGHT: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_mul(3).saturating_div(4);
// Here we assume Ethereum's base fee of 21000 gas and convert to weight, but we
// subtract roughly the cost of a balance transfer from it (about 1/3 the cost)
// and some cost to account for per-byte-fee.
// TODO: we should use benchmarking's overhead feature to measure this
pub const EXTRINSIC_BASE_WEIGHT: Weight = Weight::from_ref_time(10000 * WEIGHT_PER_GAS);

pub struct RuntimeBlockWeights;
impl Get<frame_system::limits::BlockWeights> for RuntimeBlockWeights {
	fn get() -> frame_system::limits::BlockWeights {
		frame_system::limits::BlockWeights::builder()
			.for_class(DispatchClass::Normal, |weights| {
				weights.base_extrinsic = EXTRINSIC_BASE_WEIGHT;
				weights.max_total = NORMAL_WEIGHT.into();
			})
			.for_class(DispatchClass::Operational, |weights| {
				weights.max_total = MAXIMUM_BLOCK_WEIGHT.into();
				weights.reserved = (MAXIMUM_BLOCK_WEIGHT - NORMAL_WEIGHT).into();
			})
			.avg_block_initialization(Perbill::from_percent(10))
			.build()
			.expect("Provided BlockWeight definitions are valid, qed")
	}
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	/// We allow for 5 MB blocks.
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = IdentityLookup<AccountId>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = ConstU32<256>;
	/// Maximum weight of each block. With a default weight system of 1byte == 1weight, 4mb is ok.
	type BlockWeights = RuntimeBlockWeights;
	/// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
	type BlockLength = BlockLength;
	/// Runtime version.
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = RocksDbWeight;
	type BaseCallFilter = frame_support::traits::Everything;
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = ConstU16<1284>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}


pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}


impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

impl pallet_balances::Config for Runtime {
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 4];
	type MaxLocks = ConstU32<50>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<0>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}


pub struct LengthToFee;
impl WeightToFeePolynomial for LengthToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		smallvec![
			WeightToFeeCoefficient {
				degree: 1,
				coeff_frac: Perbill::zero(),
				coeff_integer: currency::TRANSACTION_BYTE_FEE,
				negative: false,
			},
			WeightToFeeCoefficient {
				degree: 3,
				coeff_frac: Perbill::zero(),
				coeff_integer: 1 * currency::SUPPLY_FACTOR,
				negative: false,
			},
		]
	}
}

impl pallet_transaction_payment::Config for Runtime {
	type Event = Event;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = ConstantMultiplier<Balance, ConstU128<{ currency::WEIGHT_FEE }>>;
	type LengthToFee = LengthToFee;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Runtime>;
}

impl pallet_ethereum_chain_id::Config for Runtime {}

impl pallet_randomness_collective_flip::Config for Runtime {}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND.ref_time() / GAS_PER_SECOND;

parameter_types! {
	pub BlockGasLimit: U256 = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
    //pub BlockGasLimit: U256 = U256::from(u32::max_value());

	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly. This low value causes changes to occur slowly over time.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero` in integration_tests.rs.
	/// This value is currently only used by pallet-transaction-payment as an assertion that the
	/// next multiplier is always > min value.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);
	/// Maximum multiplier. We pick a value that is expensive but not impossibly so; it should act
	/// as a safety net.
	pub MaximumMultiplier: Multiplier = Multiplier::from(100_000u128);
    pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
	pub WeightPerGas: u64 = WEIGHT_PER_GAS;
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (U256, Weight) {
		(
			(1 * currency::GIGAWEI * currency::SUPPLY_FACTOR).into(),
			Weight::zero(),
		)
	}
}

/// Parameterized slow adjusting fee updated based on
/// https://w3f-research.readthedocs.io/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism // editorconfig-checker-disable-line
///
/// The adjustment algorithm boils down to:
///
/// diff = (previous_block_weight - target) / maximum_block_weight
/// next_multiplier = prev_multiplier * (1 + (v * diff) + ((v * diff)^2 / 2))
/// assert(next_multiplier > min)
///     where: v is AdjustmentVariable
///            target is TargetBlockFullness
///            min is MinimumMultiplier
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
	R,
	TargetBlockFullness,
	AdjustmentVariable,
	MinimumMultiplier,
	MaximumMultiplier,
>;

use frame_support::traits::FindAuthor;
//TODO It feels like this shold be able to work for any T: H160, but I tried for
// embarassingly long and couldn't figure that out.

/// The author inherent provides a AccountId20, but pallet evm needs an H160.
/// This simple adapter makes the conversion.
pub struct FindAuthorAdapter<Inner>(sp_std::marker::PhantomData<Inner>);

impl<Inner> FindAuthor<H160> for FindAuthorAdapter<Inner>
where
	Inner: FindAuthor<AccountId20>,
{
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (sp_runtime::ConsensusEngineId, &'a [u8])>,
	{
		Inner::find_author(digests).map(Into::into)
	}
}

pub struct FindAuthorTruncated<F>(sp_std::marker::PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Aura::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
		}
		None
	}
}


moonbeam_runtime_common::impl_on_charge_evm_transaction!();

impl pallet_evm::Config for Runtime {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = EnsureAddressNever<AccountId>;
	type AddressMapping = moonbeam_runtime_common::IntoAddressMapping;
	type Currency = Balances;
	type Event = Event;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
    type PrecompilesType = FrontierPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = EthereumChainId;
	type OnChargeTransaction = ();
	type BlockGasLimit = BlockGasLimit;
    type FindAuthor = FindAuthorTruncated<Aura>;
}

parameter_types! {
	pub const MaxAuthorities: u32 = 100;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

impl pallet_grandpa::Config for Runtime {
	type Event = Event;
	type Call = Call;

	type KeyOwnerProofSystem = ();

	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;

	type HandleEquivocation = ();

	type WeightInfo = ();
	type MaxAuthorities = ConstU32<32>;
}



parameter_types! {
	pub DefaultBaseFeePerGas: U256 = (1 * currency::GIGAWEI * currency::SUPPLY_FACTOR).into();
	pub DefaultElasticity: Permill = Permill::zero();
}

pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
	fn lower() -> Permill {
		Permill::zero()
	}
	fn ideal() -> Permill {
		Permill::from_parts(500_000)
	}
	fn upper() -> Permill {
		Permill::from_parts(1_000_000)
	}
}

impl pallet_base_fee::Config for Runtime {
	type Event = Event;
	type Threshold = BaseFeeThreshold;
	// Tells `pallet_base_fee` whether to calculate a new BaseFee `on_finalize` or not.
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
}






pub struct TransactionConverter;

impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for TransactionConverter {
	fn convert_transaction(&self, transaction: pallet_ethereum::Transaction) -> UncheckedExtrinsic {
		UncheckedExtrinsic::new_unsigned(
			pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
		)
	}
}

impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for TransactionConverter {
	fn convert_transaction(
		&self,
		transaction: pallet_ethereum::Transaction,
	) -> opaque::UncheckedExtrinsic {
		let extrinsic = UncheckedExtrinsic::new_unsigned(
			pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
		);
		let encoded = extrinsic.encode();
		opaque::UncheckedExtrinsic::decode(&mut &encoded[..])
			.expect("Encoded extrinsic is always valid")
	}
}

impl pallet_ethereum::Config for Runtime {
	type Event = Event;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
}


construct_runtime! {
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// System support stuff.
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>} = 0,
		//ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Storage, Inherent, Event<T>} = 1,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 2,
        Aura: pallet_aura::{Pallet, Config<T>} = 5,
		Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 3,
		//ParachainInfo: parachain_info::{Pallet, Storage, Config} = 4,

		// Monetary stuff.
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 11,


		// Consensus support.
		//ParachainStaking: pallet_parachain_staking::{Pallet, Call, Storage, Event<T>, Config<T>} = 20,
		//AuthorInherent: pallet_author_inherent::{Pallet, Call, Storage, Inherent} = 21,
		//AuthorFilter: pallet_author_slot_filter::{Pallet, Call, Storage, Event, Config} = 22,
		//AuthorMapping: pallet_author_mapping::{Pallet, Call, Config<T>, Storage, Event<T>} = 23,
		//MoonbeamOrbiters: pallet_moonbeam_orbiters::{Pallet, Call, Storage, Event<T>} = 24,

		// Handy utilities.
		//Utility: pallet_utility::{Pallet, Call, Event} = 30,
		//Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>} = 31,
		//MaintenanceMode: pallet_maintenance_mode::{Pallet, Call, Config, Storage, Event} = 32,
		//Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 33,
		//Migrations: pallet_migrations::{Pallet, Storage, Config, Event<T>} = 34,
		//ProxyGenesisCompanion: pallet_proxy_genesis_companion::{Pallet, Config<T>} = 35,

		// Has been permanently removed for safety reasons.
		// Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 40,

		// Ethereum compatibility.
		EthereumChainId: pallet_ethereum_chain_id::{Pallet, Storage, Config} = 50,
		EVM: pallet_evm::{Pallet, Config, Call, Storage, Event<T>} = 51,
		Ethereum: pallet_ethereum::{Pallet, Call, Storage, Event, Origin, Config} = 52,
		BaseFee: pallet_base_fee::{Pallet, Call, Storage, Config<T>, Event} = 53,

		// Governance stuff.
		//Scheduler: pallet_scheduler::{Pallet, Storage, Event<T>, Call} = 60,
		//Democracy: pallet_democracy::{Pallet, Storage, Config<T>, Event<T>, Call} = 61,

		// Council stuff.
		//CouncilCollective: pallet_collective::<Instance1>::{Pallet, Call, Storage, Event<T>, Origin<T>, Config<T>} = 70,
		//TechCommitteeCollective: pallet_collective::<Instance2>::{Pallet, Call, Storage, Event<T>, Origin<T>, Config<T>} = 71,
		//TreasuryCouncilCollective: pallet_collective::<Instance3>::{Pallet, Call, Storage, Event<T>, Origin<T>, Config<T>} = 72,

		// Treasury stuff.
		//Treasury: pallet_treasury::{Pallet, Storage, Config, Event<T>, Call} = 80,

		// Crowdloan stuff.
		//CrowdloanRewards: pallet_crowdloan_rewards::{Pallet, Call, Config<T>, Storage, Event<T>} = 90,

		// XCM
		//XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Storage, Event<T>} = 100,
		//CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 101,
		//DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 102,
		//PolkadotXcm: pallet_xcm::{Pallet, Storage, Call, Event<T>, Origin, Config} = 103,
		//Assets: pallet_assets::{Pallet, Call, Storage, Event<T>} = 104,
		//AssetManager: pallet_asset_manager::{Pallet, Call, Storage, Event<T>} = 105,
		//XTokens: orml_xtokens::{Pallet, Call, Storage, Event<T>} = 106,
		//XcmTransactor: pallet_xcm_transactor::{Pallet, Call, Storage, Event<T>} = 107,
		//LocalAssets: pallet_assets::<Instance1>::{Pallet, Call, Storage, Event<T>} = 108,

		// Randomness
		//Randomness: pallet_randomness::{Pallet, Call, Storage, Event<T>, Inherent} = 120,
	}
}

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	fp_self_contained::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = fp_self_contained::CheckedExtrinsic<AccountId, Call, SignedExtra, H160>;
/// Executive: handles dispatch to the various pallets.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
    AllPalletsWithSystem,

>;

// All of our runtimes share most of their Runtime API implementations.
// We use a macro to implement this common part and add runtime-specific additional implementations.
// This macro expands to :
// ```
// impl_runtime_apis! {
//     // All impl blocks shared between all runtimes.
//
//     // Specific impls provided to the `impl_runtime_apis_plus_common!` macro.
// }
// ```
moonbeam_runtime_common::impl_runtime_apis_plus_common! {
    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> fg_primitives::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }
    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx.clone(), block_hash)
		}
	}

}


moonbeam_runtime_common::impl_self_contained_call!();

// Shorthand for a Get field of a pallet Config.
#[macro_export]
macro_rules! get {
	($pallet:ident, $name:ident, $type:ty) => {
		<<$crate::Runtime as $pallet::Config>::$name as $crate::Get<$type>>::get()
	};
}

#[cfg(test)]
mod tests {
}
