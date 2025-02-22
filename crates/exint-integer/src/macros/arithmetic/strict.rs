macro_rules! strict {
  (core, $name:ident, $uint:expr) => {
    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_add(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_add(rhs);

      if overflow {
        $crate::panic::strict_add()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_sub(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_sub(rhs);

      if overflow {
        $crate::panic::strict_sub()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_mul(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_mul(rhs);

      if overflow {
        $crate::panic::strict_mul()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_shl(self, rhs: u32) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_shl(rhs);

      if overflow {
        $crate::panic::strict_shl()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_shr(self, rhs: u32) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_shr(rhs);

      if overflow {
        $crate::panic::strict_shr()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_neg(self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_neg();

      if overflow {
        $crate::panic::strict_neg()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_pow(self, mut exp: u32) -> Self {
      if exp == 0 {
        return Self::ONE;
      }

      let mut base: Self = self;
      let mut acc: Self = Self::ONE;

      loop {
        if (exp & 1) == 1 {
          acc = acc.strict_mul(base);

          if exp == 1 {
            return acc;
          }
        }

        exp /= 2;
        base = base.strict_mul(base);
      }
    }
  };
  (uint) => {
    $crate::macros::strict!(core, uint, true);

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_add_signed(self, rhs: $crate::int<S>) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_add_signed(rhs);

      if overflow {
        $crate::panic::strict_add()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_div(self, rhs: Self) -> Self {
      self.const_div(rhs)
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_div_euclid(self, rhs: Self) -> Self {
      self.const_div(rhs)
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_rem(self, rhs: Self) -> Self {
      self.const_rem(rhs)
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
      self.const_rem(rhs)
    }
  };
  (int) => {
    $crate::macros::strict!(core, int, false);

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_add_unsigned(self, rhs: $crate::uint<S>) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_add_unsigned(rhs);

      if overflow {
        $crate::panic::strict_add()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_sub_unsigned(self, rhs: $crate::uint<S>) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_sub_unsigned(rhs);

      if overflow {
        $crate::panic::strict_sub()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_div(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_div(rhs);

      if overflow {
        $crate::panic::strict_div()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_div_euclid(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_div_euclid(rhs);

      if overflow {
        $crate::panic::strict_div()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_rem(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_rem(rhs);

      if overflow {
        $crate::panic::strict_rem()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
      let (result, overflow): (Self, bool) = self.overflowing_rem_euclid(rhs);

      if overflow {
        $crate::panic::strict_rem()
      } else {
        result
      }
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    #[must_use]
    #[inline]
    pub const fn strict_abs(self) -> Self {
      if self.is_negative() {
        self.strict_neg()
      } else {
        self
      }
    }
  };
}

pub(crate) use strict;
