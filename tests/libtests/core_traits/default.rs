macro_rules! test {
  ($type:ident) => {
    mod $type {
      use exint::primitive::$type;

      #[test]
      fn test_default() {
        assert_eq!($type::default(), $type::from_u8(0));
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
