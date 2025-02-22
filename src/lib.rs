//! Exint - Exotic Integer Types

#![cfg_attr(feature = "ascii_char",           feature(ascii_char))]
#![cfg_attr(feature = "const_trait_impl",     feature(const_trait_impl))]
#![cfg_attr(feature = "core_intrinsics",      feature(core_intrinsics))]
#![cfg_attr(feature = "docs",                 feature(doc_auto_cfg))]
#![cfg_attr(feature = "docs",                 feature(doc_cfg_hide))]
#![cfg_attr(feature = "f16",                  feature(f16))]
#![cfg_attr(feature = "f128",                 feature(f128))]
#![cfg_attr(feature = "integer_atomics",      feature(integer_atomics))]
#![cfg_attr(feature = "never_type",           feature(never_type))]
#![cfg_attr(feature = "random",               feature(random))]
#![cfg_attr(feature = "specialization",       feature(min_specialization))]
#![cfg_attr(feature = "step_trait",           feature(step_trait))]
#![cfg_attr(feature = "structural_match",     feature(structural_match))]
#![cfg_attr(feature = "trusted_step",         feature(trusted_step))]
#![cfg_attr(feature = "unsized_const_params", feature(unsized_const_params))]
#![cfg_attr(feature = "utf16_extra",          feature(utf16_extra))]
#![cfg_attr(feature = "core_intrinsics",      allow(internal_features))]
#![cfg_attr(feature = "unsized_const_params", allow(incomplete_features))]
#![cfg_attr(feature = "docs",                 doc(cfg_hide(target_pointer_width = "16")))]
#![cfg_attr(feature = "docs",                 doc(cfg_hide(target_pointer_width = "32")))]
#![cfg_attr(feature = "docs",                 doc(cfg_hide(target_pointer_width = "64")))]
#![cfg_attr(not(feature = "std"), no_std)]
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
pub use self::types::ToAtomic;
pub use self::types::ToNonZero;

pub mod primitive {
  //! Integer type-aliases.
  pub use crate::alias::*;
}
