use ::core::cmp::Ordering;
use ::core::marker::Copy;

use crate::export::arithmetic::SpecSadd;
use crate::export::arithmetic::SpecSdiv;
use crate::export::arithmetic::SpecSmul;
use crate::export::arithmetic::SpecSsub;
use crate::export::arithmetic::SpecUadd;
use crate::export::arithmetic::SpecUdiv;
use crate::export::arithmetic::SpecUmul;
use crate::export::arithmetic::SpecUsub;
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
  (($type:ident, bool) from $expr:expr) => {{
    let out: ($crate::types::Int<S>, bool) = $expr;
    let num: $type = cast!($type from out.0);

    (num, out.1)
  }};
  ($expr:expr => $type:ty) => {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { $crate::utils::transmute::<_, $type>($expr) }
  };
}

// -----------------------------------------------------------------------------
// Type Conversion
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn cast<const T: usize, const U: usize, const UINT: bool>(integer: Int<T>) -> Int<U> {
  ::core::panic!("intrinsics::cast")
}

// -----------------------------------------------------------------------------
// Comparison
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn eq<T: Copy, const S: usize>(lhs: T, rhs: T) -> bool {
  assert_size_of!(T, S);
  SpecCompare::eq(cast!(int(S) from lhs), cast!(int(S) from rhs))
}

#[must_use]
#[inline]
#[track_caller]
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
#[track_caller]
pub const fn band<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::and(cast!(int(S) from lhs), cast!(int(S) from rhs)))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn bor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::or(cast!(int(S) from lhs), cast!(int(S) from rhs)))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn bxor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::xor(cast!(int(S) from lhs), cast!(int(S) from rhs)))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn bnot<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecBitwise::not(cast!(int(S) from integer)))
}

// -----------------------------------------------------------------------------
// Bit Conversion
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn swap1<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::swap1(cast!(int(S) from integer)))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn swap8<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::swap8(cast!(int(S) from integer)))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn rotl<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::rotl(cast!(int(S) from integer), bits))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn rotr<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  assert_size_of!(T, S);
  cast!(T from SpecConvert::rotr(cast!(int(S) from integer), bits))
}

// -----------------------------------------------------------------------------
// Bit Inspection
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn ctpop<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::ctpop(cast!(int(S) from integer))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn ctlz<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::ctlz(cast!(int(S) from integer))
}

#[must_use]
#[inline]
#[track_caller]
pub const fn cttz<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::cttz(cast!(int(S) from integer))
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn ctlz_nonzero<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::ctlz_nonzero(cast!(int(S) from integer))
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn cttz_nonzero<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  SpecInspect::cttz_nonzero(cast!(int(S) from integer))
}

// -----------------------------------------------------------------------------
// Arithmetic - Overflowing
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn overflowing_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> (T, bool) {
  assert_size_of!(T, S);
  if UINT {
    cast!((T, bool) from SpecUadd::oadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!((T, bool) from SpecSadd::oadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const fn overflowing_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> (T, bool) {
  assert_size_of!(T, S);
  if UINT {
    cast!((T, bool) from SpecUsub::osub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!((T, bool) from SpecSsub::osub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const fn overflowing_mul<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> (T, bool) {
  assert_size_of!(T, S);
  if UINT {
    cast!((T, bool) from SpecUmul::omul(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!((T, bool) from SpecSmul::omul(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Saturating
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn saturating_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUadd::sadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSadd::sadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const fn saturating_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUsub::ssub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSsub::ssub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Unchecked
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUadd::uadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSadd::uadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUsub::usub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSsub::usub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_mul<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUmul::umul(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSmul::umul(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_div<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUdiv::udiv(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSdiv::udiv(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_rem<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUdiv::urem(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSdiv::urem(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_shl<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: u32) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::unchecked_shl")
}

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_shr<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: u32) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::unchecked_shr")
}

// -----------------------------------------------------------------------------
// Arithmetic - Wrapping
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const fn wrapping_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUadd::wadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSadd::wadd(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const fn wrapping_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUsub::wsub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSsub::wsub(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

#[must_use]
#[inline]
#[track_caller]
pub const fn wrapping_mul<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUmul::wmul(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSmul::wmul(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Misc.
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn exact_div<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  if UINT {
    cast!(T from SpecUdiv::ediv(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  } else {
    cast!(T from SpecSdiv::ediv(cast!(int(S) from lhs), cast!(int(S) from rhs)))
  }
}
