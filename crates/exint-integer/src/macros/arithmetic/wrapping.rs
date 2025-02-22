macro_rules! wrapping {
  (core, $name:ident, $uint:expr) => {
    #[must_use]
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
      $crate::intrinsics::wrapping_add::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
      $crate::intrinsics::wrapping_sub::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
      $crate::intrinsics::wrapping_mul::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
      // SAFETY: the masking by the bitsize of the type ensures that we do not
      //         shift out of bounds.
      unsafe {
        self.unchecked_shl(rhs & (Self::BITS - 1))
      }
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
      // SAFETY: the masking by the bitsize of the type ensures that we do not
      //         shift out of bounds.
      unsafe {
        self.unchecked_shr(rhs & (Self::BITS - 1))
      }
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_neg(self) -> Self {
      Self::ZERO.wrapping_sub(self)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_pow(self, mut exp: u32) -> Self {
      if exp == 0 {
        return Self::ONE;
      }

      let mut base: Self = self;
      let mut acc: Self = Self::ONE;

      if is_val_statically_known(exp) {
        while exp > 1 {
          if (exp & 1) == 1 {
            acc = acc.wrapping_mul(base);
          }

          exp /= 2;
          base = base.wrapping_mul(base);
        }

        acc.wrapping_mul(base)
      } else {
        loop {
          if (exp & 1) == 1 {
            acc = acc.wrapping_mul(base);

            if exp == 1 {
              return acc;
            }
          }

          exp /= 2;
          base = base.wrapping_mul(base);
        }
      }
    }
  };
  (uint) => {
    $crate::macros::wrapping!(core, uint, true);

    #[must_use]
    #[inline]
    pub const fn wrapping_add_signed(self, rhs: $crate::int<S>) -> Self {
      self.wrapping_add(rhs.to_uint())
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_sub_signed(self, rhs: $crate::int<S>) -> Self {
      self.wrapping_sub(rhs.to_uint())
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
      self.const_div(rhs)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
      self.const_div(rhs)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
      self.const_rem(rhs)
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
      self.const_rem(rhs)
    }

    #[cfg(feature = "wrapping_next_power_of_two")]
    #[doc(cfg(feature = "wrapping_next_power_of_two"))]
    #[must_use]
    #[inline]
    pub const fn wrapping_next_power_of_two(self) -> Self {
      self.one_less_than_next_power_of_two().wrapping_add(Self::ONE)
    }
  };
  (int) => {
    $crate::macros::wrapping!(core, int, false);

    #[must_use]
    #[inline]
    pub const fn wrapping_add_unsigned(self, rhs: $crate::uint<S>) -> Self {
      self.wrapping_add(rhs.to_int())
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_sub_unsigned(self, rhs: $crate::uint<S>) -> Self {
      self.wrapping_sub(rhs.to_int())
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
      self.overflowing_div(rhs).0
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
      self.overflowing_div_euclid(rhs).0
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
      self.overflowing_rem(rhs).0
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
      self.overflowing_rem_euclid(rhs).0
    }

    #[must_use]
    #[inline]
    pub const fn wrapping_abs(self) -> Self {
      if self.is_negative() {
        self.wrapping_neg()
      } else {
        self
      }
    }
  };
}

pub(crate) use wrapping;
