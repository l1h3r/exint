//! Exint Backend

#![no_std]
#![no_implicit_prelude]

mod export;
mod types;

pub mod intrinsics {
  #[doc(inline)]
  pub use crate::export::*;
}
