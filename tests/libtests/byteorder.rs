macro_rules! test {
  ($type:ident) => {
    mod $type {
      use exint::primitive::$type;

      const A: $type = $type::from_u8(0b00101100);
      const B: $type = $type::from_u8(0b00100001);
      const C: $type = $type::from_u8(0b01111001);

      const X: $type = $type::from_u8(0);
      const Y: $type = $type::from_i8(-1);

      #[test]
      fn test_be() {
        assert_eq!($type::from_be(A.to_be()), A);
        assert_eq!($type::from_be(B.to_be()), B);
        assert_eq!($type::from_be(C.to_be()), C);
        assert_eq!($type::from_be(X.to_be()), X);
        assert_eq!($type::from_be(Y.to_be()), Y);

        assert_eq!($type::from_be_bytes(A.to_be_bytes()), A);
        assert_eq!($type::from_be_bytes(B.to_be_bytes()), B);
        assert_eq!($type::from_be_bytes(C.to_be_bytes()), C);
        assert_eq!($type::from_be_bytes(X.to_ne_bytes()), X);
        assert_eq!($type::from_be_bytes(Y.to_ne_bytes()), Y);

        assert_eq!(X.to_be(), X);
        assert_eq!(Y.to_be(), Y);
      }

      #[test]
      fn test_le() {
        assert_eq!($type::from_le(A.to_le()), A);
        assert_eq!($type::from_le(B.to_le()), B);
        assert_eq!($type::from_le(C.to_le()), C);
        assert_eq!($type::from_le(X.to_le()), X);
        assert_eq!($type::from_le(Y.to_le()), Y);

        assert_eq!($type::from_le_bytes(A.to_le_bytes()), A);
        assert_eq!($type::from_le_bytes(B.to_le_bytes()), B);
        assert_eq!($type::from_le_bytes(C.to_le_bytes()), C);
        assert_eq!($type::from_le_bytes(X.to_ne_bytes()), X);
        assert_eq!($type::from_le_bytes(Y.to_ne_bytes()), Y);

        assert_eq!(X.to_le(), X);
        assert_eq!(Y.to_le(), Y);
      }

      #[test]
      fn test_ne() {
        assert_eq!($type::from_ne_bytes(A.to_ne_bytes()), A);
        assert_eq!($type::from_ne_bytes(B.to_ne_bytes()), B);
        assert_eq!($type::from_ne_bytes(C.to_ne_bytes()), C);
        assert_eq!($type::from_ne_bytes(X.to_ne_bytes()), X);
        assert_eq!($type::from_ne_bytes(Y.to_ne_bytes()), Y);
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
