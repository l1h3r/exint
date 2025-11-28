use crate::llapi;
use crate::types::int;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "unchecked_add")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_sadd::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_sub")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_ssub::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_mul")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_smul::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_div_exact")]
  #[cfg(feature = "exact_div")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_div_exact(self, rhs: Self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_sdiv_exact::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_shl")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_shl::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_shr")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_ashr::<Self, N>(self, rhs) }
  }

  #[doc = include_doc!(int, "unchecked_shl_exact")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_shl_exact(self, rhs: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { self.unchecked_shl(rhs) }
  }

  #[doc = include_doc!(int, "unchecked_shr_exact")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const unsafe fn unchecked_shr_exact(self, rhs: u32) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { self.unchecked_shr(rhs) }
  }

  #[doc = include_doc!(int, "unchecked_neg")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const unsafe fn unchecked_neg(self) -> Self {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { llapi::unchecked_ssub::<Self, N>(Self::ZERO, self) }
  }
}
