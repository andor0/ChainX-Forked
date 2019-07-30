// Copyright 2018 Akropolis.

//! A `CodeExecutor` specialisation which uses natively compiled runtime when the wasm to be
//! executed is equivalent to the natively compiled code.

extern crate akro_runtime;
#[macro_use]
extern crate substrate_executor;
#[cfg_attr(test, macro_use)]
extern crate substrate_primitives as primitives;

pub use substrate_executor::NativeExecutor;
native_executor_instance!(pub Executor, akro_runtime::api::dispatch, akro_runtime::native_version,
  include_bytes!("../../runtime/wasm/target/wasm32-unknown-unknown/release/akro_runtime.compact.wasm"));
