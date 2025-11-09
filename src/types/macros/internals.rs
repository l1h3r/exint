macro_rules! internals {
  (int) => {
    $crate::types::macros::internals!(@core, int);

    pub(crate) const NEG_ONE: Self = Self::from_i8(-1);

    #[inline]
    pub(crate) const fn const_cmp(&self, other: &Self) -> ::core::cmp::Ordering {
      $crate::llapi::scmp::<Self, N>(*self, *other)
    }

    #[inline]
    pub(crate) const fn const_neg(self) -> Self {
      arithmetic_select! {
        message: $crate::panic::neg,
        checked: self.checked_neg(),
        wrapped: self.wrapping_neg(),
      }
    }
  };
  (uint) => {
    $crate::types::macros::internals!(@core, uint);

    pub(crate) const TEN: Self = Self::from_u8(10);

    #[inline]
    pub(crate) const fn const_cmp(&self, other: &Self) -> ::core::cmp::Ordering {
      $crate::llapi::ucmp::<Self, N>(*self, *other)
    }

    #[inline]
    const fn one_less_than_next_power_of_two(self) -> Self {
      if self.const_le(&Self::ONE) {
        return Self::ZERO;
      }

      // SAFETY: `self - 1` is at *least* 2.
      Self::MAX.const_shr(unsafe {
        $crate::llapi::ctlz_nonzero::<Self, N>(self.const_sub(Self::ONE))
      })
    }
  };
  (Saturating, $inner:ident) => {
    $crate::types::macros::internals!(@core Saturating, $inner);

    // -------------------------------------------------------------------------
    // Constant Ops
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_add(self, rhs: Self) -> Self {
      Self(self.0.saturating_add(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_sub(self, rhs: Self) -> Self {
      Self(self.0.saturating_sub(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_mul(self, rhs: Self) -> Self {
      Self(self.0.saturating_mul(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_div(self, rhs: Self) -> Self {
      Self(self.0.saturating_div(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_rem(self, rhs: Self) -> Self {
      Self(self.0.const_rem(rhs.0))
    }
  };
  (Wrapping, $inner:ident) => {
    $crate::types::macros::internals!(@core Wrapping, $inner);

    // -------------------------------------------------------------------------
    // Constant Ops
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_add(self, rhs: Self) -> Self {
      Self(self.0.wrapping_add(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_sub(self, rhs: Self) -> Self {
      Self(self.0.wrapping_sub(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_mul(self, rhs: Self) -> Self {
      Self(self.0.wrapping_mul(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_div(self, rhs: Self) -> Self {
      Self(self.0.wrapping_div(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_rem(self, rhs: Self) -> Self {
      Self(self.0.wrapping_rem(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_neg(self) -> Self {
      Self(self.0.wrapping_neg())
    }
  };
  (@core, $name:ident) => {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    pub(crate) const ZERO: Self = Self::from_u8(0);
    pub(crate) const ONE:  Self = Self::from_u8(1);
    pub(crate) const TWO:  Self = Self::from_u8(2);

    pub(crate) const POW2: bool = Self::BITS.is_power_of_two();

    // -------------------------------------------------------------------------
    // Misc. Helpers
    // -------------------------------------------------------------------------

    #[inline]
    const fn mask(bits: u32) -> u32 {
      if Self::POW2 {
        bits & (Self::BITS - 1)
      } else {
        bits % Self::BITS
      }
    }

    // -------------------------------------------------------------------------
    // Constant Eq
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_eq(&self, other: &Self) -> bool {
      $crate::llapi::eq::<Self, N>(*self, *other)
    }

    #[inline]
    pub(crate) const fn is_zero(&self) -> bool {
      $crate::llapi::eq::<Self, N>(*self, Self::ZERO)
    }

    // -------------------------------------------------------------------------
    // Constant Cmp
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_lt(&self, other: &Self) -> bool {
      self.const_cmp(other).is_lt()
    }

    #[inline]
    pub(crate) const fn const_le(&self, other: &Self) -> bool {
      self.const_cmp(other).is_le()
    }

    #[inline]
    pub(crate) const fn const_gt(&self, other: &Self) -> bool {
      self.const_cmp(other).is_gt()
    }

    #[inline]
    pub(crate) const fn const_ge(&self, other: &Self) -> bool {
      self.const_cmp(other).is_ge()
    }

    // -------------------------------------------------------------------------
    // Constant Bitwise Ops
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_band(self, rhs: Self) -> Self {
      $crate::llapi::band::<Self, N>(self, rhs)
    }

    #[inline]
    pub(crate) const fn const_bor(self, rhs: Self) -> Self {
      $crate::llapi::bor::<Self, N>(self, rhs)
    }

    #[inline]
    pub(crate) const fn const_bxor(self, rhs: Self) -> Self {
      $crate::llapi::bxor::<Self, N>(self, rhs)
    }

    #[inline]
    pub(crate) const fn const_not(self) -> Self {
      $crate::llapi::bnot::<Self, N>(self)
    }

    // -------------------------------------------------------------------------
    // Constant Ops
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_add(self, rhs: Self) -> Self {
      arithmetic_select! {
        message: $crate::panic::add,
        checked: self.checked_add(rhs),
        wrapped: self.wrapping_add(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_sub(self, rhs: Self) -> Self {
      arithmetic_select! {
        message: $crate::panic::sub,
        checked: self.checked_sub(rhs),
        wrapped: self.wrapping_sub(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_mul(self, rhs: Self) -> Self {
      arithmetic_select! {
        message: $crate::panic::mul,
        checked: self.checked_mul(rhs),
        wrapped: self.wrapping_mul(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_div(self, rhs: Self) -> Self {
      match self.checked_div(rhs) {
        ::core::option::Option::Some(result) => result,
        ::core::option::Option::None if rhs.is_zero() => $crate::panic::div_zero(),
        ::core::option::Option::None => $crate::panic::div(),
      }
    }

    #[inline]
    pub(crate) const fn const_rem(self, rhs: Self) -> Self {
      match self.checked_rem(rhs) {
        ::core::option::Option::Some(result) => result,
        ::core::option::Option::None if rhs.is_zero() => $crate::panic::rem_zero(),
        ::core::option::Option::None => $crate::panic::rem(),
      }
    }

    #[inline]
    pub(crate) const fn const_shl(self, rhs: u32) -> Self {
      arithmetic_select! {
        message: $crate::panic::shl,
        checked: self.checked_shl(rhs),
        wrapped: self.wrapping_shl(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_shr(self, rhs: u32) -> Self {
      arithmetic_select! {
        message: $crate::panic::shr,
        checked: self.checked_shr(rhs),
        wrapped: self.wrapping_shr(rhs),
      }
    }
  };
  (@core $_outer:ident, $inner:ident) => {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    pub(crate) const ZERO: Self = Self($crate::$inner::ZERO);
    pub(crate) const ONE:  Self = Self($crate::$inner::ONE);

    // -------------------------------------------------------------------------
    // Constant Bitwise Ops
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_band(self, rhs: Self) -> Self {
      Self(self.0.const_band(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_bor(self, rhs: Self) -> Self {
      Self(self.0.const_bor(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_bxor(self, rhs: Self) -> Self {
      Self(self.0.const_bxor(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_not(self) -> Self {
      Self(self.0.const_not())
    }
  };
}

pub(crate) use internals;
