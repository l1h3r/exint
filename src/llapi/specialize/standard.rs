//! Specialization for (`8|16|32|64|128`)-bit Integers
//!
//! Implementations for generic integers that have an **equivalent** built-in primitive type.
//!
//! The following implementations are defined:
//!   - [`I8`]
//!     - built-in type: [`u8`]/[`i8`]
//!     - required size: `1`
//!   - [`I16`]
//!     - built-in type: [`u16`]/[`i16`]
//!     - required size: `2`
//!   - [`I32`]
//!     - built-in type: [`u32`]/[`i32`]
//!     - required size: `4`
//!   - [`I64`]
//!     - built-in type: [`u64`]/[`i64`]
//!     - required size: `8`
//!   - [`I128`]
//!     - built-in type: [`u128`]/[`i128`]
//!     - required size: `16`
//!
//! # Safety
//!
//! The functions here expect the externally-provided generic integers to have
//! the same size and representation as their equivalent built-in primitive type.
//!
//! All invariants of [`Uint`] are required.
//!
//! # Additional
//!
//! The implementations use [`core::intrinsics`] when the `core_intrinsics`
//! feature is enabled, otherwise a stable equivalent or polyfill is used.
//!
//! [`core::intrinsics`]: ::core::intrinsics
//! [`Uint`]: crate::llapi::Uint

macro_rules! define_standard_integer {
  ($name:ident, $size:literal, $uint:ty, $sint:ty) => {
    pub(crate) struct $name;

    impl $name {
      // -----------------------------------------------------------------------
      // Bitwise Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn band(lhs: $uint, rhs: $uint) -> $uint {
        lhs & rhs
      }

      #[inline]
      pub(crate) const fn bor(lhs: $uint, rhs: $uint) -> $uint {
        lhs | rhs
      }

      #[inline]
      pub(crate) const fn bxor(lhs: $uint, rhs: $uint) -> $uint {
        lhs ^ rhs
      }

      #[inline]
      pub(crate) const fn bnot(int: $uint) -> $uint {
        !int
      }

      // -----------------------------------------------------------------------
      // Comparison Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn eq(lhs: $uint, rhs: $uint) -> bool {
        lhs == rhs
      }

      #[inline]
      pub(crate) const fn ucmp(lhs: $uint, rhs: $uint) -> ::core::cmp::Ordering {
        $crate::llapi::intrinsics::three_way_compare!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn scmp(lhs: $sint, rhs: $sint) -> ::core::cmp::Ordering {
        $crate::llapi::intrinsics::three_way_compare!(lhs, rhs)
      }

      // -----------------------------------------------------------------------
      // Bit Conversion Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn swap1(int: $uint) -> $uint {
        $crate::llapi::intrinsics::bitreverse!(int)
      }

      #[inline]
      pub(crate) const fn swap8(int: $uint) -> $uint {
        $crate::llapi::intrinsics::bswap!(int)
      }

      #[inline]
      pub(crate) const fn rotl(int: $uint, bits: u32) -> $uint {
        $crate::llapi::intrinsics::rotate_left!(int, bits)
      }

      #[inline]
      pub(crate) const fn rotr(int: $uint, bits: u32) -> $uint {
        $crate::llapi::intrinsics::rotate_right!(int, bits)
      }

      // -----------------------------------------------------------------------
      // Bit Inspection Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn ctpop(int: $uint) -> u32 {
        $crate::llapi::intrinsics::ctpop!(int)
      }

      #[inline]
      pub(crate) const fn ctlz(int: $uint) -> u32 {
        $crate::llapi::intrinsics::ctlz!(int)
      }

      #[inline]
      pub(crate) const fn cttz(int: $uint) -> u32 {
        $crate::llapi::intrinsics::cttz!(int)
      }

      #[inline]
      pub(crate) const unsafe fn ctlz_nonzero(int: $uint) -> u32 {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::ctlz_nonzero!(int) }
      }

      #[inline]
      pub(crate) const unsafe fn cttz_nonzero(int: $uint) -> u32 {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::cttz_nonzero!(int) }
      }

      // -----------------------------------------------------------------------
      // Overflowing Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn overflowing_uadd(lhs: $uint, rhs: $uint) -> ($uint, bool) {
        $crate::llapi::intrinsics::overflowing_add!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn overflowing_usub(lhs: $uint, rhs: $uint) -> ($uint, bool) {
        $crate::llapi::intrinsics::overflowing_sub!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn overflowing_umul(lhs: $uint, rhs: $uint) -> ($uint, bool) {
        $crate::llapi::intrinsics::overflowing_mul!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn overflowing_sadd(lhs: $sint, rhs: $sint) -> ($sint, bool) {
        $crate::llapi::intrinsics::overflowing_add!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn overflowing_ssub(lhs: $sint, rhs: $sint) -> ($sint, bool) {
        $crate::llapi::intrinsics::overflowing_sub!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn overflowing_smul(lhs: $sint, rhs: $sint) -> ($sint, bool) {
        $crate::llapi::intrinsics::overflowing_mul!(lhs, rhs)
      }

      // -----------------------------------------------------------------------
      // Saturating Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn saturating_uadd(lhs: $uint, rhs: $uint) -> $uint {
        $crate::llapi::intrinsics::saturating_add!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn saturating_usub(lhs: $uint, rhs: $uint) -> $uint {
        $crate::llapi::intrinsics::saturating_sub!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn saturating_sadd(lhs: $sint, rhs: $sint) -> $sint {
        $crate::llapi::intrinsics::saturating_add!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn saturating_ssub(lhs: $sint, rhs: $sint) -> $sint {
        $crate::llapi::intrinsics::saturating_sub!(lhs, rhs)
      }

      // -----------------------------------------------------------------------
      // Unchecked Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const unsafe fn unchecked_uadd(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_add!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_usub(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_sub!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_umul(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_mul!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_udiv(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_div!(lhs, rhs) }
      }

      #[cfg(feature = "exact_div")]
      #[inline]
      pub(crate) const unsafe fn unchecked_udiv_exact(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_div_exact!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_urem(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_rem!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_sadd(lhs: $sint, rhs: $sint) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_add!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_ssub(lhs: $sint, rhs: $sint) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_sub!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_smul(lhs: $sint, rhs: $sint) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_mul!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_sdiv(lhs: $sint, rhs: $sint) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_div!(lhs, rhs) }
      }

      #[cfg(feature = "exact_div")]
      #[inline]
      pub(crate) const unsafe fn unchecked_sdiv_exact(lhs: $sint, rhs: $sint) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_div_exact!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_srem(lhs: $sint, rhs: $sint) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_rem!(lhs, rhs) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_shl(int: $uint, bits: u32) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_shl!(int, bits) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_lshr(int: $uint, bits: u32) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_shr!(int, bits) }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_ashr(int: $sint, bits: u32) -> $sint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_shr!(int, bits) }
      }

      #[cfg(feature = "funnel_shifts")]
      #[inline]
      pub(crate) const unsafe fn unchecked_fshl(lhs: $uint, rhs: $uint, bits: u32) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_funnel_shl!(lhs, rhs, bits) }
      }

      #[cfg(feature = "funnel_shifts")]
      #[inline]
      pub(crate) const unsafe fn unchecked_fshr(lhs: $uint, rhs: $uint, bits: u32) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::unchecked_funnel_shr!(lhs, rhs, bits) }
      }

      // -----------------------------------------------------------------------
      // Wrapping Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn wrapping_add(lhs: $uint, rhs: $uint) -> $uint {
        $crate::llapi::intrinsics::wrapping_add!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn wrapping_sub(lhs: $uint, rhs: $uint) -> $uint {
        $crate::llapi::intrinsics::wrapping_sub!(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn wrapping_mul(lhs: $uint, rhs: $uint) -> $uint {
        $crate::llapi::intrinsics::wrapping_mul!(lhs, rhs)
      }

      // -----------------------------------------------------------------------
      // Misc. Operations
      // -----------------------------------------------------------------------

      #[cfg(feature = "disjoint_bitor")]
      #[inline]
      pub(crate) const unsafe fn disjoint_bor(lhs: $uint, rhs: $uint) -> $uint {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::disjoint_bitor!(lhs, rhs) }
      }

      #[cfg(feature = "bigint_helper_methods")]
      #[inline]
      pub(crate) const fn carrying_umul_uadd(
        lhs: $uint,
        rhs: $uint,
        add: $uint,
        carry: $uint,
      ) -> ($uint, $uint) {
        $crate::llapi::intrinsics::carrying_mul_add!(lhs, rhs, add, carry)
      }

      #[cfg(feature = "bigint_helper_methods")]
      #[inline]
      pub(crate) const fn carrying_smul_sadd(
        lhs: $sint,
        rhs: $sint,
        add: $sint,
        carry: $sint,
      ) -> ($uint, $sint) {
        $crate::llapi::intrinsics::carrying_mul_add!(lhs, rhs, add, carry)
      }
    }
  };
}

define_standard_integer!(I8, 1, u8, i8);
define_standard_integer!(I16, 2, u16, i16);
define_standard_integer!(I32, 4, u32, i32);
define_standard_integer!(I64, 8, u64, i64);
define_standard_integer!(I128, 16, u128, i128);
