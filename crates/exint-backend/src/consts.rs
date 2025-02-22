//! Common constants, for internal use only.

/// Sign digit for a two's complement integer.
///
/// - `0` indicates number is signed as positive.
/// - `1` indicates number is signed as negative.
pub(crate) const SIGN: u8 = 0b10000000;
