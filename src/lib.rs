//! Exint - Exotic Integer Types

#![no_std]

#[doc(inline)]
pub use exint_integer::*;

pub mod backend {
  //! Platform-specific implementation details

  #[doc(inline)]
  pub use exint_backend::*;
}
