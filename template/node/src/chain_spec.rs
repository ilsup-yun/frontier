use std::{collections::BTreeMap, str::FromStr};

use serde::{Deserialize, Serialize};
// Substrate
use sc_chain_spec::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ecdsa, storage::Storage, Pair, Public, H160, U256};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_state_machine::BasicExternalities;
// Frontier
use frontier_template_runtime::{
	AccountId, EnableManualSeal, GenesisConfig, SS58Prefix, Signature, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Specialized `ChainSpec` for development.
pub type DevChainSpec = sc_service::GenericChainSpec<DevGenesisExt>;

/// Extension for the dev genesis config to support a custom changes to the genesis state.
#[derive(Serialize, Deserialize)]
pub struct DevGenesisExt {
	/// Genesis config.
	genesis_config: GenesisConfig,
	/// The flag that if enable manual-seal mode.
	enable_manual_seal: Option<bool>,
}

impl sp_runtime::BuildStorage for DevGenesisExt {
	fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
		BasicExternalities::execute_with_storage(storage, || {
			if let Some(enable_manual_seal) = &self.enable_manual_seal {
				EnableManualSeal::set(enable_manual_seal);
			}
		});
		self.genesis_config.assimilate_storage(storage)
	}
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

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

fn properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());
	properties
}

pub fn development_config(enable_manual_seal: Option<bool>) -> DevChainSpec {
	let wasm_binary = WASM_BINARY.expect("WASM not available");

	DevChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			DevGenesisExt {
				genesis_config: testnet_genesis(
					wasm_binary,
					// Sudo account
					get_account_id_from_seed::<ecdsa::Public>("Alice"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<ecdsa::Public>("Alice"),
						get_account_id_from_seed::<ecdsa::Public>("Bob"),
						get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
						get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
					],
					// Initial PoA authorities
					vec![authority_keys_from_seed("Alice")],
					// Ethereum chain ID
					SS58Prefix::get() as u64,
				),
				enable_manual_seal,
			}
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
		Some(properties()),
		// Extensions
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	let wasm_binary = WASM_BINARY.expect("WASM not available");

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				// Sudo account
				get_account_id_from_seed::<ecdsa::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<ecdsa::Public>("Alice"),
					get_account_id_from_seed::<ecdsa::Public>("Bob"),
					get_account_id_from_seed::<ecdsa::Public>("Charlie"),
					get_account_id_from_seed::<ecdsa::Public>("Dave"),
					get_account_id_from_seed::<ecdsa::Public>("Eve"),
					get_account_id_from_seed::<ecdsa::Public>("Ferdie"),
					get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
					get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
					get_account_id_from_seed::<ecdsa::Public>("Charlie//stash"),
					get_account_id_from_seed::<ecdsa::Public>("Dave//stash"),
					get_account_id_from_seed::<ecdsa::Public>("Eve//stash"),
					get_account_id_from_seed::<ecdsa::Public>("Ferdie//stash"),
				],
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				42,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	sudo_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	chain_id: u64,
) -> GenesisConfig {
	use frontier_template_runtime::{
		AuraConfig, BalancesConfig, EVMChainIdConfig, EVMConfig, GrandpaConfig, SudoConfig,
		SystemConfig,
	};

	GenesisConfig {
		// System
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(sudo_key),
		},

		// Monetary
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		transaction_payment: Default::default(),

		// Consensus
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},

		// EVM compatibility
		evm_chain_id: EVMChainIdConfig { chain_id },
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
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
				map.insert(
					// H160 address for benchmark usage
					H160::from_str("1000000000000000000000000000000000000001")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						nonce: U256::from(1),
						balance: U256::from(1_000_000_000_000_000_000_000_000u128),
						storage: Default::default(),
						code: vec![0x00],
					},
				);
				map
			},
		},
		ethereum: Default::default(),
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}
