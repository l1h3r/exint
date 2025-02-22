//! Exint Backend

#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(min_specialization)]
#![feature(never_type)]
#![feature(portable_simd)]
#![no_std]
#![no_implicit_prelude]
#![allow(internal_features)]
#![allow(unused_unsafe)]
#![deny(missing_docs)]

mod bridge;
mod consts;
mod macros;
mod traits;
mod types;
mod utils;

#[doc(inline)]
pub use crate::bridge::*;
