//! Exint - Exotic Integer Types

#![cfg_attr(feature = "const_trait_impl", feature(const_trait_impl))]
#![cfg_attr(feature = "core_intrinsics",  feature(core_intrinsics))]
#![cfg_attr(feature = "docs",             feature(doc_auto_cfg))]
#![cfg_attr(feature = "never_type",       feature(never_type))]
#![cfg_attr(feature = "random",           feature(random))]
#![cfg_attr(feature = "specialization",   feature(min_specialization))]
#![cfg_attr(feature = "step_trait",       feature(step_trait))]
#![cfg_attr(feature = "trusted_step",     feature(trusted_step))]
#![cfg_attr(feature = "core_intrinsics",  allow(internal_features))]
#![no_std]
#![no_implicit_prelude]

mod alias;
mod error;
mod llapi;
mod panic;
mod parse;
mod types;
mod utils;

pub use self::error::IntErrorKind;
pub use self::error::ParseIntError;
pub use self::error::TryFromIntError;
pub use self::types::int;
pub use self::types::uint;

pub mod primitive {
  //! Integer type-aliases.
  pub use crate::alias::*;
}
