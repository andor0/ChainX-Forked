// Copyright 2018 Akropolis.

//! Shareable Akro types.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

extern crate parity_codec as codec;
extern crate sr_primitives as runtime_primitives;
extern crate sr_std as rstd;
extern crate substrate_primitives as primitives;

#[cfg(test)]
extern crate substrate_serializer;

#[cfg(feature = "std")]
extern crate serde;
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate parity_codec_derive;

#[cfg(feature = "std")]
use primitives::bytes;

use rstd::prelude::*;
use runtime_primitives::generic;
use runtime_primitives::traits::BlakeTwo256;

/// Signature on candidate's block data by a collator.
pub type CandidateSignature = ::runtime_primitives::Ed25519Signature;

/// The Ed25519 pub key of an session that belongs to an authority of the relay chain. This is
/// exactly equivalent to what the substrate calls an "authority".
pub type SessionKey = primitives::AuthorityId;

/// A hash of some data used by the relay chain.
pub type Hash = primitives::H256;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, SessionKey>>;

/// Opaque, encoded, unchecked extrinsic.
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with = "bytes"))] pub Vec<u8>);

/// A "future-proof" block type for Polkadot. This will be resilient to upgrades in transaction
/// format, because it doesn't attempt to decode extrinsics.
///
/// Specialized code needs to link to (at least one version of) the runtime directly
/// in order to handle the extrinsics within.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// An index to a block.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
/// TODO: switch to u32
pub type BlockNumber = u64;

/// Alias to Ed25519 pubkey that identifies an account on the relay chain.
pub type AccountId = primitives::H256;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u64;

/// Indentifier for a chain. 32-bit should be plenty.
pub type ChainId = u32;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Index = u64;

pub type Signature = runtime_primitives::Ed25519Signature;

/// A timestamp: seconds since the unix epoch.
pub type Timestamp = u64;

/// The balance of an account.
/// 128-bits (or 38 significant decimal figures) will allow for 10m currency (10^7) at a resolution
/// to all for one second's worth of an annualised 50% reward be paid to a unit holder (10^11 unit
/// denomination), or 10^18 total atomic units, to grow at 50%/year for 51 years (10^9 multiplier)
/// for an eventual total of 10^27 units (27 significant decimal figures).
/// We round denomination to 10^12 (12 sdf), and leave the other redundancy at the upper end so
/// that 32 bits may be multiplied with a balance in 128 bits without worrying about overflow.
pub type Balance = u128;

/// "generic" block ID for the future-proof block type.
// TODO: parameterize blockid only as necessary.
pub type BlockId = generic::BlockId<Block>;

/// Inherent data to include in a block.
#[derive(Encode, Decode)]
pub struct InherentData {
    /// Current timestamp.
    pub timestamp: Timestamp,
    /// Indices of offline validators.
    pub offline_indices: Vec<u32>,
    /// block producer
    pub block_producer: AccountId,
}

/// Candidate receipt type.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
pub struct CandidateReceipt {
    /// akro account id.
    pub collator: AccountId,
    /// Signature on blake2-256 of the block data by collator.
    pub signature: CandidateSignature,
    /// blake2-256 Hash of block data.
    pub block_data_hash: Hash,
}

impl CandidateReceipt {
    /// Get the blake2_256 hash
    #[cfg(feature = "std")]
    pub fn hash(&self) -> Hash {
        use runtime_primitives::traits::{BlakeTwo256, Hash};
        BlakeTwo256::hash_of(self)
    }

    /// Check integrity vs. provided block data.
    pub fn check_signature(&self) -> Result<(), ()> {
        use runtime_primitives::traits::Verify;

        if self
            .signature
            .verify(&self.block_data_hash.0[..], &self.collator)
        {
            Ok(())
        } else {
            Err(())
        }
    }
}
