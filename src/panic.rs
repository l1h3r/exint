#![allow(dead_code, reason = "Crate might be configured to not panic")]

use ::core::panic;

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn add() -> ! {
  panic!("attempt to add with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn sub() -> ! {
  panic!("attempt to subtract with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn mul() -> ! {
  panic!("attempt to multiply with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn div() -> ! {
  panic!("attempt to divide with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn rem() -> ! {
  panic!("attempt to calculate the remainder with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn neg() -> ! {
  panic!("attempt to negate with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn shl() -> ! {
  panic!("attempt to shift left with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn shr() -> ! {
  panic!("attempt to shift right with overflow")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn div_zero() -> ! {
  panic!("attempt to divide by zero")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn rem_zero() -> ! {
  panic!("attempt to calculate the remainder with a divisor of zero")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn ilog() -> ! {
  panic!("argument of integer logarithm must be positive")
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn isqrt() -> ! {
  panic!("argument of integer square root cannot be negative")
}
