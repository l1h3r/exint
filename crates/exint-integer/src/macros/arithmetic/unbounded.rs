macro_rules! unbounded {
  (core, $name:ident, $uint:expr) => {
    #[cfg(feature = "unbounded_shifts")]
    #[doc(cfg(feature = "unbounded_shifts"))]
    #[must_use]
    #[inline]
    pub const fn unbounded_shl(self, rhs: u32) -> Self {
      if rhs < Self::BITS {
        // SAFETY: We just ensured that `rhs` is in-range.
        unsafe { self.unchecked_shl(rhs) }
      } else {
        Self::ZERO
      }
    }
  };
  (uint) => {
    $crate::macros::unbounded!(core, uint, true);

    #[cfg(feature = "unbounded_shifts")]
    #[doc(cfg(feature = "unbounded_shifts"))]
    #[must_use]
    #[inline]
    pub const fn unbounded_shr(self, rhs: u32) -> Self {
      if rhs < Self::BITS {
        // SAFETY: We just ensured that `rhs` is in-range.
        unsafe { self.unchecked_shr(rhs) }
      } else {
        Self::ZERO
      }
    }
  };
  (int) => {
    $crate::macros::unbounded!(core, int, false);

    #[cfg(feature = "unbounded_shifts")]
    #[doc(cfg(feature = "unbounded_shifts"))]
    #[must_use]
    #[inline]
    pub const fn unbounded_shr(self, rhs: u32) -> Self {
      if rhs < Self::BITS {
        // SAFETY: We just ensured that `rhs` is in-range.
        unsafe { self.unchecked_shr(rhs) }
      } else {
        // SAFETY: `Self::BITS - 1` is guaranteed to be less than `Self::BITS`.
        unsafe { self.unchecked_shr(Self::BITS - 1) }
      }
    }
  };
}

pub(crate) use unbounded;
