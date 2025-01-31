// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <https://www.gnu.org/licenses/>.

//! Test utilities

#![cfg(test)]

use primitives::testing::{Digest, DigestItem, Header};
use primitives::BuildStorage;
use primitives::{traits::Identity, Perbill};
use runtime_io;
use substrate_primitives::{Blake2Hasher, H256};
use tokenbalances::{DescString, SymbolString};
use {
    associations, balances, consensus, arml_system, arml_support, session, system, timestamp,
    tokenbalances, GenesisConfig, Module, Trait,
};

impl_outer_origin! {
    pub enum Origin for Test {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Test;

impl consensus::Trait for Test {
    const NOTE_OFFLINE_POSITION: u32 = 1;
    type Log = DigestItem;
    type SessionKey = u64;
    type OnOfflineValidator = ();
}
impl system::Trait for Test {
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = ::primitives::traits::BlakeTwo256;
    type Digest = Digest;
    type AccountId = u64;
    type Header = Header;
    type Event = ();
    type Log = DigestItem;
}
impl balances::Trait for Test {
    type Balance = u64;
    type AccountIndex = u64;
    type OnFreeBalanceZero = Staking;
    type EnsureAccountLiquid = Staking;
    type Event = ();
}
impl session::Trait for Test {
    type ConvertAccountIdToSessionKey = Identity;
    type OnSessionChange = Staking;
    type Event = ();
}
impl timestamp::Trait for Test {
    const TIMESTAMP_SET_POSITION: u32 = 0;
    type Moment = u64;
}
impl arml_support::Trait for Test {}
impl arml_system::Trait for Test {}
impl arml_associations::Trait for Test {
    type OnCalcFee = arml_support::Module<Test>;
    type Event = ();
}
pub type TokenBalance = u128;
impl tokenbalances::Trait for Test {
    const AKRO_SYMBOL: SymbolString = b"pcx";
    const AKRO_TOKEN_DESC: DescString = b"this is pcx for mock";
    type TokenBalance = TokenBalance;
    type OnMoveToken = ();
    type Event = ();
}
impl Trait for Test {
    type OnNewSessionForTokenStaking = ();
    type OnRewardMinted = ();
    type OnReward = ();
    type Event = ();
}

pub fn new_test_ext(
    ext_deposit: u64,
    session_length: u64,
    sessions_per_era: u64,
    current_era: u64,
    monied: bool,
    reward: u64,
) -> runtime_io::TestExternalities<Blake2Hasher> {
    let mut t = system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let balance_factor = if ext_deposit > 0 { 256 } else { 1 };
    t.extend(
        consensus::GenesisConfig::<Test> {
            code: vec![],
            authorities: vec![],
        }
        .build_storage()
        .unwrap(),
    );
    t.extend(
        session::GenesisConfig::<Test> {
            session_length,
            validators: vec![10],
        }
        .build_storage()
        .unwrap(),
    );
    t.extend(
        balances::GenesisConfig::<Test> {
            balances: if monied {
                if reward > 0 {
                    vec![
                        (1, 10_000 * balance_factor),
                        (2, 20_000 * balance_factor),
                        (3, 30_000 * balance_factor),
                        (4, 40_000 * balance_factor),
                        (10, 1000 * balance_factor),
                        (20, 1000 * balance_factor),
                    ]
                } else {
                    vec![
                        (1, 10_000 * balance_factor),
                        (2, 20_000 * balance_factor),
                        (3, 30_000 * balance_factor),
                        (4, 40_000 * balance_factor),
                    ]
                }
            } else {
                vec![(10, balance_factor), (20, balance_factor)]
            },
            transaction_base_fee: 0,
            transaction_byte_fee: 0,
            existential_deposit: ext_deposit,
            transfer_fee: 0,
            creation_fee: 0,
            reclaim_rebate: 0,
        }
        .build_storage()
        .unwrap(),
    );
    let initial_authorities = vec![10];
    t.extend(
        GenesisConfig::<Test> {
            sessions_per_era,
            current_era,
            intentions: vec![10],
            intention_profiles: initial_authorities
                .clone()
                .into_iter()
                .map(|i| (i, b"Akro".to_vec(), b"url".to_vec()))
                .collect(),
            validator_count: 7,
            shares_per_cert: 45,
            activation_per_share: 100_000_000,
            maximum_cert_owner_count: 200,
            intention_threshold: 9000,
            minimum_validator_count: 1,
            bonding_duration: sessions_per_era * session_length,
            offline_slash: if monied {
                Perbill::from_percent(40)
            } else {
                Perbill::zero()
            },
            current_session_reward: reward,
            current_offline_slash: 20,
            offline_slash_grace: 0,
            cert_owner: 10,
            register_fee: 0,
            claim_fee: 0,
            stake_fee: 0,
            unstake_fee: 0,
            activate_fee: 0,
            deactivate_fee: 0,
            nominate_fee: 0,
            unnominate_fee: 0,
        }
        .build_storage()
        .unwrap(),
    );
    t.extend(
        timestamp::GenesisConfig::<Test> { period: 5 }
            .build_storage()
            .unwrap(),
    );
    runtime_io::TestExternalities::new(t)
}

pub type System = system::Module<Test>;
pub type Balances = balances::Module<Test>;
pub type Session = session::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;
pub type Associations = arml_associations::Module<Test>;
pub type Staking = Module<Test>;
