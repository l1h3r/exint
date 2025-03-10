use exint::primitive::*;

#[test]
fn test_copy() {
  assert_impls!(i8: Copy);
  assert_impls!(i16: Copy);
  assert_impls!(i24: Copy);
  assert_impls!(i32: Copy);
  assert_impls!(i40: Copy);
  assert_impls!(i48: Copy);
  assert_impls!(i56: Copy);
  assert_impls!(i64: Copy);
  assert_impls!(i72: Copy);
  assert_impls!(i80: Copy);
  assert_impls!(i88: Copy);
  assert_impls!(i96: Copy);
  assert_impls!(i104: Copy);
  assert_impls!(i112: Copy);
  assert_impls!(i120: Copy);
  assert_impls!(i128: Copy);
  assert_impls!(i256: Copy);
  assert_impls!(i512: Copy);
  assert_impls!(i1024: Copy);
  assert_impls!(i2048: Copy);
  assert_impls!(i4096: Copy);

  assert_impls!(u8: Copy);
  assert_impls!(u16: Copy);
  assert_impls!(u24: Copy);
  assert_impls!(u32: Copy);
  assert_impls!(u40: Copy);
  assert_impls!(u48: Copy);
  assert_impls!(u56: Copy);
  assert_impls!(u64: Copy);
  assert_impls!(u72: Copy);
  assert_impls!(u80: Copy);
  assert_impls!(u88: Copy);
  assert_impls!(u96: Copy);
  assert_impls!(u104: Copy);
  assert_impls!(u112: Copy);
  assert_impls!(u120: Copy);
  assert_impls!(u128: Copy);
  assert_impls!(u256: Copy);
  assert_impls!(u512: Copy);
  assert_impls!(u1024: Copy);
  assert_impls!(u2048: Copy);
  assert_impls!(u4096: Copy);
}
