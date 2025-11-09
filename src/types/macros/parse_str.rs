macro_rules! parse_str {
  (int) => {
    $crate::types::macros::parse_str!(@core, int);

    stability! {
      #[unstable(feature = "int_from_ascii")]
      #[allow(rustdoc::private_doc_tests, reason = "feature-dependant")]
      #[doc = include_doc!(int, "int_from_ascii")]
      #[inline]
      pub const fn from_ascii_radix(
        src: &[u8],
        radix: u32,
      ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
        Self::__from_ascii_radix::<true>(src, radix)
      }
    }
  };
  (uint) => {
    $crate::types::macros::parse_str!(@core, uint);

    stability! {
      #[unstable(feature = "int_from_ascii")]
      #[allow(rustdoc::private_doc_tests, reason = "feature-dependant")]
      #[doc = include_doc!(uint, "int_from_ascii")]
      #[inline]
      pub const fn from_ascii_radix(
        src: &[u8],
        radix: u32,
      ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
        Self::__from_ascii_radix::<false>(src, radix)
      }
    }
  };
  (@core, $name:ident) => {
    #[doc = include_doc!($name, "from_str_radix")]
    #[inline]
    pub const fn from_str_radix(
      src: &str,
      radix: u32,
    ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
      Self::from_ascii_radix(src.as_bytes(), radix)
    }

    #[doc = include_doc!($name, "from_ascii")]
    #[cfg(feature = "int_from_ascii")]
    #[inline]
    pub const fn from_ascii(
      src: &[u8],
    ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
      Self::from_ascii_radix(src, 10)
    }

    #[inline]
    pub(crate) const fn __from_ascii_radix<const SIGNED: bool>(
      src: &[u8],
      radix: u32,
    ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
      #[inline]
      const fn no_overflow<T, const SIGNED: bool>(radix: u32, bytes: &[u8]) -> bool {
        radix <= 16 && bytes.len() <= ::core::mem::size_of::<T>() * 2 - SIGNED as usize
      }

      if 2 > radix || radix > 36 {
        $crate::panic::from_ascii_radix(radix)
      }

      macro_rules! error {
        ($kind:ident) => {
          ::core::result::Result::Err(
            $crate::error::ParseIntError::new($crate::error::IntErrorKind::$kind)
          )
        };
      }

      macro_rules! tri {
        ($expr:expr, $error:ident) => {
          match $expr {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => return error!($error),
          }
        };
      }

      if src.is_empty() {
        return error!(Empty);
      }

      let (positive, mut digits): (bool, &[u8]) = match src {
        [b'+' | b'-'] => return error!(InvalidDigit),
        [b'+', tail @ ..] => (true, tail),
        [b'-', tail @ ..] if SIGNED => (false, tail),
        _ => (true, src),
      };

      let mut result: Self = Self::ZERO;

      if no_overflow::<Self, SIGNED>(radix, digits) {
        macro_rules! unchecked_loop {
          ($op:ident) => {{
            while let [digit, tail @ ..] = digits {
              let digit: u32 = tri!((*digit as char).to_digit(radix), InvalidDigit);
              result = result.const_mul(Self::from_u32(radix));
              result = Self::$op(result, Self::from_u32(digit));
              digits = tail;
            }
          }};
        }

        if positive {
          unchecked_loop!(const_add)
        } else {
          unchecked_loop!(const_sub)
        }
      } else {
        macro_rules! checked_loop {
          ($op:ident, $error:ident) => {{
            while let [digit, tail @ ..] = digits {
              let multi: ::core::option::Option<Self> = result.checked_mul(Self::from_u32(radix));
              let digit: u32 = tri!((*digit as char).to_digit(radix), InvalidDigit);
              result = tri!(multi, $error);
              result = tri!(Self::$op(result, Self::from_u32(digit)), $error);
              digits = tail;
            }
          }};
        }

        if positive {
          checked_loop!(checked_add, PosOverflow);
        } else {
          checked_loop!(checked_sub, NegOverflow);
        }
      }

      ::core::result::Result::Ok(result)
    }
  };
}

pub(crate) use parse_str;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::IntErrorKind;
  use crate::ParseIntError;
  use crate::primitive::u8;
  use crate::tests::*;

  macro_rules! assert_from_str_err {
    ($name:ident, $pattern:expr, $error:ident) => {{
      let value: Result<$name, ParseIntError> = $name::from_ascii_radix($pattern.as_bytes(), 10);
      let ekind: Result<&$name, &IntErrorKind> = value.as_ref().map_err(ParseIntError::kind);

      assert_eq!(ekind, Err(&IntErrorKind::$error));
    }};
  }

  const ERROR: ParseIntError = ParseIntError::new(IntErrorKind::InvalidDigit);

  test!(test_from_str_radix, () => {
    // Forwards to from_ascii_radix
  });

  test!(test_from_ascii, () => {
    // Forwards to from_ascii_radix
  });

  #[test]
  #[should_panic = "from_ascii_radix: radix must lie in the range `[2, 36]`"]
  fn test_from_ascii_radix_0() {
    let _ = u8::from_ascii_radix(b"", 0);
  }

  #[test]
  #[should_panic = "from_ascii_radix: radix must lie in the range `[2, 36]`"]
  fn test_from_ascii_radix_1() {
    let _ = u8::from_ascii_radix(b"", 1);
  }

  #[test]
  #[should_panic = "from_ascii_radix: radix must lie in the range `[2, 36]`"]
  fn test_from_ascii_radix_37() {
    let _ = u8::from_ascii_radix(b"", 37);
  }

  test!(test_from_ascii_radix_2, () => {
    assert_eq!(T::from_ascii_radix(b"0", 2), Ok(T::P_0));
    assert_eq!(T::from_ascii_radix(b"1", 2), Ok(T::P_1));
    assert_eq!(T::from_ascii_radix(b"2", 2), Err(ERROR));
  });

  test!(test_from_ascii_radix_7, () => {
    assert_eq!(T::from_ascii_radix(b"0", 7), Ok(T::P_0));
    assert_eq!(T::from_ascii_radix(b"1", 7), Ok(T::P_1));
    assert_eq!(T::from_ascii_radix(b"2", 7), Ok(T::P_2));
    assert_eq!(T::from_ascii_radix(b"3", 7), Ok(T::P_3));
    assert_eq!(T::from_ascii_radix(b"4", 7), Ok(T::P_4));
    assert_eq!(T::from_ascii_radix(b"5", 7), Ok(T::P_5));
    assert_eq!(T::from_ascii_radix(b"6", 7), Ok(T::P_6));
    assert_eq!(T::from_ascii_radix(b"7", 7), Err(ERROR));
  });

  test!(test_from_ascii_radix_10, () => {
    assert_eq!(T::from_ascii_radix(b"0", 10), Ok(T::P_0));
    assert_eq!(T::from_ascii_radix(b"1", 10), Ok(T::P_1));
    assert_eq!(T::from_ascii_radix(b"2", 10), Ok(T::P_2));
    assert_eq!(T::from_ascii_radix(b"3", 10), Ok(T::P_3));
    assert_eq!(T::from_ascii_radix(b"4", 10), Ok(T::P_4));
    assert_eq!(T::from_ascii_radix(b"5", 10), Ok(T::P_5));
    assert_eq!(T::from_ascii_radix(b"6", 10), Ok(T::P_6));
    assert_eq!(T::from_ascii_radix(b"7", 10), Ok(T::P_7));
    assert_eq!(T::from_ascii_radix(b"8", 10), Ok(T::P_8));
    assert_eq!(T::from_ascii_radix(b"9", 10), Ok(T::P_9));
    assert_eq!(T::from_ascii_radix(b"A", 10), Err(ERROR));
  });

  test!(test_from_ascii_radix_16, () => {
    assert_eq!(T::from_ascii_radix(b"0", 16), Ok(T::P_0));
    assert_eq!(T::from_ascii_radix(b"1", 16), Ok(T::P_1));
    assert_eq!(T::from_ascii_radix(b"2", 16), Ok(T::P_2));
    assert_eq!(T::from_ascii_radix(b"3", 16), Ok(T::P_3));
    assert_eq!(T::from_ascii_radix(b"4", 16), Ok(T::P_4));
    assert_eq!(T::from_ascii_radix(b"5", 16), Ok(T::P_5));
    assert_eq!(T::from_ascii_radix(b"6", 16), Ok(T::P_6));
    assert_eq!(T::from_ascii_radix(b"7", 16), Ok(T::P_7));
    assert_eq!(T::from_ascii_radix(b"8", 16), Ok(T::P_8));
    assert_eq!(T::from_ascii_radix(b"9", 16), Ok(T::P_9));
    assert_eq!(T::from_ascii_radix(b"A", 16), Ok(T::P_10));
    assert_eq!(T::from_ascii_radix(b"B", 16), Ok(T::P_11));
    assert_eq!(T::from_ascii_radix(b"C", 16), Ok(T::P_12));
    assert_eq!(T::from_ascii_radix(b"D", 16), Ok(T::P_13));
    assert_eq!(T::from_ascii_radix(b"E", 16), Ok(T::P_14));
    assert_eq!(T::from_ascii_radix(b"F", 16), Ok(T::P_15));
    assert_eq!(T::from_ascii_radix(b"G", 16), Err(ERROR));
  });

  test!(test_from_ascii_radix_min_max, () => {
    assert_eq!(T::from_ascii_radix(T::MIN_STR.as_bytes(), 10), Ok(T::MIN));
    assert_eq!(T::from_ascii_radix(T::MAX_STR.as_bytes(), 10), Ok(T::MAX));
  });

  test!(test_from_ascii_radix_positive, () => {
    assert_eq!(T::from_ascii_radix(b"10", 10), Ok(T::P_10));
    assert_eq!(T::from_ascii_radix(b"11", 10), Ok(T::P_11));
    assert_eq!(T::from_ascii_radix(b"20", 10), Ok(T::P_20));
    assert_eq!(T::from_ascii_radix(b"97", 10), Ok(T::P_97));

    assert_eq!(T::from_ascii_radix(b"+10", 10), Ok(T::P_10));
    assert_eq!(T::from_ascii_radix(b"+11", 10), Ok(T::P_11));
    assert_eq!(T::from_ascii_radix(b"+20", 10), Ok(T::P_20));
    assert_eq!(T::from_ascii_radix(b"+97", 10), Ok(T::P_97));

    assert_eq!(T::from_ascii_radix(b"000010", 10), Ok(T::P_10));
    assert_eq!(T::from_ascii_radix(b"000011", 10), Ok(T::P_11));
    assert_eq!(T::from_ascii_radix(b"000020", 10), Ok(T::P_20));
    assert_eq!(T::from_ascii_radix(b"000097", 10), Ok(T::P_97));
  });

  test!(test_from_ascii_invalid, () => {
    assert_from_str_err!(T, "", Empty);

    assert_from_str_err!(T, "+", InvalidDigit);
    assert_from_str_err!(T, "-", InvalidDigit);

    assert_from_str_err!(T, "0 ", InvalidDigit);
    assert_from_str_err!(T, "0_", InvalidDigit);
    assert_from_str_err!(T, "0z", InvalidDigit);

    assert_from_str_err!(T, " 0", InvalidDigit);
    assert_from_str_err!(T, "_0", InvalidDigit);
    assert_from_str_err!(T, "z0", InvalidDigit);

    assert_from_str_err!(T, "   ", InvalidDigit);
    assert_from_str_err!(T, "0_0", InvalidDigit);
    assert_from_str_err!(T, "owo", InvalidDigit);
    assert_from_str_err!(T, "uwu", InvalidDigit);
  });

  test!(@uint, test_from_ascii_overflow_uint, () => {
    assert_from_str_err!(T, T::MAX_STR_OVERFLOW, PosOverflow);
  });

  test!(@sint, test_from_ascii_radix_negative_sint, () => {
    assert_eq!(T::from_ascii_radix(b"-10", 10), Ok(T::N_10));
    assert_eq!(T::from_ascii_radix(b"-11", 10), Ok(T::N_11));
    assert_eq!(T::from_ascii_radix(b"-20", 10), Ok(T::N_20));
    assert_eq!(T::from_ascii_radix(b"-97", 10), Ok(T::N_97));

    assert_eq!(T::from_ascii_radix(b"-000010", 10), Ok(T::N_10));
    assert_eq!(T::from_ascii_radix(b"-000011", 10), Ok(T::N_11));
    assert_eq!(T::from_ascii_radix(b"-000020", 10), Ok(T::N_20));
    assert_eq!(T::from_ascii_radix(b"-000097", 10), Ok(T::N_97));
  });

  test!(@sint, test_from_ascii_overflow_sint, () => {
    assert_from_str_err!(T, T::MIN_STR_OVERFLOW, NegOverflow);
    assert_from_str_err!(T, T::MAX_STR_OVERFLOW, PosOverflow);
  });
}
