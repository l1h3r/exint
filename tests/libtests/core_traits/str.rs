macro_rules! test {
  ($type:ident) => {
    mod $type {
      use core::str::FromStr;
      use exint::IntErrorKind;
      use exint::ParseIntError;
      use exint::primitive::$type;

      #[test]
      fn test_from_str() {
        assert_eq!($type::from_str("0"), Ok($type::from_u8(0)));
        assert_eq!($type::from_str("1"), Ok($type::from_u8(1)));
        assert_eq!($type::from_str("10"), Ok($type::from_u8(10)));
        assert_eq!($type::from_str("100"), Ok($type::from_u8(100)));

        assert_eq!($type::from_str("00000"), Ok($type::from_u8(0)));
        assert_eq!($type::from_str("00001"), Ok($type::from_u8(1)));
        assert_eq!($type::from_str("000010"), Ok($type::from_u8(10)));
        assert_eq!($type::from_str("0000100"), Ok($type::from_u8(100)));

        assert_eq!($type::from_str("+0"), Ok($type::from_u8(0)));
        assert_eq!($type::from_str("+1"), Ok($type::from_u8(1)));
        assert_eq!($type::from_str("+10"), Ok($type::from_u8(10)));
        assert_eq!($type::from_str("+100"), Ok($type::from_u8(100)));

        if !<$type as exint::backend::Uint>::UINT {
          assert_eq!($type::from_str("-0"), Ok($type::from_i8(-0)));
          assert_eq!($type::from_str("-1"), Ok($type::from_i8(-1)));
          assert_eq!($type::from_str("-10"), Ok($type::from_i8(-10)));
          assert_eq!($type::from_str("-100"), Ok($type::from_i8(-100)));
        }

        assert_eq!($type::from_str(&$type::MIN.to_string()), Ok($type::MIN));
        assert_eq!($type::from_str(&$type::MAX.to_string()), Ok($type::MAX));

        for (value, error) in $crate::BAD_STRINGS {
          let value: Result<$type, ParseIntError> = $type::from_str(value);
          let ekind: Result<&$type, &IntErrorKind> = value.as_ref().map_err(ParseIntError::kind);

          assert_eq!(ekind, Err(error));
        }

        assert_matches!(
          $type::from_str(&format!("{}0", $type::MAX)),
          Err(error) if error.kind() == &IntErrorKind::PosOverflow,
        );

        if !<$type as exint::backend::Uint>::UINT {
          assert_matches!(
            $type::from_str(&format!("{}0", $type::MIN)),
            Err(error) if error.kind() == &IntErrorKind::NegOverflow,
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
