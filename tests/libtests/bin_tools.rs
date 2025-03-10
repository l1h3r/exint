macro_rules! test {
  ($type:ident) => {
    mod $type {
      use core::ops::Not;
      use exint::primitive::$type;

      const A: $type = $type::from_u8(0b00101100);
      const B: $type = $type::from_u8(0b00100001);
      const C: $type = $type::from_u8(0b01111001);

      const X: $type = $type::from_u8(0);
      const Y: $type = $type::from_i8(-1);

      const DIFF: ::core::primitive::u32 = $type::BITS - u8::BITS;

      const LSB: $type = $type::from_u8(1);
      const MSB: $type = LSB.checked_shl($type::BITS - 1).unwrap();

      #[test]
      fn test_reverse_bits() {
        assert_eq!(A.reverse_bits().reverse_bits(), A);
        assert_eq!(B.reverse_bits().reverse_bits(), B);
        assert_eq!(C.reverse_bits().reverse_bits(), C);

        assert_eq!(X.reverse_bits(), X);
        assert_eq!(Y.reverse_bits(), Y);
      }

      #[test]
      fn test_swap_bytes() {
        assert_eq!(A.swap_bytes().swap_bytes(), A);
        assert_eq!(B.swap_bytes().swap_bytes(), B);
        assert_eq!(C.swap_bytes().swap_bytes(), C);

        assert_eq!(X.swap_bytes(), X);
        assert_eq!(Y.swap_bytes(), Y);
      }

      #[test]
      fn test_rotate_left() {
        assert_eq!(A.rotate_left(3).rotate_left(2).rotate_left($type::BITS - 5), A);
        assert_eq!(B.rotate_left(3).rotate_left(2).rotate_left($type::BITS - 5), B);
        assert_eq!(C.rotate_left(3).rotate_left(2).rotate_left($type::BITS - 5), C);

        for index in 0..$type::BITS {
          assert_eq!(A.rotate_left($type::BITS * index), A);
          assert_eq!(B.rotate_left($type::BITS * index), B);
          assert_eq!(C.rotate_left($type::BITS * index), C);
        }

        for index in 0..$type::BITS {
          assert_eq!(X.rotate_left(index), X);
          assert_eq!(Y.rotate_left(index), Y);
        }
      }

      #[test]
      fn test_rotate_right() {
        assert_eq!(A.rotate_right(3).rotate_right(2).rotate_right($type::BITS - 5), A);
        assert_eq!(B.rotate_right(3).rotate_right(2).rotate_right($type::BITS - 5), B);
        assert_eq!(C.rotate_right(3).rotate_right(2).rotate_right($type::BITS - 5), C);

        for index in 0..$type::BITS {
          assert_eq!(A.rotate_right($type::BITS * index), A);
          assert_eq!(B.rotate_right($type::BITS * index), B);
          assert_eq!(C.rotate_right($type::BITS * index), C);
        }

        for index in 0..$type::BITS {
          assert_eq!(X.rotate_right(index), X);
          assert_eq!(Y.rotate_right(index), Y);
        }
      }

      #[test]
      fn test_count_ones() {
        assert_eq!(A.count_ones(), 3);
        assert_eq!(B.count_ones(), 2);
        assert_eq!(C.count_ones(), 5);

        assert_eq!(A.not().count_ones(), $type::BITS - 3);
        assert_eq!(B.not().count_ones(), $type::BITS - 2);
        assert_eq!(C.not().count_ones(), $type::BITS - 5);

        assert_eq!(X.count_ones(), 0);
        assert_eq!(Y.count_ones(), $type::BITS);
      }

      #[test]
      fn test_count_zeros() {
        assert_eq!(A.count_zeros(), $type::BITS - 3);
        assert_eq!(B.count_zeros(), $type::BITS - 2);
        assert_eq!(C.count_zeros(), $type::BITS - 5);

        assert_eq!(A.not().count_zeros(), 3);
        assert_eq!(B.not().count_zeros(), 2);
        assert_eq!(C.not().count_zeros(), 5);

        assert_eq!(X.count_zeros(),  $type::BITS);
        assert_eq!(Y.count_zeros(), 0);
      }

      #[test]
      fn test_leading_ones() {
        assert_eq!(A.leading_ones(), 0);
        assert_eq!(B.leading_ones(), 0);
        assert_eq!(C.leading_ones(), 0);

        assert_eq!(A.not().leading_ones(), $type::BITS - 6);
        assert_eq!(B.not().leading_ones(), $type::BITS - 6);
        assert_eq!(C.not().leading_ones(), $type::BITS - 7);

        assert_eq!(X.leading_ones(), 0);
        assert_eq!(Y.leading_ones(), $type::BITS);
      }

      #[test]
      fn test_leading_zeros() {
        assert_eq!(A.leading_zeros(), DIFF + 2);
        assert_eq!(B.leading_zeros(), DIFF + 2);
        assert_eq!(C.leading_zeros(), DIFF + 1);

        assert_eq!(A.not().leading_zeros(), 0);
        assert_eq!(B.not().leading_zeros(), 0);
        assert_eq!(C.not().leading_zeros(), 0);

        assert_eq!(X.leading_zeros(), $type::BITS);
        assert_eq!(Y.leading_zeros(), 0);
      }

      #[test]
      fn test_trailing_ones() {
        assert_eq!(A.trailing_ones(), 0);
        assert_eq!(B.trailing_ones(), 1);
        assert_eq!(C.trailing_ones(), 1);

        assert_eq!(A.not().trailing_ones(), 2);
        assert_eq!(B.not().trailing_ones(), 0);
        assert_eq!(C.not().trailing_ones(), 0);

        assert_eq!(X.trailing_ones(), 0);
        assert_eq!(Y.trailing_ones(), $type::BITS);
      }

      #[test]
      fn test_trailing_zeros() {
        assert_eq!(A.trailing_zeros(), 2);
        assert_eq!(B.trailing_zeros(), 0);
        assert_eq!(C.trailing_zeros(), 0);

        assert_eq!(A.not().trailing_zeros(), 0);
        assert_eq!(B.not().trailing_zeros(), 1);
        assert_eq!(C.not().trailing_zeros(), 1);

        assert_eq!(X.trailing_zeros(), $type::BITS);
        assert_eq!(Y.trailing_zeros(), 0);
      }

      #[test]
      fn test_isolate_most_significant_one() {
        for index in 0..$type::BITS {
          let this: $type = Y.checked_shr(index).unwrap();
          let that: $type = MSB.checked_shr(index).unwrap();

          assert_eq!(
            this.isolate_most_significant_one(),
            that.isolate_most_significant_one(),
          );
        }
      }

      #[test]
      fn test_isolate_least_significant_one() {
        for index in 0..$type::BITS {
          let this: $type = Y.checked_shl(index).unwrap();
          let that: $type = LSB.checked_shl(index).unwrap();

          assert_eq!(
            this.isolate_least_significant_one(),
            that.isolate_least_significant_one(),
          );
        }
      }
    }
  };
}

test!(i8);
test!(i16);
test!(i24);
test!(i32);
test!(i40);
test!(i48);
test!(i56);
test!(i64);
test!(i72);
test!(i80);
test!(i88);
test!(i96);
test!(i104);
test!(i112);
test!(i120);
test!(i128);
test!(i256);
test!(i512);
test!(i1024);
test!(i2048);
test!(i4096);

test!(u8);
test!(u16);
test!(u24);
test!(u32);
test!(u40);
test!(u48);
test!(u56);
test!(u64);
test!(u72);
test!(u80);
test!(u88);
test!(u96);
test!(u104);
test!(u112);
test!(u120);
test!(u128);
test!(u256);
test!(u512);
test!(u1024);
test!(u2048);
test!(u4096);
