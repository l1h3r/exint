macro_rules! maybe_convert_arg {
  (Shl, $expr:expr_2021) => {
    maybe_convert_arg!("shift left", $expr)
  };
  (Shr, $expr:expr_2021) => {
    maybe_convert_arg!("shift right", $expr)
  };
  ($_trait:ident, $expr:expr_2021) => {
    // This conversion is only used to support the Wrapper<T> types
    ::core::convert::Into::into($expr)
  };
  ($method:literal, $expr:expr_2021) => {
    $crate::utils::TryConvert::<u32>::try_convert($expr).unwrap_or(u32::MAX)
  };
}

macro_rules! implement_binary {
  // Forwarding (1)
  (impl $trait:ident::$func:ident for $type:ty as $impl:ident) => {
    implement_binary!(impl $trait::$func for $type, $type as $impl);
  };
  // Forwarding (2)
  (impl $trait:ident::$func:ident for $type:ty as $base:ident::$impl:ident) => {
    implement_binary!(impl $trait::$func for $type, $type as $base::$impl);
  };
  // Implementation - Binary Op
  (impl $trait:ident::$func:ident for $lhs:ty, $rhs:ty as $impl:ident) => {
    impl<const N: usize> ::core::ops::$trait<$rhs> for $lhs {
      type Output = Self;

      #[inline]
      fn $func(self, rhs: $rhs) -> Self::Output {
        Self::$impl(self, maybe_convert_arg!($trait, rhs))
      }
    }

    implement_binary!(impl ref $trait::$func for $lhs, $rhs);
  };
  // Implementation - Binary Assign Op
  (impl $trait:ident::$func:ident for $lhs:ty, $rhs:ty as $base:ident::$impl:ident) => {
    impl<const N: usize> ::core::ops::$trait<$rhs> for $lhs {
      #[inline]
      fn $func(&mut self, rhs: $rhs) {
        *self = ::core::ops::$base::$impl(*self, rhs);
      }
    }

    implement_binary!(impl ref assign $trait::$func for $lhs, $rhs);
  };
  // Implementation - Binary Op Reference
  (impl ref $trait:ident::$func:ident for $lhs:ty, $rhs:ty) => {
    impl<'a, const N: usize> ::core::ops::$trait<$rhs> for &'a $lhs {
      type Output = <$lhs as ::core::ops::$trait<$rhs>>::Output;

      #[inline]
      fn $func(self, rhs: $rhs) -> Self::Output {
        ::core::ops::$trait::$func(*self, rhs)
      }
    }

    impl<const N: usize> ::core::ops::$trait<&'_ $rhs> for $lhs {
      type Output = <$lhs as ::core::ops::$trait<$rhs>>::Output;

      #[inline]
      fn $func(self, rhs: &'_ $rhs) -> Self::Output {
        ::core::ops::$trait::$func(self, *rhs)
      }
    }

    impl<const N: usize> ::core::ops::$trait<&'_ $rhs> for &'_ $lhs {
      type Output = <$lhs as ::core::ops::$trait<$rhs>>::Output;

      #[inline]
      fn $func(self, rhs: &'_ $rhs) -> Self::Output {
        ::core::ops::$trait::$func(*self, *rhs)
      }
    }
  };
  // Implementation - Binary Assign Op Reference
  (impl ref assign $trait:ident::$func:ident for $lhs:ty, $rhs:ty) => {
    impl<const N: usize> ::core::ops::$trait<&'_ $rhs> for $lhs {
      #[inline]
      fn $func(&mut self, rhs: &'_ $rhs) {
        ::core::ops::$trait::$func(self, *rhs);
      }
    }
  };
  // Entrypoint (1)
  ($name:ident) => {
    // Standard Operators
    implement_binary!(impl Add::add for $crate::$name<N> as const_add);
    implement_binary!(impl Div::div for $crate::$name<N> as const_div);
    implement_binary!(impl Mul::mul for $crate::$name<N> as const_mul);
    implement_binary!(impl Rem::rem for $crate::$name<N> as const_rem);
    implement_binary!(impl Sub::sub for $crate::$name<N> as const_sub);

    // Bitwise Operators
    implement_binary!(impl BitAnd::bitand for $crate::$name<N> as const_band);
    implement_binary!(impl BitOr::bitor   for $crate::$name<N> as const_bor);
    implement_binary!(impl BitXor::bitxor for $crate::$name<N> as const_bxor);

    // Standard Operators (Assign)
    implement_binary!(impl AddAssign::add_assign for $crate::$name<N> as Add::add);
    implement_binary!(impl DivAssign::div_assign for $crate::$name<N> as Div::div);
    implement_binary!(impl MulAssign::mul_assign for $crate::$name<N> as Mul::mul);
    implement_binary!(impl RemAssign::rem_assign for $crate::$name<N> as Rem::rem);
    implement_binary!(impl SubAssign::sub_assign for $crate::$name<N> as Sub::sub);

    // Bitwise Operators (Assign)
    implement_binary!(impl BitAndAssign::bitand_assign for $crate::$name<N> as BitAnd::bitand);
    implement_binary!(impl BitOrAssign::bitor_assign   for $crate::$name<N> as BitOr::bitor);
    implement_binary!(impl BitXorAssign::bitxor_assign for $crate::$name<N> as BitXor::bitxor);

    // Shift Operators (Crate Types)
    implement_binary!(impl Shl::shl for $crate::$name<N> as const_shl);
    implement_binary!(impl Shr::shr for $crate::$name<N> as const_shr);

    // Shift Operators (Core Types)
    implement_binary!(impl Shl::shl for $crate::$name<N>, u8    as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, u16   as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, u32   as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, u64   as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, u128  as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, usize as const_shl);

    implement_binary!(impl Shl::shl for $crate::$name<N>, i8    as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, i16   as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, i32   as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, i64   as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, i128  as const_shl);
    implement_binary!(impl Shl::shl for $crate::$name<N>, isize as const_shl);

    implement_binary!(impl Shr::shr for $crate::$name<N>, u8    as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, u16   as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, u32   as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, u64   as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, u128  as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, usize as const_shr);

    implement_binary!(impl Shr::shr for $crate::$name<N>, i8    as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, i16   as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, i32   as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, i64   as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, i128  as const_shr);
    implement_binary!(impl Shr::shr for $crate::$name<N>, isize as const_shr);

    // Shift Assign Operators (Crate Types)
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N> as Shl::shl);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N> as Shr::shr);

    // Shift Assign Operators (Core Types)
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, u8    as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, u16   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, u32   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, u64   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, u128  as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, usize as Shl::shl);

    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, i8    as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, i16   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, i32   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, i64   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, i128  as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $crate::$name<N>, isize as Shl::shl);

    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, u8    as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, u16   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, u32   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, u64   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, u128  as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, usize as Shr::shr);

    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, i8    as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, i16   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, i32   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, i64   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, i128  as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $crate::$name<N>, isize as Shr::shr);

    // TODO: impl<const N: usize, const M: usize> Shl<$name<M>> for $name<N>;
    // TODO: impl<const N: usize, const M: usize> Shr<$name<M>> for $name<N>;
    // TODO: impl<const N: usize, const M: usize> ShlAssign<$name<M>> for $name<N>;
    // TODO: impl<const N: usize, const M: usize> ShrAssign<$name<M>> for $name<N>;
  };
  // Entrypoint (2)
  ($outer:ident<$inner:ident>) => {
    // Standard Operators
    implement_binary!(impl Add::add for $crate::$outer<$crate::$inner<N>> as const_add);
    implement_binary!(impl Div::div for $crate::$outer<$crate::$inner<N>> as const_div);
    implement_binary!(impl Mul::mul for $crate::$outer<$crate::$inner<N>> as const_mul);
    implement_binary!(impl Rem::rem for $crate::$outer<$crate::$inner<N>> as const_rem);
    implement_binary!(impl Sub::sub for $crate::$outer<$crate::$inner<N>> as const_sub);

    implement_binary!(impl Add::add for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_add);
    implement_binary!(impl Div::div for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_div);
    implement_binary!(impl Mul::mul for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_mul);
    implement_binary!(impl Rem::rem for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_rem);
    implement_binary!(impl Sub::sub for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_sub);

    // Bitwise Operators
    implement_binary!(impl BitAnd::bitand for $crate::$outer<$crate::$inner<N>> as const_band);
    implement_binary!(impl BitOr::bitor   for $crate::$outer<$crate::$inner<N>> as const_bor);
    implement_binary!(impl BitXor::bitxor for $crate::$outer<$crate::$inner<N>> as const_bxor);

    implement_binary!(impl BitAnd::bitand for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_band);
    implement_binary!(impl BitOr::bitor   for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_bor);
    implement_binary!(impl BitXor::bitxor for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as const_bxor);

    // Standard Operators (Assign)
    implement_binary!(impl AddAssign::add_assign for $crate::$outer<$crate::$inner<N>> as Add::add);
    implement_binary!(impl DivAssign::div_assign for $crate::$outer<$crate::$inner<N>> as Div::div);
    implement_binary!(impl MulAssign::mul_assign for $crate::$outer<$crate::$inner<N>> as Mul::mul);
    implement_binary!(impl RemAssign::rem_assign for $crate::$outer<$crate::$inner<N>> as Rem::rem);
    implement_binary!(impl SubAssign::sub_assign for $crate::$outer<$crate::$inner<N>> as Sub::sub);

    implement_binary!(impl AddAssign::add_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as Add::add);
    implement_binary!(impl DivAssign::div_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as Div::div);
    implement_binary!(impl MulAssign::mul_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as Mul::mul);
    implement_binary!(impl RemAssign::rem_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as Rem::rem);
    implement_binary!(impl SubAssign::sub_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as Sub::sub);

    // Bitwise Operators (Assign)
    implement_binary!(impl BitAndAssign::bitand_assign for $crate::$outer<$crate::$inner<N>> as BitAnd::bitand);
    implement_binary!(impl BitOrAssign::bitor_assign   for $crate::$outer<$crate::$inner<N>> as BitOr::bitor);
    implement_binary!(impl BitXorAssign::bitxor_assign for $crate::$outer<$crate::$inner<N>> as BitXor::bitxor);

    implement_binary!(impl BitAndAssign::bitand_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as BitAnd::bitand);
    implement_binary!(impl BitOrAssign::bitor_assign   for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as BitOr::bitor);
    implement_binary!(impl BitXorAssign::bitxor_assign for $crate::$outer<$crate::$inner<N>>, $crate::$inner<N> as BitXor::bitxor);
  };
}

implement_binary!(int);
implement_binary!(uint);

implement_binary!(Saturating<int>);
implement_binary!(Saturating<uint>);

#[cfg(feature = "strict_overflow_ops")]
implement_binary!(Strict<int>);
#[cfg(feature = "strict_overflow_ops")]
implement_binary!(Strict<uint>);

implement_binary!(Wrapping<int>);
implement_binary!(Wrapping<uint>);

macro_rules! implement_unary {
  (impl $trait:ident::$func:ident for $type:ty as $impl:ident) => {
    impl<const N: usize> ::core::ops::$trait for $type {
      type Output = Self;

      #[inline]
      fn $func(self) -> Self::Output {
        Self::$impl(self)
      }
    }

    impl<const N: usize> ::core::ops::$trait for &'_ $type {
      type Output = <$type as ::core::ops::$trait>::Output;

      #[inline]
      fn $func(self) -> Self::Output {
        ::core::ops::$trait::$func(*self)
      }
    }
  };
  (int) => {
    implement_unary!(impl Neg::neg for $crate::int<N> as const_neg);
    implement_unary!(impl Not::not for $crate::int<N> as const_not);
  };
  (uint) => {
    implement_unary!(impl Not::not for $crate::uint<N> as const_not);
  };
  (Saturating<uint>) => {
    implement_unary!(impl Not::not for $crate::Saturating<$crate::uint<N>> as const_not);
  };
  ($outer:ident<$inner:ident>) => {
    implement_unary!(impl Neg::neg for $crate::$outer<$crate::$inner<N>> as const_neg);
    implement_unary!(impl Not::not for $crate::$outer<$crate::$inner<N>> as const_not);
  };
}

implement_unary!(int);
implement_unary!(uint);

implement_unary!(Saturating<int>);
implement_unary!(Saturating<uint>);

#[cfg(feature = "strict_overflow_ops")]
implement_unary!(Strict<int>);
#[cfg(feature = "strict_overflow_ops")]
implement_unary!(Strict<uint>);

implement_unary!(Wrapping<int>);
implement_unary!(Wrapping<uint>);
