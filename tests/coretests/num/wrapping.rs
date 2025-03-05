use exint::int;
use exint::primitive::*;
use exint::uint;
use exint::Wrapping;

macro_rules! ihex {
  ($value:literal $uint:ident) => {
    uint!($value $uint).cast_signed()
  };
}

macro_rules! wrapping_operation {
  ($result:expr, $lhs:ident $op:tt $rhs:expr) => {
    assert_eq!($result, $lhs $op $rhs);
    assert_eq!($result, &$lhs $op $rhs);
    assert_eq!($result, $lhs $op &$rhs);
    assert_eq!($result, &$lhs $op &$rhs);
  };
  ($result:expr, $op:tt $expr:expr) => {
    assert_eq!($result, $op $expr);
    assert_eq!($result, $op &$expr);
  };
}

macro_rules! wrapping_assignment {
  ($result:expr, $lhs:ident $op:tt $rhs:expr) => {
    let mut lhs = $lhs;
    lhs $op $rhs;
    assert_eq!($result, lhs);

    let mut lhs = $lhs;
    lhs $op &$rhs;
    assert_eq!($result, lhs);
  };
}

macro_rules! wrapping_test {
  ($test:ident, $type:ty, $min:expr, $max:expr) => {
    #[test]
    fn $test() {
      let zero: Wrapping<$type> = Wrapping(<$type>::from_u8(0));
      let one: Wrapping<$type> = Wrapping(<$type>::from_u8(1));
      let min: Wrapping<$type> = Wrapping($min);
      let max: Wrapping<$type> = Wrapping($max);

      wrapping_operation!(min, max + one);
      wrapping_assignment!(min, max += one);

      wrapping_operation!(max, min - one);
      wrapping_assignment!(max, min -= one);

      wrapping_operation!(max, max * one);
      wrapping_assignment!(max, max *= one);

      wrapping_operation!(max, max / one);
      wrapping_assignment!(max, max /= one);

      wrapping_operation!(zero, max % one);
      wrapping_assignment!(zero, max %= one);

      wrapping_operation!(zero, zero & max);
      wrapping_assignment!(zero, zero &= max);

      wrapping_operation!(max, zero | max);
      wrapping_assignment!(max, zero |= max);

      wrapping_operation!(zero, max ^ max);
      wrapping_assignment!(zero, max ^= max);

      // wrapping_operation!(zero, zero << 1usize);
      // wrapping_assignment!(zero, zero <<= 1usize);

      // wrapping_operation!(zero, zero >> 1usize);
      // wrapping_assignment!(zero, zero >>= 1usize);

      wrapping_operation!(zero, -zero);
      wrapping_operation!(max, !min);
    }
  };
}

wrapping_test!(test_wrapping_i8, i8, i8::MIN, i8::MAX);
wrapping_test!(test_wrapping_i16, i16, i16::MIN, i16::MAX);
wrapping_test!(test_wrapping_i24, i24, i24::MIN, i24::MAX);
wrapping_test!(test_wrapping_i32, i32, i32::MIN, i32::MAX);
wrapping_test!(test_wrapping_i40, i40, i40::MIN, i40::MAX);
wrapping_test!(test_wrapping_i48, i48, i48::MIN, i48::MAX);
wrapping_test!(test_wrapping_i56, i56, i56::MIN, i56::MAX);
wrapping_test!(test_wrapping_i64, i64, i64::MIN, i64::MAX);
wrapping_test!(test_wrapping_i72, i72, i72::MIN, i72::MAX);
wrapping_test!(test_wrapping_i80, i80, i80::MIN, i80::MAX);
wrapping_test!(test_wrapping_i88, i88, i88::MIN, i88::MAX);
wrapping_test!(test_wrapping_i96, i96, i96::MIN, i96::MAX);
wrapping_test!(test_wrapping_i104, i104, i104::MIN, i104::MAX);
wrapping_test!(test_wrapping_i112, i112, i112::MIN, i112::MAX);
wrapping_test!(test_wrapping_i120, i120, i120::MIN, i120::MAX);
wrapping_test!(test_wrapping_i128, i128, i128::MIN, i128::MAX);

wrapping_test!(test_wrapping_u8, u8, u8::MIN, u8::MAX);
wrapping_test!(test_wrapping_u16, u16, u16::MIN, u16::MAX);
wrapping_test!(test_wrapping_u24, u24, u24::MIN, u24::MAX);
wrapping_test!(test_wrapping_u32, u32, u32::MIN, u32::MAX);
wrapping_test!(test_wrapping_u40, u40, u40::MIN, u40::MAX);
wrapping_test!(test_wrapping_u48, u48, u48::MIN, u48::MAX);
wrapping_test!(test_wrapping_u56, u56, u56::MIN, u56::MAX);
wrapping_test!(test_wrapping_u64, u64, u64::MIN, u64::MAX);
wrapping_test!(test_wrapping_u72, u72, u72::MIN, u72::MAX);
wrapping_test!(test_wrapping_u80, u80, u80::MIN, u80::MAX);
wrapping_test!(test_wrapping_u88, u88, u88::MIN, u88::MAX);
wrapping_test!(test_wrapping_u96, u96, u96::MIN, u96::MAX);
wrapping_test!(test_wrapping_u104, u104, u104::MIN, u104::MAX);
wrapping_test!(test_wrapping_u112, u112, u112::MIN, u112::MAX);
wrapping_test!(test_wrapping_u120, u120, u120::MIN, u120::MAX);
wrapping_test!(test_wrapping_u128, u128, u128::MIN, u128::MAX);

#[test]
fn wrapping_add_int_api_unsigned() {
  assert_eq!(u8::MAX.wrapping_add(uint!(1 u8)), u8::MIN);
  assert_eq!(u16::MAX.wrapping_add(uint!(1 u16)), u16::MIN);
  assert_eq!(u24::MAX.wrapping_add(uint!(1 u24)), u24::MIN);
  assert_eq!(u32::MAX.wrapping_add(uint!(1 u32)), u32::MIN);
  assert_eq!(u40::MAX.wrapping_add(uint!(1 u40)), u40::MIN);
  assert_eq!(u48::MAX.wrapping_add(uint!(1 u48)), u48::MIN);
  assert_eq!(u56::MAX.wrapping_add(uint!(1 u56)), u56::MIN);
  assert_eq!(u64::MAX.wrapping_add(uint!(1 u64)), u64::MIN);
  assert_eq!(u72::MAX.wrapping_add(uint!(1 u72)), u72::MIN);
  assert_eq!(u80::MAX.wrapping_add(uint!(1 u80)), u80::MIN);
  assert_eq!(u88::MAX.wrapping_add(uint!(1 u88)), u88::MIN);
  assert_eq!(u96::MAX.wrapping_add(uint!(1 u96)), u96::MIN);
  assert_eq!(u104::MAX.wrapping_add(uint!(1 u104)), u104::MIN);
  assert_eq!(u112::MAX.wrapping_add(uint!(1 u112)), u112::MIN);
  assert_eq!(u120::MAX.wrapping_add(uint!(1 u120)), u120::MIN);
  assert_eq!(u128::MAX.wrapping_add(uint!(1 u128)), u128::MIN);
}

#[test]
fn wrapping_sub_int_api_unsigned() {
  assert_eq!(u8::MIN.wrapping_sub(uint!(1 u8)), u8::MAX);
  assert_eq!(u16::MIN.wrapping_sub(uint!(1 u16)), u16::MAX);
  assert_eq!(u24::MIN.wrapping_sub(uint!(1 u24)), u24::MAX);
  assert_eq!(u32::MIN.wrapping_sub(uint!(1 u32)), u32::MAX);
  assert_eq!(u40::MIN.wrapping_sub(uint!(1 u40)), u40::MAX);
  assert_eq!(u48::MIN.wrapping_sub(uint!(1 u48)), u48::MAX);
  assert_eq!(u56::MIN.wrapping_sub(uint!(1 u56)), u56::MAX);
  assert_eq!(u64::MIN.wrapping_sub(uint!(1 u64)), u64::MAX);
  assert_eq!(u72::MIN.wrapping_sub(uint!(1 u72)), u72::MAX);
  assert_eq!(u80::MIN.wrapping_sub(uint!(1 u80)), u80::MAX);
  assert_eq!(u88::MIN.wrapping_sub(uint!(1 u88)), u88::MAX);
  assert_eq!(u96::MIN.wrapping_sub(uint!(1 u96)), u96::MAX);
  assert_eq!(u104::MIN.wrapping_sub(uint!(1 u104)), u104::MAX);
  assert_eq!(u112::MIN.wrapping_sub(uint!(1 u112)), u112::MAX);
  assert_eq!(u120::MIN.wrapping_sub(uint!(1 u120)), u120::MAX);
  assert_eq!(u128::MIN.wrapping_sub(uint!(1 u128)), u128::MAX);
}

#[test]
fn wrapping_mul_int_api_unsigned() {
  assert_eq!(uint!(0xfe u8).wrapping_mul(uint!(16 u8)), uint!(0xe0 u8));
  assert_eq!(uint!(0xfedc u16).wrapping_mul(uint!(16 u16)), uint!(0xedc0 u16));
  assert_eq!(uint!(0xfedcba u24).wrapping_mul(uint!(16 u24)), uint!(0xedcba0 u24));
  assert_eq!(uint!(0xfedcba98 u32).wrapping_mul(uint!(16 u32)), uint!(0xedcba980 u32));
  assert_eq!(uint!(0xfedcba9876 u40).wrapping_mul(uint!(16 u40)), uint!(0xedcba98760 u40));
  assert_eq!(uint!(0xfedcba987654 u48).wrapping_mul(uint!(16 u48)), uint!(0xedcba9876540 u48));
  assert_eq!(uint!(0xfedcba98765432 u56).wrapping_mul(uint!(16 u56)), uint!(0xedcba987654320 u56));
  assert_eq!(uint!(0xfedcba9876543217 u64).wrapping_mul(uint!(16 u64)), uint!(0xedcba98765432170 u64));
}

#[test]
fn wrapping_add_int_api_signed() {
  assert_eq!(i8::MAX.wrapping_add(int!(1 i8)), i8::MIN);
  assert_eq!(i16::MAX.wrapping_add(int!(1 i16)), i16::MIN);
  assert_eq!(i24::MAX.wrapping_add(int!(1 i24)), i24::MIN);
  assert_eq!(i32::MAX.wrapping_add(int!(1 i32)), i32::MIN);
  assert_eq!(i40::MAX.wrapping_add(int!(1 i40)), i40::MIN);
  assert_eq!(i48::MAX.wrapping_add(int!(1 i48)), i48::MIN);
  assert_eq!(i56::MAX.wrapping_add(int!(1 i56)), i56::MIN);
  assert_eq!(i64::MAX.wrapping_add(int!(1 i64)), i64::MIN);
  assert_eq!(i72::MAX.wrapping_add(int!(1 i72)), i72::MIN);
  assert_eq!(i80::MAX.wrapping_add(int!(1 i80)), i80::MIN);
  assert_eq!(i88::MAX.wrapping_add(int!(1 i88)), i88::MIN);
  assert_eq!(i96::MAX.wrapping_add(int!(1 i96)), i96::MIN);
  assert_eq!(i104::MAX.wrapping_add(int!(1 i104)), i104::MIN);
  assert_eq!(i112::MAX.wrapping_add(int!(1 i112)), i112::MIN);
  assert_eq!(i120::MAX.wrapping_add(int!(1 i120)), i120::MIN);
  assert_eq!(i128::MAX.wrapping_add(int!(1 i128)), i128::MIN);
}

#[test]
fn wrapping_sub_int_api_signed() {
  assert_eq!(i8::MIN.wrapping_sub(int!(1 i8)), i8::MAX);
  assert_eq!(i16::MIN.wrapping_sub(int!(1 i16)), i16::MAX);
  assert_eq!(i24::MIN.wrapping_sub(int!(1 i24)), i24::MAX);
  assert_eq!(i32::MIN.wrapping_sub(int!(1 i32)), i32::MAX);
  assert_eq!(i40::MIN.wrapping_sub(int!(1 i40)), i40::MAX);
  assert_eq!(i48::MIN.wrapping_sub(int!(1 i48)), i48::MAX);
  assert_eq!(i56::MIN.wrapping_sub(int!(1 i56)), i56::MAX);
  assert_eq!(i64::MIN.wrapping_sub(int!(1 i64)), i64::MAX);
  assert_eq!(i72::MIN.wrapping_sub(int!(1 i72)), i72::MAX);
  assert_eq!(i80::MIN.wrapping_sub(int!(1 i80)), i80::MAX);
  assert_eq!(i88::MIN.wrapping_sub(int!(1 i88)), i88::MAX);
  assert_eq!(i96::MIN.wrapping_sub(int!(1 i96)), i96::MAX);
  assert_eq!(i104::MIN.wrapping_sub(int!(1 i104)), i104::MAX);
  assert_eq!(i112::MIN.wrapping_sub(int!(1 i112)), i112::MAX);
  assert_eq!(i120::MIN.wrapping_sub(int!(1 i120)), i120::MAX);
  assert_eq!(i128::MIN.wrapping_sub(int!(1 i128)), i128::MAX);
}

#[test]
fn wrapping_mul_int_api_signed() {
  assert_eq!(ihex!(0xfe u8).wrapping_mul(int!(16 i8)), ihex!(0xe0 u8));
  assert_eq!(ihex!(0xfedc u16).wrapping_mul(int!(16 i16)), ihex!(0xedc0 u16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_mul(int!(16 i24)), ihex!(0xedcba0 u24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_mul(int!(16 i32)), ihex!(0xedcba980 u32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_mul(int!(16 i40)), ihex!(0xedcba98760 u40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_mul(int!(16 i48)), ihex!(0xedcba9876540 u48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_mul(int!(16 i56)), ihex!(0xedcba987654320 u56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_mul(int!(16 i64)), ihex!(0xedcba98765432170 u64));
}

#[test]
fn wrapping_div_int_api_signed() {
  assert_eq!(ihex!(0xfe u8).wrapping_div(int!(-1 i8)), ihex!(0xfe u8) / int!(-1 i8));
  assert_eq!(ihex!(0xfedc u16).wrapping_div(int!(-1 i16)), ihex!(0xfedc u16) / int!(-1 i16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_div(int!(-1 i24)), ihex!(0xfedcba u24) / int!(-1 i24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_div(int!(-1 i32)), ihex!(0xfedcba98 u32) / int!(-1 i32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_div(int!(-1 i40)), ihex!(0xfedcba9876 u40) / int!(-1 i40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_div(int!(-1 i48)), ihex!(0xfedcba987654 u48) / int!(-1 i48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_div(int!(-1 i56)), ihex!(0xfedcba98765432 u56) / int!(-1 i56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_div(int!(-1 i64)), ihex!(0xfedcba9876543217 u64) / int!(-1 i64));

  assert_eq!(ihex!(0xfe u8).wrapping_div(int!(-2 i8)), ihex!(0xfe u8) / int!(-2 i8));
  assert_eq!(ihex!(0xfedc u16).wrapping_div(int!(-2 i16)), ihex!(0xfedc u16) / int!(-2 i16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_div(int!(-2 i24)), ihex!(0xfedcba u24) / int!(-2 i24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_div(int!(-2 i32)), ihex!(0xfedcba98 u32) / int!(-2 i32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_div(int!(-2 i40)), ihex!(0xfedcba9876 u40) / int!(-2 i40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_div(int!(-2 i48)), ihex!(0xfedcba987654 u48) / int!(-2 i48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_div(int!(-2 i56)), ihex!(0xfedcba98765432 u56) / int!(-2 i56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_div(int!(-2 i64)), ihex!(0xfedcba9876543217 u64) / int!(-2 i64));

  assert_eq!(ihex!(0xfe u8).wrapping_div(int!(2 i8)), ihex!(0xfe u8) / int!(2 i8));
  assert_eq!(ihex!(0xfedc u16).wrapping_div(int!(2 i16)), ihex!(0xfedc u16) / int!(2 i16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_div(int!(2 i24)), ihex!(0xfedcba u24) / int!(2 i24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_div(int!(2 i32)), ihex!(0xfedcba98 u32) / int!(2 i32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_div(int!(2 i40)), ihex!(0xfedcba9876 u40) / int!(2 i40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_div(int!(2 i48)), ihex!(0xfedcba987654 u48) / int!(2 i48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_div(int!(2 i56)), ihex!(0xfedcba98765432 u56) / int!(2 i56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_div(int!(2 i64)), ihex!(0xfedcba9876543217 u64) / int!(2 i64));

  assert_eq!(i8::MIN.wrapping_div(int!(-1 i8)), i8::MIN);
  assert_eq!(i16::MIN.wrapping_div(int!(-1 i16)), i16::MIN);
  assert_eq!(i24::MIN.wrapping_div(int!(-1 i24)), i24::MIN);
  assert_eq!(i32::MIN.wrapping_div(int!(-1 i32)), i32::MIN);
  assert_eq!(i40::MIN.wrapping_div(int!(-1 i40)), i40::MIN);
  assert_eq!(i48::MIN.wrapping_div(int!(-1 i48)), i48::MIN);
  assert_eq!(i56::MIN.wrapping_div(int!(-1 i56)), i56::MIN);
  assert_eq!(i64::MIN.wrapping_div(int!(-1 i64)), i64::MIN);
}

#[test]
fn wrapping_rem_int_api_signed() {
  assert_eq!(ihex!(0xfe u8).wrapping_rem(int!(-1 i8)), ihex!(0xfe u8) % int!(-1 i8));
  assert_eq!(ihex!(0xfedc u16).wrapping_rem(int!(-1 i16)), ihex!(0xfedc u16) % int!(-1 i16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_rem(int!(-1 i24)), ihex!(0xfedcba u24) % int!(-1 i24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_rem(int!(-1 i32)), ihex!(0xfedcba98 u32) % int!(-1 i32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_rem(int!(-1 i40)), ihex!(0xfedcba9876 u40) % int!(-1 i40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_rem(int!(-1 i48)), ihex!(0xfedcba987654 u48) % int!(-1 i48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_rem(int!(-1 i56)), ihex!(0xfedcba98765432 u56) % int!(-1 i56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_rem(int!(-1 i64)), ihex!(0xfedcba9876543217 u64) % int!(-1 i64));

  assert_eq!(ihex!(0xfe u8).wrapping_rem(int!(-2 i8)), ihex!(0xfe u8) % int!(-2 i8));
  assert_eq!(ihex!(0xfedc u16).wrapping_rem(int!(-2 i16)), ihex!(0xfedc u16) % int!(-2 i16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_rem(int!(-2 i24)), ihex!(0xfedcba u24) % int!(-2 i24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_rem(int!(-2 i32)), ihex!(0xfedcba98 u32) % int!(-2 i32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_rem(int!(-2 i40)), ihex!(0xfedcba9876 u40) % int!(-2 i40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_rem(int!(-2 i48)), ihex!(0xfedcba987654 u48) % int!(-2 i48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_rem(int!(-2 i56)), ihex!(0xfedcba98765432 u56) % int!(-2 i56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_rem(int!(-2 i64)), ihex!(0xfedcba9876543217 u64) % int!(-2 i64));

  assert_eq!(ihex!(0xfe u8).wrapping_rem(int!(2 i8)), ihex!(0xfe u8) % int!(2 i8));
  assert_eq!(ihex!(0xfedc u16).wrapping_rem(int!(2 i16)), ihex!(0xfedc u16) % int!(2 i16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_rem(int!(2 i24)), ihex!(0xfedcba u24) % int!(2 i24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_rem(int!(2 i32)), ihex!(0xfedcba98 u32) % int!(2 i32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_rem(int!(2 i40)), ihex!(0xfedcba9876 u40) % int!(2 i40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_rem(int!(2 i48)), ihex!(0xfedcba987654 u48) % int!(2 i48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_rem(int!(2 i56)), ihex!(0xfedcba98765432 u56) % int!(2 i56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_rem(int!(2 i64)), ihex!(0xfedcba9876543217 u64) % int!(2 i64));

  assert_eq!(i8::MIN.wrapping_rem(int!(-1 i8)), int!(0 i8));
  assert_eq!(i16::MIN.wrapping_rem(int!(-1 i16)), int!(0 i16));
  assert_eq!(i24::MIN.wrapping_rem(int!(-1 i24)), int!(0 i24));
  assert_eq!(i32::MIN.wrapping_rem(int!(-1 i32)), int!(0 i32));
  assert_eq!(i40::MIN.wrapping_rem(int!(-1 i40)), int!(0 i40));
  assert_eq!(i48::MIN.wrapping_rem(int!(-1 i48)), int!(0 i48));
  assert_eq!(i56::MIN.wrapping_rem(int!(-1 i56)), int!(0 i56));
  assert_eq!(i64::MIN.wrapping_rem(int!(-1 i64)), int!(0 i64));
}

#[test]
fn wrapping_neg_int_api_signed() {
  assert_eq!(ihex!(0xfe u8).wrapping_neg(), -ihex!(0xfe u8));
  assert_eq!(ihex!(0xfedc u16).wrapping_neg(), -ihex!(0xfedc u16));
  assert_eq!(ihex!(0xfedcba u24).wrapping_neg(), -ihex!(0xfedcba u24));
  assert_eq!(ihex!(0xfedcba98 u32).wrapping_neg(), -ihex!(0xfedcba98 u32));
  assert_eq!(ihex!(0xfedcba9876 u40).wrapping_neg(), -ihex!(0xfedcba9876 u40));
  assert_eq!(ihex!(0xfedcba987654 u48).wrapping_neg(), -ihex!(0xfedcba987654 u48));
  assert_eq!(ihex!(0xfedcba98765432 u56).wrapping_neg(), -ihex!(0xfedcba98765432 u56));
  assert_eq!(ihex!(0xfedcba9876543217 u64).wrapping_neg(), -ihex!(0xfedcba9876543217 u64));

  assert_eq!(i8::MIN.wrapping_neg(), i8::MIN);
  assert_eq!(i16::MIN.wrapping_neg(), i16::MIN);
  assert_eq!(i24::MIN.wrapping_neg(), i24::MIN);
  assert_eq!(i32::MIN.wrapping_neg(), i32::MIN);
  assert_eq!(i40::MIN.wrapping_neg(), i40::MIN);
  assert_eq!(i48::MIN.wrapping_neg(), i48::MIN);
  assert_eq!(i56::MIN.wrapping_neg(), i56::MIN);
  assert_eq!(i64::MIN.wrapping_neg(), i64::MIN);
}

// TODO: Enable this once const traits are supported
// #[test]
// fn wrapping_const() {
//   const _: () = {
//     assert!(int::MIN.wrapping_div(int!(-1)) == i32::MIN);
//     assert!(int::MIN.wrapping_rem(int!(-1)) == int!(0));
//   };
// }
