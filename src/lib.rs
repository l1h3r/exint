//! Exint - Exotic Integer Types

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]
#![no_implicit_prelude]

#[doc(inline)]
pub use ::exint_integer::*;

#[cfg(feature = "unstable_internals")]
pub mod backend {
  #[doc(inline)]
  pub use ::exint_backend::*;
}
