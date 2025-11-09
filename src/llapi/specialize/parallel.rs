//! SIMD-Based Specialization
//!
//! This experimental implementation is feature-gated by the `portable_simd`
//! feature.
//!
//! Currently only supports integers that can be expressed as 64-bit [SIMD lanes].
//!
//! # Safety
//!
//! The functions here expect the externally-provided generic integers to have
//! a size where `1 <= size <= 64` and `size % size_of(u64) == 0`.
//!
//! All invariants of [`Int`] are required.
//!
//! All invariants of [`Uint`] are required.
//!
//! [SIMD lanes]: ::core::simd::SupportedLaneCount
//! [`Int`]: crate::llapi::specialize::Int
//! [`Uint`]: crate::llapi::Uint

use ::core::cmp::Ordering;
use ::core::intrinsics;
use ::core::mem;
use ::core::simd::LaneCount;
use ::core::simd::Simd;
use ::core::simd::SupportedLaneCount;
use ::core::simd::num::SimdUint as _;

use crate::llapi::specialize::Int;
use crate::llapi::utils;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct I64xN<const BYTES: usize, const LANES: usize>([u64; LANES])
where
  LaneCount<LANES>: SupportedLaneCount;

impl<const B: usize, const L: usize> I64xN<B, L>
where
  LaneCount<L>: SupportedLaneCount,
{
  const ASSERT: () = {
    assert!(B.is_multiple_of(mem::size_of::<u64>()));
    assert!(B / mem::size_of::<u64>() == L);
  };

  #[inline]
  const fn cast(self) -> Int<B> {
    let _: () = const { Self::ASSERT };
    unsafe { utils::transmute(self) }
  }

  #[inline]
  const fn read_cast(that: Int<B>) -> Self {
    let _: () = const { Self::ASSERT };
    unsafe { utils::transmute(that) }
  }

  #[inline]
  const fn simd(self) -> Simd<u64, L> {
    Simd::from_array(self.0)
  }

  #[inline]
  const fn read_simd(that: Simd<u64, L>) -> Self {
    Self(that.to_array())
  }

  // -----------------------------------------------------------------------
  // Bitwise Operations
  // -----------------------------------------------------------------------

  #[inline]
  const fn band_ct(lhs: Self, rhs: Self) -> Self {
    Self::read_cast(Int::band(lhs.cast(), rhs.cast()))
  }

  #[inline]
  fn band_rt(lhs: Self, rhs: Self) -> Self {
    Self::read_simd(lhs.simd() & rhs.simd())
  }

  #[inline]
  pub(crate) const fn band(lhs: Self, rhs: Self) -> Self {
    intrinsics::const_eval_select((lhs, rhs), Self::band_ct, Self::band_rt)
  }

  #[inline]
  const fn bor_ct(lhs: Self, rhs: Self) -> Self {
    Self::read_cast(Int::bor(lhs.cast(), rhs.cast()))
  }

  #[inline]
  fn bor_rt(lhs: Self, rhs: Self) -> Self {
    Self::read_simd(lhs.simd() | rhs.simd())
  }

  #[inline]
  pub(crate) const fn bor(lhs: Self, rhs: Self) -> Self {
    intrinsics::const_eval_select((lhs, rhs), Self::bor_ct, Self::bor_rt)
  }

  #[inline]
  const fn bxor_ct(lhs: Self, rhs: Self) -> Self {
    Self::read_cast(Int::bxor(lhs.cast(), rhs.cast()))
  }

  #[inline]
  fn bxor_rt(lhs: Self, rhs: Self) -> Self {
    Self::read_simd(lhs.simd() ^ rhs.simd())
  }

  #[inline]
  pub(crate) const fn bxor(lhs: Self, rhs: Self) -> Self {
    intrinsics::const_eval_select((lhs, rhs), Self::bxor_ct, Self::bxor_rt)
  }

  #[inline]
  const fn bnot_ct(int: Self) -> Self {
    Self::read_cast(Int::bnot(int.cast()))
  }

  #[inline]
  fn bnot_rt(int: Self) -> Self {
    Self::read_simd(!int.simd())
  }

  #[inline]
  pub(crate) const fn bnot(int: Self) -> Self {
    intrinsics::const_eval_select((int,), Self::bnot_ct, Self::bnot_rt)
  }

  // -----------------------------------------------------------------------
  // Comparison Operations
  // -----------------------------------------------------------------------

  #[inline]
  const fn eq_ct(lhs: Self, rhs: Self) -> bool {
    Int::eq(lhs.cast(), rhs.cast())
  }

  #[inline]
  fn eq_rt(lhs: Self, rhs: Self) -> bool {
    lhs.simd() == rhs.simd()
  }

  #[inline]
  pub(crate) const fn eq(lhs: Self, rhs: Self) -> bool {
    intrinsics::const_eval_select((lhs, rhs), Self::eq_ct, Self::eq_rt)
  }

  #[inline]
  pub(crate) const fn ucmp(lhs: Int<B>, rhs: Int<B>) -> Ordering {
    Int::ucmp(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn scmp(lhs: Int<B>, rhs: Int<B>) -> Ordering {
    Int::scmp(lhs, rhs)
  }

  // -----------------------------------------------------------------------
  // Bit Conversion Operations
  // -----------------------------------------------------------------------

  #[inline]
  const fn swap1_ct(int: Self) -> Self {
    Self::read_cast(Int::swap1(int.cast()))
  }

  #[inline]
  fn swap1_rt(int: Self) -> Self {
    Self::read_simd(int.simd().reverse().reverse_bits())
  }

  #[inline]
  pub(crate) const fn swap1(int: Self) -> Self {
    intrinsics::const_eval_select((int,), Self::swap1_ct, Self::swap1_rt)
  }

  #[inline]
  const fn swap8_ct(int: Self) -> Self {
    Self::read_cast(Int::swap8(int.cast()))
  }

  #[inline]
  fn swap8_rt(int: Self) -> Self {
    Self::read_simd(int.simd().reverse().swap_bytes())
  }

  #[inline]
  pub(crate) const fn swap8(int: Self) -> Self {
    intrinsics::const_eval_select((int,), Self::swap8_ct, Self::swap8_rt)
  }

  #[inline]
  pub(crate) const fn rotl(int: Int<B>, bits: u32) -> Int<B> {
    Int::rotl(int, bits)
  }

  #[inline]
  pub(crate) const fn rotr(int: Int<B>, bits: u32) -> Int<B> {
    Int::rotr(int, bits)
  }

  // -----------------------------------------------------------------------
  // Bit Inspection Operations
  // -----------------------------------------------------------------------

  #[inline]
  pub(crate) const fn ctpop(int: Int<B>) -> u32 {
    Int::ctpop(int)
  }

  #[inline]
  pub(crate) const fn ctlz(int: Int<B>) -> u32 {
    Int::ctlz(int)
  }

  #[inline]
  pub(crate) const fn cttz(int: Int<B>) -> u32 {
    Int::cttz(int)
  }

  #[inline]
  pub(crate) const unsafe fn ctlz_nonzero(int: Int<B>) -> u32 {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::ctlz_nonzero(int) }
  }

  #[inline]
  pub(crate) const unsafe fn cttz_nonzero(int: Int<B>) -> u32 {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::cttz_nonzero(int) }
  }

  // -----------------------------------------------------------------------
  // Overflowing Operations
  // -----------------------------------------------------------------------

  #[inline]
  pub(crate) const fn overflowing_uadd(lhs: Int<B>, rhs: Int<B>) -> (Int<B>, bool) {
    Int::overflowing_uadd(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn overflowing_usub(lhs: Int<B>, rhs: Int<B>) -> (Int<B>, bool) {
    Int::overflowing_usub(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn overflowing_umul(lhs: Int<B>, rhs: Int<B>) -> (Int<B>, bool) {
    Int::overflowing_umul(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn overflowing_sadd(lhs: Int<B>, rhs: Int<B>) -> (Int<B>, bool) {
    Int::overflowing_sadd(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn overflowing_ssub(lhs: Int<B>, rhs: Int<B>) -> (Int<B>, bool) {
    Int::overflowing_ssub(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn overflowing_smul(lhs: Int<B>, rhs: Int<B>) -> (Int<B>, bool) {
    Int::overflowing_smul(lhs, rhs)
  }

  // -----------------------------------------------------------------------
  // Saturating Operations
  // -----------------------------------------------------------------------

  #[inline]
  pub(crate) const fn saturating_uadd(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::saturating_uadd(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn saturating_usub(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::saturating_usub(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn saturating_sadd(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::saturating_sadd(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn saturating_ssub(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::saturating_ssub(lhs, rhs)
  }

  // -----------------------------------------------------------------------
  // Unchecked Operations
  // -----------------------------------------------------------------------

  #[inline]
  pub(crate) const unsafe fn unchecked_uadd(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_uadd(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_usub(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_usub(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_umul(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_umul(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_udiv(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_udiv(lhs, rhs) }
  }

  #[cfg(feature = "exact_div")]
  #[inline]
  pub(crate) const unsafe fn unchecked_udiv_exact(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_udiv_exact(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_urem(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_urem(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_sadd(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_sadd(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_ssub(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_ssub(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_smul(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_smul(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_sdiv(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_sdiv(lhs, rhs) }
  }

  #[cfg(feature = "exact_div")]
  #[inline]
  pub(crate) const unsafe fn unchecked_sdiv_exact(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_sdiv_exact(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_srem(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_srem(lhs, rhs) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_shl(int: Int<B>, bits: u32) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_shl(int, bits) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_lshr(int: Int<B>, bits: u32) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_lshr(int, bits) }
  }

  #[inline]
  pub(crate) const unsafe fn unchecked_ashr(int: Int<B>, bits: u32) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_ashr(int, bits) }
  }

  #[cfg(feature = "funnel_shifts")]
  #[inline]
  pub(crate) const unsafe fn unchecked_fshl(lhs: Int<B>, rhs: Int<B>, bits: u32) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_fshl(lhs, rhs, bits) }
  }

  #[cfg(feature = "funnel_shifts")]
  #[inline]
  pub(crate) const unsafe fn unchecked_fshr(lhs: Int<B>, rhs: Int<B>, bits: u32) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::unchecked_fshr(lhs, rhs, bits) }
  }

  // -----------------------------------------------------------------------
  // Wrapping Operations
  // -----------------------------------------------------------------------

  #[inline]
  pub(crate) const fn wrapping_add(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::wrapping_add(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn wrapping_sub(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::wrapping_sub(lhs, rhs)
  }

  #[inline]
  pub(crate) const fn wrapping_mul(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    Int::wrapping_mul(lhs, rhs)
  }

  // -----------------------------------------------------------------------
  // Misc. Operations
  // -----------------------------------------------------------------------

  #[cfg(feature = "disjoint_bitor")]
  #[inline]
  pub(crate) const unsafe fn disjoint_bor(lhs: Int<B>, rhs: Int<B>) -> Int<B> {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Int::disjoint_bor(lhs, rhs) }
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  pub(crate) const fn carrying_umul_uadd(
    lhs: Int<B>,
    rhs: Int<B>,
    add: Int<B>,
    carry: Int<B>,
  ) -> (Int<B>, Int<B>) {
    Int::carrying_umul_uadd(lhs, rhs, add, carry)
  }

  #[cfg(feature = "bigint_helper_methods")]
  #[inline]
  pub(crate) const fn carrying_smul_sadd(
    lhs: Int<B>,
    rhs: Int<B>,
    add: Int<B>,
    carry: Int<B>,
  ) -> (Int<B>, Int<B>) {
    Int::carrying_smul_sadd(lhs, rhs, add, carry)
  }
}
