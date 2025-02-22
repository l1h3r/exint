macro_rules! convert {
  (core, $_name:ident, $_uint:expr) => {
    #[must_use]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
      $crate::intrinsics::swap1::<Self, S>(self)
    }

    #[must_use]
    #[inline]
    pub const fn rotate_left(self, n: u32) -> Self {
      $crate::intrinsics::rotl::<Self, S>(self, n)
    }

    #[must_use]
    #[inline]
    pub const fn rotate_right(self, n: u32) -> Self {
      $crate::intrinsics::rotr::<Self, S>(self, n)
    }

    #[must_use]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
      $crate::intrinsics::swap8::<Self, S>(self)
    }
  };
  (uint) => {
    $crate::macros::convert!(core, uint, true);
  };
  (int) => {
    $crate::macros::convert!(core, int, false);
  };
}

pub(crate) use convert;
