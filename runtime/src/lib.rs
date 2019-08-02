// Copyright 2018 Akropolis.

//! The Akro runtime. This can be compiled with ``#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "std")]
extern crate serde;

#[macro_use]
extern crate sr_io as runtime_io;
#[macro_use]
extern crate srml_support;
#[macro_use]
extern crate sr_primitives as runtime_primitives;
extern crate parity_codec as codec;
extern crate substrate_primitives as primitives;
extern crate substrate_client as client;
#[macro_use]
extern crate parity_codec_derive;
#[cfg_attr(not(feature = "std"), macro_use)]
extern crate sr_std as rstd;
extern crate srml_indices as indices;
extern crate srml_balances as balances;
extern crate srml_consensus as consensus;
extern crate srml_aura as aura;
extern crate srml_contract as contract;
extern crate srml_council as council;
extern crate srml_democracy as democracy;
extern crate srml_executive as executive;
extern crate srml_session as session;
extern crate srml_system as system;
extern crate srml_timestamp as timestamp;
extern crate srml_treasury as treasury;
extern crate substrate_primitives;

// akro runtime module
extern crate arml_associations as associations;
extern crate arml_support as akro_support;
extern crate arml_system as akro_system;
// akro mining staking
extern crate arml_mining_staking as staking;
extern crate arml_mining_tokenstaking as tokenstaking;
// funds
extern crate arml_funds_financialrecords as financialrecords;
extern crate arml_funds_withdrawal as withdrawal;
// exchange
extern crate arml_exchange_pendingorders as pendingorders;

#[macro_use]
extern crate sr_version as version;
extern crate akro_primitives;
extern crate arml_bridge_btc as bridge_btc;
extern crate arml_tokenbalances as tokenbalances;

extern crate arml_tokenbalances as tokenbalances;

#[cfg(feature = "std")]
mod checked_block;

//pub use balances::address::Address as RawAddress;
#[cfg(feature = "std")]
pub use checked_block::CheckedBlock;
pub use runtime_primitives::{Perbill, Permill, create_runtime_str};
pub use tokenbalances::Token;

//use akro_primitives::InherentData;
use akro_primitives::{
    AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, SessionKey, Signature,
};
pub use consensus::Call as ConsensusCall;
use council::{motions as council_motions, voting as council_voting};
use arml_system::Call as AkroSystemCall;
use rstd::prelude::*;
use primitives::bytes;
use primitives::{ed25519, sr25519, OpaqueMetadata};
use runtime_primitives::{ApplyResult, transaction_validity::TransactionValidity, generic};
use runtime_primitives::traits::{self, BlakeTwo256, Block as BlockT, Convert, NumberFor, StaticLookup, Verify};
use srml_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
use client::{
	block_builder::api::{CheckInherentsResult, InherentData, self as block_builder_api},
	runtime_api, impl_runtime_apis
};
use substrate_primitives::u32_trait::{_2, _4};
use timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
use version::NativeVersion;
use version::{ApiId, RuntimeVersion};

// for set consensus period
pub use srml_support::StorageValue;
pub use timestamp::BlockPeriod;


pub fn inherent_extrinsics(data: InherentData) -> Vec<UncheckedExtrinsic> {
    let mut inherent = vec![generic::UncheckedMortalExtrinsic::new_unsigned(
        Call::Timestamp(TimestampCall::set(data.timestamp)),
    )];

    inherent.push(generic::UncheckedMortalExtrinsic::new_unsigned(
        Call::AkroSystem(AkroSystemCall::set_block_producer(data.block_producer)),
    ));

    if !data.offline_indices.is_empty() {
        inherent.push(generic::UncheckedMortalExtrinsic::new_unsigned(
            Call::Consensus(ConsensusCall::note_offline(data.offline_indices)),
        ));
    }

    inherent
}

#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;

const INHERENT: ApiId = *b"inherent";
const VALIDATX: ApiId = *b"validatx";

/// The type that is used for identifying authorities.
pub type AuthorityId = <AuthoritySignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AuthoritySignature = ed25519::Signature;

/// The type used by authorities to prove their ID.
pub type AccountSignature = sr25519::Signature;

/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;

/// The position of the timestamp set extrinsic.
pub const TIMESTAMP_SET_POSITION: u32 = 0;
/// The position of the offline nodes noting extrinsic.
pub const NOTE_OFFLINE_POSITION: u32 = 2;

pub const BLOCK_PRODUCER_POSITION: u32 = 1;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
	use super::*;

	/// Opaque, encoded, unchecked extrinsic.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);
	#[cfg(feature = "std")]
	impl std::fmt::Debug for UncheckedExtrinsic {
		fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(fmt, "{}", primitives::hexdisplay::HexDisplay::from(&self.0))
		}
	}
	impl traits::Extrinsic for UncheckedExtrinsic {
		fn is_signed(&self) -> Option<bool> {
			None
		}
	}
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, AuthorityId, AuthoritySignature>>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque session key type.
	pub type SessionKey = AuthorityId;
}


/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("akro"),
    impl_name: create_runtime_str!("Akropolis-akro"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
};

/// Native version.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

impl system::Trait for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = Indices;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Nonce;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header digest type.
    type Digest = generic::Digest<Log>;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous log type.
    type Log = Log;
    /// The ubiquitous origin type.
    type Origin = Origin;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;

	/// What to do if an account's free balance gets zeroed.
    //TODO: actualize
    //type OnFreeBalanceZero = (Staking, Contract);
    type OnFreeBalanceZero = ();

	/// What to do if a new account is created.
	type OnNewAccount = Indices;
	/// The uniquitous event type.
	type Event = Event;

	type TransactionPayment = ();
	type DustRemoval = ();
	type TransferPayment = ();
}

impl aura::Trait for Runtime {
	type HandleReport = ();
}

impl consensus::Trait for Runtime {
	/// The identifier we use to refer to authorities.
	type SessionKey = AuthorityId;

	// The aura module handles offline-reports internally
	// rather than using an explicit report system.
    //TODO: actualize
    //type OnOfflineValidator = Staking;
	type InherentOfflineReport = ();

	/// The ubiquitous log type.
	type Log = Log;
}

impl indices::Trait for Runtime {
	/// The type for recording indexing into the account enumeration. If this ever overflows, there
	/// will be problems!
	type AccountIndex = u32;
	/// Use the standard means of resolving an index hint from an id.
	type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
	/// Determine whether an account is dead.
	type IsDeadAccount = Balances;
	/// The uniquitous event type.
	type Event = Event;
}

impl timestamp::Trait for Runtime {
	/// A timestamp: seconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
}


/// Session key conversion.
pub struct SessionKeyConversion;

impl Convert<AccountId, SessionKey> for SessionKeyConversion {
    fn convert(a: AccountId) -> SessionKey {
        a
    }
}

impl Convert<AccountId, Option<SessionKey>> for SessionKeyConversion {
    fn convert(a: AccountId) -> Option<SessionKey> {
        Some(a)
    }
}

impl session::Trait for Runtime {
    type ConvertAccountIdToSessionKey = SessionKeyConversion;
    type OnSessionChange = Staking;
    type Event = Event;
}

impl treasury::Trait for Runtime {
    /// The staking balance.
    type Currency = balances::Module<Self>;

    /// Origin from which approvals must come.
    //type ApproveOrigin: EnsureOrigin<Self::Origin>;
    type ApproveOrigin = council_motions::EnsureMembers<_4>;

    /// Origin from which rejections must come.
    //type RejectOrigin: EnsureOrigin<Self::Origin>;
    type RejectOrigin = council_motions::EnsureMembers<_2>;

    /// The overarching event type.
    //type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Event = Event;

    /// Handler for the unbalanced increase when minting cash from the "Pot".
    type MintedForSpending = ();

    /// Handler for the unbalanced decrease when slashing for a rejected proposal.
    type ProposalRejection = ();
}

impl democracy::Trait for Runtime {
    type Currency = balances::Module<Self>;
    type Proposal = Call;
    type Event = Event;
}

impl council::Trait for Runtime {
    type Event = Event;
    type BadPresentation = ();
    type BadReaper = ();
}

impl contract::Trait for Runtime {
    type Currency = balances::Module<Runtime>;
    type Call = Call;
    type Event = Event;
    type Gas = u64;
    type DetermineContractAddress = contract::SimpleAddressDeterminator<Runtime>;
    type ComputeDispatchFee = contract::DefaultDispatchFeeComputor<Runtime>;
    type TrieIdGenerator = contract::TrieIdFromParentCounter<Runtime>;
    type GasPayment = ();
}


// TODO add voting and motions at here
impl council::voting::Trait for Runtime {
    type Event = Event;
}

impl council::motions::Trait for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
}

impl akro_support::Trait for Runtime {}

impl checked_block::arml_system::Trait for Runtime {}

impl tokenbalances::Trait for Runtime {
    const AKRO_SYMBOL: tokenbalances::SymbolString = b"PCX";
    const AKRO_TOKEN_DESC: tokenbalances::DescString = b"Polkadot Akro";
    type TokenBalance = TokenBalance;
    type Event = Event;
    type OnMoveToken = ();
}

impl associations::Trait for Runtime {
    type OnCalcFee = AkroSupport;
    type Event = Event;
}

// mining staking
impl staking::Trait for Runtime {
    type OnRewardMinted = Treasury;
    type Event = Event;
    type OnNewSessionForTokenStaking = ();
    type OnReward = ();
}

impl tokenstaking::Trait for Runtime {
    type Event = Event;
}

// bridge
impl bridge_btc::Trait for Runtime {
    type Event = Event;
}

// funds
impl financialrecords::Trait for Runtime {
    type Event = Event;
    type OnDepositToken = ();
    type OnWithdrawToken = ();
}

impl withdrawal::Trait for Runtime {}

// exchange
impl pendingorders::Trait for Runtime {
    type Amount = TokenBalance;
    type Price = TokenBalance;
    type Event = Event;
}

//TODO: actualize
/*
impl DigestItem for Log {
    type Hash = Hash;
    type AuthorityId = SessionKey;

    fn as_authorities_change(&self) -> Option<&[Self::AuthorityId]> {
        match self.0 {
            InternalLog::consensus(ref item) => item.as_authorities_change(),
            _ => None,
        }
    }

    fn as_changes_trie_root(&self) -> Option<&Self::Hash> {
        match self.0 {
            InternalLog::system(ref item) => item.as_changes_trie_root(),
            _ => None,
        }
    }
}
*/

construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId, AuthoritySignature>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{default, Log(ChangesTrieRoot)},
        Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange)},
		//Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
        Aura: aura::{Module},
        Timestamp: timestamp::{Module, Call, Storage, Config<T>},
        Session: session,
        Staking: staking,
        Democracy: democracy,
        Council: council::{Module, Call, Storage, Event<T>},
        CouncilVoting: council_voting,
        CouncilMotions: council_motions::{Module, Call, Storage, Event<T>, Origin},
        Treasury: treasury,
        Contract: contract::{Module, Call, Config<T>, Event<T>},
        Indices: indices,
        Balances: balances::{Module, Storage, Config<T>, Event<T>}, //no call for public
        // akro runtime module
        TokenBalances: tokenbalances,
        Associations: associations,
        // funds
        FinancialRecords: financialrecords::{Module, Call, Storage, Event<T>},
        Withdrawal: withdrawal::{Module, Call, Config<T>},
        // exchange
        PendingOrders : pendingorders,
        // bridge
        BridgeOfBTC: bridge_btc,
        // mining staking
        TokenStaking: tokenstaking,

        // put end of this marco
        AkroSupport: akro_support::{Module},

        // must put end of all chainx runtime module
        AkroSystem: akro_system::{Module, Call, Storage, Config<T>},
    }
);

// define tokenbalances module type
pub type TokenBalance = u128;
/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, AccountSignature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
	impl runtime_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}

		fn authorities() -> Vec<AuthorityId> {
			panic!("Deprecated, please use `AuthoritiesApi`.")
		}
	}

	impl runtime_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl block_builder_api::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			System::random_seed()
		}
	}

	impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}

	impl consensus_aura::AuraApi<Block> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}
	}

	impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(n: NumberFor<Block>) {
			Executive::offchain_worker(n)
		}
	}

	impl consensus_authorities::AuthoritiesApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityId> {
			Consensus::authorities()
		}
	}
}
