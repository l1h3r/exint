macro_rules! internals {
  (core, $name:ident, $uint:expr) => {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    pub(crate) const ZERO: Self = Self::from_ne_bytes([0x00; S]);
    pub(crate) const ONE:  Self = panic!("ONE");

    // -------------------------------------------------------------------------
    // Constant Eq
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn const_eq(&self, other: &Self) -> bool {
      $crate::intrinsics::eq::<Self, S>(*self, *other)
    }

    // -------------------------------------------------------------------------
    // Constant Cmp
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn const_cmp(&self, other: &Self) -> ::core::cmp::Ordering {
      $crate::intrinsics::cmp::<Self, S, $uint>(*self, *other)
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_lt(&self, other: &Self) -> bool {
      self.const_cmp(other).is_lt()
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_le(&self, other: &Self) -> bool {
      self.const_cmp(other).is_le()
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_gt(&self, other: &Self) -> bool {
      self.const_cmp(other).is_gt()
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_ge(&self, other: &Self) -> bool {
      self.const_cmp(other).is_ge()
    }

    // -------------------------------------------------------------------------
    // Constant Bitwise Ops
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn const_band(self, rhs: Self) -> Self {
      $crate::intrinsics::band::<Self, S>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_bor(self, rhs: Self) -> Self {
      $crate::intrinsics::bor::<Self, S>(self, rhs)
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_bxor(self, rhs: Self) -> Self {
      $crate::intrinsics::bxor::<Self, S>(self, rhs)
    }

    // -------------------------------------------------------------------------
    // Constant Binary Ops
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn const_add(self, rhs: Self) -> Self {
      $crate::macros::arithmetic_select! {
        message: "attempt to add with overflow",
        checked: self.checked_add(rhs),
        wrapped: self.wrapping_add(rhs),
      }
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_sub(self, rhs: Self) -> Self {
      $crate::macros::arithmetic_select! {
        message: "attempt to subtract with overflow",
        checked: self.checked_sub(rhs),
        wrapped: self.wrapping_sub(rhs),
      }
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_mul(self, rhs: Self) -> Self {
      $crate::macros::arithmetic_select! {
        message: "attempt to multiply with overflow",
        checked: self.checked_mul(rhs),
        wrapped: self.wrapping_mul(rhs),
      }
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_div(self, rhs: Self) -> Self {
      self
        .checked_div(rhs)
        .expect("attempt to divide by zero")
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_rem(self, rhs: Self) -> Self {
      self
        .checked_rem(rhs)
        .expect("attempt to calculate the remainder with a divisor of zero")
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_shl(self, rhs: u32) -> Self {
      $crate::macros::arithmetic_select! {
        message: "attempt to shift left with overflow",
        checked: self.checked_shl(rhs),
        wrapped: self.wrapping_shl(rhs),
      }
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_shr(self, rhs: u32) -> Self {
      $crate::macros::arithmetic_select! {
        message: "attempt to shift right with overflow",
        checked: self.checked_shr(rhs),
        wrapped: self.wrapping_shr(rhs),
      }
    }

    // -------------------------------------------------------------------------
    // Constant Unary Ops
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn const_not(self) -> Self {
      $crate::intrinsics::bnot::<Self, S>(self)
    }

    #[must_use]
    #[inline]
    pub(crate) const fn const_neg(self) -> Self {
      $crate::macros::arithmetic_select! {
        message: "attempt to negate with overflow",
        checked: self.checked_neg(),
        wrapped: self.wrapping_neg(),
      }
    }
  };
  (uint) => {
    $crate::macros::internals!(core, uint, true);

    #[must_use]
    #[inline]
    pub(crate) const fn to_int(self) -> $crate::int<S> {
      $crate::int::from_ne_bytes(self.to_ne_bytes())
    }
  };
  (int) => {
    $crate::macros::internals!(core, int, false);

    #[must_use]
    #[inline]
    pub(crate) const fn to_uint(self) -> $crate::uint<S> {
      $crate::uint::from_ne_bytes(self.to_ne_bytes())
    }
  };
}

pub(crate) use internals;
