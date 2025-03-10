macro_rules! test {
  ($type:ident) => {
    mod $type {
      use exint::IntErrorKind;
      use exint::ParseIntError;
      use exint::primitive::$type;

      #[test]
      #[should_panic = "from_ascii_radix: radix must lie in the range `[2, 36]`"]
      fn test_from_str_radix_invalid_radix_0() {
        let _ = $type::from_str_radix("", 0);
      }

      #[test]
      #[should_panic = "from_ascii_radix: radix must lie in the range `[2, 36]`"]
      fn test_from_str_radix_invalid_radix_1() {
        let _ = $type::from_str_radix("", 1);
      }

      #[test]
      #[should_panic = "from_ascii_radix: radix must lie in the range `[2, 36]`"]
      fn test_from_str_radix_invalid_radix_37() {
        let _ = $type::from_str_radix("", 37);
      }

      #[test]
      fn test_from_str_radix() {
        for radix in 2..=36 {
          for value in 0..radix.min(10) {
            let data: String = format!("{value}");
            let this: Result<$type, _> = $type::from_str_radix(&data, radix);
            let that: $type = $type::from_u32(value);

            assert_eq!(this, Ok(that));
          }
        }
      }

      #[test]
      fn test_from_ascii() {
        assert_eq!($type::from_ascii(b"0"), $type::from_ascii_radix(b"0", 10));
        assert_eq!($type::from_ascii(b"1"), $type::from_ascii_radix(b"1", 10));
        assert_eq!($type::from_ascii(b"12"), $type::from_ascii_radix(b"12", 10));
        assert_eq!($type::from_ascii(b"123"), $type::from_ascii_radix(b"123", 10));
      }

      #[test]
      fn test_from_ascii_radix() {
        assert_eq!($type::from_ascii_radix(b"0", 2), Ok($type::from_u8(0b0)));
        assert_eq!($type::from_ascii_radix(b"1", 2), Ok($type::from_u8(0b1)));

        assert_eq!($type::from_ascii_radix(b"0", 7), Ok($type::from_u8(0o0)));
        assert_eq!($type::from_ascii_radix(b"1", 7), Ok($type::from_u8(0o1)));
        assert_eq!($type::from_ascii_radix(b"2", 7), Ok($type::from_u8(0o2)));
        assert_eq!($type::from_ascii_radix(b"3", 7), Ok($type::from_u8(0o3)));
        assert_eq!($type::from_ascii_radix(b"4", 7), Ok($type::from_u8(0o4)));
        assert_eq!($type::from_ascii_radix(b"5", 7), Ok($type::from_u8(0o5)));
        assert_eq!($type::from_ascii_radix(b"6", 7), Ok($type::from_u8(0o6)));

        assert_eq!($type::from_ascii_radix(b"A", 16), Ok($type::from_u8(0xA)));
        assert_eq!($type::from_ascii_radix(b"B", 16), Ok($type::from_u8(0xB)));
        assert_eq!($type::from_ascii_radix(b"C", 16), Ok($type::from_u8(0xC)));
        assert_eq!($type::from_ascii_radix(b"D", 16), Ok($type::from_u8(0xD)));
        assert_eq!($type::from_ascii_radix(b"E", 16), Ok($type::from_u8(0xE)));
        assert_eq!($type::from_ascii_radix(b"F", 16), Ok($type::from_u8(0xF)));

        for (value, error) in $crate::BAD_STRINGS {
          let value: Result<$type, ParseIntError> = $type::from_ascii_radix(value.as_bytes(), 10);
          let ekind: Result<&$type, &IntErrorKind> = value.as_ref().map_err(ParseIntError::kind);

          assert_eq!(ekind, Err(error));
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
