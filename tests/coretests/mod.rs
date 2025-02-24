//! Test core integer behavior

#[macro_use]
mod int_macros;

#[macro_use]
mod uint_macros;

mod int_log;
mod midpoint;
mod wrapping;

fn test_num<T>(ten: T, two: T)
where
  T: PartialEq + Copy,
  T: ::core::fmt::Debug,
  T: ::core::ops::Add<Output = T>,
  T: ::core::ops::Sub<Output = T>,
  T: ::core::ops::Mul<Output = T>,
  T: ::core::ops::Div<Output = T>,
  T: ::core::ops::Rem<Output = T>,
{
  assert_eq!(ten.add(two), ten + two);
  assert_eq!(ten.sub(two), ten - two);
  assert_eq!(ten.mul(two), ten * two);
  assert_eq!(ten.div(two), ten / two);
  assert_eq!(ten.rem(two), ten % two);
}

macro_rules! define_tests {
  ($mod:ident, int<$size:literal>, $bigint:ident) => {
    mod $mod {
      int_tests!(int<$size>, $bigint);
    }
  };
  ($mod:ident, uint<$size:literal>, $bigint:ident) => {
    mod $mod {
      uint_tests!(uint<$size>, $bigint);
    }
  };
}

define_tests!(i8, int<1>, bigint_pass);
define_tests!(i16, int<2>, bigint_pass);
define_tests!(i24, int<3>, bigint_fail);
define_tests!(i32, int<4>, bigint_pass);
define_tests!(i40, int<5>, bigint_fail);
define_tests!(i48, int<6>, bigint_fail);
define_tests!(i56, int<7>, bigint_fail);
define_tests!(i64, int<8>, bigint_pass);
define_tests!(i72, int<9>, bigint_fail);
define_tests!(i80, int<10>, bigint_fail);
define_tests!(i88, int<11>, bigint_fail);
define_tests!(i96, int<12>, bigint_fail);
define_tests!(i104, int<13>, bigint_fail);
define_tests!(i112, int<14>, bigint_fail);
define_tests!(i120, int<15>, bigint_fail);
define_tests!(i128, int<16>, bigint_pass);

define_tests!(u8, uint<1>, bigint_pass);
define_tests!(u16, uint<2>, bigint_pass);
define_tests!(u24, uint<3>, bigint_fail);
define_tests!(u32, uint<4>, bigint_pass);
define_tests!(u40, uint<5>, bigint_fail);
define_tests!(u48, uint<6>, bigint_fail);
define_tests!(u56, uint<7>, bigint_fail);
define_tests!(u64, uint<8>, bigint_pass);
define_tests!(u72, uint<9>, bigint_fail);
define_tests!(u80, uint<10>, bigint_fail);
define_tests!(u88, uint<11>, bigint_fail);
define_tests!(u96, uint<12>, bigint_fail);
define_tests!(u104, uint<13>, bigint_fail);
define_tests!(u112, uint<14>, bigint_fail);
define_tests!(u120, uint<15>, bigint_fail);
define_tests!(u128, uint<16>, bigint_pass);
