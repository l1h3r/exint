macro_rules! uint_tests {
  ($type:ident) => {
    use ::core::ops::BitAnd;
    use ::core::ops::BitOr;
    use ::core::ops::BitXor;
    use ::core::ops::Not;
    use ::core::ops::Shl;
    use ::core::ops::Shr;
    use ::core::str::FromStr;

    use exint::primitive::$type;

    macro_rules! uint {
      ($value:tt) => {
        $type::from_u128($value)
      };
      ($head:tt $tail:tt) => {
        $type::from_u128($head $tail)
      };
    }

    #[test]
    fn test_overflows() {
      assert!($type::MAX > uint!(0));
      assert!($type::MIN <= uint!(0));
      assert!(($type::MIN + $type::MAX).wrapping_add(uint!(1)) == uint!(0));
    }

    #[test]
    fn test_num() {
      super::test_num(uint!(10), uint!(2));
    }

    #[test]
    fn test_bitwise_operators() {
      assert!(uint!(0b1110) == uint!(0b1100).bitor(uint!(0b1010)));
      assert!(uint!(0b1000) == uint!(0b1100).bitand(uint!(0b1010)));
      assert!(uint!(0b0110) == uint!(0b1100).bitxor(uint!(0b1010)));
      assert!(uint!(0b1110) == uint!(0b0111).shl(1));
      assert!(uint!(0b0111) == uint!(0b1110).shr(1));
      assert!($type::MAX - uint!(0b1011) == uint!(0b1011).not());
    }

    const A: $type = uint!(0b0101100);
    const B: $type = uint!(0b0100001);
    const C: $type = uint!(0b1111001);

    const _0: $type = uint!(0);
    const _1: $type = uint!(!0);

    #[test]
    fn test_count_ones() {
      assert!(A.count_ones() == 3);
      assert!(B.count_ones() == 2);
      assert!(C.count_ones() == 5);
    }

    #[test]
    fn test_count_zeros() {
      assert!(A.count_zeros() == $type::BITS - 3);
      assert!(B.count_zeros() == $type::BITS - 2);
      assert!(C.count_zeros() == $type::BITS - 5);
    }

    #[test]
    fn test_leading_trailing_ones() {
      const A: $type = uint!(0b0101_1111);
      assert_eq!(A.trailing_ones(), 5);
      assert_eq!((!A).leading_ones(), $type::BITS - 7);

      assert_eq!(A.reverse_bits().leading_ones(), 5);

      assert_eq!(_1.leading_ones(), $type::BITS);
      assert_eq!(_1.trailing_ones(), $type::BITS);

      assert_eq!((_1 << 1_i32).trailing_ones(), 0);
      assert_eq!((_1 >> 1_i32).leading_ones(), 0);

      assert_eq!((_1 << 1_i32).leading_ones(), $type::BITS - 1);
      assert_eq!((_1 >> 1_i32).trailing_ones(), $type::BITS - 1);

      assert_eq!(_0.leading_ones(), 0);
      assert_eq!(_0.trailing_ones(), 0);

      const X: $type = uint!(0b0010_1100);
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
    fn test_reverse_bits() {
      assert_eq!(A.reverse_bits().reverse_bits(), A);
      assert_eq!(B.reverse_bits().reverse_bits(), B);
      assert_eq!(C.reverse_bits().reverse_bits(), C);

      // Swapping these should make no difference
      assert_eq!(_0.reverse_bits(), _0);
      assert_eq!(_1.reverse_bits(), _1);
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
    fn test_unsigned_checked_div() {
      assert!(uint!(10).checked_div(uint!(2)) == Some(uint!(5)));
      assert!(uint!(5).checked_div(uint!(0)) == None);
    }

    #[test]
    fn test_isolate_most_significant_one() {
      const BITS: $type = $type::MAX;
      const MOST_SIG_ONE: $type = uint!(1).checked_shl($type::BITS - 1).unwrap();

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
      const BITS: $type = $type::MAX;
      const LEAST_SIG_ONE: $type = uint!(1);

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

      assert_eq!(from_str("0"), Some(uint!(0)));
      assert_eq!(from_str("3"), Some(uint!(3)));
      assert_eq!(from_str("10"), Some(uint!(10)));
      assert_eq!(from_str("123456789"), Some(exint::uint!(123456789)));
      assert_eq!(from_str("00100"), Some(uint!(100)));

      assert_eq!(from_str::<$type>(""), None);
      assert_eq!(from_str::<$type>(" "), None);
      assert_eq!(from_str::<$type>("x"), None);
    }

    #[test]
    fn test_parse_bytes() {
      assert_eq!($type::from_str_radix("123", 10), Ok(uint!(123)));
      assert_eq!($type::from_str_radix("1001", 2), Ok(uint!(9)));
      assert_eq!($type::from_str_radix("123", 8), Ok(uint!(83)));
      assert_eq!(exint::uint::from_str_radix("123", 16), Ok(exint::uint!(291 u16)));
      assert_eq!(exint::uint::from_str_radix("ffff", 16), Ok(exint::uint!(65535 u16)));
      assert_eq!($type::from_str_radix("z", 36), Ok(uint!(35)));

      assert!($type::from_str_radix("Z", 10).is_err());
      assert!($type::from_str_radix("_", 2).is_err());
    }

    #[test]
    fn test_pow() {
      const R1: $type = uint!(2);
      assert_eq!(R1.pow(2), uint!(4));
      assert_eq!(R1.pow(0), uint!(1));
      assert_eq!(R1.wrapping_pow(2), uint!(4));
      assert_eq!(R1.wrapping_pow(0), uint!(1));
      assert_eq!(R1.checked_pow(2), Some(uint!(4)));
      assert_eq!(R1.checked_pow(0), Some(uint!(1)));
      assert_eq!(R1.overflowing_pow(2), (uint!(4), false));
      assert_eq!(R1.overflowing_pow(0), (uint!(1), false));
      assert_eq!(R1.saturating_pow(2), uint!(4));
      assert_eq!(R1.saturating_pow(0), uint!(1));

      const R2: $type = $type::MAX;
      assert_eq!(R2.wrapping_pow(2), uint!(1));
      assert_eq!(R2.checked_pow(2), None);
      assert_eq!(R2.overflowing_pow(2), (uint!(1), true));
      assert_eq!(R2.saturating_pow(2), $type::MAX);
    }

    #[test]
    fn test_isqrt() {
      assert_eq!(uint!(0).isqrt(), uint!(0));
      assert_eq!(uint!(1).isqrt(), uint!(1));
      assert_eq!(uint!(2).isqrt(), uint!(1));
      assert_eq!(uint!(99).isqrt(), uint!(9));
      assert_eq!(uint!(100).isqrt(), uint!(10));
      assert_eq!($type::MAX.isqrt(), (uint!(1) << ($type::BITS / 2)) - uint!(1));
    }

    #[cfg(not(miri))] // Miri is too slow
    #[test]
    fn test_lots_of_isqrt() {
      let n_max_sat: ::core::primitive::u128 = 2_u128.saturating_pow($type::BITS) - 1;
      let n_max: $type = $type::from_u128((1024 * 1024).min(n_max_sat));

      for n in uint!(0)..=n_max {
        let isqrt: $type = n.isqrt();

        assert!(isqrt.pow(2) <= n);
        assert!(isqrt + uint!(1) == uint!(1) << ($type::BITS / 2) || (isqrt + uint!(1)).pow(2) > n);
      }

      for n in ($type::MAX - uint!(255))..=$type::MAX {
        let isqrt: $type = n.isqrt();

        assert!(isqrt.pow(2) <= n);
        assert!(isqrt + uint!(1) == uint!(1) << ($type::BITS / 2) || (isqrt + uint!(1)).pow(2) > n);
      }
    }

    #[test]
    fn test_div_floor() {
      assert_eq!(uint!(8).div_floor(uint!(3)), uint!(2));
    }

    #[test]
    fn test_div_ceil() {
      assert_eq!(uint!(8).div_ceil(uint!(3)), uint!(3));
    }

    #[test]
    fn test_next_multiple_of() {
      assert_eq!(uint!(16).next_multiple_of(uint!(8)), uint!(16));
      assert_eq!(uint!(23).next_multiple_of(uint!(8)), uint!(24));
      assert_eq!($type::MAX.next_multiple_of(uint!(1)), $type::MAX);
    }

    #[test]
    fn test_checked_next_multiple_of() {
      assert_eq!(uint!(16).checked_next_multiple_of(uint!(8)), Some(uint!(16)));
      assert_eq!(uint!(23).checked_next_multiple_of(uint!(8)), Some(uint!(24)));
      assert_eq!(uint!(1).checked_next_multiple_of(uint!(0)), None);
      assert_eq!($type::MAX.checked_next_multiple_of(uint!(2)), None);
    }

    #[test]
    fn test_carrying_add() {
      assert_eq!($type::MAX.carrying_add(uint!(1), false), (uint!(0), true));
      assert_eq!($type::MAX.carrying_add(uint!(0), true), (uint!(0), true));
      assert_eq!($type::MAX.carrying_add(uint!(1), true), (uint!(1), true));

      assert_eq!($type::MIN.carrying_add($type::MAX, false), ($type::MAX, false));
      assert_eq!($type::MIN.carrying_add(uint!(0), true), (uint!(1), false));
      assert_eq!($type::MIN.carrying_add($type::MAX, true), (uint!(0), true));
    }

    #[test]
    fn test_borrowing_sub() {
      assert_eq!($type::MIN.borrowing_sub(uint!(1), false), ($type::MAX, true));
      assert_eq!($type::MIN.borrowing_sub(uint!(0), true), ($type::MAX, true));
      assert_eq!($type::MIN.borrowing_sub(uint!(1), true), ($type::MAX - uint!(1), true));

      assert_eq!($type::MAX.borrowing_sub($type::MAX, false), (uint!(0), false));
      assert_eq!($type::MAX.borrowing_sub(uint!(0), true), ($type::MAX - uint!(1), false));
      assert_eq!($type::MAX.borrowing_sub($type::MAX, true), ($type::MAX, true));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_widening_mul() {
      assert_eq!($type::MAX.widening_mul($type::MAX), (uint!(1), $type::MAX - uint!(1)));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_carrying_mul() {
      assert_eq!($type::MAX.carrying_mul($type::MAX, uint!(0)), (uint!(1), $type::MAX - uint!(1)));
      assert_eq!($type::MAX.carrying_mul($type::MAX, $type::MAX), (uint!(0), $type::MAX));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_carrying_mul_add() {
      assert_eq!($type::MAX.carrying_mul_add($type::MAX, uint!(0), uint!(0)), (uint!(1), $type::MAX - uint!(1)));
      assert_eq!($type::MAX.carrying_mul_add($type::MAX, $type::MAX, uint!(0)), (uint!(0), $type::MAX));
      assert_eq!($type::MAX.carrying_mul_add($type::MAX, $type::MAX, $type::MAX), ($type::MAX, $type::MAX));
    }

    #[test]
    fn test_midpoint() {
      assert_eq!($type::midpoint(uint!(1), uint!(3)), uint!(2));
      assert_eq!($type::midpoint(uint!(3), uint!(1)), uint!(2));

      assert_eq!($type::midpoint(uint!(0), uint!(0)), uint!(0));
      assert_eq!($type::midpoint(uint!(0), uint!(2)), uint!(1));
      assert_eq!($type::midpoint(uint!(2), uint!(0)), uint!(1));
      assert_eq!($type::midpoint(uint!(2), uint!(2)), uint!(2));

      assert_eq!($type::midpoint(uint!(1), uint!(4)), uint!(2));
      assert_eq!($type::midpoint(uint!(4), uint!(1)), uint!(2));
      assert_eq!($type::midpoint(uint!(3), uint!(4)), uint!(3));
      assert_eq!($type::midpoint(uint!(4), uint!(3)), uint!(3));

      assert_eq!($type::midpoint($type::MIN, $type::MAX), ($type::MAX - $type::MIN) / uint!(2));
      assert_eq!($type::midpoint($type::MAX, $type::MIN), ($type::MAX - $type::MIN) / uint!(2));
      assert_eq!($type::midpoint($type::MIN, $type::MIN), $type::MIN);
      assert_eq!($type::midpoint($type::MAX, $type::MAX), $type::MAX);

      assert_eq!($type::midpoint($type::MIN, uint!(6)), $type::MIN / uint!(2) + uint!(3));
      assert_eq!($type::midpoint(uint!(6), $type::MIN), $type::MIN / uint!(2) + uint!(3));
      assert_eq!($type::midpoint($type::MAX, uint!(6)), ($type::MAX - $type::MIN) / uint!(2) + uint!(3));
      assert_eq!($type::midpoint(uint!(6), $type::MAX), ($type::MAX - $type::MIN) / uint!(2) + uint!(3));
    }
  };
}
