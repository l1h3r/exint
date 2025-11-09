use ::core::convert::From;

use crate::llapi::Uint;
use crate::types::int;
use crate::types::uint;

/// Provides intentionally-wrapped arithmetic on `T`.
///
/// Operations like `+` on `u32` values are intended to never overflow, and
/// in some debug configurations overflow is detected and results in a panic.
/// While most arithmetic falls into this category, some code explicitly
/// expects and relies upon modular arithmetic (e.g., hashing).
///
/// Wrapping arithmetic can be achieved either through methods like
/// `wrapping_add`, or through the `Wrapping<T>` type, which says that all
/// standard arithmetic operations on the underlying value are intended to
/// have wrapping semantics.
///
/// The underlying value can be retrieved through the `.0` index of the
/// `Wrapping` tuple.
///
/// # Layout
///
/// `Wrapping<T>` is guaranteed to have the same layout and ABI as `T`.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Wrapping<T>(pub T);

// -----------------------------------------------------------------------------
// Implementation - internal
// -----------------------------------------------------------------------------

// SAFETY: `Wrapping<T>` has the same layout and ABI as `T`.
unsafe impl<T: Uint> Uint for Wrapping<T> {}

// Helpers for types::traits::ops
const_trait_if! {
  #[feature("const_convert")]
  impl<const N: usize> const From<int<N>> for Wrapping<int<N>> {
    #[doc(hidden)]
    #[inline]
    fn from(other: int<N>) -> Self {
      Self(other)
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const From<uint<N>> for Wrapping<uint<N>> {
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

impl<const N: usize> Wrapping<int<N>> {
  crate::types::macros::internals!(Wrapping, int);
}

impl<const N: usize> Wrapping<int<N>> {
  crate::types::macros::constants!(Wrapping, int);
}

impl<const N: usize> Wrapping<int<N>> {
  crate::types::macros::binary!(Wrapping, int);
}

impl<const N: usize> Wrapping<int<N>> {
  crate::types::macros::byteorder!(Wrapping, int);
}

impl<const N: usize> Wrapping<int<N>> {
  /// Raises self to the power of `exp`, using exponentiation by squaring.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use exint::Wrapping;
  ///
  /// # exint::uint! {
  /// assert_eq!(Wrapping(3_i24).pow(4), Wrapping(81_i24));
  /// # }
  /// ```
  ///
  /// Results that are too large are wrapped:
  ///
  /// ```
  /// use exint::Wrapping;
  ///
  /// # exint::uint! {
  /// assert_eq!(Wrapping(3_i8).pow(5), Wrapping(-13_i8));
  /// assert_eq!(Wrapping(3_i8).pow(6), Wrapping(-39_i8));
  /// # }
  /// ```
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn pow(self, exp: u32) -> Self {
    Self(self.0.wrapping_pow(exp))
  }

  /// Wrapping (modular) absolute value. Computes `self.0.abs()`,
  /// wrapping around at the boundary of the type.
  ///
  /// The only case where such wrapping can occur is when one takes the
  /// absolute value of the negative minimal value for the type; this is a
  /// positive value that is too large to represent in the type. In such a
  /// case, this function returns `MIN` itself.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use exint::{primitive::i24, Wrapping};
  ///
  /// # exint::uint! {
  /// assert_eq!(Wrapping(100_i24).abs(), Wrapping(100_i24));
  /// assert_eq!(Wrapping(-100_i24).abs(), Wrapping(100_i24));
  /// assert_eq!(Wrapping(i24::MIN).abs(), Wrapping(i24::MIN));
  /// assert_eq!(Wrapping(-128_i24).abs().0.cast_unsigned(), 128_u24);
  /// # }
  /// ```
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs(self) -> Self {
    Self(self.0.wrapping_abs())
  }
}

// -----------------------------------------------------------------------------
// Implementation - uint
// -----------------------------------------------------------------------------

impl<const N: usize> Wrapping<uint<N>> {
  crate::types::macros::internals!(Wrapping, uint);
}

impl<const N: usize> Wrapping<uint<N>> {
  crate::types::macros::constants!(Wrapping, uint);
}

impl<const N: usize> Wrapping<uint<N>> {
  crate::types::macros::binary!(Wrapping, uint);
}

impl<const N: usize> Wrapping<uint<N>> {
  crate::types::macros::byteorder!(Wrapping, uint);
}

impl<const N: usize> Wrapping<uint<N>> {
  /// Raises self to the power of `exp`, using exponentiation by squaring.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use exint::Wrapping;
  ///
  /// # exint::uint! {
  /// assert_eq!(Wrapping(3_i24).pow(4), Wrapping(81_i24));
  /// # }
  /// ```
  ///
  /// Results that are too large are wrapped:
  ///
  /// ```
  /// use exint::Wrapping;
  ///
  /// # exint::uint! {
  /// assert_eq!(Wrapping(3_i8).pow(5), Wrapping(-13_i8));
  /// assert_eq!(Wrapping(3_i8).pow(6), Wrapping(-39_i8));
  /// # }
  /// ```
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn pow(self, exp: u32) -> Self {
    Self(self.0.wrapping_pow(exp))
  }
}
