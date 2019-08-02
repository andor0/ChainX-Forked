// Copyright 2018 Akropolis.

//! this module is for akro system

#![cfg_attr(not(feature = "std"), no_std)]
// for encode/decode
// Needed for deriving `Serialize` and `Deserialize` for various types.
// We only implement the serde traits for std builds - they're unneeded
// in the wasm runtime.
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;

// Needed for deriving `Encode` and `Decode` for `RawEvent`.
//#[macro_use]
//extern crate parity_codec_derive;
extern crate parity_codec as codec;

// TODO: actualize
// for substrate
// Needed for the set of mock primitives used in our tests.
//#[cfg(feature = "std")]
//extern crate substrate_primitives;

// for substrate runtime
// map!, vec! marco.
extern crate sr_std as rstd;

#[cfg(feature = "std")]
extern crate sr_io as runtime_io;
extern crate sr_primitives as runtime_primitives;
// for substrate runtime module lib
// Needed for type-safe access to storage DB.
#[macro_use]
extern crate srml_support as runtime_support;
extern crate srml_system as system;

#[cfg(test)]
mod tests;

use rstd::prelude::*;
use runtime_primitives::traits::OnFinalize;
use runtime_support::dispatch::Result;
use runtime_support::StorageValue;

use system::ensure_inherent;

pub trait Trait: system::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_block_producer(origin, producer: T::AccountId) -> Result {
            ensure_inherent(origin)?;
            BlockProdocer::<T>::put(producer);
            Ok(())
        }
    }
}

// TODO: actualize
//impl<T: Trait> OnFinalize<T::BlockNumber> for Module<T> {
//    fn on_finalize(_: T::BlockNumber) {
//        BlockProdocer::<T>::kill();
//    }
//}

decl_storage! {
    trait Store for Module<T: Trait> as AkroSystem {
        pub BlockProdocer get(block_producer): Option<T::AccountId>;
        pub DeathAccount get(death_account) config(): T::AccountId;
        pub FeeBuyAccount get(fee_buy_account) config(): T::AccountId;
    }
}

impl<T: Trait> Module<T> {}
