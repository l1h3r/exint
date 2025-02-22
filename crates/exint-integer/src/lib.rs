//! Exint Integer

#![feature(doc_cfg)]
#![cfg_attr(feature = "random",       feature(random))]
#![cfg_attr(feature = "step_trait",   feature(step_trait))]
#![cfg_attr(feature = "trusted_step", feature(min_specialization))]
#![cfg_attr(feature = "trusted_step", feature(trusted_step))]
#![no_std]

mod macros;
mod traits;
mod types;

// TODO
pub mod errors {
  pub type ParseIntError = ();
}

pub use self::types::sint::int;
pub use self::types::uint::uint;
