macro_rules! maybe_convert_arg {
  (Shl, $expr:expr) => {
    maybe_convert_arg!("shift left", $expr)
  };
  (Shr, $expr:expr) => {
    maybe_convert_arg!("shift right", $expr)
  };
  ($_trait:ident, $expr:expr) => {
    $expr
  };
  ($method:literal, $expr:expr) => {
    $crate::utils::TryConvert::<u32>::try_convert($expr).unwrap_or(u32::MAX)
  };
}

macro_rules! implement_binary {
  // Forwarding (1)
  (impl $trait:ident::$func:ident for $name:ident as $impl:ident) => {
    implement_binary!(impl $trait::$func for $crate::$name<N>, $crate::$name<N> as $impl);
  };
  // Forwarding (2)
  (impl $trait:ident::$func:ident for $name:ident as $base:ident::$impl:ident) => {
    implement_binary!(impl $trait::$func for $crate::$name<N>, $crate::$name<N> as $base::$impl);
  };
  // Forwarding (3)
  (impl $trait:ident::$func:ident for $name:ident, $rhs:ty as $impl:ident) => {
    implement_binary!(impl $trait::$func for $crate::$name<N>, $rhs as $impl);
  };
  // Forwarding (4)
  (impl $trait:ident::$func:ident for $name:ident, $rhs:ty as $base:ident::$impl:ident) => {
    implement_binary!(impl $trait::$func for $crate::$name<N>, $rhs as $base::$impl);
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
  // Entrypoint
  ($name:ident) => {
    // Standard Operators
    implement_binary!(impl Add::add for $name as const_add);
    implement_binary!(impl Div::div for $name as const_div);
    implement_binary!(impl Mul::mul for $name as const_mul);
    implement_binary!(impl Rem::rem for $name as const_rem);
    implement_binary!(impl Sub::sub for $name as const_sub);

    // Bitwise Operators
    implement_binary!(impl BitAnd::bitand for $name as const_band);
    implement_binary!(impl BitOr::bitor   for $name as const_bor);
    implement_binary!(impl BitXor::bitxor for $name as const_bxor);

    // Standard Operators (Assign)
    implement_binary!(impl AddAssign::add_assign for $name as Add::add);
    implement_binary!(impl DivAssign::div_assign for $name as Div::div);
    implement_binary!(impl MulAssign::mul_assign for $name as Mul::mul);
    implement_binary!(impl RemAssign::rem_assign for $name as Rem::rem);
    implement_binary!(impl SubAssign::sub_assign for $name as Sub::sub);

    // Bitwise Operators (Assign)
    implement_binary!(impl BitAndAssign::bitand_assign for $name as BitAnd::bitand);
    implement_binary!(impl BitOrAssign::bitor_assign   for $name as BitOr::bitor);
    implement_binary!(impl BitXorAssign::bitxor_assign for $name as BitXor::bitxor);

    // Shift Operators (Crate Types)
    implement_binary!(impl Shl::shl for $name as const_shl);
    implement_binary!(impl Shr::shr for $name as const_shr);

    // Shift Operators (Core Types)
    implement_binary!(impl Shl::shl for $name, u8    as const_shl);
    implement_binary!(impl Shl::shl for $name, u16   as const_shl);
    implement_binary!(impl Shl::shl for $name, u32   as const_shl);
    implement_binary!(impl Shl::shl for $name, u64   as const_shl);
    implement_binary!(impl Shl::shl for $name, u128  as const_shl);
    implement_binary!(impl Shl::shl for $name, usize as const_shl);

    implement_binary!(impl Shl::shl for $name, i8    as const_shl);
    implement_binary!(impl Shl::shl for $name, i16   as const_shl);
    implement_binary!(impl Shl::shl for $name, i32   as const_shl);
    implement_binary!(impl Shl::shl for $name, i64   as const_shl);
    implement_binary!(impl Shl::shl for $name, i128  as const_shl);
    implement_binary!(impl Shl::shl for $name, isize as const_shl);

    implement_binary!(impl Shr::shr for $name, u8    as const_shr);
    implement_binary!(impl Shr::shr for $name, u16   as const_shr);
    implement_binary!(impl Shr::shr for $name, u32   as const_shr);
    implement_binary!(impl Shr::shr for $name, u64   as const_shr);
    implement_binary!(impl Shr::shr for $name, u128  as const_shr);
    implement_binary!(impl Shr::shr for $name, usize as const_shr);

    implement_binary!(impl Shr::shr for $name, i8    as const_shr);
    implement_binary!(impl Shr::shr for $name, i16   as const_shr);
    implement_binary!(impl Shr::shr for $name, i32   as const_shr);
    implement_binary!(impl Shr::shr for $name, i64   as const_shr);
    implement_binary!(impl Shr::shr for $name, i128  as const_shr);
    implement_binary!(impl Shr::shr for $name, isize as const_shr);

    // Shift Assign Operators (Crate Types)
    implement_binary!(impl ShlAssign::shl_assign for $name as Shl::shl);
    implement_binary!(impl ShrAssign::shr_assign for $name as Shr::shr);

    // Shift Assign Operators (Core Types)
    implement_binary!(impl ShlAssign::shl_assign for $name, u8    as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, u16   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, u32   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, u64   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, u128  as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, usize as Shl::shl);

    implement_binary!(impl ShlAssign::shl_assign for $name, i8    as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, i16   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, i32   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, i64   as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, i128  as Shl::shl);
    implement_binary!(impl ShlAssign::shl_assign for $name, isize as Shl::shl);

    implement_binary!(impl ShrAssign::shr_assign for $name, u8    as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, u16   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, u32   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, u64   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, u128  as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, usize as Shr::shr);

    implement_binary!(impl ShrAssign::shr_assign for $name, i8    as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, i16   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, i32   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, i64   as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, i128  as Shr::shr);
    implement_binary!(impl ShrAssign::shr_assign for $name, isize as Shr::shr);

    // TODO: impl<const T: usize, const U: usize> Shl<$name<U>> for $name<T>;
    // TODO: impl<const T: usize, const U: usize> Shr<$name<U>> for $name<T>;
    // TODO: impl<const T: usize, const U: usize> ShlAssign<$name<U>> for $name<T>;
    // TODO: impl<const T: usize, const U: usize> ShrAssign<$name<U>> for $name<T>;
  };
}

implement_binary!(int);
implement_binary!(uint);

macro_rules! implement_unary {
  (impl $trait:ident::$func:ident for $name:ident as $impl:ident) => {
    impl<const N: usize> ::core::ops::$trait for $crate::$name<N> {
      type Output = Self;

      #[inline]
      fn $func(self) -> Self::Output {
        Self::$impl(self)
      }
    }

    impl<const N: usize> ::core::ops::$trait for &'_ $crate::$name<N> {
      type Output = <$crate::$name<N> as ::core::ops::$trait>::Output;

      #[inline]
      fn $func(self) -> Self::Output {
        ::core::ops::$trait::$func(*self)
      }
    }
  };
  (int) => {
    implement_unary!(impl Neg::neg for int as const_neg);
    implement_unary!(impl Not::not for int as const_not);
  };
  (uint) => {
    implement_unary!(impl Not::not for uint as const_not);
  };
}

implement_unary!(int);
implement_unary!(uint);
