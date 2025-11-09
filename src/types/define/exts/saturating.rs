use ::core::convert::From;

use crate::llapi::Uint;
use crate::types::int;
use crate::types::uint;

/// Provides intentionally-saturating arithmetic on `T`.
///
/// Operations like `+` on `u32` values are intended to never overflow, and
/// in some debug configurations overflow is detected and results in a panic.
/// While most arithmetic falls into this category, some code explicitly
/// expects and relies upon saturating arithmetic.
///
/// Saturating arithmetic can be achieved either through methods like
/// `saturating_add`, or through the `Saturating<T>` type, which says that all
/// standard arithmetic operations on the underlying value are intended to
/// have saturating semantics.
///
/// The underlying value can be retrieved through the `.0` index of the
/// `Saturating` tuple.
///
/// # Layout
///
/// `Saturating<T>` is guaranteed to have the same layout and ABI as `T`.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Saturating<T>(pub T);

// -----------------------------------------------------------------------------
// Implementation - internal
// -----------------------------------------------------------------------------

// SAFETY: `Saturating<T>` has the same layout and ABI as `T`.
unsafe impl<T: Uint> Uint for Saturating<T> {}

// Helpers for types::traits::ops
const_trait_if! {
  #[feature("const_convert")]
  impl<const N: usize> const From<int<N>> for Saturating<int<N>> {
    #[doc(hidden)]
    #[inline]
    fn from(other: int<N>) -> Self {
      Self(other)
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const From<uint<N>> for Saturating<uint<N>> {
    #[doc(hidden)]
    #[inline]
    fn from(other: uint<N>) -> Self {
      Self(other)
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - int
// -----------------------------------------------------------------------------

impl<const N: usize> Saturating<int<N>> {
  crate::types::macros::internals!(Saturating, int);

  #[inline]
  pub(crate) const fn const_neg(self) -> Self {
    Self(self.0.saturating_neg())
  }
}

impl<const N: usize> Saturating<int<N>> {
  crate::types::macros::constants!(Saturating, int);
}

impl<const N: usize> Saturating<int<N>> {
  crate::types::macros::binary!(Saturating, int);
}

impl<const N: usize> Saturating<int<N>> {
  crate::types::macros::byteorder!(Saturating, int);
}

impl<const N: usize> Saturating<int<N>> {
  /// Raises self to the power of `exp`, using exponentiation by squaring.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use exint::Saturating;
  ///
  /// # exint::uint! {
  /// assert_eq!(Saturating(3_i24).pow(4), Saturating(81_i24));
  /// # }
  /// ```
  ///
  /// Results that are too large are saturated:
  ///
  /// ```
  /// use exint::Saturating;
  ///
  /// # exint::uint! {
  /// assert_eq!(Saturating(3_i8).pow(5), Saturating(127_i8));
  /// assert_eq!(Saturating(3_i8).pow(6), Saturating(127_i8));
  /// # }
  /// ```
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn pow(self, exp: u32) -> Self {
    Self(self.0.saturating_pow(exp))
  }

  /// Saturating absolute value. Computes `self.0.abs()`,
  /// returning `MAX` if `self == MIN` instead of overflowing.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use exint::{primitive::i24, Saturating};
  ///
  /// # exint::uint! {
  /// assert_eq!(Saturating(100_i24).abs(), Saturating(100_i24));
  /// assert_eq!(Saturating(-100_i24).abs(), Saturating(100_i24));
  /// assert_eq!(Saturating(i24::MIN).abs(), Saturating((i24::MIN + 1_i24).abs()));
  /// assert_eq!(Saturating(i24::MIN).abs(), Saturating(i24::MIN.saturating_abs()));
  /// assert_eq!(Saturating(i24::MIN).abs(), Saturating(i24::MAX));
  /// # }
  /// ```
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs(self) -> Self {
    Self(self.0.saturating_abs())
  }
}

// -----------------------------------------------------------------------------
// Implementation - uint
// -----------------------------------------------------------------------------

impl<const N: usize> Saturating<uint<N>> {
  crate::types::macros::internals!(Saturating, uint);
}

impl<const N: usize> Saturating<uint<N>> {
  crate::types::macros::constants!(Saturating, uint);
}

impl<const N: usize> Saturating<uint<N>> {
  crate::types::macros::binary!(Saturating, uint);
}

impl<const N: usize> Saturating<uint<N>> {
  crate::types::macros::byteorder!(Saturating, uint);
}

impl<const N: usize> Saturating<uint<N>> {
  /// Raises self to the power of `exp`, using exponentiation by squaring.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use exint::Saturating;
  ///
  /// # exint::uint! {
  /// assert_eq!(Saturating(3_i24).pow(4), Saturating(81_i24));
  /// # }
  /// ```
  ///
  /// Results that are too large are saturated:
  ///
  /// ```
  /// use exint::Saturating;
  ///
  /// # exint::uint! {
  /// assert_eq!(Saturating(3_i8).pow(5), Saturating(127_i8));
  /// assert_eq!(Saturating(3_i8).pow(6), Saturating(127_i8));
  /// # }
  /// ```
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn pow(self, exp: u32) -> Self {
    Self(self.0.saturating_pow(exp))
  }
}
