//! Test core integer behavior

#[macro_use]
mod int_macros;

#[macro_use]
mod uint_macros;

mod int_log;
mod midpoint;
mod wrapping; // TODO: missing shifts

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

mod i8 { int_tests!(i8, u8); }
mod i16 { int_tests!(i16, u16); }
mod i24 { int_tests!(i24, u24); }
mod i32 { int_tests!(i32, u32); }
mod i40 { int_tests!(i40, u40); }
mod i48 { int_tests!(i48, u48); }
mod i56 { int_tests!(i56, u56); }
mod i64 { int_tests!(i64, u64); }
mod i72 { int_tests!(i72, u72); }
mod i80 { int_tests!(i80, u80); }
mod i88 { int_tests!(i88, u88); }
mod i96 { int_tests!(i96, u96); }
mod i104 { int_tests!(i104, u104); }
mod i112 { int_tests!(i112, u112); }
mod i120 { int_tests!(i120, u120); }
mod i128 { int_tests!(i128, u128); }

mod u8 { uint_tests!(u8); }
mod u16 { uint_tests!(u16); }
mod u24 { uint_tests!(u24); }
mod u32 { uint_tests!(u32); }
mod u40 { uint_tests!(u40); }
mod u48 { uint_tests!(u48); }
mod u56 { uint_tests!(u56); }
mod u64 { uint_tests!(u64); }
mod u72 { uint_tests!(u72); }
mod u80 { uint_tests!(u80); }
mod u88 { uint_tests!(u88); }
mod u96 { uint_tests!(u96); }
mod u104 { uint_tests!(u104); }
mod u112 { uint_tests!(u112); }
mod u120 { uint_tests!(u120); }
mod u128 { uint_tests!(u128); }
