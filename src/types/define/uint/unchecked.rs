use crate::llapi;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "unchecked_add")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_uadd::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_sub")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_usub::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_mul")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_umul::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_div_exact")]
  #[cfg(feature = "exact_div")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_div_exact(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_udiv_exact::<Self, N>(self, rhs) }
  }

  stability! {
    #[unstable(feature = "unchecked_shifts")]
    #[doc = include_doc!(uint, "unchecked_shl")]
    #[must_use = must_use_doc!()]
    #[track_caller]
    #[inline]
    pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { llapi::unchecked_shl::<Self, N>(self, rhs) }
    }
  }

  stability! {
    #[unstable(feature = "unchecked_shifts")]
    #[doc = include_doc!(uint, "unchecked_shr")]
    #[must_use = must_use_doc!()]
    #[track_caller]
    #[inline]
    pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { llapi::unchecked_lshr::<Self, N>(self, rhs) }
    }
  }

  #[doc = include_doc!(uint, "unchecked_shl_exact")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_shl_exact(self, rhs: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { self.unchecked_shl(rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_shr_exact")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_shr_exact(self, rhs: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { self.unchecked_shr(rhs) }
  }

  #[doc = include_doc!(uint, "unchecked_disjoint_bitor")]
  #[cfg(feature = "disjoint_bitor")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_disjoint_bitor(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::disjoint_bor::<Self, N>(self, rhs) }
  }
}
