//! Exint low-level API

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "const_trait_impl",   feature(const_trait_impl))]
#![cfg_attr(feature = "core_intrinsics",    feature(core_intrinsics))]
#![cfg_attr(feature = "disjoint_bitor",     feature(disjoint_bitor))]
#![cfg_attr(feature = "min_specialization", feature(min_specialization))]
#![cfg_attr(feature = "internal_features",  allow(internal_features))]
#![no_std]
#![no_implicit_prelude]

mod export;
mod impls;
mod macros;
mod traits;
mod utils;

pub use self::export::*;
pub use self::traits::Uint;
