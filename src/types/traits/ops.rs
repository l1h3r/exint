use ::core::ops::Add;
use ::core::ops::AddAssign;
use ::core::ops::BitAnd;
use ::core::ops::BitAndAssign;
use ::core::ops::BitOr;
use ::core::ops::BitOrAssign;
use ::core::ops::BitXor;
use ::core::ops::BitXorAssign;
use ::core::ops::Div;
use ::core::ops::DivAssign;
use ::core::ops::Mul;
use ::core::ops::MulAssign;
use ::core::ops::Neg;
use ::core::ops::Not;
use ::core::ops::Rem;
use ::core::ops::RemAssign;
use ::core::ops::Shl;
use ::core::ops::ShlAssign;
use ::core::ops::Shr;
use ::core::ops::ShrAssign;
use ::core::ops::Sub;
use ::core::ops::SubAssign;

use crate::types::Saturating;
use crate::types::Wrapping;
use crate::types::int;
use crate::types::uint;

macro_rules! maybe_convert_arg {
  (Shl, $expr:expr) => {
    maybe_convert_arg!("shift left", $expr)
  };
  (Shr, $expr:expr) => {
    maybe_convert_arg!("shift right", $expr)
  };
  ($method:literal, $expr:expr) => {
    $crate::utils::TryFrom::try_from($expr).unwrap_or(u32::MAX)
  };
  ($_trait:ident, $expr:expr) => {
    // This conversion is only used to support the Wrapper<T> types
    ::core::convert::Into::into($expr)
  };
}

macro_rules! implement_binary {
  (impl<$($generic:ident),+> $trait:ident::$func:ident for $lhs:ty, $rhs:ty as $impl:ident) => {
    const_trait_if! {
      #[feature("const_ops")]
      impl<$(const $generic: usize),+> const $trait<$rhs> for $lhs {
        type Output = Self;

        #[inline]
        fn $func(self, rhs: $rhs) -> Self::Output {
          Self::$impl(self, maybe_convert_arg!($trait, rhs))
        }
      }

      #[feature("const_ops")]
      impl<$(const $generic: usize),+>const $trait<$rhs> for &$lhs {
        type Output = <$lhs as $trait<$rhs>>::Output;

        #[inline]
        fn $func(self, rhs: $rhs) -> Self::Output {
          $trait::$func(*self, rhs)
        }
      }

      #[feature("const_ops")]
      impl<$(const $generic: usize),+> const $trait<&'_ $rhs> for $lhs {
        type Output = <$lhs as $trait<$rhs>>::Output;

        #[inline]
        fn $func(self, rhs: &'_ $rhs) -> Self::Output {
          $trait::$func(self, *rhs)
        }
      }

      #[feature("const_ops")]
      impl<$(const $generic: usize),+> const $trait<&'_ $rhs> for &'_ $lhs {
        type Output = <$lhs as $trait<$rhs>>::Output;

        #[inline]
        fn $func(self, rhs: &'_ $rhs) -> Self::Output {
          $trait::$func(*self, *rhs)
        }
      }
    }
  };
  (impl<$($generic:ident),+> $trait:ident::$func:ident for $lhs:ty, $rhs:ty as $base:ident::$impl:ident) => {
    const_trait_if! {
      #[feature("const_ops")]
      impl<$(const $generic: usize),+> const $trait<$rhs> for $lhs {
        #[inline]
        fn $func(&mut self, rhs: $rhs) {
          *self = $base::$impl(*self, rhs);
        }
      }

      #[feature("const_ops")]
      impl<$(const $generic: usize),+> const $trait<&'_ $rhs> for $lhs {
        #[inline]
        fn $func(&mut self, rhs: &'_ $rhs) {
          $trait::$func(self, *rhs);
        }
      }
    }
  };
}

macro_rules! implement_shift {
  (impl $trait:ident::$func:ident for $name:ident as $impl:ident) => {
    // Crate Types
    implement_binary!(impl<N, M> $trait::$func for $name<N>, int<M> as $impl);
    implement_binary!(impl<N, M> $trait::$func for $name<N>, uint<M> as $impl);
    // Core Types - Unsigned
    implement_binary!(impl<N> $trait::$func for $name<N>, u8    as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u16   as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u32   as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u64   as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u128  as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, usize as $impl);
    // Core Types - Signed
    implement_binary!(impl<N> $trait::$func for $name<N>, i8    as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i16   as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i32   as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i64   as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i128  as $impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, isize as $impl);
  };
  (impl $trait:ident::$func:ident for $name:ident as $base:ident::$impl:ident) => {
    // Crate Types
    implement_binary!(impl<N, M> $trait::$func for $name<N>, int<M> as $base::$impl);
    implement_binary!(impl<N, M> $trait::$func for $name<N>, uint<M> as $base::$impl);
    // Core Types - Unsigned
    implement_binary!(impl<N> $trait::$func for $name<N>, u8    as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u16   as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u32   as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u64   as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, u128  as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, usize as $base::$impl);
    // Core Types - Signed
    implement_binary!(impl<N> $trait::$func for $name<N>, i8    as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i16   as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i32   as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i64   as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, i128  as $base::$impl);
    implement_binary!(impl<N> $trait::$func for $name<N>, isize as $base::$impl);
  };
}

macro_rules! implement_unary {
  (impl $trait:ident::$func:ident for $type:ty as $impl:ident) => {
    const_trait_if! {
      #[feature("const_ops")]
      impl<const N: usize> const $trait for $type {
        type Output = Self;

        #[inline]
        fn $func(self) -> Self::Output {
          Self::$impl(self)
        }
      }

      #[feature("const_ops")]
      impl<const N: usize> const $trait for &'_ $type {
        type Output = <$type as $trait>::Output;

        #[inline]
        fn $func(self) -> Self::Output {
          $trait::$func(*self)
        }
      }
    }
  };
}

macro_rules! implement {
  (@binary for $lhs:ty, $rhs:ty) => {
    implement_binary!(impl<N> Add::add for $lhs, $rhs as const_add);
    implement_binary!(impl<N> Sub::sub for $lhs, $rhs as const_sub);
    implement_binary!(impl<N> Mul::mul for $lhs, $rhs as const_mul);
    implement_binary!(impl<N> Div::div for $lhs, $rhs as const_div);
    implement_binary!(impl<N> Rem::rem for $lhs, $rhs as const_rem);

    implement_binary!(impl<N> AddAssign::add_assign for $lhs, $rhs as Add::add);
    implement_binary!(impl<N> SubAssign::sub_assign for $lhs, $rhs as Sub::sub);
    implement_binary!(impl<N> MulAssign::mul_assign for $lhs, $rhs as Mul::mul);
    implement_binary!(impl<N> DivAssign::div_assign for $lhs, $rhs as Div::div);
    implement_binary!(impl<N> RemAssign::rem_assign for $lhs, $rhs as Rem::rem);
  };
  (@bitwise for $lhs:ty, $rhs:ty) => {
    implement_binary!(impl<N> BitAnd::bitand for $lhs, $rhs as const_band);
    implement_binary!(impl<N> BitOr::bitor for $lhs, $rhs as const_bor);
    implement_binary!(impl<N> BitXor::bitxor for $lhs, $rhs as const_bxor);

    implement_binary!(impl<N> BitAndAssign::bitand_assign for $lhs, $rhs as BitAnd::bitand);
    implement_binary!(impl<N> BitOrAssign::bitor_assign for $lhs, $rhs as BitOr::bitor);
    implement_binary!(impl<N> BitXorAssign::bitxor_assign for $lhs, $rhs as BitXor::bitxor);
  };
  (@shifts for $name:ident) => {
    implement_shift!(impl Shl::shl for $name as const_shl);
    implement_shift!(impl Shr::shr for $name as const_shr);

    implement_shift!(impl ShlAssign::shl_assign for $name as Shl::shl);
    implement_shift!(impl ShrAssign::shr_assign for $name as Shr::shr);
  };
}

// Implement Neg
implement_unary!(impl Neg::neg for int<N> as const_neg);
implement_unary!(impl Neg::neg for Saturating<int<N>> as const_neg);
implement_unary!(impl Neg::neg for Wrapping<int<N>> as const_neg);
implement_unary!(impl Neg::neg for Wrapping<uint<N>> as const_neg);

// Implement Not
implement_unary!(impl Not::not for int<N> as const_not);
implement_unary!(impl Not::not for uint<N> as const_not);
implement_unary!(impl Not::not for Saturating<int<N>> as const_not);
implement_unary!(impl Not::not for Saturating<uint<N>> as const_not);
implement_unary!(impl Not::not for Wrapping<int<N>> as const_not);
implement_unary!(impl Not::not for Wrapping<uint<N>> as const_not);

// Implement Binary Ops
implement!(@binary for int<N>, int<N>);
implement!(@binary for uint<N>, uint<N>);

implement!(@binary for Saturating<int<N>>, int<N>);
implement!(@binary for Saturating<int<N>>, Saturating<int<N>>);

implement!(@binary for Saturating<uint<N>>, uint<N>);
implement!(@binary for Saturating<uint<N>>, Saturating<uint<N>>);

implement!(@binary for Wrapping<int<N>>, int<N>);
implement!(@binary for Wrapping<int<N>>, Wrapping<int<N>>);

implement!(@binary for Wrapping<uint<N>>, uint<N>);
implement!(@binary for Wrapping<uint<N>>, Wrapping<uint<N>>);

// Implement Bitwise Ops
implement!(@bitwise for int<N>, int<N>);
implement!(@bitwise for uint<N>, uint<N>);

implement!(@bitwise for Saturating<int<N>>, int<N>);
implement!(@bitwise for Saturating<int<N>>, Saturating<int<N>>);

implement!(@bitwise for Saturating<uint<N>>, uint<N>);
implement!(@bitwise for Saturating<uint<N>>, Saturating<uint<N>>);

implement!(@bitwise for Wrapping<int<N>>, int<N>);
implement!(@bitwise for Wrapping<int<N>>, Wrapping<int<N>>);

implement!(@bitwise for Wrapping<uint<N>>, uint<N>);
implement!(@bitwise for Wrapping<uint<N>>, Wrapping<uint<N>>);

// Implement Shift Ops
implement!(@shifts for int);
implement!(@shifts for uint);

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::*;

  macro_rules! assert_binary {
    ($trait:ident::$method:ident(&mut $lhs:expr, $rhs:expr) == $result:expr) => {
      let mut lhs = $lhs;
      $trait::$method(&mut lhs, $rhs);
      assert_eq!(lhs, $result);

      let mut lhs = $lhs;
      $trait::$method(&mut lhs, &$rhs);
      assert_eq!(lhs, $result);
    };
    ($trait:ident::$method:ident($lhs:expr, $rhs:expr) == $result:expr) => {
      assert_eq!($trait::$method($lhs, $rhs), $result);
      assert_eq!($trait::$method($lhs, &$rhs), $result);
      assert_eq!($trait::$method(&$lhs, $rhs), $result);
      assert_eq!($trait::$method(&$lhs, &$rhs), $result);
    };
  }

  macro_rules! assert_unary {
    ($trait:ident::$method:ident($value:expr) == $result:expr) => {
      assert_eq!($trait::$method($value), $result);
      assert_eq!($trait::$method(&$value), $result);
    };
  }

  macro_rules! assert_shift {
    ($trait:ident::$method:ident(&mut $value:expr) == $result:expr) => {
      assert_binary!($trait::$method(&mut $value, 0_i8) == $result);
      assert_binary!($trait::$method(&mut $value, 0_i16) == $result);
      assert_binary!($trait::$method(&mut $value, 0_i32) == $result);
      assert_binary!($trait::$method(&mut $value, 0_i64) == $result);
      assert_binary!($trait::$method(&mut $value, 0_i128) == $result);
      assert_binary!($trait::$method(&mut $value, 0_isize) == $result);

      assert_binary!($trait::$method(&mut $value, 0_u8) == $result);
      assert_binary!($trait::$method(&mut $value, 0_u16) == $result);
      assert_binary!($trait::$method(&mut $value, 0_u32) == $result);
      assert_binary!($trait::$method(&mut $value, 0_u64) == $result);
      assert_binary!($trait::$method(&mut $value, 0_u128) == $result);
      assert_binary!($trait::$method(&mut $value, 0_usize) == $result);
    };
    ($trait:ident::$method:ident($value:expr) == $result:expr) => {
      assert_binary!($trait::$method($value, 0_i8) == $result);
      assert_binary!($trait::$method($value, 0_i16) == $result);
      assert_binary!($trait::$method($value, 0_i32) == $result);
      assert_binary!($trait::$method($value, 0_i64) == $result);
      assert_binary!($trait::$method($value, 0_i128) == $result);
      assert_binary!($trait::$method($value, 0_isize) == $result);

      assert_binary!($trait::$method($value, 0_u8) == $result);
      assert_binary!($trait::$method($value, 0_u16) == $result);
      assert_binary!($trait::$method($value, 0_u32) == $result);
      assert_binary!($trait::$method($value, 0_u64) == $result);
      assert_binary!($trait::$method($value, 0_u128) == $result);
      assert_binary!($trait::$method($value, 0_usize) == $result);
    };
  }

  test!(@uint, test_unary_uint, () => {
    assert_unary!(Not::not(T::MIN) == T::MAX);
  });

  test!(@sint, test_unary_sint, () => {
    assert_unary!(Neg::neg(T::P_1) == T::N_1);
    assert_unary!(Not::not(T::P_0) == T::N_1);
  });

  test!(test_binary, () => {
    assert_binary!(Add::add(T::P_0, T::P_0) == T::P_0);
    assert_binary!(Sub::sub(T::P_0, T::P_0) == T::P_0);
    assert_binary!(Mul::mul(T::P_0, T::P_0) == T::P_0);
    assert_binary!(Div::div(T::P_0, T::P_1) == T::P_0);
    assert_binary!(Rem::rem(T::P_0, T::P_1) == T::P_0);

    assert_binary!(AddAssign::add_assign(&mut T::P_0, T::P_0) == T::P_0);
    assert_binary!(SubAssign::sub_assign(&mut T::P_0, T::P_0) == T::P_0);
    assert_binary!(MulAssign::mul_assign(&mut T::P_0, T::P_0) == T::P_0);
    assert_binary!(DivAssign::div_assign(&mut T::P_0, T::P_1) == T::P_0);
    assert_binary!(RemAssign::rem_assign(&mut T::P_0, T::P_1) == T::P_0);
  });

  test!(test_bitwise, () => {
    assert_binary!(BitAnd::bitand(T::P_0, T::P_0) == T::P_0);
    assert_binary!(BitOr::bitor(T::P_0, T::P_0) == T::P_0);
    assert_binary!(BitXor::bitxor(T::P_0, T::P_0) == T::P_0);

    assert_binary!(BitAndAssign::bitand_assign(&mut T::P_0, T::P_0) == T::P_0);
    assert_binary!(BitOrAssign::bitor_assign(&mut T::P_0, T::P_0) == T::P_0);
    assert_binary!(BitXorAssign::bitxor_assign(&mut T::P_0, T::P_0) == T::P_0);
  });

  test!(test_shift, () => {
    assert_shift!(Shl::shl(T::P_0) == T::P_0);
    assert_shift!(Shr::shr(T::P_0) == T::P_0);

    assert_shift!(ShlAssign::shl_assign(&mut T::P_0) == T::P_0);
    assert_shift!(ShrAssign::shr_assign(&mut T::P_0) == T::P_0);
  });
}
