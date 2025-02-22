use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::llapi;
use crate::panic;
use crate::types::int;
use crate::types::macros;
use crate::types::macros::tri;
use crate::utils::include_doc;
use crate::utils::must_use_doc;

/// The generic unsigned integer type.
///
/// ## Examples
///
/// Note that the examples here use [`uint<4>`] for simplicity and rely on
/// the [`uint`] macro for constructing literals.
///
/// [`uint<4>`]: Self
/// [`uint`]: crate::uint!
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct uint<const N: usize = 4> {
  bytes: [u8; N],
}

impl<const N: usize> uint<N> {
  macros::internals!(uint);

  #[inline]
  const fn one_less_than_next_power_of_two(self) -> Self {
    if self.const_le(&Self::ONE) {
      return Self::ZERO;
    }

    // SAFETY: `self - 1` is at *least* 2.
    Self::MAX.const_shr(unsafe { llapi::ctlz_nonzero::<Self, N>(self.const_sub(Self::ONE)) })
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "BITS")]
  pub const BITS: u32 = Self::MAX.count_ones();

  #[doc = include_doc!(uint, "MAX")]
  pub const MAX: Self = Self::MIN.const_not();

  #[doc = include_doc!(uint, "MIN")]
  pub const MIN: Self = Self::ZERO;
}

impl<const N: usize> uint<N> {
  macros::byteorder!(uint);
}

impl<const N: usize> uint<N> {
  macros::bin_tools!(uint);
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "max_value")]
  #[deprecated(since = "TBD", note = "replaced by the `MAX` associated constant on this type")]
  #[must_use]
  #[inline]
  pub const fn max_value() -> Self {
    Self::MAX
  }

  #[doc = include_doc!(uint, "min_value")]
  #[deprecated(since = "TBD", note = "replaced by the `MIN` associated constant on this type")]
  #[must_use]
  #[inline]
  pub const fn min_value() -> Self {
    Self::MIN
  }

  macros::stability! {
    #[unstable(feature = "integer_sign_cast")]
    #[doc = include_doc!(uint, "cast_signed")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn cast_signed(self) -> int<N> {
      int::from_ne_bytes(self.to_ne_bytes())
    }
  }

  // TODO: Optimize with wide type
  #[doc = include_doc!(uint, "midpoint")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn midpoint(self, rhs: Self) -> Self {
    self
      .const_bxor(rhs)
      .const_shr(1)
      .const_add(self.const_band(rhs))
  }

  #[doc = include_doc!(uint, "div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_euclid(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn rem_euclid(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "div_ceil")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_ceil(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if rem.const_gt(&Self::ZERO) {
      div.const_add(Self::ONE)
    } else {
      div
    }
  }

  #[doc = include_doc!(uint, "div_floor")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_floor(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "next_multiple_of")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn next_multiple_of(self, rhs: Self) -> Self {
    let rem: Self = self.const_rem(rhs);

    if rem.is_zero() {
      self
    } else {
      self.const_add(rhs.const_sub(rem))
    }
  }

  #[doc = include_doc!(uint, "pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn pow(self, mut exp: u32) -> Self {
    if exp == 0 {
      return Self::ONE;
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;

    #[cfg(feature = "core_intrinsics")]
    if ::core::intrinsics::is_val_statically_known(exp) {
      while exp > 1 {
        if (exp & 1) == 1 {
          acc = acc.const_mul(base);
        }

        exp /= 2;
        base = base.const_mul(base);
      }

      return acc.const_mul(base);
    }

    loop {
      if (exp & 1) == 1 {
        acc = acc.const_mul(base);

        if exp == 1 {
          return acc;
        }
      }

      exp /= 2;
      base = base.const_mul(base);
    }
  }

  #[doc = include_doc!(uint, "ilog")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog(self, base: Self) -> u32 {
    assert!(
      base.const_ge(&Self::TWO),
      "base of integer logarithm must be at least 2",
    );

    match self.checked_ilog(base) {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(uint, "ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog2(self) -> u32 {
    match self.checked_ilog2() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(uint, "ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog10(self) -> u32 {
    match self.checked_ilog10() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  // TODO: Optimize with Karatsuba
  // https://en.wikipedia.org/wiki/Integer_square_root#Karatsuba_square_root_algorithm
  #[doc = include_doc!(uint, "isqrt")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn isqrt(self) -> Self {
    if self.const_lt(&Self::TWO) {
      return self;
    }

    // -------------------------------------------------------------------------
    // This implementation is based on:
    // https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Binary_numeral_system_(base_2)
    // -------------------------------------------------------------------------

    let mut val: Self = self;
    let mut res: Self = Self::ZERO;
    let mut one: Self = Self::ONE.const_shl(self.ilog2() & !1);

    while !one.is_zero() {
      if val.const_ge(&res.const_add(one)) {
        val = val.const_sub(res.const_add(one));
        res = res.const_shr(1).const_add(one);
      } else {
        res = res.const_shr(1);
      }

      one = one.const_shr(2);
    }

    // SAFETY: the result is positive and fits in an integer with half as many
    //         bits. Inform the optimizer about it.
    unsafe {
      ::core::hint::assert_unchecked(Self::ZERO.const_lt(&res));
      ::core::hint::assert_unchecked(res.const_lt(&Self::ONE.const_shl(Self::BITS / 2)));
    }

    res
  }

  // TODO: Optimize for u8
  #[doc = include_doc!(uint, "abs_diff")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs_diff(self, other: Self) -> Self {
    if self.const_lt(&other) {
      other.const_sub(self)
    } else {
      self.const_sub(other)
    }
  }

  #[doc = include_doc!(uint, "next_power_of_two")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn next_power_of_two(self) -> Self {
    self.one_less_than_next_power_of_two().const_add(Self::ONE)
  }

  #[doc = include_doc!(uint, "is_power_of_two")]
  #[must_use]
  #[inline]
  pub const fn is_power_of_two(self) -> bool {
    self.count_ones() == 1
  }

  #[doc = include_doc!(uint, "is_multiple_of")]
  #[cfg(feature = "unsigned_is_multiple_of")]
  #[must_use]
  #[inline]
  pub const fn is_multiple_of(self, rhs: Self) -> bool {
    if rhs.is_zero() {
      self.is_zero()
    } else {
      self.const_rem(rhs).is_zero()
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "carrying_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_add(rhs);
    let (c, d): (Self, bool) = a.overflowing_add(Self::from_bool(carry));

    (c, b | d)
  }

  #[doc = include_doc!(uint, "borrowing_sub")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_sub(rhs);
    let (c, d): (Self, bool) = a.overflowing_sub(Self::from_bool(borrow));

    (c, b | d)
  }

  #[doc = include_doc!(uint, "carrying_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
    Self::carrying_mul_add(self, rhs, carry, Self::ZERO)
  }

  #[doc = include_doc!(uint, "carrying_mul_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Self, Self) {
    llapi::carrying_mul_add::<Self, Self, N>(self, rhs, carry, add)
  }

  #[doc = include_doc!(uint, "widening_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
    Self::carrying_mul_add(self, rhs, Self::ZERO, Self::ZERO)
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "checked_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(llapi::overflowing_uadd::<Self, N>(self, rhs).1) {
      None
    } else {
      // SAFETY: We just ensured that this does not overflow.
      Some(unsafe { llapi::unchecked_uadd::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add_signed(self, rhs: int<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_add_signed(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(uint, "checked_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    if self.const_lt(&rhs) {
      None
    } else {
      // SAFETY: We just ensured that this does not overflow.
      Some(unsafe { llapi::unchecked_usub::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_sub_signed")]
  #[cfg(feature = "mixed_integer_ops_unsigned_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub_signed(self, rhs: int<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_sub_signed(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(uint, "checked_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_mul(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(uint, "checked_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero.
      Some(unsafe { llapi::unchecked_udiv::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      Some(self.div_euclid(rhs))
    }
  }

  #[doc = include_doc!(uint, "checked_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero.
      Some(unsafe { llapi::unchecked_urem::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      Some(self.rem_euclid(rhs))
    }
  }

  #[doc = include_doc!(uint, "checked_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shl(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(uint, "checked_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shr(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(uint, "checked_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_neg(self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_neg();

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(uint, "checked_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_pow(self, mut exp: u32) -> Option<Self> {
    if exp == 0 {
      return Some(Self::ONE);
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;

    loop {
      if (exp & 1) == 1 {
        acc = tri!(acc.checked_mul(base));

        if exp == 1 {
          return Some(acc);
        }
      }

      exp /= 2;
      base = tri!(base.checked_mul(base));
    }
  }

  #[doc = include_doc!(uint, "checked_next_power_of_two")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_next_power_of_two(self) -> Option<Self> {
    self
      .one_less_than_next_power_of_two()
      .checked_add(Self::ONE)
  }

  #[doc = include_doc!(uint, "checked_next_multiple_of")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
    match self.checked_rem(rhs) {
      Some(result) if result.is_zero() => Some(self),
      Some(result) => self.checked_add(rhs.const_sub(result)),
      None => None,
    }
  }

  // TODO: Optimize for u128
  #[doc = include_doc!(uint, "checked_ilog")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog(self, base: Self) -> Option<u32> {
    if self.const_le(&Self::ZERO) || base.const_le(&Self::ONE) {
      None
    } else if self.const_lt(&base) {
      Some(0)
    } else {
      let mut ilog: u32 = 1;
      let mut data: Self = base;

      while data.const_le(&self.const_div(base)) {
        ilog += 1;
        data = data.const_mul(base);
      }

      Some(ilog)
    }
  }

  // TODO: Optimize with NonZero
  #[doc = include_doc!(uint, "checked_ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog2(self) -> Option<u32> {
    if !self.is_zero() {
      // SAFETY: We just ensured that `self` is not zero.
      Some(Self::BITS - 1 - unsafe { llapi::ctlz_nonzero::<Self, N>(self) })
    } else {
      None
    }
  }

  // TODO: Optimize with Specialization
  #[doc = include_doc!(uint, "checked_ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog10(self) -> Option<u32> {
    self.checked_ilog(Self::from_u8(10))
  }

  #[doc = include_doc!(uint, "checked_signed_diff")]
  #[cfg(feature = "unsigned_signed_diff")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_signed_diff(self, rhs: Self) -> Option<int<N>> {
    let out: int<N> = self.wrapping_sub(rhs).cast_signed();
    let cmp: bool = self.const_ge(&rhs) == out.is_negative();

    if llapi::unlikely(cmp) {
      None
    } else {
      Some(out)
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "overflowing_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_uadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "overflowing_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add_signed(self, rhs: int<N>) -> (Self, bool) {
    let out: (Self, bool) = self.overflowing_add(rhs.cast_unsigned());
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(uint, "overflowing_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_usub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "overflowing_sub_signed")]
  #[cfg(feature = "mixed_integer_ops_unsigned_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub_signed(self, rhs: int<N>) -> (Self, bool) {
    let out: (Self, bool) = self.overflowing_sub(rhs.cast_unsigned());
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(uint, "overflowing_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_umul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "overflowing_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
    (self.const_div(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
    (self.const_div(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
    (self.const_rem(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
    (self.const_rem(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shl(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(uint, "overflowing_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shr(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(uint, "overflowing_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_neg(self) -> (Self, bool) {
    (self.const_not().wrapping_add(Self::ONE), !self.is_zero())
  }

  #[doc = include_doc!(uint, "overflowing_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
    if exp == 0 {
      return (Self::ONE, false);
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;
    let mut overflow: bool = false;
    let mut result: (Self, bool);

    loop {
      if (exp & 1) == 1 {
        result = acc.overflowing_mul(base);

        if exp == 1 {
          result.1 |= overflow;
          return result;
        }

        acc = result.0;
        overflow |= result.1;
      }

      exp /= 2;
      result = base.overflowing_mul(base);
      base = result.0;
      overflow |= result.1;
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "saturating_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add(self, rhs: Self) -> Self {
    llapi::saturating_uadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "saturating_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add_signed(self, rhs: int<N>) -> Self {
    let (result, overflow): (Self, bool) = self.overflowing_add(rhs.cast_unsigned());

    if overflow == rhs.is_negative() {
      result
    } else if overflow {
      Self::MAX
    } else {
      Self::ZERO
    }
  }

  #[doc = include_doc!(uint, "saturating_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub(self, rhs: Self) -> Self {
    llapi::saturating_usub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "saturating_sub_signed")]
  #[cfg(feature = "mixed_integer_ops_unsigned_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub_signed(self, rhs: int<N>) -> Self {
    let (result, overflow): (Self, bool) = self.overflowing_sub_signed(rhs);

    if !overflow {
      result
    } else if rhs.is_negative() {
      Self::MAX
    } else {
      Self::ZERO
    }
  }

  #[doc = include_doc!(uint, "saturating_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None => Self::MAX,
    }
  }

  #[doc = include_doc!(uint, "saturating_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_div(self, rhs: Self) -> Self {
    self.wrapping_div(rhs)
  }

  // TODO: Remove this - only exists for macro code in types/macros/internals
  #[must_use = must_use_doc!()]
  #[inline]
  pub(crate) const fn saturating_neg(self) -> Self {
    // SAFETY: This is never even called
    unsafe { ::core::hint::unreachable_unchecked() }
  }

  #[doc = include_doc!(uint, "saturating_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None => Self::MAX,
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "strict_add")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_add(self, rhs: Self) -> Self {
    match self.checked_add(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(uint, "strict_add_signed")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_add_signed(self, rhs: int<N>) -> Self {
    match self.checked_add_signed(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(uint, "strict_sub")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_sub(self, rhs: Self) -> Self {
    match self.checked_sub(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(uint, "strict_mul")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None => panic::mul(),
    }
  }

  #[doc = include_doc!(uint, "strict_div")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_div(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "strict_div_euclid")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_div_euclid(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "strict_rem")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_rem(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "strict_rem_euclid")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "strict_shl")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_shl(self, rhs: u32) -> Self {
    match self.checked_shl(rhs) {
      Some(result) => result,
      None => panic::shl(),
    }
  }

  #[doc = include_doc!(uint, "strict_shr")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_shr(self, rhs: u32) -> Self {
    match self.checked_shr(rhs) {
      Some(result) => result,
      None => panic::shr(),
    }
  }

  #[doc = include_doc!(uint, "strict_neg")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_neg(self) -> Self {
    match self.checked_neg() {
      Some(result) => result,
      None => panic::neg(),
    }
  }

  #[doc = include_doc!(uint, "strict_pow")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None => panic::mul(),
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "unbounded_shl")]
  #[cfg(feature = "unbounded_shifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unbounded_shl(self, rhs: u32) -> Self {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      unsafe { self.unchecked_shl(rhs) }
    } else {
      Self::ZERO
    }
  }

  #[doc = include_doc!(uint, "unbounded_shr")]
  #[cfg(feature = "unbounded_shifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unbounded_shr(self, rhs: u32) -> Self {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      unsafe { self.unchecked_shr(rhs) }
    } else {
      Self::ZERO
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "unchecked_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_uadd::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_usub::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_umul::<Self, N>(self, rhs) }
  }

  macros::stability! {
    #[unstable(feature = "unchecked_shifts")]
    #[doc = include_doc!(uint, "unchecked_shl")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { llapi::unchecked_shl::<Self, N>(self, rhs) }
    }
  }

  macros::stability! {
    #[unstable(feature = "unchecked_shifts")]
    #[doc = include_doc!(uint, "unchecked_shr")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { llapi::unchecked_lshr::<Self, N>(self, rhs) }
    }
  }
}

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "wrapping_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add(self, rhs: Self) -> Self {
    llapi::wrapping_add::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "wrapping_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add_signed(self, rhs: int<N>) -> Self {
    self.wrapping_add(rhs.cast_unsigned())
  }

  #[doc = include_doc!(uint, "wrapping_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub(self, rhs: Self) -> Self {
    llapi::wrapping_sub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "wrapping_sub_signed")]
  #[cfg(feature = "mixed_integer_ops_unsigned_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub_signed(self, rhs: int<N>) -> Self {
    self.wrapping_sub(rhs.cast_unsigned())
  }

  #[doc = include_doc!(uint, "wrapping_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_mul(self, rhs: Self) -> Self {
    llapi::wrapping_mul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "wrapping_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shl(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shl(rhs & (Self::BITS - 1)) }
  }

  #[doc = include_doc!(uint, "wrapping_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shr(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shr(rhs & (Self::BITS - 1)) }
  }

  #[doc = include_doc!(uint, "wrapping_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_neg(self) -> Self {
    Self::ZERO.wrapping_sub(self)
  }

  #[doc = include_doc!(uint, "wrapping_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_pow(self, mut exp: u32) -> Self {
    if exp == 0 {
      return Self::ONE;
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;

    #[cfg(feature = "core_intrinsics")]
    if ::core::intrinsics::is_val_statically_known(exp) {
      while exp > 1 {
        if (exp & 1) == 1 {
          acc = acc.wrapping_mul(base);
        }

        exp /= 2;
        base = base.wrapping_mul(base);
      }

      return acc.wrapping_mul(base);
    }

    loop {
      if (exp & 1) == 1 {
        acc = acc.wrapping_mul(base);

        if exp == 1 {
          return acc;
        }
      }

      exp /= 2;
      base = base.wrapping_mul(base);
    }
  }

  #[doc = include_doc!(uint, "wrapping_next_power_of_two")]
  #[cfg(feature = "wrapping_next_power_of_two")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_next_power_of_two(self) -> Self {
    self
      .one_less_than_next_power_of_two()
      .wrapping_add(Self::ONE)
  }
}

impl<const N: usize> uint<N> {
  macros::parse_str!(uint);
}

// -----------------------------------------------------------------------------
// u8 Extensions
// -----------------------------------------------------------------------------

impl uint<1> {
  #[doc = include_doc!(uint, "is_ascii")]
  #[must_use]
  #[inline]
  pub const fn is_ascii(&self) -> bool {
    self.into_u8().is_ascii()
  }

  #[doc = include_doc!(uint, "is_ascii_alphabetic")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_alphabetic(&self) -> bool {
    self.into_u8().is_ascii_alphabetic()
  }

  #[doc = include_doc!(uint, "is_ascii_alphanumeric")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_alphanumeric(&self) -> bool {
    self.into_u8().is_ascii_alphanumeric()
  }

  #[doc = include_doc!(uint, "is_ascii_digit")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_digit(&self) -> bool {
    self.into_u8().is_ascii_digit()
  }

  #[doc = include_doc!(uint, "is_ascii_uppercase")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_uppercase(&self) -> bool {
    self.into_u8().is_ascii_uppercase()
  }

  #[doc = include_doc!(uint, "is_ascii_lowercase")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_lowercase(&self) -> bool {
    self.into_u8().is_ascii_lowercase()
  }

  #[doc = include_doc!(uint, "is_ascii_hexdigit")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_hexdigit(&self) -> bool {
    self.into_u8().is_ascii_hexdigit()
  }

  #[doc = include_doc!(uint, "is_ascii_octdigit")]
  #[cfg(feature = "is_ascii_octdigit")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_octdigit(&self) -> bool {
    self.into_u8().is_ascii_octdigit()
  }

  #[doc = include_doc!(uint, "is_ascii_punctuation")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_punctuation(&self) -> bool {
    self.into_u8().is_ascii_punctuation()
  }

  #[doc = include_doc!(uint, "is_ascii_graphic")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_graphic(&self) -> bool {
    self.into_u8().is_ascii_graphic()
  }

  #[doc = include_doc!(uint, "is_ascii_whitespace")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_whitespace(&self) -> bool {
    self.into_u8().is_ascii_whitespace()
  }

  #[doc = include_doc!(uint, "is_ascii_control")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_control(&self) -> bool {
    self.into_u8().is_ascii_control()
  }

  #[doc = include_doc!(uint, "as_ascii")]
  #[cfg(feature = "ascii_char")]
  #[must_use]
  #[inline]
  pub const fn as_ascii(&self) -> Option<::core::ascii::Char> {
    self.into_u8().as_ascii()
  }

  #[doc = include_doc!(uint, "to_ascii_uppercase")]
  #[must_use = "to uppercase the value in-place, use `make_ascii_uppercase()`"]
  #[inline]
  pub const fn to_ascii_uppercase(&self) -> uint<1> {
    Self::from_u8(self.into_u8().to_ascii_uppercase())
  }

  #[doc = include_doc!(uint, "to_ascii_lowercase")]
  #[must_use = "to lowercase the value in-place, use `make_ascii_lowercase()`"]
  #[inline]
  pub const fn to_ascii_lowercase(&self) -> uint<1> {
    Self::from_u8(self.into_u8().to_ascii_lowercase())
  }

  #[doc = include_doc!(uint, "make_ascii_uppercase")]
  #[inline]
  pub const fn make_ascii_uppercase(&mut self) {
    *self = self.to_ascii_uppercase();
  }

  #[doc = include_doc!(uint, "make_ascii_lowercase")]
  #[inline]
  pub const fn make_ascii_lowercase(&mut self) {
    *self = self.to_ascii_lowercase();
  }

  #[doc = include_doc!(uint, "eq_ignore_ascii_case")]
  #[inline]
  pub const fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
    self.into_u8().eq_ignore_ascii_case(&other.into_u8())
  }

  #[doc = include_doc!(uint, "escape_ascii")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub fn escape_ascii(self) -> ::core::ascii::EscapeDefault {
    self.into_u8().escape_ascii()
  }
}

// -----------------------------------------------------------------------------
// u16 Extensions
// -----------------------------------------------------------------------------

impl uint<2> {
  #[doc = include_doc!(uint, "is_utf16_surrogate")]
  #[cfg(feature = "utf16_extra")]
  #[must_use]
  #[inline]
  pub const fn is_utf16_surrogate(self) -> bool {
    self.into_u16().is_utf16_surrogate()
  }
}
