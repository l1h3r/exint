//! Exint Macros

#![cfg_attr(feature = "proc_macro_diagnostic", feature(proc_macro_diagnostic))]
#![allow(dead_code, reason = "feature-dependant")]

use proc_macro::TokenStream;

#[macro_use]
mod backports;
mod expanders;
mod utilities;

mod error;

use expanders::uint;
use expanders::uint::OutputFormat;
use expanders::uint::TargetMode;

// -----------------------------------------------------------------------------
// Entrypoint
// -----------------------------------------------------------------------------

/// Create a generic integer from a literal expression.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// exint::uint! {
///   let a = 123_u24;
///   let b = 456_u136;
///   let c = 789_i80;
/// }
/// ```
///
/// You can also use binary/octal/hex:
///
/// ```
/// exint::uint! {
///   let a = 0b1010_u24;
///   let b = 0xABCD_u136;
///   let c = 0o3737_i80;
/// }
/// ```
///
/// Both forms are equivalent:
///
/// ```
/// let a = exint::uint!(12345_u24);
///
/// exint::uint! {
///   let a = 12345_u24;
/// }
/// ```
#[proc_macro]
pub fn uint(stream: TokenStream) -> TokenStream {
  uint::expand(stream, TargetMode::IncludeStd, OutputFormat::ANY)
}

/// Similar to [`uint!`] but always generates code in big-endian format.
#[proc_macro]
pub fn uint_be(stream: TokenStream) -> TokenStream {
  uint::expand(stream, TargetMode::IncludeStd, OutputFormat::BE)
}

/// Similar to [`uint!`] but always generates code in little-endian format.
#[proc_macro]
pub fn uint_le(stream: TokenStream) -> TokenStream {
  uint::expand(stream, TargetMode::IncludeStd, OutputFormat::LE)
}

/// Similar to [`uint!`] but don't convert the built-in integer types (eg. `u8`, `u32`, `i64`).
#[proc_macro]
pub fn uint_strict(stream: TokenStream) -> TokenStream {
  uint::expand(stream, TargetMode::ExcludeStd, OutputFormat::ANY)
}

/// Similar to [`uint_strict!`] but always generates code in big-endian format.
#[proc_macro]
pub fn uint_strict_be(stream: TokenStream) -> TokenStream {
  uint::expand(stream, TargetMode::ExcludeStd, OutputFormat::BE)
}

/// Similar to [`uint_strict!`] but always generates code in little-endian format.
#[proc_macro]
pub fn uint_strict_le(stream: TokenStream) -> TokenStream {
  uint::expand(stream, TargetMode::ExcludeStd, OutputFormat::LE)
}
