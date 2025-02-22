use ::core::cmp::Ordering;
use ::core::marker::Copy;

use crate::export::bitwise::SpecBitwise;
use crate::export::compare::SpecCompare;
use crate::export::convert::SpecConvert;
use crate::export::inspect::SpecInspect;
use crate::types::Int;

macro_rules! assert_size_of {
  ($type:ident, $size:ident) => {
    ::core::debug_assert!(
      ::core::mem::size_of::<$type>() == $size,
      "unexpected size_of(T)",
    );
  };
}

macro_rules! cast {
  (int($size:ident) from $expr:expr) => {
    cast!($expr => Int<$size>)
  };
  ($type:ident from $expr:expr) => {
    cast!($expr => $type)
  };
  ($expr:expr => $type:ty) => {
    #[allow(unused_unsafe)]
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { $crate::utils::transmute::<_, $type>($expr) }
  };
}

// -----------------------------------------------------------------------------
// Type Conversion
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn cast<const T: usize, const U: usize, const UINT: bool>(integer: Int<T>) -> Int<U> {
  ::core::panic!("intrinsics::cast")
}

// -----------------------------------------------------------------------------
// Comparison
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn eq<T: Copy, const S: usize>(lhs: T, rhs: T) -> bool {
  assert_size_of!(T, S);
  SpecCompare::eq(cast!(int(S) from lhs), cast!(int(S) from rhs))
}

#[must_use]
#[inline]
pub const fn cmp<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> Ordering {
  assert_size_of!(T, S);
  if UINT {
    SpecCompare::ucmp(cast!(int(S) from lhs), cast!(int(S) from rhs))
  } else {
    SpecCompare::scmp(cast!(int(S) from lhs), cast!(int(S) from rhs))
  }
}

// -----------------------------------------------------------------------------
// Bitwise Ops
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn band<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::and(cast!(int(S) from lhs), cast!(int(S) from rhs)))
}

#[must_use]
#[inline]
pub const fn bor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::or(cast!(int(S) from lhs), cast!(int(S) from rhs)))
}

#[must_use]
#[inline]
pub const fn bxor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::xor(cast!(int(S) from lhs), cast!(int(S) from rhs)))
}

#[must_use]
#[inline]
pub const fn bnot<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::not(cast!(int(S) from integer)))
}

// -----------------------------------------------------------------------------
// Bit Conversion
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn swap1<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::swap1(cast!(int(S) from integer)))
}

#[must_use]
#[inline]
pub const fn swap8<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::swap8(cast!(int(S) from integer)))
}

#[must_use]
#[inline]
pub const fn rotl<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::rotl(cast!(int(S) from integer), bits))
}

#[must_use]
#[inline]
pub const fn rotr<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::rotr(cast!(int(S) from integer), bits))
}

// -----------------------------------------------------------------------------
// Bit Inspection
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn ctpop<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::ctpop(cast!(int(S) from integer))
}

#[must_use]
#[inline]
pub const fn ctlz<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::ctlz(cast!(int(S) from integer))
}

#[must_use]
#[inline]
pub const fn cttz<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::cttz(cast!(int(S) from integer))
}
