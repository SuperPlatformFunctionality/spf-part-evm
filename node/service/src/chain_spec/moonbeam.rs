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

//! Moonbeam Chain Specifications and utilities for building them.
//!
//! Learn more about Substrate chain specifications at
//! https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec

#[cfg(test)]
use crate::chain_spec::{derive_bip44_pairs_from_mnemonic, get_account_id_from_pair};
use crate::chain_spec::{generate_accounts, get_from_seed, get_from_seed_with_password, Extensions};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use moonbeam_runtime::EligibilityValue;
use moonbeam_runtime::{
	currency::GLMR, currency::SUPPLY_FACTOR, AccountId, AuraConfig, Balance, BalancesConfig,
	EVMConfig, EthereumChainIdConfig, EthereumConfig, SpfSettingConfig, GenesisAccount, GenesisConfig, GrandpaConfig,
	InflationInfo, Range, Signature, SystemConfig, HOURS, WASM_BINARY,
};
use nimbus_primitives::NimbusId;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
#[cfg(test)]
use sp_core::ecdsa;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::{Perbill, Percent};
use std::{collections::BTreeMap, str::FromStr};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

/// Generate a chain spec for use with the development service.
pub fn development_chain_spec(mnemonic: Option<String>, num_accounts: Option<u32>) -> ChainSpec {
	// Default mnemonic if none was provided
	let parent_mnemonic = mnemonic.unwrap_or_else(|| {
		"bottom drive obey lake curtain smoke basket hold race lonely fit walk".to_string()
	});
	/*
	let mut init_accounts = generate_accounts(parent_mnemonic, num_accounts.unwrap_or(10));
	// We add Gerald here
	init_accounts.push(AccountId::from(hex!(
		"6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b"
	)));
	*/
	let mut init_accounts = generate_accounts(parent_mnemonic, num_accounts.unwrap_or(1));
	ChainSpec::from_genesis(
		"SPF Testnet",
		"moonbeam_spf_testnet",
		ChainType::Development,
		move || {
			testnet_genesis(
				init_accounts.clone(),
				Default::default(), // para_id
				1280,                 //ChainId
				vec![
					authority_keys_from_seed("Alice")
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 18, \"tokenSymbol\": \"GLMR\", \"SS58Prefix\": 1284}",
			)
			.expect("Provided valid json map"),
		),
		// Extensions
		Extensions {
			relay_chain: "dev-service".into(),
			para_id: Default::default(),
		},
	)
}

/// Generate a default spec for the parachain service. Use this as a starting point when launching
/// a custom chain.
pub fn get_chain_spec() -> ChainSpec {
	ChainSpec::from_genesis(
		// TODO Apps depends on this string to determine whether the chain is an ethereum compat
		// or not. We should decide the proper strings, and update Apps accordingly.
		// Or maybe Apps can be smart enough to say if the string contains "moonbeam" at all...
		"SPF MainNet",
		"moonbeam_spf_mainnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Endowed: Alith, Baltathar, Charleth and Dorothy
				vec![
//					AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
//					AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")),
//					AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")),
//					AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")),
				],
				Default::default(), // para_id
				1280,               //ChainId
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
					authority_keys_from_seed("Charlie"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 18, \"tokenSymbol\": \"GLMR\", \"SS58Prefix\": 1284}",
			)
			.expect("Provided valid json map"),
		),
		// Extensions
		Extensions {
			relay_chain: "polkadot-local".into(),
			para_id: Default::default(),
		},
	)
}

const COLLATOR_COMMISSION: Perbill = Perbill::from_percent(20);
const PARACHAIN_BOND_RESERVE_PERCENT: Percent = Percent::from_percent(30);
const BLOCKS_PER_ROUND: u32 = 6 * HOURS;
pub fn moonbeam_inflation_config() -> InflationInfo<Balance> {
	fn to_round_inflation(annual: Range<Perbill>) -> Range<Perbill> {
		use pallet_parachain_staking::inflation::{
			perbill_annual_to_perbill_round, BLOCKS_PER_YEAR,
		};
		perbill_annual_to_perbill_round(
			annual,
			// rounds per year
			BLOCKS_PER_YEAR / BLOCKS_PER_ROUND,
		)
	}
	let annual = Range {
		min: Perbill::from_percent(4),
		ideal: Perbill::from_percent(5),
		max: Perbill::from_percent(5),
	};
	InflationInfo {
		// staking expectations
		expect: Range {
			min: 100_000 * GLMR * SUPPLY_FACTOR,
			ideal: 200_000 * GLMR * SUPPLY_FACTOR,
			max: 500_000 * GLMR * SUPPLY_FACTOR,
		},
		// annual inflation
		annual,
		round: to_round_inflation(annual),
	}
}

pub fn testnet_genesis(
	endowed_accounts: Vec<AccountId>,
	para_id: ParaId,
	chain_id: u64,
	initial_authorities: Vec<(AuraId, GrandpaId)>,
) -> GenesisConfig {
	// This is the simplest bytecode to revert without returning any data.
	// We will pre-deploy it under all of our precompiles to ensure they can be called from
	// within contracts.
	// (PUSH1 0x00 PUSH1 0x00 REVERT)
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	GenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
//				.map(|k| (k, 1 << 80))
				.map(|k| (k, 1000000 * u128::pow(10,18)))
				.collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		ethereum_chain_id: EthereumChainIdConfig { chain_id },
		evm: EVMConfig {
			// We need _some_ code inserted at the precompile address so that
			// the evm will actually call the address.
			accounts: {
				let mut map = BTreeMap::new();
				/*
				map.insert(
					// H160 address of Alice dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)

					H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);

				map.insert(
					H160::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);

				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				*/
				map
			},
		},
		ethereum: EthereumConfig {},
		base_fee: Default::default(),
		spf_setting:SpfSettingConfig {
			spf_foundation_accounts:vec![
				AccountId::from(hex!("C670813707C92dC82a43f741008D711bD2249bd8"))
			],
			vec_virtual_miners:vec![
				AccountId::from(hex!("b91944FEf72B09629b5Cf79b134f20bD722F9d82")),
				AccountId::from(hex!("E8C6c1fCb6669e7fe513F27080B0CfdB820090F9")),
				AccountId::from(hex!("b2f60A2EA554B9139f0bD9983F1DA315a0F65248")),
				AccountId::from(hex!("5518C505F1989a9ad212Ef14c31b370440117706")),
				AccountId::from(hex!("F6aE36ddDE5B4986169E072Be691A83C46123D4d")),
				AccountId::from(hex!("60Aa7e50F78B2F9a6D4874BE181900AA08813c45")),
				AccountId::from(hex!("6Ab06bbA843d2B7D5bec1e8CB642681010c9Fdc7")),
				AccountId::from(hex!("E7C69584C29d08df87D4B675CD759175E82Df8fE")),
				AccountId::from(hex!("79E448aa6738Ea9e3E221a3C22dB87784fe3274F")),
				AccountId::from(hex!("87944DE36fFCbBA0f4eb2db826bB8fb67cc41a89")),
				AccountId::from(hex!("175268D0dAA7EFf1af2D3fF3A9A85b68c11F36eD")),
				AccountId::from(hex!("14B2F60De647742BCE403bF6E5B1b4F3bc6691c6")),
				AccountId::from(hex!("2A91B5e4c73060f0AabC589e718D38D456c07544")),
				AccountId::from(hex!("d3db1c538485aC3Be0958f5F999a0Ac72795B403")),
				AccountId::from(hex!("EdccaE47004A8F8Ea6A8ED874478CfBDB4A15677")),
				AccountId::from(hex!("351DE59eA8ffB4d5D7C0Da3Ae3A879208d325D30")),
				AccountId::from(hex!("4e2787B473684eb29fd8da01195B7D5D5a10b2EA")),
				AccountId::from(hex!("64b0ea35e1b51cBCc68b0e8C98338d849bebAfe0")),
				AccountId::from(hex!("1fF6fF9DECdf62b3F6cc1D2B223d933c80E03C98")),
				AccountId::from(hex!("21DB53fc2c7aD068316D2b7815e7f3a38f0db474")),
				AccountId::from(hex!("0454c3A187d50B90eB8Db3c3a8052cD8a82A8738"))
			],
			map_virtual_node_weight :vec![
				(AccountId::from(hex!("8358Cc1d77F700E7D401239ccf5106afE7332bDe")), 10),
				(AccountId::from(hex!("9A405e3218c84D029c4aEF1b99E57216c9D17F0b")), 20),
				(AccountId::from(hex!("6FFC840Fe25202e59ED54055d48362A9F1cbb194")), 50)
			]
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_derived_pairs_1() {
		let mnemonic =
			"bottom drive obey lake curtain smoke basket hold race lonely fit walk".to_string();
		let accounts = 10;
		let pairs = derive_bip44_pairs_from_mnemonic::<ecdsa::Public>(&mnemonic, accounts);
		let first_account = get_account_id_from_pair(pairs.first().unwrap().clone()).unwrap();
		let last_account = get_account_id_from_pair(pairs.last().unwrap().clone()).unwrap();

		let expected_first_account =
			AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"));
		let expected_last_account =
			AccountId::from(hex!("2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423"));
		assert_eq!(first_account, expected_first_account);
		assert_eq!(last_account, expected_last_account);
		assert_eq!(pairs.len(), 10);
	}
	#[test]
	fn test_derived_pairs_2() {
		let mnemonic =
			"slab nerve salon plastic filter inherit valve ozone crash thumb quality whale"
				.to_string();
		let accounts = 20;
		let pairs = derive_bip44_pairs_from_mnemonic::<ecdsa::Public>(&mnemonic, accounts);
		let first_account = get_account_id_from_pair(pairs.first().unwrap().clone()).unwrap();
		let last_account = get_account_id_from_pair(pairs.last().unwrap().clone()).unwrap();

		let expected_first_account =
			AccountId::from(hex!("1e56ca71b596f2b784a27a2fdffef053dbdeff83"));
		let expected_last_account =
			AccountId::from(hex!("4148202BF0c0Ad7697Cff87EbB83340C80c947f8"));
		assert_eq!(first_account, expected_first_account);
		assert_eq!(last_account, expected_last_account);
		assert_eq!(pairs.len(), 20);

		println!("test 1 {:?}, {:?}, {:?}", get_from_seed::<AuraId>("Alice"), get_from_seed::<GrandpaId>("Alice"), get_from_seed::<NimbusId>("Alice"));

		let account_aura = get_from_seed_with_password::<AuraId>("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", "");
		let account_grandpa = get_from_seed_with_password::<GrandpaId>("0xabf8e5bdbe30c65656c0a3cbd181ff8a56294a69dfedd27982aace4a76909115", "");
		let account_nimbus = get_from_seed_with_password::<NimbusId>("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", "");
		println!("test 2 {:?}, {:?}, {:?}", account_aura, account_grandpa, account_nimbus);

//		let s = AuraId::from_str("5D4buZeSvZtZxyFWH7CQv1RRf7WvYuLwYuTMQ8uu3WL9dmdk", None);
//		println!("{:?}", s);
	}
}
