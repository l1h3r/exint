use ::core::intrinsics;

use crate::bridge::traits::BaseBitwise;
use crate::bridge::traits::SpecBitwise;
use crate::macros::specialize;
use crate::traits::SimdExt;
use crate::types::Integer;

specialize! {
  impl const SpecBitwise for Integer<32|64|128|256|512> {
    #[inline]
    fn and(self, other: Self) -> Self {
      #[inline]
      fn rt(lhs: Integer<S>, rhs: Integer<S>) -> Integer<S> {
        Integer::from_simd(lhs.into_simd() & rhs.into_simd())
      }

      intrinsics::const_eval_select((self, other), BaseBitwise::and, rt)
    }

    #[inline]
    fn or(self, other: Self) -> Self {
      #[inline]
      fn rt(lhs: Integer<S>, rhs: Integer<S>) -> Integer<S> {
        Integer::from_simd(lhs.into_simd() | rhs.into_simd())
      }

      intrinsics::const_eval_select((self, other), BaseBitwise::or, rt)
    }

    #[inline]
    fn xor(self, other: Self) -> Self {
      #[inline]
      fn rt(lhs: Integer<S>, rhs: Integer<S>) -> Integer<S> {
        Integer::from_simd(lhs.into_simd() ^ rhs.into_simd())
      }

      intrinsics::const_eval_select((self, other), BaseBitwise::xor, rt)
    }

    #[inline]
    fn not(self) -> Self {
      #[inline]
      fn rt(integer: Integer<S>) -> Integer<S> {
        Integer::from_simd(!integer.into_simd())
      }

      intrinsics::const_eval_select((self,), BaseBitwise::not, rt)
    }
  }
}
