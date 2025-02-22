use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::llapi;
use crate::panic;
use crate::types::macros;
use crate::types::macros::tri;
use crate::types::uint;
use crate::utils::include_doc;
use crate::utils::must_use_doc;

/// The generic signed integer type.
///
/// ## Examples
///
/// Note that the examples here use [`int<4>`] for simplicity and rely on
/// the [`int`] macro for constructing literals.
///
/// [`int<4>`]: Self
/// [`int`]: crate::int!
#[expect(non_camel_case_types)]
#[repr(transparent)]
pub struct int<const N: usize = 4> {
  bytes: [u8; N],
}

impl<const N: usize> int<N> {
  macros::internals!(int);
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "BITS")]
  pub const BITS: u32 = uint::<N>::BITS;

  #[doc = include_doc!(int, "MAX")]
  pub const MAX: Self = uint::<N>::MAX.const_shr(1).cast_signed();

  #[doc = include_doc!(int, "MIN")]
  pub const MIN: Self = Self::MAX.const_not();
}

impl<const N: usize> int<N> {
  macros::byteorder!(int);
}

impl<const N: usize> int<N> {
  macros::bin_tools!(int);
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "max_value")]
  #[deprecated(note = "replaced by the `MAX` associated constant on this type")]
  #[must_use]
  #[inline]
  pub const fn max_value() -> Self {
    Self::MAX
  }

  #[doc = include_doc!(int, "min_value")]
  #[deprecated(note = "replaced by the `MIN` associated constant on this type")]
  #[must_use]
  #[inline]
  pub const fn min_value() -> Self {
    Self::MIN
  }

  #[doc = include_doc!(int, "cast_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn cast_unsigned(self) -> uint<N> {
    uint::from_ne_bytes(self.to_ne_bytes())
  }

  // TODO: Optimize with wide type
  #[doc = include_doc!(int, "midpoint")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn midpoint(self, rhs: Self) -> Self {
    let out: Self = self
      .const_bxor(rhs)
      .const_shr(1)
      .const_add(self.const_band(rhs));

    let add: Self = if out.is_negative() {
      Self::ONE.const_band(self.const_bxor(rhs))
    } else {
      Self::ZERO.const_band(self.const_bxor(rhs))
    };

    out.const_add(add)
  }

  #[doc = include_doc!(int, "div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_euclid(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if rem.is_negative() {
      if rhs.is_positive() {
        div.const_sub(Self::ONE)
      } else {
        div.const_add(Self::ONE)
      }
    } else {
      div
    }
  }

  #[doc = include_doc!(int, "rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn rem_euclid(self, rhs: Self) -> Self {
    let rem: Self = self.const_rem(rhs);

    if rem.is_negative() {
      rem.wrapping_add(rhs.wrapping_abs())
    } else {
      rem
    }
  }

  #[doc = include_doc!(int, "div_ceil")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_ceil(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if !rem.is_zero() {
      self
        .const_bxor(rhs)
        .const_shr(Self::BITS - 1)
        .const_add(Self::ONE)
        .const_add(div)
    } else {
      div
    }
  }

  #[doc = include_doc!(int, "div_floor")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_floor(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if !rem.is_zero() {
      self
        .const_bxor(rhs)
        .const_shr(Self::BITS - 1)
        .const_add(div)
    } else {
      div
    }
  }

  #[doc = include_doc!(int, "next_multiple_of")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn next_multiple_of(self, rhs: Self) -> Self {
    if rhs.const_eq(&Self::NEG_ONE) {
      return self;
    }

    let mut rem: Self = self.const_rem(rhs);

    if (rem.is_positive() && rhs.is_negative()) || (rem.is_negative() && rhs.is_positive()) {
      rem = rem.const_add(rhs);
    }

    if rem.is_zero() {
      self
    } else {
      self.const_add(rhs.const_sub(rem))
    }
  }

  #[doc = include_doc!(int, "pow")]
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

  #[doc = include_doc!(int, "ilog")]
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

  #[doc = include_doc!(int, "ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog2(self) -> u32 {
    match self.checked_ilog2() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(int, "ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog10(self) -> u32 {
    match self.checked_ilog10() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(int, "isqrt")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn isqrt(self) -> Self {
    match self.checked_isqrt() {
      Some(sqrt) => sqrt,
      None => panic::isqrt(),
    }
  }

  #[doc = include_doc!(int, "abs_diff")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs_diff(self, other: Self) -> uint<N> {
    if self.const_lt(&other) {
      other.cast_unsigned().wrapping_sub(self.cast_unsigned())
    } else {
      self.cast_unsigned().wrapping_sub(other.cast_unsigned())
    }
  }

  #[doc = include_doc!(int, "abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs(self) -> Self {
    if self.is_negative() {
      self.const_neg()
    } else {
      self
    }
  }

  #[doc = include_doc!(int, "unsigned_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unsigned_abs(self) -> uint<N> {
    self.wrapping_abs().cast_unsigned()
  }

  #[doc = include_doc!(int, "is_negative")]
  #[must_use]
  #[inline]
  pub const fn is_negative(self) -> bool {
    self.const_lt(&Self::ZERO)
  }

  #[doc = include_doc!(int, "is_positive")]
  #[must_use]
  #[inline]
  pub const fn is_positive(self) -> bool {
    self.const_gt(&Self::ZERO)
  }

  #[doc = include_doc!(int, "signum")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn signum(self) -> Self {
    if self.is_negative() {
      Self::NEG_ONE
    } else if self.is_zero() {
      Self::ZERO
    } else {
      Self::ONE
    }
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "carrying_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_add(rhs);
    let (c, d): (Self, bool) = a.overflowing_add(Self::from_bool(carry));

    (c, b != d)
  }

  #[doc = include_doc!(int, "borrowing_sub")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_sub(rhs);
    let (c, d): (Self, bool) = a.overflowing_sub(Self::from_bool(borrow));

    (c, b != d)
  }

  #[doc = include_doc!(int, "carrying_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (uint<N>, Self) {
    Self::carrying_mul_add(self, rhs, carry, Self::ZERO)
  }

  #[doc = include_doc!(int, "carrying_mul_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (uint<N>, Self) {
    llapi::carrying_mul_add::<Self, uint<N>, N>(self, rhs, carry, add)
  }

  #[doc = include_doc!(int, "widening_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn widening_mul(self, rhs: Self) -> (uint<N>, Self) {
    Self::carrying_mul_add(self, rhs, Self::ZERO, Self::ZERO)
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "checked_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_add(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add_unsigned(self, rhs: uint<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_add_unsigned(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_sub(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub_unsigned(self, rhs: uint<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_sub_unsigned(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_mul")]
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

  #[doc = include_doc!(int, "checked_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
      Some(unsafe { llapi::unchecked_sdiv::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(int, "checked_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      Some(self.div_euclid(rhs))
    }
  }

  #[doc = include_doc!(int, "checked_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
      Some(unsafe { llapi::unchecked_srem::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(int, "checked_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      Some(self.rem_euclid(rhs))
    }
  }

  #[doc = include_doc!(int, "checked_shl")]
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

  #[doc = include_doc!(int, "checked_shr")]
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

  #[doc = include_doc!(int, "checked_neg")]
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

  #[doc = include_doc!(int, "checked_pow")]
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

  #[doc = include_doc!(int, "checked_next_multiple_of")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
    if rhs.const_eq(&Self::NEG_ONE) {
      return Some(self);
    }

    let mut value: Self = tri!(self.checked_rem(rhs));

    if (value.is_positive() && rhs.is_negative()) || (value.is_negative() && rhs.is_positive()) {
      value = value.const_add(rhs);
    }

    if value.is_zero() {
      Some(self)
    } else {
      self.checked_add(rhs.const_sub(value))
    }
  }

  #[doc = include_doc!(int, "checked_ilog")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog(self, base: Self) -> Option<u32> {
    if self.const_le(&Self::ZERO) || base.const_le(&Self::ONE) {
      None
    } else {
      self.cast_unsigned().checked_ilog(base.cast_unsigned())
    }
  }

  // TODO: Optimize with NonZero
  #[doc = include_doc!(int, "checked_ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog2(self) -> Option<u32> {
    if self.is_positive() {
      // SAFETY: We just ensured that `self` is positive.
      Some(Self::BITS - 1 - unsafe { llapi::ctlz_nonzero::<Self, N>(self) })
    } else {
      None
    }
  }

  // TODO: Optimize with Specialization
  #[doc = include_doc!(int, "checked_ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog10(self) -> Option<u32> {
    self.checked_ilog(Self::from_u8(10))
  }

  #[doc = include_doc!(int, "checked_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_abs(self) -> Option<Self> {
    if self.is_negative() {
      self.checked_neg()
    } else {
      Some(self)
    }
  }

  // TODO: Optimize with Specialization
  #[doc = include_doc!(int, "checked_isqrt")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_isqrt(self) -> Option<Self> {
    if self.is_negative() {
      None
    } else {
      Some(self.cast_unsigned().isqrt().cast_signed())
    }
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "overflowing_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_sadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "overflowing_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add_unsigned(self, rhs: uint<N>) -> (Self, bool) {
    let rhs: Self = rhs.cast_signed();
    let out: (Self, bool) = self.overflowing_add(rhs);
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(int, "overflowing_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_ssub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "overflowing_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub_unsigned(self, rhs: uint<N>) -> (Self, bool) {
    let rhs: Self = rhs.cast_signed();
    let out: (Self, bool) = self.overflowing_sub(rhs);
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(int, "overflowing_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_smul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "overflowing_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE)) {
      (self, true)
    } else {
      (self.const_div(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE)) {
      (self, true)
    } else {
      (self.div_euclid(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(rhs.const_eq(&Self::NEG_ONE)) {
      (Self::ZERO, self.const_eq(&Self::MIN))
    } else {
      (self.const_rem(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(rhs.const_eq(&Self::NEG_ONE)) {
      (Self::ZERO, self.const_eq(&Self::MIN))
    } else {
      (self.rem_euclid(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shl(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(int, "overflowing_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shr(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(int, "overflowing_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_neg(self) -> (Self, bool) {
    if llapi::unlikely(self.const_eq(&Self::MIN)) {
      (Self::MIN, true)
    } else {
      (self.const_neg(), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_pow")]
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

  #[doc = include_doc!(int, "overflowing_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_abs(self) -> (Self, bool) {
    (self.wrapping_abs(), self.const_eq(&Self::MIN))
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "saturating_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add(self, rhs: Self) -> Self {
    llapi::saturating_sadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "saturating_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_add_unsigned(rhs) {
      Some(result) => result,
      None => Self::MAX,
    }
  }

  #[doc = include_doc!(int, "saturating_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub(self, rhs: Self) -> Self {
    llapi::saturating_ssub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "saturating_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_sub_unsigned(rhs) {
      Some(result) => result,
      None => Self::MIN,
    }
  }

  #[doc = include_doc!(int, "saturating_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None if self.is_negative() == rhs.is_negative() => Self::MAX,
      None => Self::MIN,
    }
  }

  #[doc = include_doc!(int, "saturating_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_div(self, rhs: Self) -> Self {
    match self.overflowing_div(rhs) {
      (result, false) => result,
      (_result, true) => Self::MAX,
    }
  }

  #[doc = include_doc!(int, "saturating_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_neg(self) -> Self {
    llapi::saturating_ssub::<Self, N>(Self::ZERO, self)
  }

  #[doc = include_doc!(int, "saturating_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None if self.is_negative() && exp % 2 == 1 => Self::MIN,
      None => Self::MAX,
    }
  }

  #[doc = include_doc!(int, "saturating_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_abs(self) -> Self {
    if self.is_negative() {
      self.saturating_neg()
    } else {
      self
    }
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "strict_add")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_add(self, rhs: Self) -> Self {
    match self.checked_add(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(int, "strict_add_unsigned")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_add_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_add_unsigned(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(int, "strict_sub")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_sub(self, rhs: Self) -> Self {
    match self.checked_sub(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(int, "strict_sub_unsigned")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_sub_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_sub_unsigned(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(int, "strict_mul")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None => panic::mul(),
    }
  }

  #[doc = include_doc!(int, "strict_div")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_div(self, rhs: Self) -> Self {
    match self.checked_div(rhs) {
      Some(result) => result,
      None => panic::div(),
    }
  }

  #[doc = include_doc!(int, "strict_div_euclid")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_div_euclid(self, rhs: Self) -> Self {
    match self.checked_div_euclid(rhs) {
      Some(result) => result,
      None => panic::div(),
    }
  }

  #[doc = include_doc!(int, "strict_rem")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_rem(self, rhs: Self) -> Self {
    match self.checked_rem(rhs) {
      Some(result) => result,
      None => panic::rem(),
    }
  }

  #[doc = include_doc!(int, "strict_rem_euclid")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
    match self.checked_rem_euclid(rhs) {
      Some(result) => result,
      None => panic::rem(),
    }
  }

  #[doc = include_doc!(int, "strict_shl")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_shl(self, rhs: u32) -> Self {
    match self.checked_shl(rhs) {
      Some(result) => result,
      None => panic::shl(),
    }
  }

  #[doc = include_doc!(int, "strict_shr")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_shr(self, rhs: u32) -> Self {
    match self.checked_shr(rhs) {
      Some(result) => result,
      None => panic::shr(),
    }
  }

  #[doc = include_doc!(int, "strict_neg")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_neg(self) -> Self {
    match self.checked_neg() {
      Some(result) => result,
      None => panic::neg(),
    }
  }

  #[doc = include_doc!(int, "strict_pow")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None => panic::mul(),
    }
  }

  #[doc = include_doc!(int, "strict_abs")]
  #[cfg(feature = "strict_overflow_ops")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn strict_abs(self) -> Self {
    if self.is_negative() {
      self.strict_neg()
    } else {
      self
    }
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "unbounded_shl")]
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

  #[doc = include_doc!(int, "unbounded_shr")]
  #[cfg(feature = "unbounded_shifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unbounded_shr(self, rhs: u32) -> Self {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      unsafe { self.unchecked_shr(rhs) }
    } else {
      // SAFETY: `Self::BITS - 1` is guaranteed to be less than `Self::BITS`.
      unsafe { self.unchecked_shr(Self::BITS - 1) }
    }
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "unchecked_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_sadd::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_ssub::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_smul::<Self, N>(self, rhs) }
  }

  macros::stability! {
    #[unstable(feature = "unchecked_shifts")]
    #[doc = include_doc!(int, "unchecked_shl")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { llapi::unchecked_shl::<Self, N>(self, rhs) }
    }
  }

  macros::stability! {
    #[unstable(feature = "unchecked_shifts")]
    #[doc = include_doc!(int, "unchecked_shr")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { llapi::unchecked_ashr::<Self, N>(self, rhs) }
    }
  }

  #[doc = include_doc!(int, "unchecked_neg")]
  #[cfg(feature = "unchecked_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_neg(self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_ssub::<Self, N>(Self::ZERO, self) }
  }
}

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "wrapping_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add(self, rhs: Self) -> Self {
    llapi::wrapping_add::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "wrapping_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add_unsigned(self, rhs: uint<N>) -> Self {
    self.wrapping_add(rhs.cast_signed())
  }

  #[doc = include_doc!(int, "wrapping_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub(self, rhs: Self) -> Self {
    llapi::wrapping_sub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "wrapping_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub_unsigned(self, rhs: uint<N>) -> Self {
    self.wrapping_sub(rhs.cast_signed())
  }

  #[doc = include_doc!(int, "wrapping_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_mul(self, rhs: Self) -> Self {
    llapi::wrapping_mul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "wrapping_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div(self, rhs: Self) -> Self {
    self.overflowing_div(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
    self.overflowing_div_euclid(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem(self, rhs: Self) -> Self {
    self.overflowing_rem(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
    self.overflowing_rem_euclid(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shl(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shl(rhs & (Self::BITS - 1)) }
  }

  #[doc = include_doc!(int, "wrapping_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shr(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shr(rhs & (Self::BITS - 1)) }
  }

  #[doc = include_doc!(int, "wrapping_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_neg(self) -> Self {
    Self::ZERO.wrapping_sub(self)
  }

  #[doc = include_doc!(int, "wrapping_pow")]
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

  #[doc = include_doc!(int, "wrapping_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_abs(self) -> Self {
    if self.is_negative() {
      self.wrapping_neg()
    } else {
      self
    }
  }
}

impl<const N: usize> int<N> {
  macros::parse_str!(uint);
}
