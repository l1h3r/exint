use exint::primitive::*;

#[test]
fn test_clone() {
  assert_impls!(i8: Clone);
  assert_impls!(i16: Clone);
  assert_impls!(i24: Clone);
  assert_impls!(i32: Clone);
  assert_impls!(i40: Clone);
  assert_impls!(i48: Clone);
  assert_impls!(i56: Clone);
  assert_impls!(i64: Clone);
  assert_impls!(i72: Clone);
  assert_impls!(i80: Clone);
  assert_impls!(i88: Clone);
  assert_impls!(i96: Clone);
  assert_impls!(i104: Clone);
  assert_impls!(i112: Clone);
  assert_impls!(i120: Clone);
  assert_impls!(i128: Clone);
  assert_impls!(i256: Clone);
  assert_impls!(i512: Clone);
  assert_impls!(i1024: Clone);
  assert_impls!(i2048: Clone);
  assert_impls!(i4096: Clone);

  assert_impls!(u8: Clone);
  assert_impls!(u16: Clone);
  assert_impls!(u24: Clone);
  assert_impls!(u32: Clone);
  assert_impls!(u40: Clone);
  assert_impls!(u48: Clone);
  assert_impls!(u56: Clone);
  assert_impls!(u64: Clone);
  assert_impls!(u72: Clone);
  assert_impls!(u80: Clone);
  assert_impls!(u88: Clone);
  assert_impls!(u96: Clone);
  assert_impls!(u104: Clone);
  assert_impls!(u112: Clone);
  assert_impls!(u120: Clone);
  assert_impls!(u128: Clone);
  assert_impls!(u256: Clone);
  assert_impls!(u512: Clone);
  assert_impls!(u1024: Clone);
  assert_impls!(u2048: Clone);
  assert_impls!(u4096: Clone);
}
