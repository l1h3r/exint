//! Exint Backend

#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(min_specialization)]
#![no_std]
#![no_implicit_prelude]
#![allow(internal_features)]

mod export;
mod macros;
mod traits;
mod types;
mod utils;

pub mod intrinsics {
  #[doc(inline)]
  pub use crate::export::*;
}
