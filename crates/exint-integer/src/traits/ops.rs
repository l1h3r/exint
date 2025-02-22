macro_rules! binops {
  // Forwarding (1)
  (impl $trait:ident::$func:ident for $name:ident as $impl:ident) => {
    $crate::traits::binops!(impl $trait::$func for $name<S>, $name<S> as $impl);
  };
  // Forwarding (2)
  (impl $trait:ident::$func:ident for $name:ident as $base:ident::$impl:ident) => {
    $crate::traits::binops!(impl $trait::$func for $name<S>, $name<S> as $base::$impl);
  };
  // Forwarding (3)
  (impl $trait:ident::$func:ident for $name:ident, $rhs:ty as $impl:ident) => {
    $crate::traits::binops!(impl $trait::$func for $name<S>, $rhs as $impl);
  };
  // Forwarding (4)
  (impl $trait:ident::$func:ident for $name:ident, $rhs:ty as $base:ident::$impl:ident) => {
    $crate::traits::binops!(impl $trait::$func for $name<S>, $rhs as $base::$impl);
  };
  // Implementation - Binary Op
  (impl $trait:ident::$func:ident for $lhs:ty, $rhs:ty as $impl:ident) => {
    impl<const S: usize> ::core::ops::$trait<$rhs> for $lhs {
      type Output = Self;

      fn $func(self, rhs: $rhs) -> Self::Output {
        panic!(concat!(stringify!($trait), "::", stringify!($func)))
      }
    }

    $crate::traits::binops!(impl ref $trait::$func for $lhs, $rhs);
  };
  // Implementation - Binary Assign Op
  (impl $trait:ident::$func:ident for $lhs:ty, $rhs:ty as $base:ident::$impl:ident) => {
    impl<const S: usize> ::core::ops::$trait<$rhs> for $lhs {
      fn $func(&mut self, rhs: $rhs) {
        *self = ::core::ops::$base::$impl(*self, rhs);
      }
    }

    $crate::traits::binops!(impl ref assign $trait::$func for $lhs, $rhs);
  };
  // Implementation - Binary Op Reference
  (impl ref $trait:ident::$func:ident for $lhs:ty, $rhs:ty) => {
    impl<'a, const S: usize> ::core::ops::$trait<$rhs> for &'a $lhs {
      type Output = <$lhs as ::core::ops::$trait<$rhs>>::Output;

      fn $func(self, rhs: $rhs) -> Self::Output {
        ::core::ops::$trait::$func(*self, rhs)
      }
    }

    impl<const S: usize> ::core::ops::$trait<&'_ $rhs> for $lhs {
      type Output = <$lhs as ::core::ops::$trait<$rhs>>::Output;

      fn $func(self, rhs: &'_ $rhs) -> Self::Output {
        ::core::ops::$trait::$func(self, *rhs)
      }
    }

    impl<const S: usize> ::core::ops::$trait<&'_ $rhs> for &'_ $lhs {
      type Output = <$lhs as ::core::ops::$trait<$rhs>>::Output;

      fn $func(self, rhs: &'_ $rhs) -> Self::Output {
        ::core::ops::$trait::$func(*self, *rhs)
      }
    }
  };
  // Implementation - Binary Assign Op Reference
  (impl ref assign $trait:ident::$func:ident for $lhs:ty, $rhs:ty) => {
    impl<const S: usize> ::core::ops::$trait<&'_ $rhs> for $lhs {
      fn $func(&mut self, rhs: &'_ $rhs) {
        ::core::ops::$trait::$func(self, *rhs);
      }
    }
  };
  // Entrypoint
  ($name:ident) => {
    // Standard Operators
    $crate::traits::binops!(impl Add::add for $name as add);
    $crate::traits::binops!(impl Div::div for $name as div);
    $crate::traits::binops!(impl Mul::mul for $name as mul);
    $crate::traits::binops!(impl Rem::rem for $name as rem);
    $crate::traits::binops!(impl Sub::sub for $name as sub);

    // Bitwise Operators
    $crate::traits::binops!(impl BitAnd::bitand for $name as bitand);
    $crate::traits::binops!(impl BitOr::bitor   for $name as bitor);
    $crate::traits::binops!(impl BitXor::bitxor for $name as bitxor);

    // Standard Operators (Assign)
    $crate::traits::binops!(impl AddAssign::add_assign for $name as Add::add);
    $crate::traits::binops!(impl DivAssign::div_assign for $name as Div::div);
    $crate::traits::binops!(impl MulAssign::mul_assign for $name as Mul::mul);
    $crate::traits::binops!(impl RemAssign::rem_assign for $name as Rem::rem);
    $crate::traits::binops!(impl SubAssign::sub_assign for $name as Sub::sub);

    // Bitwise Operators (Assign)
    $crate::traits::binops!(impl BitAndAssign::bitand_assign for $name as BitAnd::bitand);
    $crate::traits::binops!(impl BitOrAssign::bitor_assign   for $name as BitOr::bitor);
    $crate::traits::binops!(impl BitXorAssign::bitxor_assign for $name as BitXor::bitxor);

    // Shift Operators (Crate Types)
    $crate::traits::binops!(impl Shl::shl for $name as shl);
    $crate::traits::binops!(impl Shr::shr for $name as shr);

    // Shift Operators (Core Types)
    $crate::traits::binops!(impl Shl::shl for $name, u8    as shl);
    $crate::traits::binops!(impl Shl::shl for $name, u16   as shl);
    $crate::traits::binops!(impl Shl::shl for $name, u32   as shl);
    $crate::traits::binops!(impl Shl::shl for $name, u64   as shl);
    $crate::traits::binops!(impl Shl::shl for $name, u128  as shl);
    $crate::traits::binops!(impl Shl::shl for $name, usize as shl);

    $crate::traits::binops!(impl Shl::shl for $name, i8    as shl);
    $crate::traits::binops!(impl Shl::shl for $name, i16   as shl);
    $crate::traits::binops!(impl Shl::shl for $name, i32   as shl);
    $crate::traits::binops!(impl Shl::shl for $name, i64   as shl);
    $crate::traits::binops!(impl Shl::shl for $name, i128  as shl);
    $crate::traits::binops!(impl Shl::shl for $name, isize as shl);

    $crate::traits::binops!(impl Shr::shr for $name, u8    as shr);
    $crate::traits::binops!(impl Shr::shr for $name, u16   as shr);
    $crate::traits::binops!(impl Shr::shr for $name, u32   as shr);
    $crate::traits::binops!(impl Shr::shr for $name, u64   as shr);
    $crate::traits::binops!(impl Shr::shr for $name, u128  as shr);
    $crate::traits::binops!(impl Shr::shr for $name, usize as shr);

    $crate::traits::binops!(impl Shr::shr for $name, i8    as shr);
    $crate::traits::binops!(impl Shr::shr for $name, i16   as shr);
    $crate::traits::binops!(impl Shr::shr for $name, i32   as shr);
    $crate::traits::binops!(impl Shr::shr for $name, i64   as shr);
    $crate::traits::binops!(impl Shr::shr for $name, i128  as shr);
    $crate::traits::binops!(impl Shr::shr for $name, isize as shr);

    // Shift Assign Operators (Crate Types)
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name as Shl::shl);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name as Shr::shr);

    // Shift Assign Operators (Core Types)
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, u8    as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, u16   as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, u32   as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, u64   as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, u128  as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, usize as Shl::shl);

    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, i8    as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, i16   as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, i32   as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, i64   as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, i128  as Shl::shl);
    $crate::traits::binops!(impl ShlAssign::shl_assign for $name, isize as Shl::shl);

    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, u8    as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, u16   as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, u32   as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, u64   as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, u128  as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, usize as Shr::shr);

    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, i8    as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, i16   as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, i32   as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, i64   as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, i128  as Shr::shr);
    $crate::traits::binops!(impl ShrAssign::shr_assign for $name, isize as Shr::shr);

    // TODO: impl<const T: usize, const U: usize> Shl<$name<U>> for $name<T>;
    // TODO: impl<const T: usize, const U: usize> Shr<$name<U>> for $name<T>;
    // TODO: impl<const T: usize, const U: usize> ShlAssign<$name<U>> for $name<T>;
    // TODO: impl<const T: usize, const U: usize> ShrAssign<$name<U>> for $name<T>;
  };
}

macro_rules! unops {
  // Forwarding (1)
  (uint, $name:ident) => {
    $crate::traits::unops!(impl Not::not for $name as not);
  };
  // Forwarding (2)
  (int, $name:ident) => {
    $crate::traits::unops!(impl Neg::neg for $name as neg);
    $crate::traits::unops!(impl Not::not for $name as not);
  };
  // Implementation - Unary Op
  (impl $trait:ident::$func:ident for $name:ident as $impl:ident) => {
    impl<const S: usize> ::core::ops::$trait for $name<S> {
      type Output = Self;

      fn $func(self) -> Self::Output {
        panic!(concat!(stringify!($trait), "::", stringify!($func)))
      }
    }

    impl<const S: usize> ::core::ops::$trait for &'_ $name<S> {
      type Output = <$name<S> as ::core::ops::$trait>::Output;

      fn $func(self) -> Self::Output {
        ::core::ops::$trait::$func(*self)
      }
    }
  };
  // Entrypoint
  ($name:ident) => {
    $crate::traits::unops!($name, $name);
  };
}

pub(crate) use binops;
pub(crate) use unops;
