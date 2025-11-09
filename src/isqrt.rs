//! Integer Square Root Algorithms

use crate::types::int;
use crate::types::uint;

const ISQRT: [(u8, u8); 256] = {
  let mut value: [(u8, u8); 256] = [(0, 0); 256];
  let mut index: usize = 0;
  let mut carry: usize = 0;

  while index < value.len() {
    value[index] = (carry as u8, (index - carry.pow(2)) as u8);
    index += 1;

    if index == (carry + 1).pow(2) {
      carry += 1;
    }
  }

  value
};

/// This function uses the [Karatsuba square root algorithm][1] to
/// compute the integer square root of `uint`.
///
/// [1]: https://en.wikipedia.org/wiki/Integer_square_root#Karatsuba_square_root_algorithm
#[inline]
const fn karatsuba<const N: usize>(uint: uint<N>) -> (uint<N>, uint<N>) {
  #[inline]
  const fn __karatsuba<const N: usize>(this: uint<N>, bits: u32) -> (uint<N>, uint<N>) {
    ::core::debug_assert!(bits & 1 == 0, "uneven bits");

    // Trivial case
    if this.is_zero() {
      return (uint::ZERO, uint::ZERO);
    }

    // Base case
    if bits <= 8 {
      let (s, r): (u8, u8) = ISQRT[this.into_usize()];
      return (uint::from_u8(s), uint::from_u8(r));
    }

    let bits_1: u32 = bits >> 1;
    let bits_2: u32 = bits >> 2;
    let bits_3: uint<N> = uint::ONE.const_shl(bits_1).const_sub(uint::ONE);
    let bits_4: uint<N> = uint::ONE.const_shl(bits_2).const_sub(uint::ONE);

    let sh_enter: u32 = (this.leading_zeros() - (uint::<N>::BITS - bits)) & !1;
    let sh_leave: u32 = sh_enter >> 1;

    let this: uint<N> = this.const_shl(sh_enter);

    // ---------------------------------------------------------------------------
    // Step 1: Split into high and low parts
    // ---------------------------------------------------------------------------

    let hi: uint<N> = this.const_shr(bits_1);
    let lo: uint<N> = this.const_band(bits_3);

    // ---------------------------------------------------------------------------
    // Step 2: Recursive call on the high part
    // ---------------------------------------------------------------------------

    let (s_hi, r_hi): (uint<N>, uint<N>) = __karatsuba(hi, bits_1);

    // ---------------------------------------------------------------------------
    // Step 3: Recombine to approximate full sqrt
    // ---------------------------------------------------------------------------

    let numerator: uint<N> = r_hi.const_shl(bits_2).const_bor(lo.const_shr(bits_2));
    let denominator: uint<N> = s_hi.const_shl(1);

    let q: uint<N> = numerator.const_div(denominator);
    let u: uint<N> = numerator.const_rem(denominator);

    let mut s: uint<N> = s_hi.const_shl(bits_2).const_add(q);
    let mut r: uint<N> = u.const_shl(bits_2).const_bor(lo.const_band(bits_4));

    // ---------------------------------------------------------------------------
    // Step 4: Verification and correction
    // ---------------------------------------------------------------------------

    let overflow: bool;

    (r, overflow) = r.overflowing_sub(q.const_mul(q));

    if overflow {
      r = r.wrapping_add(uint::TWO.const_mul(s).const_sub(uint::ONE));
      s = s.const_sub(uint::ONE);
    }

    unsafe {
      ::core::hint::assert_unchecked(!s.is_zero());
    }

    (s.const_shr(sh_leave), r)
  }

  __karatsuba(uint, uint::<N>::BITS)
}

/// This function uses `Algorithm 1.13` of [Modern Computer Arithmetic][1] to
/// compute the integer square root of `m`.
///
/// [1]: https://arxiv.org/abs/1004.4710
#[inline]
const fn newton<const N: usize>(m: uint<N>) -> uint<N> {
  if m.is_zero() {
    return uint::ZERO;
  }

  // 1. u <- m (any value u >= floor(m^1/2) works)
  let mut u: uint<N> = unsafe { uint::<N>::ONE.unchecked_shl((m.bit_width() + 1) >> 1) };
  let mut s: uint<N>;

  // 2. repeat
  loop {
    // 3. s <- u
    s = u;

    // 4. t <- s + floor(m/s)
    // 5. u <- floor(t/2)
    u = s.const_add(m.const_div(s)).const_shr(1);

    // 6. until u >= s
    if u.const_ge(&s) {
      // 7. return s
      return s;
    }
  }
}

/// Returns the [integer square root] of any **nonnegative** [`int`] input.
///
/// # Safety
///
/// This results in undefined behavior when the input is negative.
///
/// [integer square root]: https://en.wikipedia.org/wiki/Integer_square_root
#[must_use = must_use_doc!()]
#[inline]
pub(crate) const unsafe fn int_sqrt<const N: usize>(this: int<N>) -> int<N> {
  ::core::debug_assert!(!this.is_negative(), "Negative input inside `isqrt`.");
  uint_sqrt(this.cast_unsigned()).cast_signed()
}

/// Returns the [integer square root] of any [`uint`] input.
///
/// [integer square root]: https://en.wikipedia.org/wiki/Integer_square_root
#[must_use = must_use_doc!()]
#[inline]
pub(crate) const fn uint_sqrt<const N: usize>(this: uint<N>) -> uint<N> {
  match N {
    1 => uint::from_u8(this.into_u8().isqrt()),
    2 => uint::from_u16(this.into_u16().isqrt()),
    3..=4 => uint::from_u32(this.into_u32().isqrt()),
    5..=8 => uint::from_u64(this.into_u64().isqrt()),
    9..=16 => uint::from_u128(this.into_u128().isqrt()),
    18 | 20 | 22 | 24 | 26 | 28 | 30 | 32 => karatsuba(this).0,
    _ => newton(this),
  }
}
