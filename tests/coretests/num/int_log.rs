// These tests are copied from rust
//
// https://github.com/rust-lang/rust/blob/bb2cc59a2172d6e35c89b409a4e6b5058d9039d7/library/coretests/tests/num/int_log.rs

use exint::int;
use exint::uint;

#[test]
fn checked_ilog() {
  assert_eq!(uint!(999).checked_ilog(uint!(10)), Some(2));
  assert_eq!(uint!(1000).checked_ilog(uint!(10)), Some(3));
  assert_eq!(uint!(555).checked_ilog(uint!(13)), Some(2));
  assert_eq!(uint!(63).checked_ilog(uint!(4)), Some(2));
  assert_eq!(uint!(64).checked_ilog(uint!(4)), Some(3));
  assert_eq!(uint!(10460353203 u64).checked_ilog(uint!(3 u64)), Some(21));
  assert_eq!(uint!(10460353202 u64).checked_ilog(uint!(3 u64)), Some(20));
  assert_eq!(uint!(147808829414345923316083210206383297601 u128).checked_ilog(uint!(3 u128)), Some(80));
  assert_eq!(uint!(147808829414345923316083210206383297600 u128).checked_ilog(uint!(3 u128)), Some(79));
  assert_eq!(uint!(22528399544939174411840147874772641 u128).checked_ilog(uint!(19683 u128)), Some(8));
  assert_eq!(int!(22528399544939174411840147874772631 i128).checked_ilog(int!(19683 i128)), Some(7));

  assert_eq!(uint!(0 u8).checked_ilog(uint!(4 u8)), None);
  assert_eq!(uint!(0 u16).checked_ilog(uint!(4 u16)), None);
  assert_eq!(int!(0 i8).checked_ilog(int!(4 i8)), None);
  assert_eq!(int!(0 i16).checked_ilog(int!(4 i16)), None);

  #[cfg(not(miri))] // Miri is too slow
  for value in int::<2>::MIN..=int!(0 i16) {
    assert_eq!(value.checked_ilog(int!(4 i16)), None, "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in int!(1 i16)..=int::<2>::MAX {
    assert_eq!(value.checked_ilog(int!(13 i16)), Some(f32::from(value).log(13.0) as u32), "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in uint!(1 u16)..=uint::<2>::MAX {
    assert_eq!(value.checked_ilog(uint!(13 u16)), Some(f32::from(value).log(13.0) as u32), "checking {value}");
  }
}

#[test]
fn checked_ilog2() {
  assert_eq!(uint!(5).checked_ilog2(), Some(2));
  assert_eq!(uint!(0 u64).checked_ilog2(), None);
  assert_eq!(int!(128).checked_ilog2(), Some(7));
  assert_eq!(int!(-55 i16).checked_ilog2(), None);

  assert_eq!(uint!(0 u8).checked_ilog2(), None);
  assert_eq!(uint!(0 u16).checked_ilog2(), None);
  assert_eq!(int!(0 i8).checked_ilog2(), None);
  assert_eq!(int!(0 i16).checked_ilog2(), None);

  assert_eq!(uint!(8192 u16).checked_ilog2(), Some((8192f32).log2() as u32));
  assert_eq!(uint!(32768 u16).checked_ilog2(), Some((32768f32).log2() as u32));
  assert_eq!(int!(8192 i16).checked_ilog2(), Some((8192f32).log2() as u32));

  for value in uint!(1 u8)..=uint::<1>::MAX {
    assert_eq!(value.checked_ilog2(), Some(f32::from(value).log2() as u32), "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in uint!(1 u16)..=uint::<2>::MAX {
    assert_eq!(value.checked_ilog2(), Some(f32::from(value).log2() as u32), "checking {value}");
  }

  for value in int::<1>::MIN..=int!(0 i8) {
    assert_eq!(value.checked_ilog2(), None, "checking {value}");
  }

  for value in int!(1 i8)..=int::<1>::MAX {
    assert_eq!(value.checked_ilog2(), Some(f32::from(value).log2() as u32), "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in int::<2>::MIN..=int!(0 i16) {
    assert_eq!(value.checked_ilog2(), None, "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in int!(1 i16)..=int::<2>::MAX {
    assert_eq!(value.checked_ilog2(), Some(f32::from(value).log2() as u32), "checking {value}");
  }
}

#[test]
fn checked_ilog10() {
  assert_eq!(uint!(0 u8).checked_ilog10(), None);
  assert_eq!(uint!(0 u16).checked_ilog10(), None);

  assert_eq!(int!(0 i8).checked_ilog10(), None);
  assert_eq!(int!(0 i16).checked_ilog10(), None);

  #[cfg(not(miri))] // Miri is too slow
  for value in int::<2>::MIN..=int!(0 i16) {
    assert_eq!(value.checked_ilog10(), None, "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in int!(1 i16)..=int::<2>::MAX {
    assert_eq!(value.checked_ilog10(), Some(f32::from(value).log10() as u32), "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in uint!(1 u16)..=uint::<2>::MAX {
    assert_eq!(value.checked_ilog10(), Some(f32::from(value).log10() as u32), "checking {value}");
  }

  #[cfg(not(miri))] // Miri is too slow
  for value in uint!(1)..=uint!(100_000) {
    assert_eq!(value.checked_ilog10(), Some((value.into_u32() as f32).log10() as u32), "checking {value}");
  }
}

macro_rules! ilog10_loop {
  ($type:ty, $size:ident, $max:expr) => {
    assert_eq!(<$type>::MAX.ilog10(), $max);

    for value in 0..=$max {
      let p = uint!(10 $size).pow(value);

      if p >= uint!(10 $size) {
        assert_eq!((p - uint!(9 $size)).ilog10(), value - 1);
        assert_eq!((p - uint!(1 $size)).ilog10(), value - 1);
      }

      assert_eq!(p.ilog10(), value);
      assert_eq!((p + uint!(1 $size)).ilog10(), value);

      if p >= uint!(10 $size) {
        assert_eq!((p + uint!(9 $size)).ilog10(), value);
      }

      // also check `x.ilog(10)`
      if p >= uint!(10 $size) {
        assert_eq!((p - uint!(9 $size)).ilog(uint!(10 $size)), value - 1);
        assert_eq!((p - uint!(1 $size)).ilog(uint!(10 $size)), value - 1);
      }

      assert_eq!(p.ilog(uint!(10 $size)), value);
      assert_eq!((p + uint!(1 $size)).ilog(uint!(10 $size)), value);

      if p >= uint!(10 $size) {
        assert_eq!((p + uint!(9 $size)).ilog(uint!(10 $size)), value);
      }
    }
  };
}

#[test]
fn ilog10_u8() {
  ilog10_loop!(uint<1>, u8, 2);
}

#[test]
fn ilog10_u16() {
  ilog10_loop!(uint<2>, u16, 4);
}

#[test]
fn ilog10_u32() {
  ilog10_loop!(uint<4>, u32, 9);
}

#[test]
fn ilog10_u64() {
  ilog10_loop!(uint<8>, u64, 19);
}

#[test]
fn ilog10_u128() {
  ilog10_loop!(uint<16>, u128, 38);
}

#[test]
#[should_panic(expected = "argument of integer logarithm must be positive")]
fn ilog2_of_0_panic() {
  let _ = uint!(0).ilog2();
}

#[test]
#[should_panic(expected = "argument of integer logarithm must be positive")]
fn ilog10_of_0_panic() {
  let _ = uint!(0).ilog10();
}

#[test]
#[should_panic(expected = "argument of integer logarithm must be positive")]
fn ilog3_of_0_panic() {
  let _ = uint!(0).ilog(uint!(3));
}

#[test]
#[should_panic(expected = "base of integer logarithm must be at least 2")]
fn ilog0_of_1_panic() {
  let _ = uint!(1).ilog(uint!(0));
}

#[test]
#[should_panic(expected = "base of integer logarithm must be at least 2")]
fn ilog1_of_1_panic() {
  let _ = uint!(1).ilog(uint!(1));
}
