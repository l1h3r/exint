use crate::utils;

pub const EXAMPLE_PRELUDE: &str = "
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
";

// Note: This *MUST* be in the same order as `VARS`.
pub const KEYS: &[&str; 14] = &[
  "$bits",
  "$uint_min",
  "$uint_max",
  "$int_min",
  "$int_max",
  "$to_swap",
  "$swapped",
  "$swap_be",
  "$swap_le",
  "$reverse",
  "$rotate_size",
  "$rotate_from",
  "$rotate_into",
  "$strict_overflow",
];

// Note: This *MUST* be in the same order as `KEYS`.
pub const VARS: &[&str; 14] = &[
  "32",                       // $bits
  "0",                        // $uint_min
  "4294967295",               // $uint_max
  "-2147483648",              // $int_min
  "2147483647",               // $int_max
  "0x12345678",               // $to_swap
  "0x78563412",               // $swapped
  "[0x12, 0x34, 0x56, 0x78]", // $swap_be
  "[0x78, 0x56, 0x34, 0x12]", // $swap_le
  "0x1E6A2C48",               // $reverse
  "16",                       // $rotate_size
  "0x12003400",               // $rotate_from
  "0x34001200",               // $rotate_into
  utils::trim(STRICT_OVERFLOW),
];

const STRICT_OVERFLOW: &str = "
# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.
";
