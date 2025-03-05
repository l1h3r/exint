macro_rules! int_tests {
  ($type:ident, $uint:ident) => {
    use ::core::ops::BitAnd;
    use ::core::ops::BitOr;
    use ::core::ops::BitXor;
    use ::core::ops::Not;
    use ::core::ops::Shl;
    use ::core::ops::Shr;
    use ::core::str::FromStr;

    use exint::primitive::$type;
    use exint::primitive::$uint;

    macro_rules! int {
      ($value:tt) => {
        $type::from_i128($value)
      };
      ($head:tt $tail:tt) => {
        $type::from_i128($head $tail)
      };
    }

    macro_rules! uint {
      ($value:literal) => {
        $uint::from_u128($value)
      };
    }

    #[test]
    fn test_overflows() {
      assert!($type::MAX > int!(0));
      assert!($type::MIN <= int!(0));
      assert_eq!($type::MIN + $type::MAX + int!(1), int!(0));
    }

    #[test]
    fn test_num() {
      super::test_num(int!(10), int!(2));
    }

    #[test]
    fn test_bitwise_operators() {
      assert_eq!(int!(0b1110), int!(0b1100).bitor(int!(0b1010)));
      assert_eq!(int!(0b1000), int!(0b1100).bitand(int!(0b1010)));
      assert_eq!(int!(0b0110), int!(0b1100).bitxor(int!(0b1010)));
      assert_eq!(int!(0b1110), int!(0b0111).shl(1));
      assert_eq!(int!(0b0111), int!(0b1110).shr(1));
      assert_eq!(-int!(0b11) - int!(1), int!(0b11).not());
    }

    #[test]
    fn test_rem_euclid() {
      assert_eq!(int!(-1).rem_euclid($type::MIN), $type::MAX);
    }

    #[test]
    fn test_abs() {
      assert_eq!(int!(1).abs(), int!(1));
      assert_eq!(int!(0).abs(), int!(0));
      assert_eq!(int!(-1).abs(), int!(1));
    }

    #[test]
    fn test_signum() {
      assert_eq!(int!(1).signum(), int!(1));
      assert_eq!(int!(0).signum(), int!(0));
      assert_eq!(int!(-0).signum(), int!(0));
      assert_eq!(int!(-1).signum(), int!(-1));
    }

    #[test]
    fn test_is_positive() {
      assert!(int!(1).is_positive());
      assert!(!int!(0).is_positive());
      assert!(!int!(-0).is_positive());
      assert!(!int!(-1).is_positive());
    }

    #[test]
    fn test_is_negative() {
      assert!(!int!(1).is_negative());
      assert!(!int!(0).is_negative());
      assert!(!int!(-0).is_negative());
      assert!(int!(-1).is_negative());
    }

    const A: $type = int!(0b0101100);
    const B: $type = int!(0b0100001);
    const C: $type = int!(0b1111001);

    const _0: $type = int!(0);
    const _1: $type = int!(!0);

    #[test]
    fn test_count_ones() {
      assert_eq!(A.count_ones(), 3);
      assert_eq!(B.count_ones(), 2);
      assert_eq!(C.count_ones(), 5);
    }

    #[test]
    fn test_count_zeros() {
      assert_eq!(A.count_zeros(), $type::BITS - 3);
      assert_eq!(B.count_zeros(), $type::BITS - 2);
      assert_eq!(C.count_zeros(), $type::BITS - 5);
    }

    #[test]
    fn test_leading_trailing_ones() {
      const A: $type = int!(0b0101_1111);
      assert_eq!(A.trailing_ones(), 5);
      assert_eq!((!A).leading_ones(), $type::BITS - 7);

      assert_eq!(A.reverse_bits().leading_ones(), 5);

      assert_eq!(_1.leading_ones(), $type::BITS);
      assert_eq!(_1.trailing_ones(), $type::BITS);

      assert_eq!((_1 << 1_i32).trailing_ones(), 0);
      assert_eq!($type::MAX.leading_ones(), 0);

      assert_eq!((_1 << 1_i32).leading_ones(), $type::BITS - 1);
      assert_eq!($type::MAX.trailing_ones(), $type::BITS - 1);

      assert_eq!(_0.leading_ones(), 0);
      assert_eq!(_0.trailing_ones(), 0);

      const X: $type = int!(0b0010_1100);
      assert_eq!(X.leading_ones(), 0);
      assert_eq!(X.trailing_ones(), 0);
    }

    #[test]
    fn test_rotate() {
      const ROT: ::core::primitive::u32 = 128_u32.next_multiple_of($type::BITS);

      assert_eq!(A.rotate_left(6).rotate_right(2).rotate_right(4), A);
      assert_eq!(B.rotate_left(3).rotate_left(2).rotate_right(5), B);
      assert_eq!(C.rotate_left(6).rotate_right(2).rotate_right(4), C);

      // Rotating these should make no difference
      //
      // We test using 124 bits because to ensure that overlong bit shifts do
      // not cause undefined behaviour. See rust-lang/rust#10183.
      assert_eq!(_0.rotate_left(124), _0);
      assert_eq!(_1.rotate_left(124), _1);
      assert_eq!(_0.rotate_right(124), _0);
      assert_eq!(_1.rotate_right(124), _1);

      // Rotating by 0 should have no effect
      assert_eq!(A.rotate_left(0), A);
      assert_eq!(B.rotate_left(0), B);
      assert_eq!(C.rotate_left(0), C);
      // Rotating by a multiple of word size should also have no effect
      assert_eq!(A.rotate_left(ROT), A);
      assert_eq!(B.rotate_left(ROT), B);
      assert_eq!(C.rotate_left(ROT), C);
    }

    #[test]
    fn test_swap_bytes() {
      assert_eq!(A.swap_bytes().swap_bytes(), A);
      assert_eq!(B.swap_bytes().swap_bytes(), B);
      assert_eq!(C.swap_bytes().swap_bytes(), C);

      // Swapping these should make no difference
      assert_eq!(_0.swap_bytes(), _0);
      assert_eq!(_1.swap_bytes(), _1);
    }

    #[test]
    fn test_le() {
      assert_eq!($type::from_le(A.to_le()), A);
      assert_eq!($type::from_le(B.to_le()), B);
      assert_eq!($type::from_le(C.to_le()), C);
      assert_eq!($type::from_le(_0), _0);
      assert_eq!($type::from_le(_1), _1);
      assert_eq!(_0.to_le(), _0);
      assert_eq!(_1.to_le(), _1);
    }

    #[test]
    fn test_be() {
      assert_eq!($type::from_be(A.to_be()), A);
      assert_eq!($type::from_be(B.to_be()), B);
      assert_eq!($type::from_be(C.to_be()), C);
      assert_eq!($type::from_be(_0), _0);
      assert_eq!($type::from_be(_1), _1);
      assert_eq!(_0.to_be(), _0);
      assert_eq!(_1.to_be(), _1);
    }

    #[test]
    fn test_signed_checked_div() {
      assert_eq!(int!(10).checked_div(int!(2)), Some(int!(5)));
      assert_eq!(int!(5).checked_div(int!(0)), None);
      assert_eq!(isize::MIN.checked_div(-1), None);
    }

    #[test]
    fn test_saturating_abs() {
      assert_eq!(int!(0).saturating_abs(), int!(0));
      assert_eq!(int!(123).saturating_abs(), int!(123));
      assert_eq!(int!(-123).saturating_abs(), int!(123));
      assert_eq!(($type::MAX - int!(2)).saturating_abs(), $type::MAX - int!(2));
      assert_eq!(($type::MAX - int!(1)).saturating_abs(), $type::MAX - int!(1));
      assert_eq!($type::MAX.saturating_abs(), $type::MAX);
      assert_eq!(($type::MIN + int!(2)).saturating_abs(), $type::MAX - int!(1));
      assert_eq!(($type::MIN + int!(1)).saturating_abs(), $type::MAX);
      assert_eq!($type::MIN.saturating_abs(), $type::MAX);
    }

    #[test]
    fn test_saturating_neg() {
      assert_eq!(int!(0).saturating_neg(), int!(0));
      assert_eq!(int!(123).saturating_neg(), int!(-123));
      assert_eq!(int!(-123).saturating_neg(), int!(123));
      assert_eq!(($type::MAX - int!(2)).saturating_neg(), $type::MIN + int!(3));
      assert_eq!(($type::MAX - int!(1)).saturating_neg(), $type::MIN + int!(2));
      assert_eq!($type::MAX.saturating_neg(), $type::MIN + int!(1));
      assert_eq!(($type::MIN + int!(2)).saturating_neg(), $type::MAX - int!(1));
      assert_eq!(($type::MIN + int!(1)).saturating_neg(), $type::MAX);
      assert_eq!($type::MIN.saturating_neg(), $type::MAX);
    }

    #[test]
    fn test_isolate_most_significant_one() {
      const BITS: $type = int!(-1);
      const MOST_SIG_ONE: $type = int!(1).checked_shl($type::BITS - 1).unwrap();

      let mut index: ::core::primitive::u32 = 0;

      // Right shift the most significant one through each
      // bit position, starting with all bits set
      while index < $type::BITS {
        assert_eq!(
          (BITS >> index).isolate_most_significant_one(),
          (MOST_SIG_ONE >> index).isolate_most_significant_one(),
        );

        index += 1;
      }
    }

    #[test]
    fn test_isolate_least_significant_one() {
      const BITS: $type = int!(-1);
      const LEAST_SIG_ONE: $type = int!(1);

      let mut index: ::core::primitive::u32 = 0;

      // Left shift the least significant one through each
      // bit position, starting with all bits set
      while index < $type::BITS {
        assert_eq!(
          (BITS << index).isolate_least_significant_one(),
          (LEAST_SIG_ONE << index).isolate_least_significant_one(),
        );

        index += 1;
      }
    }

    #[test]
    fn test_from_str() {
      fn from_str<T: FromStr>(t: &str) -> Option<T> {
        FromStr::from_str(t).ok()
      }

      assert_eq!(from_str("0"), Some(int!(0)));
      assert_eq!(from_str("3"), Some(int!(3)));
      assert_eq!(from_str("10"), Some(int!(10)));
      assert_eq!(from_str("123456789"), Some(exint::int!(123456789)));
      assert_eq!(from_str("00100"), Some(int!(100)));

      assert_eq!(from_str("-1"), Some(int!(-1)));
      assert_eq!(from_str("-3"), Some(int!(-3)));
      assert_eq!(from_str("-10"), Some(int!(-10)));
      assert_eq!(from_str("-123456789"), Some(exint::int!(-123456789)));
      assert_eq!(from_str("-00100"), Some(int!(-100)));

      assert_eq!(from_str::<$type>(""), None);
      assert_eq!(from_str::<$type>(" "), None);
      assert_eq!(from_str::<$type>("x"), None);
    }

    #[test]
    fn test_from_str_radix() {
      assert_eq!($type::from_str_radix("123", 10), Ok(int!(123)));
      assert_eq!($type::from_str_radix("1001", 2), Ok(int!(9)));
      assert_eq!($type::from_str_radix("123", 8), Ok(int!(83)));
      assert_eq!(exint::int::from_str_radix("123", 16), Ok(exint::int!(291)));
      assert_eq!(exint::int::from_str_radix("ffff", 16), Ok(exint::int!(65535)));
      assert_eq!(exint::int::from_str_radix("FFFF", 16), Ok(exint::int!(65535)));
      assert_eq!($type::from_str_radix("z", 36), Ok(int!(35)));
      assert_eq!($type::from_str_radix("Z", 36), Ok(int!(35)));

      assert_eq!($type::from_str_radix("-123", 10), Ok(int!(-123)));
      assert_eq!($type::from_str_radix("-1001", 2), Ok(int!(-9)));
      assert_eq!($type::from_str_radix("-123", 8), Ok(int!(-83)));
      assert_eq!(exint::int::from_str_radix("-123", 16), Ok(exint::int!(-291)));
      assert_eq!(exint::int::from_str_radix("-ffff", 16), Ok(exint::int!(-65535)));
      assert_eq!(exint::int::from_str_radix("-FFFF", 16), Ok(exint::int!(-65535)));
      assert_eq!($type::from_str_radix("-z", 36), Ok(int!(-35)));
      assert_eq!($type::from_str_radix("-Z", 36), Ok(int!(-35)));

      assert!($type::from_str_radix("Z", 35).is_err());
      assert!($type::from_str_radix("-9", 2).is_err());
      assert!($type::from_str_radix("10_0", 10).is_err());
      assert!($uint::from_str_radix("-9", 10).is_err());
    }

    #[test]
    fn test_pow() {
      const R1: $type = int!(2);
      assert_eq!(R1.pow(2), int!(4));
      assert_eq!(R1.pow(0), int!(1));
      assert_eq!(R1.wrapping_pow(2), int!(4));
      assert_eq!(R1.wrapping_pow(0), int!(1));
      assert_eq!(R1.checked_pow(2), Some(int!(4)));
      assert_eq!(R1.checked_pow(0), Some(int!(1)));
      assert_eq!(R1.overflowing_pow(2), (int!(4), false));
      assert_eq!(R1.overflowing_pow(0), (int!(1), false));
      assert_eq!(R1.saturating_pow(2), int!(4));
      assert_eq!(R1.saturating_pow(0), int!(1));

      const R2: $type = $type::MAX;
      assert_eq!(R2.wrapping_pow(2), int!(1));
      assert_eq!(R2.checked_pow(2), None);
      assert_eq!(R2.overflowing_pow(2), (int!(1), true));
      assert_eq!(R2.saturating_pow(2), $type::MAX);

      // test for negative exponent.
      const R3: $type = int!(-2);
      assert_eq!(R3.pow(2), int!(4));
      assert_eq!(R3.pow(3), int!(-8));
      assert_eq!(R3.pow(0), int!(1));
      assert_eq!(R3.wrapping_pow(2), int!(4));
      assert_eq!(R3.wrapping_pow(3), int!(-8));
      assert_eq!(R3.wrapping_pow(0), int!(1));
      assert_eq!(R3.checked_pow(2), Some(int!(4)));
      assert_eq!(R3.checked_pow(3), Some(int!(-8)));
      assert_eq!(R3.checked_pow(0), Some(int!(1)));
      assert_eq!(R3.overflowing_pow(2), (int!(4), false));
      assert_eq!(R3.overflowing_pow(3), (int!(-8), false));
      assert_eq!(R3.overflowing_pow(0), (int!(1), false));
      assert_eq!(R3.saturating_pow(2), int!(4));
      assert_eq!(R3.saturating_pow(3), int!(-8));
      assert_eq!(R3.saturating_pow(0), int!(1));
    }

    #[test]
    fn test_div_floor() {
      const A: $type = int!(8);
      const B: $type = int!(3);
      assert_eq!(A.div_floor(B), int!(2));
      assert_eq!(A.div_floor(-B), int!(-3));
      assert_eq!((-A).div_floor(B), int!(-3));
      assert_eq!((-A).div_floor(-B), int!(2));
    }

    #[test]
    fn test_div_ceil() {
      const A: $type = int!(8);
      const B: $type = int!(3);
      assert_eq!(A.div_ceil(B), int!(3));
      assert_eq!(A.div_ceil(-B), int!(-2));
      assert_eq!((-A).div_ceil(B), int!(-2));
      assert_eq!((-A).div_ceil(-B), int!(3));
    }

    #[test]
    fn test_next_multiple_of() {
      assert_eq!(int!(16).next_multiple_of(int!(8)), int!(16));
      assert_eq!(int!(23).next_multiple_of(int!(8)), int!(24));
      assert_eq!(int!(16).next_multiple_of(int!(-8)), int!(16));
      assert_eq!(int!(23).next_multiple_of(int!(-8)), int!(16));
      assert_eq!(int!(-16).next_multiple_of(int!(8)), int!(-16));
      assert_eq!(int!(-23).next_multiple_of(int!(8)), int!(-16));
      assert_eq!(int!(-16).next_multiple_of(int!(-8)), int!(-16));
      assert_eq!(int!(-23).next_multiple_of(int!(-8)), int!(-24));
      assert_eq!($type::MIN.next_multiple_of(int!(-1)), $type::MIN);
    }

    #[test]
    fn test_checked_next_multiple_of() {
      assert_eq!(int!(16).checked_next_multiple_of(int!(8)), Some(int!(16)));
      assert_eq!(int!(23).checked_next_multiple_of(int!(8)), Some(int!(24)));
      assert_eq!(int!(16).checked_next_multiple_of(int!(-8)), Some(int!(16)));
      assert_eq!(int!(23).checked_next_multiple_of(int!(-8)), Some(int!(16)));
      assert_eq!(int!(-16).checked_next_multiple_of(int!(8)), Some(int!(-16)));
      assert_eq!(int!(-23).checked_next_multiple_of(int!(8)), Some(int!(-16)));
      assert_eq!(int!(-16).checked_next_multiple_of(int!(-8)), Some(int!(-16)));
      assert_eq!(int!(-23).checked_next_multiple_of(int!(-8)), Some(int!(-24)));
      assert_eq!(int!(1).checked_next_multiple_of(int!(0)), None);
      assert_eq!($type::MAX.checked_next_multiple_of(int!(2)), None);
      assert_eq!($type::MIN.checked_next_multiple_of(int!(-3)), None);
      assert_eq!($type::MIN.checked_next_multiple_of(int!(-1)), Some($type::MIN));
    }

    #[test]
    fn test_carrying_add() {
      assert_eq!($type::MAX.carrying_add(int!(1), false), ($type::MIN, true));
      assert_eq!($type::MAX.carrying_add(int!(0), true), ($type::MIN, true));
      assert_eq!($type::MAX.carrying_add(int!(1), true), ($type::MIN + int!(1), true));
      assert_eq!($type::MAX.carrying_add(int!(-1), false), ($type::MAX - int!(1), false));
      assert_eq!($type::MAX.carrying_add(int!(-1), true), ($type::MAX, false)); // no intermediate overflow
      assert_eq!($type::MIN.carrying_add(int!(-1), false), ($type::MAX, true));
      assert_eq!($type::MIN.carrying_add(int!(-1), true), ($type::MIN, false)); // no intermediate overflow
      assert_eq!(int!(0).carrying_add($type::MAX, true), ($type::MIN, true));
      assert_eq!(int!(0).carrying_add($type::MIN, true), ($type::MIN + int!(1), false));
    }

    #[test]
    fn test_borrowing_sub() {
      assert_eq!($type::MIN.borrowing_sub(int!(1), false), ($type::MAX, true));
      assert_eq!($type::MIN.borrowing_sub(int!(0), true), ($type::MAX, true));
      assert_eq!($type::MIN.borrowing_sub(int!(1), true), ($type::MAX - int!(1), true));
      assert_eq!($type::MIN.borrowing_sub(int!(-1), false), ($type::MIN + int!(1), false));
      assert_eq!($type::MIN.borrowing_sub(int!(-1), true), ($type::MIN, false)); // no intermediate overflow
      assert_eq!($type::MAX.borrowing_sub(int!(-1), false), ($type::MIN, true));
      assert_eq!($type::MAX.borrowing_sub(int!(-1), true), ($type::MAX, false)); // no intermediate overflow
      assert_eq!(int!(0).borrowing_sub($type::MIN, false), ($type::MIN, true));
      assert_eq!(int!(0).borrowing_sub($type::MIN, true), ($type::MAX, false));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_widening_mul() {
      assert_eq!($type::MAX.widening_mul($type::MAX), (uint!(1), $type::MAX / int!(2)));
      assert_eq!($type::MIN.widening_mul($type::MAX), ($type::MIN.cast_unsigned(), $type::MIN / int!(2)));
      assert_eq!($type::MIN.widening_mul($type::MIN), (uint!(0), $type::MAX / int!(2) + int!(1)));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_carrying_mul() {
      assert_eq!(
        $type::MAX.carrying_mul($type::MAX, int!(0)),
        (uint!(1), $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MAX.carrying_mul($type::MAX, $type::MAX),
        ($uint::MAX / uint!(2) + uint!(1), $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MAX.carrying_mul($type::MAX, $type::MIN),
        ($uint::MAX / uint!(2) + uint!(2), $type::MAX / int!(2) - int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul($type::MAX, int!(0)),
        ($type::MIN.cast_unsigned(), $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul($type::MAX, $type::MAX),
        ($uint::MAX, $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul($type::MAX, $type::MIN),
        (uint!(0), $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul($type::MIN, int!(0)),
        (uint!(0), $type::MAX / int!(2) + int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul($type::MIN, $type::MAX),
        ($uint::MAX / uint!(2), $type::MAX / int!(2) + int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul($type::MIN, $type::MIN),
        ($uint::MAX / uint!(2) + uint!(1), $type::MAX / int!(2))
      );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_carrying_mul_add() {
      assert_eq!(
        $type::MAX.carrying_mul_add($type::MAX, int!(0), int!(0)),
        (uint!(1), $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MAX.carrying_mul_add($type::MAX, $type::MAX, int!(0)),
        ($uint::MAX / uint!(2) + uint!(1), $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MAX.carrying_mul_add($type::MAX, $type::MIN, int!(0)),
        ($uint::MAX / uint!(2) + uint!(2), $type::MAX / int!(2) - int!(1))
      );

      assert_eq!(
        $type::MAX.carrying_mul_add($type::MAX, $type::MAX, $type::MAX),
        ($uint::MAX, $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MAX.carrying_mul_add($type::MAX, $type::MAX, $type::MIN),
        (uint!(0), $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MAX.carrying_mul_add($type::MAX, $type::MIN, $type::MIN),
        (uint!(1), $type::MAX / int!(2) - int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MAX, int!(0), int!(0)),
        ($type::MIN.cast_unsigned(), $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MAX, $type::MAX, int!(0)),
        ($uint::MAX, $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MAX, $type::MIN, int!(0)),
        (uint!(0), $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MAX, $type::MAX, $type::MAX),
        ($uint::MAX / uint!(2) - uint!(1), $type::MIN / int!(2) + int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MAX, $type::MAX, $type::MIN),
        ($uint::MAX / uint!(2), $type::MIN / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MAX, $type::MIN, $type::MIN),
        ($uint::MAX / uint!(2) + uint!(1), $type::MIN / int!(2) - int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MIN, int!(0), int!(0)),
        (uint!(0), $type::MAX / int!(2) + int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MIN, $type::MAX, int!(0)),
        ($uint::MAX / uint!(2), $type::MAX / int!(2) + int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MIN, $type::MIN, int!(0)),
        ($uint::MAX / uint!(2) + uint!(1), $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MIN, $type::MAX, $type::MAX),
        ($uint::MAX - uint!(1), $type::MAX / int!(2) + int!(1))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MIN, $type::MAX, $type::MIN),
        ($uint::MAX, $type::MAX / int!(2))
      );

      assert_eq!(
        $type::MIN.carrying_mul_add($type::MIN, $type::MIN, $type::MIN),
        (uint!(0), $type::MAX / int!(2))
      );
    }

    #[test]
    fn test_midpoint() {
      assert_eq!($type::midpoint(int!(1), int!(3)), int!(2));
      assert_eq!($type::midpoint(int!(3), int!(1)), int!(2));

      assert_eq!($type::midpoint(int!(0), int!(0)), int!(0));
      assert_eq!($type::midpoint(int!(0), int!(2)), int!(1));
      assert_eq!($type::midpoint(int!(2), int!(0)), int!(1));
      assert_eq!($type::midpoint(int!(2), int!(2)), int!(2));

      assert_eq!($type::midpoint(int!(1), int!(4)), int!(2));
      assert_eq!($type::midpoint(int!(4), int!(1)), int!(2));
      assert_eq!($type::midpoint(int!(3), int!(4)), int!(3));
      assert_eq!($type::midpoint(int!(4), int!(3)), int!(3));

      assert_eq!($type::midpoint($type::MIN, $type::MAX), int!(0));
      assert_eq!($type::midpoint($type::MAX, $type::MIN), int!(0));
      assert_eq!($type::midpoint($type::MIN, $type::MIN), $type::MIN);
      assert_eq!($type::midpoint($type::MAX, $type::MAX), $type::MAX);

      assert_eq!($type::midpoint($type::MIN, int!(6)), $type::MIN / int!(2) + int!(3));
      assert_eq!($type::midpoint(int!(6), $type::MIN), $type::MIN / int!(2) + int!(3));
      assert_eq!($type::midpoint($type::MAX, int!(6)), $type::MAX / int!(2) + int!(3));
      assert_eq!($type::midpoint(int!(6), $type::MAX), $type::MAX / int!(2) + int!(3));
    }
  };
}
