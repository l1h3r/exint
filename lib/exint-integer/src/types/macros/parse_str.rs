macro_rules! parse_str {
  ($name:ident) => {
    #[doc = $crate::utils::include_doc!($name, "from_str_radix")]
    #[inline]
    pub const fn from_str_radix(
      src: &str,
      radix: u32,
    ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
      Self::from_ascii_radix(src.as_bytes(), radix)
    }

    #[doc = $crate::utils::include_doc!($name, "from_ascii")]
    #[cfg(feature = "int_from_ascii")]
    #[inline]
    pub const fn from_ascii(
      src: &[u8],
    ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
      Self::from_ascii_radix(src, 10)
    }

    macros::stability! {
      #[unstable(feature = "int_from_ascii")]
      #[allow(rustdoc::private_doc_tests, reason = "Might not be public due to features")]
      #[doc = $crate::utils::include_doc!($name, "int_from_ascii")]
      #[inline]
      pub const fn from_ascii_radix(
        src: &[u8],
        radix: u32,
      ) -> ::core::result::Result<Self, $crate::error::ParseIntError> {
        if 2 > radix || radix > 36 {
          // TODO: Runtime panic formatting
          ::core::panic!("from_ascii_radix: radix must lie in the range `[2, 36]`");
        }

        #[inline]
        const fn no_overflow<T>(radix: u32, signed: bool, bytes: &[u8]) -> bool {
          radix <= 16 && bytes.len() <= ::core::mem::size_of::<T>() * 2 - signed as usize
        }

        macro_rules! error {
          ($kind:ident) => {
            $crate::error::ParseIntError::new($crate::error::IntErrorKind::$kind)
          };
        }

        macro_rules! tri {
          ($expr:expr_2021, $error:ident) => {
            match $expr {
              Some(value) => value,
              None => return ::core::result::Result::Err(error!($error)),
            }
          };
        }

        if src.is_empty() {
          return ::core::result::Result::Err(error!(Empty));
        }

        let signed: bool = Self::ZERO.const_gt(&Self::MIN);

        let (positive, mut digits): (bool, &[u8]) = match src {
          [b'+' | b'-'] => return ::core::result::Result::Err(error!(InvalidDigit)),
          [b'+', tail @ ..] => (true, tail),
          [b'-', tail @ ..] if signed => (false, tail),
          _ => (true, src),
        };

        let mut result: Self = Self::ZERO;

        if no_overflow::<Self>(radix, signed, digits) {
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
                let multi: Option<Self> = result.checked_mul(Self::from_u32(radix));
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
    }
  };
}

pub(crate) use parse_str;
