//! Exint Integer

#![feature(core_intrinsics)]
#![feature(doc_cfg)]
#![cfg_attr(feature = "random",       feature(random))]
#![cfg_attr(feature = "step_trait",   feature(step_trait))]
#![cfg_attr(feature = "trusted_step", feature(min_specialization))]
#![cfg_attr(feature = "trusted_step", feature(trusted_step))]
#![no_std]
#![no_implicit_prelude]
#![allow(internal_features)]
// Use to support `cast!` macro.
//
// Tracking Issue: https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]

mod errors;
mod macros;
mod panic;
mod traits;
mod types;
mod value;

pub(crate) mod intrinsics {
  pub use ::exint_backend::*;
}

pub use self::errors::ParseIntError;
pub use self::errors::TryFromIntError;
pub use self::types::sint::int;
pub use self::types::uint::uint;

pub mod primitive {
  //! Integer type-aliases.
  pub use crate::types::alias::*;
}
