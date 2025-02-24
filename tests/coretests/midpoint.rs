// These tests are copied from rust
//
// https://github.com/rust-lang/rust/blob/bb2cc59a2172d6e35c89b409a4e6b5058d9039d7/library/coretests/tests/num/midpoint.rs

use exint::int;
use exint::primitive::i8;
use exint::primitive::u8;

#[test]
#[cfg(not(miri))]
fn midpoint_obvious_impl_i8() {
  for a in i8::MIN..=i8::MAX {
    for b in i8::MIN..=i8::MAX {
      assert_eq!(
        i8::midpoint(a, b),
        i8::from_i16((i16::from(a) + i16::from(b)) / 2)
      );
    }
  }
}

#[test]
#[cfg(not(miri))]
fn midpoint_obvious_impl_u8() {
  for a in u8::MIN..=u8::MAX {
    for b in u8::MIN..=u8::MAX {
      assert_eq!(
        u8::midpoint(a, b),
        u8::from_u16((u16::from(a) + u16::from(b)) / 2)
      );
    }
  }
}

#[test]
#[cfg(not(miri))]
fn midpoint_order_expectation_i8() {
  for a in i8::MIN..=i8::MAX {
    for b in i8::MIN..=i8::MAX {
      assert_eq!(i8::midpoint(a, b), i8::midpoint(b, a));
    }
  }
}

#[test]
#[cfg(not(miri))]
fn midpoint_order_expectation_u8() {
  for a in u8::MIN..=u8::MAX {
    for b in u8::MIN..=u8::MAX {
      assert_eq!(u8::midpoint(a, b), u8::midpoint(b, a));
    }
  }
}

#[test]
#[cfg(not(miri))]
fn midpoint_negative_expectation() {
  for a in int!(0 i8)..=i8::MAX {
    for b in int!(0 i8)..=i8::MAX {
      assert_eq!(i8::midpoint(-a, -b), -i8::midpoint(a, b));
    }
  }
}
