// Copyright 2018 akropolis
extern crate base58;
extern crate substrate_primitives;

use self::base58::FromBase58;
use akro_runtime::{
    BalancesConfig, BalancesConfigCopy, ConsensusConfig, ContractConfig, CouncilVotingConfig, DemocracyConfig,
    GenesisConfig, Params, Perbill, Permill, Runtime, SessionConfig, StakingConfig, 
    TimestampConfig, Token, TokenBalancesConfig, TokenStakingConfig, TreasuryConfig, 
    };

use super::cli::ChainSpec;
use ed25519;
use ed25519::Public;

pub fn testnet_genesis(chainspec: ChainSpec) -> GenesisConfig {
    let alice = ed25519::Pair::from_seed(b"Alice                           ").public();
    let bob = ed25519::Pair::from_seed(b"Bob                             ").public();
    let _charlie = ed25519::Pair::from_seed(b"Charlie                         ").public();
    let _dave = ed25519::Pair::from_seed(b"Dave                            ").public();
    let gavin = ed25519::Pair::from_seed(b"Gavin                           ").public();
    let satoshi = ed25519::Pair::from_seed(b"Satoshi                         ").public();

    let auth1 = alice.into();
    let auth2 = bob.into();
    let auth3 = gavin.into();
    let auth4 = satoshi.into();
    let initial_authorities = match chainspec {
        ChainSpec::Dev => vec![auth1],
        ChainSpec::Local => vec![auth1, auth2],
        ChainSpec::Multi => vec![auth1, auth2, auth3, auth4],
    };

    //    const MILLICENTS: u128 = 1_000_000_000;
    //    const CENTS: u128 = 1_000 * MILLICENTS;	// assume this is worth about a cent.
    //    const DOLLARS: u128 = 100 * CENTS;

    const SECS_PER_BLOCK: u64 = 3;
    const MINUTES: u64 = 60 / SECS_PER_BLOCK;
    const HOURS: u64 = MINUTES * 60;
    const DAYS: u64 = HOURS * 24;
    
    let pcx_precision = 3_u16;
    let normalize = |n: u128| n * 10_u128.pow(pcx_precision as u32);
    let balances_config = BalancesConfig {
        transaction_base_fee: 1,
        transaction_byte_fee: 0,
        existential_deposit: 0,
        transfer_fee: 0,
        creation_fee: 0,
        reclaim_rebate: 0,
        balances: vec![],
    };
    let balances_config_copy = BalancesConfigCopy::create_from_src(&balances_config).src();

    GenesisConfig {
        consensus: Some(ConsensusConfig {
            code: include_bytes!(
            "../runtime/wasm/target/wasm32-unknown-unknown/release/akro_runtime.compact.wasm"
            ).to_vec(),
            authorities: initial_authorities.clone(),
        }),
        system: None,
        balances: Some(balances_config),
        session: Some(SessionConfig {
            validators: initial_authorities
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            session_length: 28, // 28 blocks per session
        }),
        democracy: Some(DemocracyConfig {
            launch_period: 120 * 24 * 14, // 2 weeks per public referendum
            voting_period: 120 * 24 * 28, // 4 weeks to discuss & vote on an active referendum
            minimum_deposit: 1000, // 1000 as the minimum deposit for a referendum
        }),
        council_voting: Some(CouncilVotingConfig {
            cooloff_period: 4 * DAYS,
            voting_period: 1 * DAYS,
        }),
        timestamp: Some(TimestampConfig {
            period: SECS_PER_BLOCK,                  // 3 second block time.
        }),
        treasury: Some(TreasuryConfig {
            proposal_bond: Permill::from_percent(5),
            proposal_bond_minimum: 1_000_000,
            spend_period: 1 * DAYS,
            burn: Permill::from_percent(50),
        }),
        contract: Some(ContractConfig {
            contract_fee: 21,
            call_base_fee: 135,
            create_base_fee: 175,
            gas_price: 1,
            max_depth: 1024,
            block_gas_limit: 10_000_000,
        }),
        staking: Some(StakingConfig {
            current_era: 0,
            bonding_duration: 28,
            intentions: initial_authorities.clone().into_iter().map(|i| i.0.into()).collect(),
            intention_profiles: vec![(auth1.0.into(), b"Genesis".to_vec(), b"akropolis.io".to_vec())],
            minimum_validator_count: 1,
            validator_count: 7,
            sessions_per_era: 5,
            shares_per_cert: 50,
            activation_per_share: normalize(100000) as u32,
            maximum_cert_owner_count: 200,
            intention_threshold: 9000,
            offline_slash_grace: 0,
            offline_slash: Perbill::from_millionths(0),
            current_offline_slash: 0,
            current_session_reward: 0,
            cert_owner: Public::from_ss58check("5DMo9Nn6MPEWUDefRwVSUtRp4kVguvBNgNatEgyhDJ32Zakt").unwrap().0.into(),
            register_fee: 1,
            claim_fee: 1,
            stake_fee: 1,
            unstake_fee: 1,
            activate_fee: 1,
            deactivate_fee: 1,
            nominate_fee: 1,
            unnominate_fee: 1,
        }),
        tokenstaking: Some(TokenStakingConfig {
            fee: 10
        }),
    }
}
