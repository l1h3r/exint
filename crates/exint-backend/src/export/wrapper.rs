use ::core::cmp::Ordering;
use ::core::marker::Copy;

use crate::types::Int;

macro_rules! assert_size_of {
  ($type:ident, $size:ident) => {
    ::core::debug_assert!(
      ::core::mem::size_of::<$type>() == $size,
      "unexpected size_of(T)",
    );
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
  ::core::panic!("intrinsics::eq")
}

#[must_use]
#[inline]
pub const fn cmp<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> Ordering {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::cmp")
}

// -----------------------------------------------------------------------------
// Bitwise Ops
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn band<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::band")
}

#[must_use]
#[inline]
pub const fn bor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::bor")
}

#[must_use]
#[inline]
pub const fn bxor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::bxor")
}

#[must_use]
#[inline]
pub const fn bnot<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::bnot")
}

// -----------------------------------------------------------------------------
// Bit Conversion
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn swap1<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::swap1")
}

#[must_use]
#[inline]
pub const fn swap8<T: Copy, const S: usize>(integer: T) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::swap8")
}

#[must_use]
#[inline]
pub const fn rotl<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::rotl")
}

#[must_use]
#[inline]
pub const fn rotr<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::rotr")
}

// -----------------------------------------------------------------------------
// Bit Inspection
// -----------------------------------------------------------------------------

#[must_use]
#[inline]
pub const fn ctpop<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::ctpop")
}

#[must_use]
#[inline]
pub const fn ctlz<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::ctlz")
}

#[must_use]
#[inline]
pub const fn cttz<T: Copy, const S: usize>(integer: T) -> u32 {
  assert_size_of!(T, S);
  ::core::panic!("intrinsics::cttz")
}
