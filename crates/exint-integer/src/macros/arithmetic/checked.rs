macro_rules! checked {
  (core, $name:ident, $uint:expr) => {
    #[must_use]
    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_mul(rhs);

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
      if rhs < Self::BITS {
        // SAFETY: We just ensured that `rhs` is in-range.
        Some(unsafe { self.unchecked_shl(rhs) })
      } else {
        None
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
      if rhs < Self::BITS {
        // SAFETY: We just ensured that `rhs` is in-range.
        Some(unsafe { self.unchecked_shr(rhs) })
      } else {
        None
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_pow(self, mut exp: u32) -> Option<Self> {
      if exp == 0 {
        return Some(Self::ONE);
      }

      let mut base: Self = self;
      let mut acc: Self = Self::ONE;

      loop {
        if (exp & 1) == 1 {
          acc = $crate::macros::tri!(acc.checked_mul(base));

          if exp == 1 {
            return Some(acc);
          }
        }

        exp /= 2;
        base = $crate::macros::tri!(base.checked_mul(base));
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_neg();

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }
  };
  (uint) => {
    $crate::macros::checked!(core, uint, true);

    #[must_use]
    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
      let overflow: bool = $crate::intrinsics::overflowing_add::<Self, S, true>(self, rhs).1;

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        // SAFETY: We just ensured that this does not overflow.
        Some(unsafe {
          $crate::intrinsics::unchecked_add::<Self, S, true>(self, rhs)
        })
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_add_signed(self, rhs: $crate::int<S>) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_add_signed(rhs);

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
      if self.const_lt(&rhs) {
        None
      } else {
        // SAFETY: We just ensured that this does not overflow.
        Some(unsafe {
          $crate::intrinsics::unchecked_sub::<Self, S, true>(self, rhs)
        })
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_sub_signed(self, rhs: $crate::int<S>) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_sub_signed(rhs);

      if !overflow {
        Some(result)
      } else {
        None
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO)) {
        None
      } else {
        // SAFETY: We just ensured that we are not dividing by zero.
        Some(unsafe {
          $crate::intrinsics::unchecked_div::<Self, S, true>(self, rhs)
        })
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO)) {
        None
      } else {
        Some(self.div_euclid(rhs))
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO)) {
        None
      } else {
        // SAFETY: We just ensured that we are not calculating the remainder of
        //         division by zero.
        Some(unsafe {
          $crate::intrinsics::unchecked_rem::<Self, S, true>(self, rhs)
        })
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO)) {
        None
      } else {
        Some(self.rem_euclid(rhs))
      }
    }

    // TODO: Optimize
    #[must_use]
    #[inline]
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
      if self.const_le(&Self::ZERO) || base.const_le(&Self::ONE) {
        None
      } else if self.const_le(&base) {
        Some(0)
      } else {
        let mut ilog: u32 = 1;
        let mut data: Self = base;

        while data.const_le(&self.const_div(base)) {
          ilog += 1;
          data = data.const_mul(base);
        }

        Some(ilog)
      }
    }

    // TODO: Optimize
    #[must_use]
    #[inline]
    pub const fn checked_ilog2(self) -> Option<u32> {
      if !self.const_eq(&Self::ZERO) {
        Some(Self::BITS - 1 - self.leading_zeros())
      } else {
        None
      }
    }

    // TODO: Optimize
    #[must_use]
    #[inline]
    pub const fn checked_ilog10(self) -> Option<u32> {
      self.checked_ilog(Self::from_u8(10))
    }

    #[must_use]
    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
      match self.checked_rem(rhs) {
        Some(result) if result.const_eq(&Self::ZERO) => Some(self),
        Some(result) => self.checked_add(rhs.const_sub(result)),
        None => None,
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
      self.one_less_than_next_power_of_two().checked_add(Self::ONE)
    }

    #[cfg(feature = "unsigned_signed_diff")]
    #[doc(cfg(feature = "unsigned_signed_diff"))]
    #[must_use]
    #[inline]
    pub const fn checked_signed_diff(self, rhs: Self) -> Option<$crate::int<S>> {
      let result: $crate::int<S> = self.wrapping_sub(rhs).to_int();
      let overflow: bool = self.const_ge(&rhs) == result.is_negative();

      if !overflow {
        Some(result)
      } else {
        None
      }
    }
  };
  (int) => {
    $crate::macros::checked!(core, int, false);

    #[must_use]
    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_add(rhs);

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_add_unsigned(self, rhs: $crate::uint<S>) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_add_unsigned(rhs);

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_sub(rhs);

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: $crate::uint<S>) -> Option<Self> {
      let (result, overflow): (Self, bool) = self.overflowing_sub_unsigned(rhs);

      if ::core::intrinsics::unlikely(overflow) {
        None
      } else {
        Some(result)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO) || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
        None
      } else {
        // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
        Some(unsafe {
          $crate::intrinsics::unchecked_div::<Self, S, true>(self, rhs)
        })
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO) || (self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE))) {
        None
      } else {
        Some(self.div_euclid(rhs))
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO) || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
        None
      } else {
        // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
        Some(unsafe {
          $crate::intrinsics::unchecked_rem::<Self, S, true>(self, rhs)
        })
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
      if ::core::intrinsics::unlikely(rhs.const_eq(&Self::ZERO) || (self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE))) {
        None
      } else {
        Some(self.rem_euclid(rhs))
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
      if self.is_negative() {
        self.checked_neg()
      } else {
        Some(self)
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
      if self.const_le(&Self::ZERO) || base.const_le(&Self::ONE) {
        None
      } else {
        self.to_uint().checked_ilog(base.to_uint())
      }
    }

    #[must_use]
    #[inline]
    pub const fn checked_ilog2(self) -> Option<u32> {
      if self.const_le(&Self::ZERO) {
        None
      } else {
        // SAFETY: We just ensured that `self` is positive.
        Some((Self::BITS - 1) - unsafe { $crate::intrinsics::ctlz_nonzero::<Self, S>(self) })
      }
    }

    // TODO: Optimize
    #[must_use]
    #[inline]
    pub const fn checked_ilog10(self) -> Option<u32> {
      self.checked_ilog(Self::from_u8(10))
    }

    #[cfg(feature = "int_roundings")]
    #[doc(cfg(feature = "int_roundings"))]
    #[must_use]
    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
      if rhs.const_eq(&Self::NEG_ONE) {
        return Some(self);
      }

      let mut value: Self = $crate::macros::tri!(self.checked_rem(rhs));

      if (value.is_positive() && rhs.is_negative()) ||
         (value.is_negative() && rhs.is_positive())
      {
        value = value.const_add(rhs);
      }

      if value.const_eq(&Self::ZERO) {
        Some(self)
      } else {
        self.checked_add(rhs.const_sub(value))
      }
    }

    // TODO: Optimize
    #[must_use]
    #[inline]
    pub const fn checked_isqrt(self) -> Option<Self> {
      if self.is_negative() {
        None
      } else {
        Some(self.to_uint().isqrt().to_int())
      }
    }
  };
}

pub(crate) use checked;
