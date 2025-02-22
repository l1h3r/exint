macro_rules! overflowing {
  (core, $name:ident, $uint:expr) => {
    #[must_use]
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
      $crate::intrinsics::overflowing_add::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
      $crate::intrinsics::overflowing_sub::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
      $crate::intrinsics::overflowing_mul::<Self, S, $uint>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
      (self.wrapping_shl(rhs), rhs >= Self::BITS)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
      (self.wrapping_shr(rhs), rhs >= Self::BITS)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
      if exp == 0 {
        return (Self::ONE, false);
      }

      let mut base: Self = self;
      let mut acc: Self = Self::ONE;
      let mut overflow: bool = false;
      let mut mul: (Self, bool);

      loop {
        if (exp & 1) == 1 {
          mul = acc.overflowing_mul(base);

          if exp == 1 {
            mul.1 |= overflow;
            return mul;
          }

          acc = mul.0;
          overflow |= mul.1;
        }

        exp /= 2;
        mul = base.overflowing_mul(base);
        base = mul.0;
        overflow |= mul.1;
      }
    }
  };
  (uint) => {
    $crate::macros::overflowing!(core, uint, true);

    #[must_use]
    #[inline]
    pub const fn overflowing_add_signed(self, rhs: $crate::int<S>) -> (Self, bool) {
      let out: (Self, bool) = self.overflowing_add(rhs.to_uint());
      let cmp: bool = out.1 ^ rhs.is_negative();

      (out.0, cmp)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_sub_signed(self, rhs: $crate::int<S>) -> (Self, bool) {
      let out: (Self, bool) = self.overflowing_sub(rhs.to_uint());
      let cmp: bool = out.1 ^ rhs.is_negative();

      (out.0, cmp)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
      (self.const_div(rhs), false)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
      (self.const_div(rhs), false)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
      (self.const_rem(rhs), false)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
      (self.const_rem(rhs), false)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
      (
        self.const_not().wrapping_add(Self::ONE),
        !self.is_zero(),
      )
    }
  };
  (int) => {
    $crate::macros::overflowing!(core, int, false);

    #[must_use]
    #[inline]
    pub const fn overflowing_add_unsigned(self, rhs: $crate::uint<S>) -> (Self, bool) {
      let rhs: Self = rhs.to_int();
      let out: (Self, bool) = self.overflowing_add(rhs);
      let cmp: bool = out.1 ^ rhs.is_negative();

      (out.0, cmp)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_sub_unsigned(self, rhs: $crate::uint<S>) -> (Self, bool) {
      let rhs: Self = rhs.to_int();
      let out: (Self, bool) = self.overflowing_sub(rhs);
      let cmp: bool = out.1 ^ rhs.is_negative();

      (out.0, cmp)
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
      if unlikely(self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE)) {
        (self, true)
      } else {
        (self.const_div(rhs), false)
      }
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
      if unlikely(self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE)) {
        (self, true)
      } else {
        (self.div_euclid(rhs), false)
      }
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
      if unlikely(rhs.const_eq(&Self::NEG_ONE)) {
        (Self::ZERO, self.const_eq(&Self::MIN))
      } else {
        (self.const_rem(rhs), false)
      }
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
      if unlikely(rhs.const_eq(&Self::NEG_ONE)) {
        (Self::ZERO, self.const_eq(&Self::MIN))
      } else {
        (self.rem_euclid(rhs), false)
      }
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
      if unlikely(self.const_eq(&Self::MIN)) {
        (Self::MIN, true)
      } else {
        (self.const_neg(), false)
      }
    }

    #[must_use]
    #[inline]
    pub const fn overflowing_abs(self) -> (Self, bool) {
      (self.wrapping_abs(), self.const_eq(&Self::MIN))
    }
  };
}

pub(crate) use overflowing;
