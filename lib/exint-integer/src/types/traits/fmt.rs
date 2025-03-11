// The majority of this code is based on the `fmt` trait implementations for
// built-in types in libcore.
//
// https://github.com/rust-lang/rust/blob/fd17deacce374a4185c882795be162e17b557050/library/core/src/fmt/num.rs

trait Radix {
  const DIGITS: u8;
  const PREFIX: &'static str;

  fn digit(&'static self, digit: u8) -> u8;
}

macro_rules! implement_radix {
  (
    $name:ident,
    $digits:literal,
    $prefix:literal,
    $($pattern:pat => $expr:expr),+
  ) => {
    struct $name;

    impl Radix for $name {
      const DIGITS: u8 = $digits;
      const PREFIX: &'static str = $prefix;

      #[inline]
      fn digit(&'static self, digit: u8) -> u8 {
        match digit {
          $(
            $pattern => $expr,
          )+
          _ => {
            // SAFETY: Reaching this branch would only be possible due to bugs.
            unsafe { ::core::hint::unreachable_unchecked() };
          }
        }
      }
    }
  };
}

implement_radix!(Binary,   2,  "0b", digit @ 0..=1 => b'0' + digit);
implement_radix!(Octal,    8,  "0o", digit @ 0..=7 => b'0' + digit);
implement_radix!(Decimal,  10, "",   digit @ 0..=9 => b'0' + digit);
implement_radix!(UpperHex, 16, "0x", digit @ 0..=9 => b'0' + digit, digit @ 10..=15 => b'A' + (digit - 10));
implement_radix!(LowerHex, 16, "0x", digit @ 0..=9 => b'0' + digit, digit @ 10..=15 => b'a' + (digit - 10));

struct Buffer<const N: usize> {
  inner: [[::core::mem::MaybeUninit<u8>; N]; 8],
}

impl<const N: usize> Buffer<N> {
  #[inline]
  const fn new() -> Self {
    Self {
      inner: [[const { ::core::mem::MaybeUninit::uninit() }; N]; 8],
    }
  }

  #[inline]
  const fn len(&self) -> usize {
    N * 8
  }
}

impl<const N: usize> ::core::ops::Index<usize> for Buffer<N> {
  type Output = ::core::mem::MaybeUninit<u8>;

  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    &self.inner[index / N][index % N]
  }
}

impl<const N: usize> ::core::ops::IndexMut<usize> for Buffer<N> {
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.inner[index / N][index % N]
  }
}

macro_rules! fmt {
  ($this:ident, $f:ident, $base:ident, $non_negative:expr) => {{
    let non_negative: bool = $non_negative;

    if $this.is_zero() {
      return $f.pad_integral(non_negative, $base::PREFIX, "0");
    }

    let mut buffer: Buffer<N> = Buffer::new();

    let mut cursor: usize = buffer.len();
    let mut source: Self = *$this;

    let base: Self = Self::from_u8($base::DIGITS);

    if non_negative {
      loop {
        let value: Self = source % base;
        let value: u8 = $base.digit(value.into_u8());

        source /= base;
        cursor -= 1;

        let _: &mut u8 = buffer[cursor].write(value);

        if source.is_zero() {
          break;
        }
      }
    } else {
      loop {
        let value: Self = Self::ZERO - (source % base);
        let value: u8 = $base.digit(value.into_u8());

        source /= base;
        cursor -= 1;

        let _: &mut u8 = buffer[cursor].write(value);

        if source.is_zero() {
          break;
        }
      }
    }

    let ptr: *const u8 = buffer[cursor].as_ptr();
    let len: usize = buffer.len() - cursor;

    let slice: &[u8] = unsafe {
      ::core::slice::from_raw_parts(ptr, len)
    };

    // SAFETY: The only characters written to `buffer` are valid UTF-8.
    let string: &str = unsafe {
      ::core::str::from_utf8_unchecked(slice)
    };

    $f.pad_integral(non_negative, $base::PREFIX, string)
  }};
}

macro_rules! implement {
  (impl Debug for $name:ident) => {
    impl<const N: usize> ::core::fmt::Debug for $crate::$name<N> {
      #[expect(deprecated, reason = "No equivalent to current usage of `f.flags()`")]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        const DEBUG_LOWER_HEX: u32 = 1 << 4;
        const DEBUG_UPPER_HEX: u32 = 1 << 5;

        if f.flags() & DEBUG_LOWER_HEX != 0 {
          ::core::fmt::LowerHex::fmt(self, f)
        } else if f.flags() & DEBUG_UPPER_HEX != 0 {
          ::core::fmt::UpperHex::fmt(self, f)
        } else {
          ::core::fmt::Display::fmt(self, f)
        }
      }
    }
  };
  (impl Display for $name:ident) => {
    impl<const N: usize> ::core::fmt::Display for $crate::$name<N> {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt!(self, f, Decimal, self.const_ge(&Self::ZERO))
      }
    }
  };
  (impl LowerExp for $name:ident) => {
    impl<const N: usize> ::core::fmt::LowerExp for $crate::$name<N> {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if Self::BITS <= u128::BITS {
          if <Self as $crate::llapi::Uint>::UINT {
            return ::core::fmt::LowerExp::fmt(&self.into_u128(), f);
          } else {
            return ::core::fmt::LowerExp::fmt(&self.into_i128(), f);
          }
        }

        ::core::panic!("TODO: LowerExp")
      }
    }
  };
  (impl UpperExp for $name:ident) => {
    impl<const N: usize> ::core::fmt::UpperExp for $crate::$name<N> {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if Self::BITS <= u128::BITS {
          if <Self as $crate::llapi::Uint>::UINT {
            return ::core::fmt::UpperExp::fmt(&self.into_u128(), f);
          } else {
            return ::core::fmt::UpperExp::fmt(&self.into_i128(), f);
          }
        }

        ::core::panic!("TODO: UpperExp")
      }
    }
  };
  (impl $format:ident for int as $base:ident) => {
    impl<const N: usize> ::core::fmt::$format for $crate::int<N> {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::$format::fmt(&self.cast_unsigned(), f)
      }
    }
  };
  (impl $format:ident for uint as $base:ident) => {
    impl<const N: usize> ::core::fmt::$format for $crate::uint<N> {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt!(self, f, $base, true)
      }
    }
  };
  (impl $format:ident for $outer:ident<T>) => {
    impl<T: ::core::fmt::$format> ::core::fmt::$format for $crate::$outer<T> {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <T as ::core::fmt::$format>::fmt(&self.0, f)
      }
    }
  };
  (int) => {
    implement!(impl Binary   for int as Binary);
    implement!(impl Debug    for int);
    implement!(impl Display  for int);
    implement!(impl LowerExp for int);
    implement!(impl LowerHex for int as LowerHex);
    implement!(impl Octal    for int as Octal);
    implement!(impl UpperExp for int);
    implement!(impl UpperHex for int as UpperHex);
  };
  (uint) => {
    implement!(impl Binary   for uint as Binary);
    implement!(impl Debug    for uint);
    implement!(impl Display  for uint);
    implement!(impl LowerExp for uint);
    implement!(impl LowerHex for uint as LowerHex);
    implement!(impl Octal    for uint as Octal);
    implement!(impl UpperExp for uint);
    implement!(impl UpperHex for uint as UpperHex);
  };
  ($outer:ident<T>) => {
    implement!(impl Binary   for $outer<T>);
    implement!(impl Debug    for $outer<T>);
    implement!(impl Display  for $outer<T>);
    implement!(impl LowerExp for $outer<T>);
    implement!(impl LowerHex for $outer<T>);
    implement!(impl Octal    for $outer<T>);
    implement!(impl UpperExp for $outer<T>);
    implement!(impl UpperHex for $outer<T>);
  };
}

implement!(int);
implement!(uint);

implement!(Saturating<T>);
#[cfg(feature = "strict_overflow_ops")]
implement!(Strict<T>);
implement!(Wrapping<T>);
