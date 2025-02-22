mod private {
  #[allow(unnameable_types)]
  pub trait Sealed {}
}

/// Convert a generic integer to an equivalent non-zero integer type.
pub trait ToNonZero: private::Sealed {
  /// The equivalent non-zero integer type.
  type NonZero;

  /// Creates a new non-zero integer if `self` is not zero.
  fn to_nonzero(self) -> ::core::option::Option<Self::NonZero>;

  /// Creates a non-zero without checking whether the value is non-zero.
  /// This results in undefined behaviour if the value is zero.
  ///
  /// # Safety
  ///
  /// The value must not be zero.
  unsafe fn to_nonzero_unchecked(self) -> Self::NonZero;
}

macro_rules! implement {
  (@core, $name:ident<$size:literal>, $conv:ident, $nonzero:ident) => {
    impl $crate::$name<$size> {
      /// Creates a new non-zero integer if `self` is not zero.
      #[must_use]
      #[inline]
      pub const fn to_nonzero(self) -> ::core::option::Option<::core::num::$nonzero> {
        ::core::num::$nonzero::new(self.$conv())
      }

      /// Creates a non-zero without checking whether the value is non-zero.
      /// This results in undefined behaviour if the value is zero.
      ///
      /// # Safety
      ///
      /// The value must not be zero.
      #[must_use]
      #[inline]
      pub const unsafe fn to_nonzero_unchecked(self) -> ::core::num::$nonzero {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { ::core::num::$nonzero::new_unchecked(self.$conv()) }
      }
    }

    impl private::Sealed for $crate::$name<$size> {}

    impl ToNonZero for $crate::$name<$size> {
      type NonZero = ::core::num::$nonzero;

      #[must_use]
      #[inline]
      fn to_nonzero(self) -> ::core::option::Option<Self::NonZero> {
        self.to_nonzero()
      }

      #[must_use]
      #[inline]
      unsafe fn to_nonzero_unchecked(self) -> Self::NonZero {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { self.to_nonzero_unchecked() }
      }
    }

    impl ::core::ops::BitOr<$crate::$name<$size>> for ::core::num::$nonzero {
      type Output = Self;

      #[inline]
      fn bitor(self, rhs: $crate::$name<$size>) -> Self::Output {
        // SAFETY: `self` is non-zero so the result of bitwise OR is non-zero.
        unsafe { Self::new_unchecked(self.get() | rhs.$conv()) }
      }
    }

    impl ::core::ops::BitOrAssign<$crate::$name<$size>> for ::core::num::$nonzero {
      #[inline]
      fn bitor_assign(&mut self, rhs: $crate::$name<$size>) {
        *self = *self | rhs;
      }
    }

    impl ::core::ops::BitOr<::core::num::$nonzero> for $crate::$name<$size> {
      type Output = ::core::num::$nonzero;

      #[inline]
      fn bitor(self, rhs: ::core::num::$nonzero) -> Self::Output {
        // SAFETY: `self` is non-zero so the result of bitwise OR is non-zero.
        unsafe { Self::Output::new_unchecked(self.$conv() | rhs.get()) }
      }
    }
  };
  (int<$size:literal>, $conv:ident, $nonzero:ident) => {
    implement!(@core, int<$size>, $conv, $nonzero);
  };
  (uint<$size:literal>, $conv:ident, $nonzero:ident) => {
    implement!(@core, uint<$size>, $conv, $nonzero);

    impl ::core::ops::Div<::core::num::$nonzero> for $crate::uint<$size> {
      type Output = $crate::uint<$size>;

      /// Same as `self / rhs.get()`, but because `rhs` is a `NonZero<_>`,
      /// there's never a runtime check for division-by-zero.
      ///
      /// This operation rounds towards zero, truncating any fractional part of
      /// the exact result, and cannot panic.
      #[doc(alias = "unchecked_div")]
      #[inline]
      fn div(self, rhs: ::core::num::$nonzero) -> Self::Output {
        // SAFETY: Division by zero is checked because `rhs` is non-zero,
        //         and MIN/-1 is checked because `self` is an unsigned int.
        unsafe {
          $crate::llapi::unchecked_udiv::<Self, $size>(self, ::core::convert::Into::into(rhs))
        }
      }
    }

    impl ::core::ops::DivAssign<::core::num::$nonzero> for $crate::uint<$size> {
      #[inline]
      fn div_assign(&mut self, rhs: ::core::num::$nonzero) {
        *self = *self / rhs;
      }
    }

    impl ::core::ops::Rem<::core::num::$nonzero> for $crate::uint<$size> {
      type Output = $crate::uint<$size>;

      /// This operation satisfies `n % d == n - (n / d) * d`, and cannot panic.
      #[inline]
      fn rem(self, rhs: ::core::num::$nonzero) -> Self::Output {
        // SAFETY: Remainder by zero is checked because `rhs` is non-zero,
        //         and MIN/-1 is checked because `self` is an unsigned int.
        unsafe {
          $crate::llapi::unchecked_urem::<Self, $size>(self, ::core::convert::Into::into(rhs))
        }
      }
    }

    impl ::core::ops::RemAssign<::core::num::$nonzero> for $crate::uint<$size> {
      /// This operation satisfies `n % d == n - (n / d) * d`, and cannot panic.
      #[inline]
      fn rem_assign(&mut self, rhs: ::core::num::$nonzero) {
        *self = *self % rhs;
      }
    }
  };
}

implement!(uint<1>,  into_u8,   NonZeroU8);
implement!(uint<2>,  into_u16,  NonZeroU16);
implement!(uint<4>,  into_u32,  NonZeroU32);
implement!(uint<8>,  into_u64,  NonZeroU64);
implement!(uint<16>, into_u128, NonZeroU128);

implement!(int<1>,  into_i8,   NonZeroI8);
implement!(int<2>,  into_i16,  NonZeroI16);
implement!(int<4>,  into_i32,  NonZeroI32);
implement!(int<8>,  into_i64,  NonZeroI64);
implement!(int<16>, into_i128, NonZeroI128);
