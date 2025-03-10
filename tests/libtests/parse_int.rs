macro_rules! test_int {
  ($type:ident) => {
    mod $type {
      use exint::primitive::$type;

      #[test]
      fn test_int() {
        assert_eq!(int!(0 $type), $type::from_i8(0));
        assert_eq!(int!(1 $type), $type::from_i8(1));
        assert_eq!(int!(127 $type), $type::from_i8(127));
        assert_eq!(int!(-128 $type), $type::from_i8(-128));
      }
    }
  };
}

macro_rules! test_uint {
  ($type:ident) => {
    mod $type {
      use exint::primitive::$type;

      #[test]
      fn test_uint() {
        assert_eq!(uint!(0 $type), $type::from_u8(0));
        assert_eq!(uint!(1 $type), $type::from_u8(1));
        assert_eq!(uint!(255 $type), $type::from_u8(255));
      }
    }
  };
}

test_int!(i8);
test_int!(i16);
test_int!(i24);
test_int!(i32);
test_int!(i40);
test_int!(i48);
test_int!(i56);
test_int!(i64);
test_int!(i72);
test_int!(i80);
test_int!(i88);
test_int!(i96);
test_int!(i104);
test_int!(i112);
test_int!(i120);
test_int!(i128);
test_int!(i256);
test_int!(i512);
test_int!(i1024);
test_int!(i2048);
test_int!(i4096);

test_uint!(u8);
test_uint!(u16);
test_uint!(u24);
test_uint!(u32);
test_uint!(u40);
test_uint!(u48);
test_uint!(u56);
test_uint!(u64);
test_uint!(u72);
test_uint!(u80);
test_uint!(u88);
test_uint!(u96);
test_uint!(u104);
test_uint!(u112);
test_uint!(u120);
test_uint!(u128);
test_uint!(u256);
test_uint!(u512);
test_uint!(u1024);
test_uint!(u2048);
test_uint!(u4096);
