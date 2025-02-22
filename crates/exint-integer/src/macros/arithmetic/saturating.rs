macro_rules! saturating {
  (core, $name:ident, $uint:expr) => {
    #[must_use]
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
      $crate::intrinsics::saturating_add::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
      $crate::intrinsics::saturating_sub::<Self, S, $uint>(self, rhs)
    }
  };
  (uint) => {
    $crate::macros::saturating!(core, uint, true);

    #[must_use]
    #[inline]
    pub const fn saturating_add_signed(self, rhs: $crate::int<S>) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_add(rhs.to_uint());

      if overflow == rhs.const_lt(&$crate::int::ZERO) {
        result
      } else if overflow {
        Self::MAX
      } else {
        Self::ZERO
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_sub_signed(self, rhs: $crate::int<S>) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_sub_signed(rhs);

      if !overflow {
        result
      } else if rhs.const_lt(&$crate::int::ZERO) {
        Self::MAX
      } else {
        Self::ZERO
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
      match self.checked_mul(rhs) {
        Some(result) => result,
        None => Self::MAX,
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_div(self, rhs: Self) -> Self {
      self.wrapping_div(rhs)
    }

    #[must_use]
    #[inline]
    pub const fn saturating_pow(self, exp: u32) -> Self {
      match self.checked_pow(exp) {
        Some(result) => result,
        None => Self::MAX
      }
    }
  };
  (int) => {
    $crate::macros::saturating!(core, int, false);

    #[must_use]
    #[inline]
    pub const fn saturating_add_unsigned(self, rhs: $crate::uint<S>) -> Self {
      match self.checked_add_unsigned(rhs) {
        Some(result) => result,
        None => Self::MAX,
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_sub_unsigned(self, rhs: $crate::uint<S>) -> Self {
      match self.checked_sub_unsigned(rhs) {
        Some(result) => result,
        None => Self::MIN,
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
      match self.checked_mul(rhs) {
        Some(result) => result,
        None if self.const_lt(&Self::ZERO) == rhs.const_lt(&Self::ZERO) => Self::MAX,
        None => Self::MIN,
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_div(self, rhs: Self) -> Self {
      match self.overflowing_div(rhs) {
        (result, false) => result,
        (_result, true) => Self::MAX,
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_pow(self, exp: u32) -> Self {
      match self.checked_pow(exp) {
        Some(result) => result,
        None if self.const_lt(&Self::ZERO) && exp % 2 == 1 => Self::MIN,
        None => Self::MAX,
      }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_neg(self) -> Self {
      $crate::intrinsics::saturating_sub::<Self, S, false>(Self::ZERO, self)
    }

    #[must_use]
    #[inline]
    pub const fn saturating_abs(self) -> Self {
      if self.is_negative() {
        self.saturating_neg()
      } else {
        self
      }
    }
  };
}

pub(crate) use saturating;
