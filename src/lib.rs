//! Exint - Exotic Integer Types
//!
//! -----------------------------------------------------------------------------
//! Notable Differences from primitive integer types
//!
//! - `int<N>` and `uint<N>` have an alignment of `1`.
//!
//! - Some implementations of `From` and `TryFrom` are blocked by:
//!   - orphan rule
//!   - requiring support for `generic_const_exprs` or `min_specialization`
//!
//! - Overloadable operators require a nightly compiler to be used in `const`-fns
//!
//! - Literals must be constructed with the macros from `exint-macro` (eg. [`uint!`]).
//!
//! -----------------------------------------------------------------------------

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_traits"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_clone"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_cmp"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_convert"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_default"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_ops"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_option"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "const_result"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(target_pointer_width = "16"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(target_pointer_width = "32"))))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(target_pointer_width = "64"))))]
#![cfg_attr(feature = "adt_const_params",      feature(adt_const_params))]
#![cfg_attr(feature = "ascii_char",            feature(ascii_char))]
#![cfg_attr(feature = "ascii_char",            feature(ascii_char_variants))]
#![cfg_attr(feature = "bigint_helper_methods", feature(bigint_helper_methods))]
#![cfg_attr(feature = "bigint_helper_methods", feature(const_carrying_mul_add))]
#![cfg_attr(feature = "const_clone",           feature(const_clone))]
#![cfg_attr(feature = "const_cmp",             feature(const_cmp))]
#![cfg_attr(feature = "const_convert",         feature(const_convert))]
#![cfg_attr(feature = "const_default",         feature(const_default))]
#![cfg_attr(feature = "const_eval_select",     feature(const_eval_select))]
#![cfg_attr(feature = "const_ops",             feature(const_ops))]
#![cfg_attr(feature = "const_option",          feature(const_option_ops))]
#![cfg_attr(feature = "const_result",          feature(const_result_trait_fn))]
#![cfg_attr(feature = "const_traits",          feature(const_trait_impl))]
#![cfg_attr(feature = "core_intrinsics",       feature(core_intrinsics))]
#![cfg_attr(feature = "disjoint_bitor",        feature(disjoint_bitor))]
#![cfg_attr(feature = "exact_div",             feature(exact_div))]
#![cfg_attr(feature = "f16",                   feature(f16))]
#![cfg_attr(feature = "f128",                  feature(f128))]
#![cfg_attr(feature = "funnel_shifts",         feature(funnel_shifts))]
#![cfg_attr(feature = "min_specialization",    feature(min_specialization))]
#![cfg_attr(feature = "never_type",            feature(never_type))]
#![cfg_attr(feature = "portable_simd",         feature(portable_simd))]
#![cfg_attr(feature = "random",                feature(random))]
#![cfg_attr(feature = "step_trait",            feature(step_trait))]
#![cfg_attr(feature = "structural_match",      feature(structural_match))]
#![cfg_attr(feature = "trusted_step",          feature(trusted_step))]
#![cfg_attr(feature = "unsized_const_params",  feature(unsized_const_params))]
#![cfg_attr(feature = "internal_features",     allow(internal_features))]
#![cfg_attr(feature = "incomplete_features",   allow(incomplete_features))]
#![cfg_attr(not(feature = "std"), no_std)]
#![no_implicit_prelude]

#![cfg_attr(
  all(target_has_atomic = "128", feature = "integer_atomics"),
  feature(integer_atomics),
)]

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

mod alias;
mod error;
mod isqrt;
mod llapi;
mod logxx;
mod panic;
mod types;
mod utils;

pub use self::error::IntErrorKind;
pub use self::error::ParseIntError;
pub use self::error::TryFromCharError;
pub use self::error::TryFromIntError;
pub use self::types::Saturating;
pub use self::types::Wrapping;
pub use self::types::int;
pub use self::types::uint;

pub mod primitive {
  //! Integer type-aliases.
  pub use crate::alias::*;
}

// Re-export convenience macros
pub use ::exint_macro::uint;
pub use ::exint_macro::uint_be;
pub use ::exint_macro::uint_le;
pub use ::exint_macro::uint_strict;
pub use ::exint_macro::uint_strict_be;
pub use ::exint_macro::uint_strict_le;

// Only exposed for codegen tests.
#[doc(hidden)]
pub mod backend {
  pub use crate::llapi::*;
}
