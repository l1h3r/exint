use ::core::debug_assert;
use ::core::fmt::Binary;
use ::core::fmt::Debug;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use ::core::fmt::LowerExp;
use ::core::fmt::LowerHex;
use ::core::fmt::Octal;
use ::core::fmt::Result;
use ::core::fmt::UpperExp;
use ::core::fmt::UpperHex;
use ::core::hint;

use crate::types::int;
use crate::types::uint;
use crate::types::Saturating;
use crate::types::Wrapping;

mod buffer {
  use ::core::mem::MaybeUninit;
  use ::core::slice;

  pub(super) struct Buffer<const N: usize, const M: usize> {
    inner: [[MaybeUninit<u8>; N]; M],
  }

  impl<const N: usize, const M: usize> Buffer<N, M> {
    #[inline]
    pub(super) const fn new() -> Self {
      Self {
        inner: [[const { MaybeUninit::uninit() }; N]; M],
      }
    }

    #[inline]
    pub(super) const fn len(&self) -> usize {
      N * M
    }

    #[inline]
    pub(super) fn write(&mut self, offset: usize, digit: u8) {
      let context: (usize, usize) = (offset / N, offset % N);
      let _unused: &mut u8 = self.inner[context.0][context.1].write(digit);
    }

    #[inline]
    pub(super) unsafe fn as_str(&self, offset: usize) -> &str {
      // SAFETY: The only characters written to `self` are valid UTF-8.
      unsafe { str::from_utf8_unchecked(self.as_slice(offset)) }
    }

    #[inline]
    pub(super) unsafe fn as_slice(&self, offset: usize) -> &[u8] {
      // TODO: Replace with [_]::assume_init_ref
      const unsafe fn assume_init_ref(input: &[MaybeUninit<u8>]) -> &[u8] {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { &*(input as *const [MaybeUninit<u8>] as *const [u8]) }
      }

      let ptr: *const MaybeUninit<u8> = self.inner.as_ptr().cast();
      let len: usize = self.len() - offset;

      // SAFETY: This is guaranteed to be safe by the caller.
      let add: *const MaybeUninit<u8> = unsafe { ptr.add(offset) };

      // SAFETY: This is guaranteed to be safe by the caller.
      let slice: &[MaybeUninit<u8>] = unsafe { slice::from_raw_parts(add, len) };

      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { assume_init_ref(slice) }
    }
  }
}

static DECIMAL_PAIRS: &[u8; 200] = b"\
  0001020304050607080910111213141516171819\
  2021222324252627282930313233343536373839\
  4041424344454647484950515253545556575859\
  6061626364656667686970717273747576777879\
  8081828384858687888990919293949596979899";

fn fmt_display<const N: usize>(
  f: &mut Formatter<'_>,
  this: uint<N>,
  is_nonnegative: bool,
) -> Result {
  let mut buffer: buffer::Buffer<N, 3> = buffer::Buffer::new();
  let mut offset: usize = buffer.len();
  let mut remain: uint<N> = this;

  let n_10: uint<N> = uint::from_u8(10);
  let n_15: uint<N> = uint::from_u8(15);
  let n_100: uint<N> = uint::from_u8(100);

  while N > 1 && remain >= uint::from_u16(1000) {
    // SAFETY: The `while` condition ensures at least 4 digits
    // SAFETY: `offset` counts down from `buffer.len()` and cannot underflow
    unsafe {
      hint::assert_unchecked(offset >= 4);
      hint::assert_unchecked(offset <= buffer.len());
    }

    let scale: uint<N> = uint::from_u16(10000);
    let index: uint<N> = remain % scale;

    remain /= scale;
    offset -= 4;

    let pair1: usize = ((index / n_100) << 1_u32).into_usize();
    let pair2: usize = ((index % n_100) << 1_u32).into_usize();

    buffer.write(offset, DECIMAL_PAIRS[pair1]);
    buffer.write(offset + 1, DECIMAL_PAIRS[pair1 + 1]);
    buffer.write(offset + 2, DECIMAL_PAIRS[pair2]);
    buffer.write(offset + 3, DECIMAL_PAIRS[pair2 + 1]);
  }

  if remain >= n_10 {
    // SAFETY: The `if` condition ensures at least 2 digits
    // SAFETY: `offset` counts down from `buffer.len()` and cannot underflow
    unsafe {
      hint::assert_unchecked(offset >= 2);
      hint::assert_unchecked(offset <= buffer.len());
    }

    let index: usize = ((remain % n_100) << 1_u32).into_usize();

    remain /= n_100;
    offset -= 2;

    buffer.write(offset, DECIMAL_PAIRS[index]);
    buffer.write(offset + 1, DECIMAL_PAIRS[index + 1]);
  }

  if remain != uint::ZERO || this == uint::ZERO {
    // SAFETY: The `if` condition ensures at least 1 digit
    // SAFETY: `offset` counts down from `buffer.len()` and cannot underflow
    unsafe {
      hint::assert_unchecked(offset >= 1);
      hint::assert_unchecked(offset <= buffer.len());
    }

    offset -= 1;
    buffer.write(offset, DECIMAL_PAIRS[((remain & n_15) << 1_u32).into_usize() + 1]);
  }

  // SAFETY: All elements from `offset..length` have been set.
  f.pad_integral(is_nonnegative, "", unsafe { buffer.as_str(offset) })
}

fn fmt_scientific<const N: usize>(
  f: &mut Formatter<'_>,
  this: uint<N>,
  is_nonnegative: bool,
  letter: u8,
) -> Result {
  let n_10: uint<N> = uint::from_u8(10);

  let mut exponent: u32 = this.checked_ilog10().unwrap_or(0);
  let mut mantissa: uint<N> = this;

  debug_assert!(this / n_10.pow(exponent) < n_10);

  while exponent != 0 && (mantissa % n_10).is_zero() {
    mantissa /= n_10;
    exponent -= 1;
  }

  let mut buffer: buffer::Buffer<N, 3> = buffer::Buffer::new();
  let mut offset: usize = buffer.len();
  let mut remain: uint<N> = mantissa;

  match exponent {
    0..10 => {
      offset -= 1;
      buffer.write(offset, b'0' + exponent as u8);
    }
    10..100 => {
      let index: usize = (exponent << 1) as usize;

      offset -= 2;
      buffer.write(offset, DECIMAL_PAIRS[index]);
      buffer.write(offset + 1, DECIMAL_PAIRS[index + 1]);
    }
    100.. => {
      ::core::todo!()
    }
  }

  offset -= 1;
  buffer.write(offset, letter);

  if exponent != 0 {
    for _ in 0..exponent {
      let digit: u8 = (remain % n_10).into_u8();

      remain /= n_10;
      offset -= 1;

      buffer.write(offset, b'0' + digit);
    }

    offset -= 1;
    buffer.write(offset, b'.');
  }

  debug_assert!(remain < n_10);
  debug_assert!(!remain.is_zero() || mantissa.is_zero());

  offset -= 1;
  buffer.write(offset, b'0' + remain.into_u8());

  // SAFETY: All elements from `offset..length` have been set.
  f.pad_integral(is_nonnegative, "", unsafe { buffer.as_str(offset) })
}

macro_rules! implement_radix {
  ($(($trait:ident, $digits:literal, $buffer:literal, $prefix:literal, $lookup:literal))+) => {
    $(
      impl<const N: usize> $trait for int<N> {
        /// Signed integers are formatted in two's-complement form.
        #[inline]
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
          $trait::fmt(&self.cast_unsigned(), f)
        }
      }
      impl<const N: usize> $trait for uint<N> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
          let mut buffer: buffer::Buffer<N, $buffer> = buffer::Buffer::new();
          let mut offset: usize = buffer.len();
          let mut remain: Self = *self;

          let base: Self = Self::from_u8($digits);

          loop {
            let digit: Self = remain % base;

            remain /= base;
            offset -= 1;

            // SAFETY: the loop breaks below before `offset` can wrap
            unsafe {
              hint::assert_unchecked(offset < buffer.len());
            }

            buffer.write(offset, $lookup[digit.into_usize()]);

            if remain.is_zero() {
              break;
            }
          }

          // SAFETY: All elements from `offset..length` have been set.
          f.pad_integral(true, $prefix, unsafe { buffer.as_str(offset) })
        }
      }
    )+
  };
}

implement_radix! {
  (Binary,   2,  8, "0b", b"01")
  (Octal,    8,  4, "0o", b"01234567")
  (LowerHex, 16, 2, "0x", b"0123456789abcdef")
  (UpperHex, 16, 2, "0x", b"0123456789ABCDEF")
}

macro_rules! implement_debug {
  ($($name:ident)+) => {
    $(
      impl<const N: usize> Debug for $name<N> {
        #[expect(deprecated, reason = "no equivalent to `f.flags()`")]
        #[inline]
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
          const DEBUG_LOWER_HEX: u32 = 1 << 4;
          const DEBUG_UPPER_HEX: u32 = 1 << 5;

          if f.flags() & DEBUG_LOWER_HEX != 0 {
            LowerHex::fmt(self, f)
          } else if f.flags() & DEBUG_UPPER_HEX != 0 {
            UpperHex::fmt(self, f)
          } else {
            Display::fmt(self, f)
          }
        }
      }
    )+
  };
}

implement_debug!(int uint);

impl<const N: usize> Display for int<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    fmt_display(f, self.unsigned_abs(), *self >= Self::ZERO)
  }
}

impl<const N: usize> LowerExp for int<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    fmt_scientific(f, self.unsigned_abs(), *self >= Self::ZERO, b'e')
  }
}

impl<const N: usize> UpperExp for int<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    fmt_scientific(f, self.unsigned_abs(), *self >= Self::ZERO, b'E')
  }
}

impl<const N: usize> Display for uint<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    fmt_display(f, *self, true)
  }
}

impl<const N: usize> LowerExp for uint<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    fmt_scientific(f, *self, true, b'e')
  }
}

impl<const N: usize> UpperExp for uint<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    fmt_scientific(f, *self, true, b'E')
  }
}

macro_rules! implement_forwarded {
  ($($trait:ident)+; for $outer:ident<T>) => {
    $(
      impl<T: $trait> $trait for $outer<T> {
        #[inline]
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
          <T as $trait>::fmt(&self.0, f)
        }
      }
    )+
  };
}

implement_forwarded! {
  Debug Display Binary Octal LowerHex UpperHex LowerExp UpperExp;
  for Saturating<T>
}

implement_forwarded! {
  Debug Display Binary Octal LowerHex UpperHex LowerExp UpperExp;
  for Wrapping<T>
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_negative_one, () => {
    assert_eq!(format!("{}", T::N_1), "-1");
    assert_eq!(format!("{:?}", T::N_1), "-1");
    assert_eq!(format!("{:b}", T::N_1), "1".repeat(T::SIZE * 8));
    assert_eq!(format!("{:x}", T::N_1), "f".repeat(T::SIZE * 2));
    assert_eq!(format!("{:X}", T::N_1), "F".repeat(T::SIZE * 2));
    assert_eq!(format!("{:e}", T::N_1), "-1e0");
    assert_eq!(format!("{:E}", T::N_1), "-1E0");
  });

  test!(test_zero, () => {
    assert_eq!(format!("{}", T::P_0), "0");
    assert_eq!(format!("{:?}", T::P_0), "0");
    assert_eq!(format!("{:b}", T::P_0), "0");
    assert_eq!(format!("{:o}", T::P_0), "0");
    assert_eq!(format!("{:x}", T::P_0), "0");
    assert_eq!(format!("{:X}", T::P_0), "0");
    assert_eq!(format!("{:e}", T::P_0), "0e0");
    assert_eq!(format!("{:E}", T::P_0), "0E0");
  });

  test!(test_one, () => {
    assert_eq!(format!("{}", T::P_1), "1");
    assert_eq!(format!("{:?}", T::P_1), "1");
    assert_eq!(format!("{:b}", T::P_1), "1");
    assert_eq!(format!("{:o}", T::P_1), "1");
    assert_eq!(format!("{:x}", T::P_1), "1");
    assert_eq!(format!("{:X}", T::P_1), "1");
    assert_eq!(format!("{:e}", T::P_1), "1e0");
    assert_eq!(format!("{:E}", T::P_1), "1E0");
  });

  test!(test_prefix, () => {
    assert_eq!(format!("{:#}", T::P_1), "1");
    assert_eq!(format!("{:#?}", T::P_1), "1");
    assert_eq!(format!("{:#b}", T::P_1), "0b1");
    assert_eq!(format!("{:#o}", T::P_1), "0o1");
    assert_eq!(format!("{:#x}", T::P_1), "0x1");
    assert_eq!(format!("{:#X}", T::P_1), "0x1");
    assert_eq!(format!("{:#e}", T::P_1), "1e0");
    assert_eq!(format!("{:#E}", T::P_1), "1E0");
  });

  test!(test_min_max, () => {
    assert_eq!(format!("{}", T::MIN), T::MIN_STR);
    assert_eq!(format!("{}", T::MAX), T::MAX_STR);
  });

  test!(test_options, () => {
    // Align/Width
    assert_eq!(format!("{:<5}", T::P_0), "0    ");
    assert_eq!(format!("{:^5}", T::P_0), "  0  ");
    assert_eq!(format!("{:>5}", T::P_0), "    0");

    // Align/Fill/Width
    assert_eq!(format!("{:1<5}", T::P_0), "01111");
    assert_eq!(format!("{:1^5}", T::P_0), "11011");
    assert_eq!(format!("{:1>5}", T::P_0), "11110");

    // Sign
    assert_eq!(format!("{:+}", T::P_0), "+0");

    // Sign/Width
    assert_eq!(format!("{:+5}", T::P_0), "   +0");

    // Trait Prefix
    assert_eq!(format!("0b{:08b}", T::P_0), format!("{:#010b}", T::P_0));
    assert_eq!(format!("0o{:07o}", T::P_0), format!("{:#09o}", T::P_0));
    assert_eq!(format!("0x{:016x}", T::P_0), format!("{:#018x}", T::P_0));
    assert_eq!(format!("0x{:016X}", T::P_0), format!("{:#018X}", T::P_0));
  });
}
