macro_rules! test {
  ($type:ident) => {
    mod $type {
      use exint::primitive::$type;

      const I: [$type; 5] = [
        $type::from_u8(1),
        $type::from_u8(2),
        $type::from_u8(3),
        $type::from_u8(4),
        $type::from_u8(5),
      ];

      #[test]
      fn test_sum() {
        assert_eq!($type::from_u8(15), I[0..].iter().sum());
        assert_eq!($type::from_u8(14), I[1..].iter().sum());
        assert_eq!($type::from_u8(12), I[2..].iter().sum());
        assert_eq!($type::from_u8(9), I[3..].iter().sum());
        assert_eq!($type::from_u8(5), I[4..].iter().sum());
        assert_eq!($type::from_u8(0), I[5..].iter().sum());
      }

      #[test]
      fn test_product() {
        assert_eq!($type::from_u8(120), I[0..].iter().product());
        assert_eq!($type::from_u8(120), I[1..].iter().product());
        assert_eq!($type::from_u8(60), I[2..].iter().product());
        assert_eq!($type::from_u8(20), I[3..].iter().product());
        assert_eq!($type::from_u8(5), I[4..].iter().product());
        assert_eq!($type::from_u8(1), I[5..].iter().product());
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
