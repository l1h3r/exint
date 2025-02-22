macro_rules! inspect {
  (core, $_name:ident, $_uint:expr) => {
    #[doc(alias = "popcount")]
    #[doc(alias = "popcnt")]
    #[must_use]
    #[inline]
    pub const fn count_ones(self) -> u32 {
      $crate::intrinsics::ctpop::<Self, S>(self)
    }

    #[must_use]
    #[inline]
    pub const fn count_zeros(self) -> u32 {
      self.const_not().count_ones()
    }

    #[must_use]
    #[inline]
    pub const fn leading_ones(self) -> u32 {
      self.const_not().leading_zeros()
    }

    #[must_use]
    #[inline]
    pub const fn leading_zeros(self) -> u32 {
      $crate::intrinsics::ctlz::<Self, S>(self)
    }

    #[must_use]
    #[inline]
    pub const fn trailing_ones(self) -> u32 {
      self.const_not().trailing_zeros()
    }

    #[must_use]
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
      $crate::intrinsics::cttz::<Self, S>(self)
    }
  };
  (uint) => {
    $crate::macros::inspect!(core, uint, true);
  };
  (int) => {
    $crate::macros::inspect!(core, int, false);
  };
}

pub(crate) use inspect;
