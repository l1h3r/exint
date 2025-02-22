//! Test constructor macros

use exint::int;
use exint::uint;

#[test]
fn test_macro_int() {
  assert_eq!(int!(-123 i8).to_string(), "-123");
  assert_eq!(int!(-123 i16).to_string(), "-123");
  assert_eq!(int!(-123 i24).to_string(), "-123");
  assert_eq!(int!(-123 i32).to_string(), "-123");
  assert_eq!(int!(-123 i40).to_string(), "-123");
  assert_eq!(int!(-123 i48).to_string(), "-123");
  assert_eq!(int!(-123 i56).to_string(), "-123");
  assert_eq!(int!(-123 i64).to_string(), "-123");
  assert_eq!(int!(-123 i72).to_string(), "-123");
  assert_eq!(int!(-123 i80).to_string(), "-123");
  assert_eq!(int!(-123 i88).to_string(), "-123");
  assert_eq!(int!(-123 i96).to_string(), "-123");
  assert_eq!(int!(-123 i104).to_string(), "-123");
  assert_eq!(int!(-123 i112).to_string(), "-123");
  assert_eq!(int!(-123 i120).to_string(), "-123");
  assert_eq!(int!(-123 i128).to_string(), "-123");
}

#[test]
fn test_macro_uint() {
  assert_eq!(uint!(123 u8).to_string(), "123");
  assert_eq!(uint!(123 u16).to_string(), "123");
  assert_eq!(uint!(123 u24).to_string(), "123");
  assert_eq!(uint!(123 u32).to_string(), "123");
  assert_eq!(uint!(123 u40).to_string(), "123");
  assert_eq!(uint!(123 u48).to_string(), "123");
  assert_eq!(uint!(123 u56).to_string(), "123");
  assert_eq!(uint!(123 u64).to_string(), "123");
  assert_eq!(uint!(123 u72).to_string(), "123");
  assert_eq!(uint!(123 u80).to_string(), "123");
  assert_eq!(uint!(123 u88).to_string(), "123");
  assert_eq!(uint!(123 u96).to_string(), "123");
  assert_eq!(uint!(123 u104).to_string(), "123");
  assert_eq!(uint!(123 u112).to_string(), "123");
  assert_eq!(uint!(123 u120).to_string(), "123");
  assert_eq!(uint!(123 u128).to_string(), "123");
}
