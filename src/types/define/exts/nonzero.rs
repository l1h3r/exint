use ::core::convert::Into;
use ::core::ops::BitOr;
use ::core::ops::BitOrAssign;
use ::core::ops::Div;
use ::core::ops::DivAssign;
use ::core::ops::Rem;
use ::core::ops::RemAssign;
use ::core::option::Option;

use crate::types::int;
use crate::types::uint;

macro_rules! implement {
  (@core, $name:ident<$size:literal>, $conv:ident, $nonzero:ident) => {
    const _: () = {
      use ::core::num::$nonzero;

      impl $name<$size> {
        /// Creates a new non-zero integer if `self` is not zero.
        #[must_use]
        #[inline]
        pub const fn to_nonzero(self) -> Option<$nonzero> {
          $nonzero::new(self.$conv())
        }

        /// Creates a non-zero without checking whether the value is non-zero.
        /// This results in undefined behavior if the value is zero.
        ///
        /// # Safety
        ///
        /// The value must not be zero.
        #[must_use]
        #[inline]
        pub const unsafe fn to_nonzero_unchecked(self) -> $nonzero {
          // SAFETY: This is guaranteed to be safe by the caller.
          unsafe { $nonzero::new_unchecked(self.$conv()) }
        }
      }

      impl BitOr<$name<$size>> for $nonzero {
        type Output = Self;

        #[inline]
        fn bitor(self, rhs: $name<$size>) -> Self::Output {
          // SAFETY: `self` is non-zero so the result of bitwise OR is non-zero.
          unsafe { Self::new_unchecked(self.get() | rhs.$conv()) }
        }
      }

      impl BitOrAssign<$name<$size>> for $nonzero {
        #[inline]
        fn bitor_assign(&mut self, rhs: $name<$size>) {
          *self = *self | rhs;
        }
      }

      impl BitOr<$nonzero> for $name<$size> {
        type Output = $nonzero;

        #[inline]
        fn bitor(self, rhs: $nonzero) -> Self::Output {
          // SAFETY: `self` is non-zero so the result of bitwise OR is non-zero.
          unsafe { Self::Output::new_unchecked(self.$conv() | rhs.get()) }
        }
      }
    };
  };
  (int<$size:literal>, $conv:ident, $nonzero:ident) => {
    implement!(@core, int<$size>, $conv, $nonzero);
  };
  (uint<$size:literal>, $conv:ident, $nonzero:ident) => {
    implement!(@core, uint<$size>, $conv, $nonzero);

    const _: () = {
      use ::core::num::$nonzero;

      impl Div<$nonzero> for uint<$size> {
        type Output = uint<$size>;

        /// Same as `self / rhs.get()`, but because `rhs` is a `NonZero<_>`,
        /// there's never a runtime check for division-by-zero.
        ///
        /// This operation rounds towards zero, truncating any fractional part of
        /// the exact result, and cannot panic.
        #[doc(alias = "unchecked_div")]
        #[inline]
        fn div(self, rhs: $nonzero) -> Self::Output {
          // SAFETY: Division by zero is checked because `rhs` is non-zero,
          //         and MIN/-1 is checked because `self` is an unsigned int.
          unsafe { $crate::llapi::unchecked_udiv::<Self, $size>(self, Into::into(rhs)) }
        }
      }

      impl DivAssign<$nonzero> for uint<$size> {
        #[inline]
        fn div_assign(&mut self, rhs: $nonzero) {
          *self = *self / rhs;
        }
      }

      impl Rem<$nonzero> for uint<$size> {
        type Output = uint<$size>;

        /// This operation satisfies `n % d == n - (n / d) * d`, and cannot panic.
        #[inline]
        fn rem(self, rhs: $nonzero) -> Self::Output {
          // SAFETY: Remainder by zero is checked because `rhs` is non-zero,
          //         and MIN/-1 is checked because `self` is an unsigned int.
          unsafe { $crate::llapi::unchecked_urem::<Self, $size>(self, Into::into(rhs)) }
        }
      }

      impl RemAssign<$nonzero> for uint<$size> {
        /// This operation satisfies `n % d == n - (n / d) * d`, and cannot panic.
        #[inline]
        fn rem_assign(&mut self, rhs: $nonzero) {
          *self = *self % rhs;
        }
      }
    };
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
