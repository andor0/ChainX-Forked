// Copyright 2018 Akropolis.

//! Akro block evaluation and evaluation errors.
use akro_primitives::{Block, BlockNumber, Hash, Timestamp};
use akro_runtime::{Block as AkroGenericBlock, CheckedBlock};

use super::MAX_TRANSACTIONS_SIZE;

use codec::{Decode, Encode};

error_chain! {
    links {
        AkroApi(::akro_api::Error, ::akro_api::ErrorKind);
    }

    errors {
        ProposalNotForAkro {
            description("Proposal provided not a Akro block."),
            display("Proposal provided not a Akro block."),
        }
        TimestampInFuture {
            description("Proposal had timestamp too far in the future."),
            display("Proposal had timestamp too far in the future."),
        }
        TooManyCandidates(expected: usize, got: usize) {
            description("Proposal included more candidates than is possible."),
            display("Proposal included {} candidates for {} parachains", got, expected),
        }
        WrongParentHash(expected: Hash, got: Hash) {
            description("Proposal had wrong parent hash."),
            display("Proposal had wrong parent hash. Expected {:?}, got {:?}", expected, got),
        }
        WrongNumber(expected: BlockNumber, got: BlockNumber) {
            description("Proposal had wrong number."),
            display("Proposal had wrong number. Expected {:?}, got {:?}", expected, got),
        }
        ProposalTooLarge(size: usize) {
            description("Proposal exceeded the maximum size."),
            display(
                "Proposal exceeded the maximum size of {} by {} bytes.",
                MAX_TRANSACTIONS_SIZE, MAX_TRANSACTIONS_SIZE.saturating_sub(*size)
            ),
        }
    }
}

/// Attempt to evaluate a substrate block as a akro block, returning error
/// upon any initial validity checks failing.
pub fn evaluate_initial(
    proposal: &Block,
    now: Timestamp,
    parent_hash: &Hash,
    parent_number: BlockNumber,
) -> Result<CheckedBlock> {
    const MAX_TIMESTAMP_DRIFT: Timestamp = 60;

    let encoded = Encode::encode(proposal);
    let proposal = AkroGenericBlock::decode(&mut &encoded[..])
        .and_then(|b| CheckedBlock::new(b).ok())
        .ok_or_else(|| ErrorKind::ProposalNotForAkro)?;

    let transactions_size = proposal
        .extrinsics
        .iter()
        .fold(0, |a, tx| a + Encode::encode(tx).len());

    if transactions_size > MAX_TRANSACTIONS_SIZE {
        bail!(ErrorKind::ProposalTooLarge(transactions_size))
    }

    if proposal.header.parent_hash != *parent_hash {
        bail!(ErrorKind::WrongParentHash(
            *parent_hash,
            proposal.header.parent_hash,
        ));
    }

    if proposal.header.number != parent_number + 1 {
        bail!(ErrorKind::WrongNumber(
            parent_number + 1,
            proposal.header.number,
        ));
    }
    let block_timestamp = proposal.timestamp();

    // lenient maximum -- small drifts will just be delayed using a timer.
    if block_timestamp > now + MAX_TIMESTAMP_DRIFT {
        bail!(ErrorKind::TimestampInFuture)
    }

    Ok(proposal)
}
