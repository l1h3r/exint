//! Panic Utilities
//!
//! - functions used throughout the library
//! - constants used for test assertions

#![allow(dead_code, reason = "feature-dependant")]

macro_rules! define {
  ($constant:ident, $function:ident, $message:literal) => {
    pub(crate) const $constant: &'static str = $message;

    #[cold]
    #[track_caller]
    #[inline(never)]
    pub(crate) const fn $function() -> ! {
      ::core::panic!($message)
    }
  };
}

define!(ADD, add, "attempt to add with overflow");
define!(SUB, sub, "attempt to subtract with overflow");
define!(MUL, mul, "attempt to multiply with overflow");
define!(DIV, div, "attempt to divide with overflow");
define!(REM, rem, "attempt to calculate the remainder with overflow");
define!(NEG, neg, "attempt to negate with overflow");
define!(SHL, shl, "attempt to shift left with overflow");
define!(SHR, shr, "attempt to shift right with overflow");
define!(FSHL, fshl, "attempt to funnel shift left with overflow");
define!(FSHR, fshr, "attempt to funnel shift right with overflow");
define!(ILOG, ilog, "argument of integer logarithm must be positive");
define!(ILOG_BASE, ilog_base, "base of integer logarithm must be at least 2");
define!(ISQRT, isqrt, "argument of integer square root cannot be negative");
define!(DIV_ZERO, div_zero, "attempt to divide by zero");
define!(REM_ZERO, rem_zero, "attempt to calculate the remainder with a divisor of zero");
define!(EXACT_DIV, exact_div, "Failed to divide without remainder");

#[cfg_attr(not(feature = "__internal_panic_immediate_abort"), inline(never))]
#[cfg_attr(feature = "__internal_panic_immediate_abort", inline)]
#[cold]
#[track_caller]
pub(crate) const fn from_ascii_radix(radix: u32) -> ! {
  const_panic! {
    "from_ascii_radix: radix must lie in the range `[2, 36]`",
    "from_ascii_radix: radix must lie in the range `[2, 36]` - found {radix}",
    radix: u32 = radix
  }
}
