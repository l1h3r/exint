//! Exint Integer

#![feature(doc_cfg)]
#![no_std]

mod macros;
mod types;

// TODO
pub type ParseIntError = ();

pub use self::types::sint::int;
pub use self::types::uint::uint;
