macro_rules! test_int {
  ($type:ident, $bits:literal) => {
    mod $type {
      use exint::primitive::$type;

      const ONE: $type = $type::from_u8(1);
      const TWO: $type = $type::from_u8(2);

      #[test]
      fn test_constants() {
        assert_eq!($type::BITS, $bits);
        assert_eq!($type::MAX, TWO.wrapping_pow($bits - 1).wrapping_sub(ONE));
        assert_eq!($type::MIN, TWO.wrapping_pow($bits - 1));
      }
    }
  };
}

macro_rules! test_uint {
  ($type:ident, $bits:literal) => {
    mod $type {
      use exint::primitive::$type;

      const ONE: $type = $type::from_u8(1);
      const TWO: $type = $type::from_u8(2);

      #[test]
      fn test_constants() {
        assert_eq!($type::BITS, $bits);
        assert_eq!($type::MAX, TWO.wrapping_pow($bits).wrapping_sub(ONE));
        assert_eq!($type::MIN, $type::from_u8(0));
      }
    }
  };
}

test_int!(i8, 8);
test_int!(i16, 16);
test_int!(i24, 24);
test_int!(i32, 32);
test_int!(i40, 40);
test_int!(i48, 48);
test_int!(i56, 56);
test_int!(i64, 64);
test_int!(i72, 72);
test_int!(i80, 80);
test_int!(i88, 88);
test_int!(i96, 96);
test_int!(i104, 104);
test_int!(i112, 112);
test_int!(i120, 120);
test_int!(i128, 128);
test_int!(i256, 256);
test_int!(i512, 512);
test_int!(i1024, 1024);
test_int!(i2048, 2048);
test_int!(i4096, 4096);

test_uint!(u8, 8);
test_uint!(u16, 16);
test_uint!(u24, 24);
test_uint!(u32, 32);
test_uint!(u40, 40);
test_uint!(u48, 48);
test_uint!(u56, 56);
test_uint!(u64, 64);
test_uint!(u72, 72);
test_uint!(u80, 80);
test_uint!(u88, 88);
test_uint!(u96, 96);
test_uint!(u104, 104);
test_uint!(u112, 112);
test_uint!(u120, 120);
test_uint!(u128, 128);
test_uint!(u256, 256);
test_uint!(u512, 512);
test_uint!(u1024, 1024);
test_uint!(u2048, 2048);
test_uint!(u4096, 4096);
