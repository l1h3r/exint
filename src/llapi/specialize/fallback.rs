//! Fallback Implementation
//!
//! The implementation used by every non-specialized integer size.
//!
//! # Safety
//!
//! All invariants of [`Uint`] are required.
//!
//! # Additional
//!
//! This implementation is defined to support generic integers of **any** size,
//! however, suboptimal codegen is typical with "imbalance limbs".
//!
//! Redundant code is easily optimized away but care must be taken to ensure
//! cascading values (ie. carries) are not inadvertently truncated.
//!
//! [`Uint`]: crate::llapi::Uint

use ::core::cmp::Ordering;
use ::core::mem::size_of;
use ::core::ptr;
use ::core::slice;

use crate::llapi::intrinsics;
use crate::llapi::utils;

// -----------------------------------------------------------------------------
// Core Primitive Extensions
// -----------------------------------------------------------------------------

macro_rules! utils {
  ($uint:ident, $sint:ident, $wide:ident) => {
    pub(crate) mod $uint {
      #[inline]
      pub(crate) const fn carrying_add(a: $uint, b: $uint, c: bool) -> ($uint, bool) {
        let (a, b): ($uint, bool) = $uint::overflowing_add(a, b);
        let (c, d): ($uint, bool) = $uint::overflowing_add(a, c as $uint);

        (c, b | d)
      }

      #[inline]
      pub(crate) const fn borrowing_sub(a: $uint, b: $uint, c: bool) -> ($uint, bool) {
        let (a, b): ($uint, bool) = $uint::overflowing_sub(a, b);
        let (c, d): ($uint, bool) = $uint::overflowing_sub(a, c as $uint);

        (c, b | d)
      }

      #[inline]
      pub(crate) const fn widening_divrem(hi: $uint, lo: $uint, divisor: $uint) -> ($uint, $uint) {
        let wide: $wide = ((hi as $wide) << $uint::BITS) + (lo as $wide);
        let div: $uint = (wide / divisor as $wide) as $uint;
        let rem: $uint = (wide % divisor as $wide) as $uint;

        (div, rem)
      }

      #[inline]
      pub(crate) const fn carrying_shl(a: $uint, b: u32, d: $uint) -> ($uint, $uint) {
        ((a << b) | d, a >> ($uint::BITS - b))
      }

      #[inline]
      pub(crate) const fn carrying_shr(a: $uint, b: u32, d: $uint) -> ($uint, $uint) {
        ((a >> b) | d, a << ($uint::BITS - b))
      }
    }
  };
}

utils!(u8, i8, u16);
utils!(u16, i16, u32);
utils!(u32, i32, u64);
utils!(u64, i64, u128);

// -----------------------------------------------------------------------------
// Endianness Helper
// -----------------------------------------------------------------------------

#[expect(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
enum Order {
  ANY,
  LSB,
  MSB,
}

impl Order {
  #[inline]
  const fn get<const N: usize, const M: usize>(self, index: usize) -> usize {
    match self {
      #[cfg(target_endian = "little")]
      Self::ANY => index,
      #[cfg(target_endian = "little")]
      Self::LSB => index,
      #[cfg(target_endian = "little")]
      Self::MSB => N - 1 - (index + M - 1),

      #[cfg(target_endian = "big")]
      Self::ANY => index,
      #[cfg(target_endian = "big")]
      Self::LSB => N - 1 - (index + M - 1),
      #[cfg(target_endian = "big")]
      Self::MSB => index,
    }
  }
}

use self::Order::ANY;
use self::Order::LSB;
use self::Order::MSB;

// -----------------------------------------------------------------------------
// Limb Helper
// -----------------------------------------------------------------------------

struct Limbs {
  u8: usize,
  u16: usize,
  u32: usize,
  u64: usize,
}

impl Limbs {
  #[inline]
  const fn _for(bits: u32) -> Self {
    Self {
      u8: ((bits & (u16::BITS - 1)) / u8::BITS) as usize,
      u16: ((bits & (u32::BITS - 1)) / u16::BITS) as usize,
      u32: ((bits & (u64::BITS - 1)) / u32::BITS) as usize,
      u64: (bits / u64::BITS) as usize,
    }
  }
}

// -----------------------------------------------------------------------------
// Fallback Int
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct Int<const N: usize>([u8; N]);

impl<const N: usize> Int<N> {
  const UMAX: Self = Self::new(<[u8; N] as utils::Consts>::UMAX);
  const UMIN: Self = Self::new(<[u8; N] as utils::Consts>::UMIN);

  const SMAX: Self = Self::new(<[u8; N] as utils::Consts>::SMAX);
  const SMIN: Self = Self::new(<[u8; N] as utils::Consts>::SMIN);

  const ONE: Self = Self::new(<[u8; N] as utils::Consts>::ONE);

  const BITS: u32 = <[u8; N] as utils::Consts>::BITS;

  #[cfg(target_endian = "little")]
  const MSB: usize = (N * 8) - 1;
  #[cfg(target_endian = "big")]
  const MSB: usize = 0;

  const LIMBS: Limbs = Limbs::_for(Self::BITS);

  #[inline]
  const fn new(data: [u8; N]) -> Self {
    Self(data)
  }

  #[inline]
  const fn new_digit(digit: u64) -> Self {
    let mut this: Self = Self::UMIN;
    this.set_u64(0, digit, LSB);
    this
  }

  #[inline]
  const fn slice(&self, index: usize, count: usize) -> &[u8] {
    unsafe { slice::from_raw_parts(self.0.as_ptr().add(index), count) }
  }

  #[inline]
  const fn slice_mut(&mut self, index: usize, count: usize) -> &mut [u8] {
    unsafe { slice::from_raw_parts_mut(self.0.as_mut_ptr().add(index), count) }
  }

  #[inline]
  const fn get<const M: usize>(&self, index: usize, order: Order) -> [u8; M] {
    let mut data: [u8; M] = [0; M];
    data.copy_from_slice(self.slice(order.get::<N, M>(index), M));
    data
  }

  #[inline]
  const fn set<const M: usize>(&mut self, index: usize, value: [u8; M], order: Order) {
    self
      .slice_mut(order.get::<N, M>(index), M)
      .copy_from_slice(&value);
  }

  // ---------------------------------------------------------------------------
  // Internal - Limb Has
  // ---------------------------------------------------------------------------

  #[inline]
  const fn has_u64(index: usize) -> bool {
    index + 8 <= N
  }

  #[inline]
  const fn has_u32(index: usize) -> bool {
    index + 4 <= N
  }

  #[inline]
  const fn has_u16(index: usize) -> bool {
    index + 2 <= N
  }

  #[inline]
  const fn has_u8(index: usize) -> bool {
    index < N
  }

  // ---------------------------------------------------------------------------
  // Internal - Limb Get
  // ---------------------------------------------------------------------------

  #[inline]
  const fn get_u64(&self, index: usize, order: Order) -> u64 {
    u64::from_ne_bytes(self.get(index, order))
  }

  #[inline]
  const fn get_u32(&self, index: usize, order: Order) -> u32 {
    u32::from_ne_bytes(self.get(index, order))
  }

  #[inline]
  const fn get_u16(&self, index: usize, order: Order) -> u16 {
    u16::from_ne_bytes(self.get(index, order))
  }

  #[inline]
  const fn get_u8(&self, index: usize, order: Order) -> u8 {
    self.0[order.get::<N, 1>(index)]
  }

  // ---------------------------------------------------------------------------
  // Internal - Limb Set
  // ---------------------------------------------------------------------------

  #[inline]
  const fn set_u64(&mut self, index: usize, value: u64, order: Order) {
    self.set(index, value.to_ne_bytes(), order);
  }

  #[inline]
  const fn set_u32(&mut self, index: usize, value: u32, order: Order) {
    self.set(index, value.to_ne_bytes(), order);
  }

  #[inline]
  const fn set_u16(&mut self, index: usize, value: u16, order: Order) {
    self.set(index, value.to_ne_bytes(), order);
  }

  #[inline]
  const fn set_u8(&mut self, index: usize, value: u8, order: Order) {
    self.0[order.get::<N, 1>(index)] = value;
  }

  // ---------------------------------------------------------------------------
  // Internal - Bits
  // ---------------------------------------------------------------------------

  #[inline]
  const fn set_bit(&mut self, index: usize, value: bool) {
    self.set_u8(index >> 3, (self.get_u8(index >> 3, ANY) & !(1 << (index & 7))) | ((value as u8) << (index & 7)), ANY);
  }

  #[inline]
  const fn get_bit(&self, index: usize) -> bool {
    ((self.get_u8(index >> 3, ANY) & (1 << (index & 7))) >> (index & 7)) == 1
  }

  // ---------------------------------------------------------------------------
  // Internal - Add/Sub
  // ---------------------------------------------------------------------------

  #[inline]
  const fn addc(x: Self, y: Self, mut d: bool) -> (Self, bool) {
    let mut c: Self = Self::UMIN;
    let mut i: usize = 0;

    let mut x64: u64;
    let mut x32: u32;
    let mut x16: u16;
    let mut x8: u8;

    while Self::has_u64(i) {
      (x64, d) = u64::carrying_add(x.get_u64(i, LSB), y.get_u64(i, LSB), d);

      c.set_u64(i, x64, LSB);
      i += size_of::<u64>();
    }

    while Self::has_u32(i) {
      (x32, d) = u32::carrying_add(x.get_u32(i, LSB), y.get_u32(i, LSB), d);

      c.set_u32(i, x32, LSB);
      i += size_of::<u32>();
    }

    while Self::has_u16(i) {
      (x16, d) = u16::carrying_add(x.get_u16(i, LSB), y.get_u16(i, LSB), d);

      c.set_u16(i, x16, LSB);
      i += size_of::<u16>();
    }

    while Self::has_u8(i) {
      (x8, d) = u8::carrying_add(x.get_u8(i, LSB), y.get_u8(i, LSB), d);

      c.set_u8(i, x8, LSB);
      i += size_of::<u8>();
    }

    (c, d)
  }

  #[inline]
  const fn subb(x: Self, y: Self, mut d: bool) -> (Self, bool) {
    let mut c: Self = Self::UMIN;
    let mut i: usize = 0;

    let mut x64: u64;
    let mut x32: u32;
    let mut x16: u16;
    let mut x8: u8;

    while Self::has_u64(i) {
      (x64, d) = u64::borrowing_sub(x.get_u64(i, LSB), y.get_u64(i, LSB), d);

      c.set_u64(i, x64, LSB);
      i += size_of::<u64>();
    }

    while Self::has_u32(i) {
      (x32, d) = u32::borrowing_sub(x.get_u32(i, LSB), y.get_u32(i, LSB), d);

      c.set_u32(i, x32, LSB);
      i += size_of::<u32>();
    }

    while Self::has_u16(i) {
      (x16, d) = u16::borrowing_sub(x.get_u16(i, LSB), y.get_u16(i, LSB), d);

      c.set_u16(i, x16, LSB);
      i += size_of::<u16>();
    }

    while Self::has_u8(i) {
      (x8, d) = u8::borrowing_sub(x.get_u8(i, LSB), y.get_u8(i, LSB), d);

      c.set_u8(i, x8, LSB);
      i += size_of::<u8>();
    }

    (c, d)
  }

  // ---------------------------------------------------------------------------
  // Internal - Mul
  // ---------------------------------------------------------------------------

  #[inline]
  const fn mul_kochanski(x: Self, y: Self) -> (Self, bool) {
    let mut o: bool = false;
    let mut c: bool;
    let mut p: Self = Self::UMIN;
    let mut i: usize = N * 8;

    while i > 0 {
      i -= 1;
      o |= p.get_bit(Self::MSB);

      // SAFETY: A shift of `1` is always in-bounds.
      p = unsafe { Self::unchecked_shl(p, 1) };

      if y.get_bit(i) {
        (p, c) = Self::addc(p, x, false);
        o |= c;
      }
    }

    (p, o)
  }

  // ---------------------------------------------------------------------------
  // Internal - Div
  // ---------------------------------------------------------------------------

  #[inline]
  const unsafe fn udivrem_limb(lhs: Self, rhs: u64) -> (Self, Self) {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe {
      ::core::hint::assert_unchecked(rhs != 0);
    }

    let mut div: Self = lhs;
    let mut rem: u64 = 0;

    let mut idx: usize = 0;
    let mut tmp: usize = 0;

    while idx < Self::LIMBS.u8 * size_of::<u8>() {
      let x: u8 = div.get_u8(idx, MSB);
      let y: (u8, u8) = u8::widening_divrem(rem as u8, x, rhs as u8);

      div.set_u8(idx, y.0, MSB);
      rem = y.1 as u64;
      idx += size_of::<u8>();
    }

    idx = 0;
    tmp += Self::LIMBS.u8 * size_of::<u8>();

    while idx < Self::LIMBS.u16 * size_of::<u16>() {
      let x: u16 = div.get_u16(tmp + idx, MSB);
      let y: (u16, u16) = u16::widening_divrem(rem as u16, x, rhs as u16);

      div.set_u16(tmp + idx, y.0, MSB);
      rem = y.1 as u64;
      idx += size_of::<u16>();
    }

    idx = 0;
    tmp += Self::LIMBS.u16 * size_of::<u16>();

    while idx < Self::LIMBS.u32 * size_of::<u32>() {
      let x: u32 = div.get_u32(tmp + idx, MSB);
      let y: (u32, u32) = u32::widening_divrem(rem as u32, x, rhs as u32);

      div.set_u32(tmp + idx, y.0, MSB);
      rem = y.1 as u64;
      idx += size_of::<u32>();
    }

    idx = 0;
    tmp += Self::LIMBS.u32 * size_of::<u32>();

    while idx < Self::LIMBS.u64 * size_of::<u64>() {
      let x: u64 = div.get_u64(tmp + idx, MSB);
      let y: (u64, u64) = u64::widening_divrem(rem, x, rhs);

      div.set_u64(tmp + idx, y.0, MSB);
      rem = y.1;
      idx += size_of::<u64>();
    }

    (div, Self::new_digit(rem))
  }

  #[inline]
  const fn udivrem_long(lhs: Self, rhs: Self) -> (Self, Self) {
    let mut i: usize = lhs.bit_width_max(&rhs) as usize;
    let mut q: Self = Self::UMIN;
    let mut r: Self = Self::UMIN;

    while i > 0 {
      i -= 1;

      // SAFETY: A shift of `1` is always in-bounds.
      r = unsafe { Self::unchecked_shl(r, 1) };
      r.set_bit(0, lhs.get_bit(i));

      if Self::ucmp(r, rhs).is_ge() {
        r = Self::wrapping_sub(r, rhs);
        q.set_bit(i, true);
      }
    }

    (q, r)
  }

  #[inline]
  const unsafe fn udivrem_full(lhs: Self, rhs: Self) -> (Self, Self) {
    match (Self::LIMBS.u8, Self::LIMBS.u16, Self::LIMBS.u32, rhs.bit_width()) {
      (_, _, _, 0..=8) => {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { Self::udivrem_limb(lhs, rhs.get_u8(0, LSB) as u64) }
      }
      (0, _, _, 9..=16) => {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { Self::udivrem_limb(lhs, rhs.get_u16(0, LSB) as u64) }
      }
      (0, 0, _, 17..=32) => {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { Self::udivrem_limb(lhs, rhs.get_u32(0, LSB) as u64) }
      }
      (0, 0, 0, 33..=64) => {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { Self::udivrem_limb(lhs, rhs.get_u64(0, LSB)) }
      }
      _ => {
        Self::udivrem_long(lhs, rhs)
      }
    }
  }

  #[inline]
  const unsafe fn udivrem<const EXACT: bool>(lhs: Self, rhs: Self) -> (Self, Self) {
    match Self::ucmp(lhs, rhs) {
      Ordering::Less => (Self::UMIN, lhs),
      Ordering::Equal => (Self::ONE, Self::UMIN),
      Ordering::Greater if Self::eq(lhs, Self::UMIN) => (Self::UMIN, Self::UMIN),
      // SAFETY: This is guaranteed to be safe by the caller.
      Ordering::Greater => unsafe { Self::udivrem_full(lhs, rhs) },
    }
  }

  #[inline]
  const unsafe fn sdivrem<const EXACT: bool>(lhs: Self, rhs: Self) -> (Self, Self) {
    // SAFETY: This is guaranteed to be safe by the caller.
    let mut out: (Self, Self) = unsafe {
      Self::udivrem::<EXACT>(lhs.abs(), rhs.abs())
    };

    if lhs.is_negative() ^ rhs.is_negative() {
      out.0 = out.0.neg();
    }

    if lhs.is_negative() {
      out.1 = out.1.neg();
    }

    out
  }

  // ---------------------------------------------------------------------------
  // Internal - Shl/Shr
  // ---------------------------------------------------------------------------

  #[inline]
  const fn shl(int: Self, bits: u32) -> Self {
    let shift_word: usize = (bits as usize) >> u8::BITS.trailing_zeros();
    let shift_bits: u32 = bits & (u8::BITS - 1);

    let mut value: Self = Self::UMIN;

    // LE = [0xD, 0xC, 0xB, 0xA] - [0..N-1-B] -> [B..N-1]
    // BE = [0xA, 0xB, 0xC, 0xD] - [B..N-1] -> [0..N-1-B]
    #[cfg(target_endian = "little")]
    unsafe {
      ptr::copy_nonoverlapping(int.0.as_ptr(), value.0.as_mut_ptr().add(shift_word), N - shift_word);
    }

    #[cfg(target_endian = "big")]
    unsafe {
      ptr::copy_nonoverlapping(int.0.as_ptr().add(shift_word), value.0.as_mut_ptr(), N - shift_word);
    }

    if shift_bits != 0 {
      let mut carry: u64 = 0;
      let mut index: usize = shift_word;

      while Self::has_u64(index) {
        let x: u64 = value.get_u64(index, LSB);
        let y: (u64, u64) = u64::carrying_shl(x, shift_bits, carry);

        value.set_u64(index, y.0, LSB);
        carry = y.1;
        index += size_of::<u64>();
      }

      while Self::has_u32(index) {
        let x: u32 = value.get_u32(index, LSB);
        let y: (u32, u32) = u32::carrying_shl(x, shift_bits, carry as u32);

        value.set_u32(index, y.0, LSB);
        carry = y.1 as u64;
        index += size_of::<u32>();
      }

      while Self::has_u16(index) {
        let x: u16 = value.get_u16(index, LSB);
        let y: (u16, u16) = u16::carrying_shl(x, shift_bits, carry as u16);

        value.set_u16(index, y.0, LSB);
        carry = y.1 as u64;
        index += size_of::<u16>();
      }

      while Self::has_u8(index) {
        let x: u8 = value.get_u8(index, LSB);
        let y: (u8, u8) = u8::carrying_shl(x, shift_bits, carry as u8);

        value.set_u8(index, y.0, LSB);
        carry = y.1 as u64;
        index += size_of::<u8>();
      }
    }

    value
  }

  #[inline]
  const fn shr<const LOGICAL: bool>(int: Self, bits: u32) -> Self {
    let shift_word: usize = (bits as usize) >> u8::BITS.trailing_zeros();
    let shift_bits: u32 = bits & (u8::BITS - 1);

    let mut value: Self = if LOGICAL {
      Self::UMIN
    } else {
      Self::new([int.sign_bit() * u8::MAX; N])
    };

    // LE = [0xD, 0xC, 0xB, 0xA] - [B..N-1] -> [0..N-1-B]
    // BE = [0xA, 0xB, 0xC, 0xD] - [0..N-1-B] -> [B..N-1]
    #[cfg(target_endian = "little")]
    unsafe {
      ptr::copy_nonoverlapping(int.0.as_ptr().add(shift_word), value.0.as_mut_ptr(), N - shift_word);
    }

    #[cfg(target_endian = "big")]
    unsafe {
      ptr::copy_nonoverlapping(int.0.as_ptr(), value.0.as_mut_ptr().add(shift_word), N - shift_word);
    }

    if shift_bits != 0 {
      let mut carry: u64 = if LOGICAL {
        0
      } else {
        !((int.sign_bit() as u64) << (u64::BITS - shift_bits)).wrapping_sub(1)
      };

      let mut index: usize = shift_word;

      while Self::has_u64(index) {
        let x: u64 = value.get_u64(index, MSB);
        let y: (u64, u64) = u64::carrying_shr(x, shift_bits, carry);

        value.set_u64(index, y.0, MSB);
        carry = y.1;
        index += size_of::<u64>();
      }

      carry >>= 32;

      while Self::has_u32(index) {
        let x: u32 = value.get_u32(index, MSB);
        let y: (u32, u32) = u32::carrying_shr(x, shift_bits, carry as u32);

        value.set_u32(index, y.0, MSB);
        carry = y.1 as u64;
        index += size_of::<u32>();
      }

      carry >>= 16;

      while Self::has_u16(index) {
        let x: u16 = value.get_u16(index, MSB);
        let y: (u16, u16) = u16::carrying_shr(x, shift_bits, carry as u16);

        value.set_u16(index, y.0, MSB);
        carry = y.1 as u64;
        index += size_of::<u16>();
      }

      carry >>= 8;

      while Self::has_u8(index) {
        let x: u8 = value.get_u8(index, MSB);
        let y: (u8, u8) = u8::carrying_shr(x, shift_bits, carry as u8);

        value.set_u8(index, y.0, MSB);
        carry = y.1 as u64;
        index += size_of::<u8>();
      }
    }

    value
  }

  // ---------------------------------------------------------------------------
  // Internal - Misc.
  // ---------------------------------------------------------------------------

  #[inline]
  const fn bit_width_max(&self, other: &Self) -> u32 {
    let a: u32 = self.bit_width();
    let b: u32 = other.bit_width();

    if a > b { a } else { b }
  }

  #[inline]
  const fn bit_width(&self) -> u32 {
    Self::BITS - Self::ctlz(*self)
  }

  #[inline]
  const fn sign_raw(&self) -> u8 {
    self.0[MSB.get::<N, 1>(0)] & utils::SIGN
  }

  #[inline]
  const fn sign_bit(&self) -> u8 {
    self.sign_raw() >> (u8::BITS - 1)
  }

  #[inline]
  const fn is_negative(&self) -> bool {
    self.sign_raw() == utils::SIGN
  }

  #[inline]
  const fn abs(&self) -> Self {
    if self.is_negative() {
      self.neg()
    } else {
      *self
    }
  }

  #[inline]
  const fn neg(&self) -> Self {
    Self::wrapping_sub(Self::UMIN, *self)
  }

  // ---------------------------------------------------------------------------
  // Bitwise Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn band(lhs: Self, rhs: Self) -> Self {
    let mut out: Self = Self::UMIN;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out.set_u64(idx, lhs.get_u64(idx, ANY) & rhs.get_u64(idx, ANY), ANY);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out.set_u32(idx, lhs.get_u32(idx, ANY) & rhs.get_u32(idx, ANY), ANY);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out.set_u16(idx, lhs.get_u16(idx, ANY) & rhs.get_u16(idx, ANY), ANY);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out.set_u8(idx, lhs.get_u8(idx, ANY) & rhs.get_u8(idx, ANY), ANY);
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn bor(lhs: Self, rhs: Self) -> Self {
    let mut out: Self = Self::UMIN;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out.set_u64(idx, lhs.get_u64(idx, ANY) | rhs.get_u64(idx, ANY), ANY);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out.set_u32(idx, lhs.get_u32(idx, ANY) | rhs.get_u32(idx, ANY), ANY);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out.set_u16(idx, lhs.get_u16(idx, ANY) | rhs.get_u16(idx, ANY), ANY);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out.set_u8(idx, lhs.get_u8(idx, ANY) | rhs.get_u8(idx, ANY), ANY);
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn bxor(lhs: Self, rhs: Self) -> Self {
    let mut out: Self = Self::UMIN;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out.set_u64(idx, lhs.get_u64(idx, ANY) ^ rhs.get_u64(idx, ANY), ANY);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out.set_u32(idx, lhs.get_u32(idx, ANY) ^ rhs.get_u32(idx, ANY), ANY);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out.set_u16(idx, lhs.get_u16(idx, ANY) ^ rhs.get_u16(idx, ANY), ANY);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out.set_u8(idx, lhs.get_u8(idx, ANY) ^ rhs.get_u8(idx, ANY), ANY);
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn bnot(int: Self) -> Self {
    let mut out: Self = Self::UMIN;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out.set_u64(idx, !int.get_u64(idx, ANY), ANY);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out.set_u32(idx, !int.get_u32(idx, ANY), ANY);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out.set_u16(idx, !int.get_u16(idx, ANY), ANY);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out.set_u8(idx, !int.get_u8(idx, ANY), ANY);
      idx += size_of::<u8>();
    }

    out
  }

  // ---------------------------------------------------------------------------
  // Comparison Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn eq(lhs: Self, rhs: Self) -> bool {
    let mut out: bool = true;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out &= lhs.get_u64(idx, ANY) == rhs.get_u64(idx, ANY);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out &= lhs.get_u32(idx, ANY) == rhs.get_u32(idx, ANY);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out &= lhs.get_u16(idx, ANY) == rhs.get_u16(idx, ANY);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out &= lhs.get_u8(idx, ANY) == rhs.get_u8(idx, ANY);
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn ucmp(lhs: Self, rhs: Self) -> Ordering {
    let mut out: Ordering = Ordering::Equal;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out = out.then(intrinsics::three_way_compare!(lhs.get_u64(idx, MSB), rhs.get_u64(idx, MSB)));
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out = out.then(intrinsics::three_way_compare!(lhs.get_u32(idx, MSB), rhs.get_u32(idx, MSB)));
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out = out.then(intrinsics::three_way_compare!(lhs.get_u16(idx, MSB), rhs.get_u16(idx, MSB)));
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out = out.then(intrinsics::three_way_compare!(lhs.get_u8(idx, MSB), rhs.get_u8(idx, MSB)));
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn scmp(lhs: Self, rhs: Self) -> Ordering {
    match (lhs.is_negative(), rhs.is_negative()) {
      (true, false) => Ordering::Less,
      (false, true) => Ordering::Greater,
      (true, true) | (false, false) => Self::ucmp(lhs, rhs),
    }
  }

  // ---------------------------------------------------------------------------
  // Bit Conversion Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn swap1(int: Self) -> Self {
    let mut out: Self = Self::UMIN;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out.set_u64(idx, intrinsics::bitreverse!(int.get_u64(idx, LSB)), MSB);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out.set_u32(idx, intrinsics::bitreverse!(int.get_u32(idx, LSB)), MSB);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out.set_u16(idx, intrinsics::bitreverse!(int.get_u16(idx, LSB)), MSB);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out.set_u8(idx, intrinsics::bitreverse!(int.get_u8(idx, LSB)), MSB);
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn swap8(int: Self) -> Self {
    let mut out: Self = Self::UMIN;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out.set_u64(idx, intrinsics::bswap!(int.get_u64(idx, LSB)), MSB);
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out.set_u32(idx, intrinsics::bswap!(int.get_u32(idx, LSB)), MSB);
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out.set_u16(idx, intrinsics::bswap!(int.get_u16(idx, LSB)), MSB);
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out.set_u8(idx, int.get_u8(idx, LSB), MSB);
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn rotl(int: Self, bits: u32) -> Self {
    // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
    unsafe { Self::unchecked_fshl(int, int, bits % Self::BITS) }
  }

  #[inline]
  pub(crate) const fn rotr(int: Self, bits: u32) -> Self {
    // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
    unsafe { Self::unchecked_fshr(int, int, bits % Self::BITS) }
  }

  // ---------------------------------------------------------------------------
  // Bit Inspection Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn ctpop(int: Self) -> u32 {
    let mut out: u32 = 0;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      out += intrinsics::ctpop!(int.get_u64(idx, ANY));
      idx += size_of::<u64>();
    }

    while Self::has_u32(idx) {
      out += intrinsics::ctpop!(int.get_u32(idx, ANY));
      idx += size_of::<u32>();
    }

    while Self::has_u16(idx) {
      out += intrinsics::ctpop!(int.get_u16(idx, ANY));
      idx += size_of::<u16>();
    }

    while Self::has_u8(idx) {
      out += intrinsics::ctpop!(int.get_u8(idx, ANY));
      idx += size_of::<u8>();
    }

    out
  }

  #[inline]
  pub(crate) const fn ctlz(int: Self) -> u32 {
    let mut out: u32 = 0;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      let x: u64 = int.get_u64(idx, MSB);

      out += intrinsics::ctlz!(x);
      idx += size_of::<u64>();

      if x != 0 {
        return out;
      }
    }

    while Self::has_u32(idx) {
      let x: u32 = int.get_u32(idx, MSB);

      out += intrinsics::ctlz!(x);
      idx += size_of::<u32>();

      if x != 0 {
        return out;
      }
    }

    while Self::has_u16(idx) {
      let x: u16 = int.get_u16(idx, MSB);

      out += intrinsics::ctlz!(x);
      idx += size_of::<u16>();

      if x != 0 {
        return out;
      }
    }

    while Self::has_u8(idx) {
      let x: u8 = int.get_u8(idx, MSB);

      out += intrinsics::ctlz!(x);
      idx += size_of::<u8>();

      if x != 0 {
        return out;
      }
    }

    Self::BITS
  }

  #[inline]
  pub(crate) const fn cttz(int: Self) -> u32 {
    let mut out: u32 = 0;
    let mut idx: usize = 0;

    while Self::has_u64(idx) {
      let x: u64 = int.get_u64(idx, LSB);

      out += intrinsics::cttz!(x);
      idx += size_of::<u64>();

      if x != 0 {
        return out;
      }
    }

    while Self::has_u32(idx) {
      let x: u32 = int.get_u32(idx, LSB);

      out += intrinsics::cttz!(x);
      idx += size_of::<u32>();

      if x != 0 {
        return out;
      }
    }

    while Self::has_u16(idx) {
      let x: u16 = int.get_u16(idx, LSB);

      out += intrinsics::cttz!(x);
      idx += size_of::<u16>();

      if x != 0 {
        return out;
      }
    }

    while Self::has_u8(idx) {
      let x: u8 = int.get_u8(idx, LSB);

      out += intrinsics::cttz!(x);
      idx += size_of::<u8>();

      if x != 0 {
        return out;
      }
    }

    Self::BITS
  }

  #[inline]
  pub(crate) const unsafe fn ctlz_nonzero(int: Self) -> u32 {
    Self::ctlz(int)
  }

  #[inline]
  pub(crate) const unsafe fn cttz_nonzero(int: Self) -> u32 {
    Self::cttz(int)
  }

  // ---------------------------------------------------------------------------
  // Overflowing Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
    Self::addc(lhs, rhs, false)
  }

  #[inline]
  pub(crate) const fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
    Self::subb(lhs, rhs, false)
  }

  #[inline]
  pub(crate) const fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
    Self::mul_kochanski(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
    let out: Self = Self::wrapping_add(lhs, rhs);
    let cmp: bool = lhs.is_negative() == rhs.is_negative() && lhs.is_negative() != out.is_negative();

    (out, cmp)
  }

  #[inline]
  pub(crate) const fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
    let out: Self = Self::wrapping_sub(lhs, rhs);
    let cmp: bool = lhs.is_negative() != rhs.is_negative() && lhs.is_negative() != out.is_negative();

    (out, cmp)
  }

  #[inline]
  pub(crate) const fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
    let (out, cmp): (Self, bool) = Self::overflowing_umul(lhs.abs(), rhs.abs());

    if lhs.is_negative() ^ rhs.is_negative() {
      (out.neg(), cmp || (Self::eq(out, Self::SMIN) ^ out.is_negative()))
    } else {
      (out, cmp || out.is_negative())
    }
  }

  // ---------------------------------------------------------------------------
  // Saturating Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
    match Self::overflowing_uadd(lhs, rhs) {
      (_value, true) => Self::UMAX,
      (value, false) => value,
    }
  }

  #[inline]
  pub(crate) const fn saturating_usub(lhs: Self, rhs: Self) -> Self {
    match Self::overflowing_usub(lhs, rhs) {
      (_value, true) => Self::UMIN,
      (value, false) => value,
    }
  }

  #[inline]
  pub(crate) const fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
    match Self::overflowing_sadd(lhs, rhs) {
      (_value, true) if lhs.is_negative() => Self::SMIN,
      (_value, true) => Self::SMAX,
      (value, false) => value,
    }
  }

  #[inline]
  pub(crate) const fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
    match Self::overflowing_ssub(lhs, rhs) {
      (_value, true) if lhs.is_negative() => Self::SMIN,
      (_value, true) => Self::SMAX,
      (value, false) => value,
    }
  }

  // ---------------------------------------------------------------------------
  // Unchecked Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
    Self::wrapping_add(lhs, rhs)
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
    Self::wrapping_sub(lhs, rhs)
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
    Self::wrapping_mul(lhs, rhs)
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::udivrem::<false>(lhs, rhs).0 }
  }

  #[cfg(feature = "exact_div")]
  #[inline]
  pub(crate) const unsafe fn unchecked_udiv_exact(lhs: Self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::udivrem::<true>(lhs, rhs).0 }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::udivrem::<false>(lhs, rhs).1 }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
    Self::wrapping_add(lhs, rhs)
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
    Self::wrapping_sub(lhs, rhs)
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
    Self::wrapping_mul(lhs, rhs)
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::sdivrem::<false>(lhs, rhs).0 }
  }

  #[cfg(feature = "exact_div")]
  #[inline]
  pub(crate) const unsafe fn unchecked_sdiv_exact(lhs: Self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::sdivrem::<true>(lhs, rhs).0 }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::sdivrem::<false>(lhs, rhs).1 }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_shl(int: Self, bits: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::shl(int, bits) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_lshr(int: Self, bits: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::shr::<true>(int, bits) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_ashr(int: Self, bits: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Self::shr::<false>(int, bits) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_fshl(lhs: Self, rhs: Self, bits: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe {
      ::core::hint::assert_unchecked(bits < Self::BITS);
    }

    if bits == 0 {
      lhs
    } else {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        Self::bor(
          Self::shl(lhs, bits),
          Self::shr::<true>(rhs, Self::BITS - bits),
        )
      }
    }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_fshr(lhs: Self, rhs: Self, bits: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe {
      ::core::hint::assert_unchecked(bits < Self::BITS);
    }

    if bits == 0 {
      rhs
    } else {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        Self::bor(
          Self::shl(lhs, Self::BITS - bits),
          Self::shr::<true>(rhs, bits),
        )
      }
    }
  }

  // ---------------------------------------------------------------------------
  // Wrapping Operations
  // ---------------------------------------------------------------------------

  #[inline]
  pub(crate) const fn wrapping_add(lhs: Self, rhs: Self) -> Self {
    Self::overflowing_uadd(lhs, rhs).0
  }

  #[inline]
  pub(crate) const fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
    Self::overflowing_usub(lhs, rhs).0
  }

  #[inline]
  pub(crate) const fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
    Self::overflowing_umul(lhs, rhs).0
  }

  // ---------------------------------------------------------------------------
  // Misc. Operations
  // ---------------------------------------------------------------------------

  #[cfg(feature = "disjoint_bitor")]
  #[inline]
  pub(crate) const unsafe fn disjoint_bor(lhs: Self, rhs: Self) -> Self {
    Self::bor(lhs, rhs)
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  const fn cma_split(int: Self) -> [Self; 2] {
    let mask: Self = unsafe { Self::shl(Self::ONE, Self::BITS >> 1) };
    let mask: Self = Self::wrapping_sub(mask, Self::ONE);

    [
      Self::band(int, mask),
      unsafe { Self::shr::<true>(int, Self::BITS >> 1) },
    ]
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  const fn cma_mul([a0, a1]: [Self; 2], b: Self) -> [Self; 3] {
    let [x, c]: [Self; 2] = Self::cma_split(Self::wrapping_mul(b, a0));
    let [y, z]: [Self; 2] = Self::cma_split(Self::wrapping_add(Self::wrapping_mul(b, a1), c));

    [x, y, z]
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  const fn cma_join(int: [Self; 2]) -> Self {
    Self::bor(int[0], unsafe { Self::shl(int[1], Self::BITS >> 1) })
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  const fn cma_bool(value: bool) -> Self {
    Self(crate::llapi::cast_bytes::<true, 1, { N }>([value as u8]))
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  const fn widening_mul(a: Self, b: Self) -> (Self, Self) {
    let a: [Self; 2] = Self::cma_split(a);
    let b: [Self; 2] = Self::cma_split(b);

    let lo: [Self; 3] = Self::cma_mul(a, b[0]);
    let hi: [Self; 3] = Self::cma_mul(a, b[1]);

    let [r1, c]: [Self; 2] = Self::cma_split(Self::wrapping_add(lo[1], hi[0]));
    let [r2, c]: [Self; 2] = Self::cma_split(Self::wrapping_add(Self::wrapping_add(lo[2], hi[1]), c));

    let r0: Self = lo[0];
    let r3: Self = Self::wrapping_add(hi[2], c);

    (Self::cma_join([r0, r1]), Self::cma_join([r2, r3]))
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  pub(crate) const fn carrying_umul_uadd(
    lhs: Self,
    rhs: Self,
    add1: Self,
    add2: Self,
  ) -> (Self, Self) {
    let (lo, mut hi): (Self, Self) = Self::widening_mul(lhs, rhs);

    let (lo, carry): (Self, bool) = Self::overflowing_uadd(lo, add1);
    hi = Self::wrapping_add(hi, Self::cma_bool(carry));

    let (lo, carry): (Self, bool) = Self::overflowing_uadd(lo, add2);
    hi = Self::wrapping_add(hi, Self::cma_bool(carry));

    (lo, hi)
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  pub(crate) const fn carrying_smul_sadd(
    lhs: Self,
    rhs: Self,
    add1: Self,
    add2: Self,
  ) -> (Self, Self) {
    let (lo, mut hi): (Self, Self) = Self::widening_mul(lhs, rhs);

    let x1: Self = unsafe { Self::shr::<false>(lhs, Self::BITS - 1) };
    let y1: Self = unsafe { Self::shr::<false>(rhs, Self::BITS - 1) };

    let x2: Self = unsafe { Self::shr::<false>(add1, Self::BITS - 1) };
    let y2: Self = unsafe { Self::shr::<false>(add2, Self::BITS - 1) };

    hi = Self::wrapping_add(hi, Self::wrapping_mul(x1, rhs));
    hi = Self::wrapping_add(hi, Self::wrapping_mul(lhs, y1));

    let (lo, carry): (Self, bool) = Self::overflowing_uadd(lo, add1);
    hi = Self::wrapping_add(hi, Self::wrapping_add(Self::cma_bool(carry), x2));

    let (lo, carry): (Self, bool) = Self::overflowing_uadd(lo, add2);
    hi = Self::wrapping_add(hi, Self::wrapping_add(Self::cma_bool(carry), y2));

    (lo, hi)
  }
}
