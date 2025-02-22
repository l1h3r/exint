macro_rules! unchecked {
  (core, $name:ident, $uint:expr) => {
    #[must_use]
    #[inline]
    pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        $crate::intrinsics::unchecked_add::<Self, S, $uint>(self, rhs)
      }
    }

    #[must_use]
    #[inline]
    pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        $crate::intrinsics::unchecked_mul::<Self, S, $uint>(self, rhs)
      }
    }

    #[cfg(feature = "unchecked_shifts")]
    #[doc(cfg(feature = "unchecked_shifts"))]
    #[must_use]
    #[inline]
    pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        $crate::intrinsics::unchecked_shl::<Self, S, $uint>(self, rhs)
      }
    }

    #[cfg(feature = "unchecked_shifts")]
    #[doc(cfg(feature = "unchecked_shifts"))]
    #[must_use]
    #[inline]
    pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        $crate::intrinsics::unchecked_shr::<Self, S, $uint>(self, rhs)
      }
    }

    #[must_use]
    #[inline]
    pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        $crate::intrinsics::unchecked_sub::<Self, S, $uint>(self, rhs)
      }
    }
  };
  (uint) => {
    $crate::macros::unchecked!(core, uint, true);
  };
  (int) => {
    $crate::macros::unchecked!(core, int, false);

    #[cfg(feature = "unchecked_neg")]
    #[doc(cfg(feature = "unchecked_neg"))]
    #[must_use]
    #[inline]
    pub const unsafe fn unchecked_neg(self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        $crate::intrinsics::unchecked_sub::<Self, S, false>(Self::ZERO, self)
      }
    }
  };
}

pub(crate) use unchecked;
