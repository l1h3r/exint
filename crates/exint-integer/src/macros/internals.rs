macro_rules! internals {
  (conv, $name:ident, $from_fn:ident, $into_fn:ident) => {
    #[must_use]
    #[inline]
    pub(crate) const fn $from_fn(other: $name) -> Self {
      $crate::macros::cast!($name as Self, other)
    }

    #[must_use]
    #[inline]
    pub(crate) const fn $into_fn(self) -> $name {
      $crate::macros::cast!(Self as $name, self)
    }
  };
  (core, $name:ident, $uint:expr) => {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    pub(crate) const ZERO: Self = Self::from_u8(0);
    pub(crate) const ONE:  Self = Self::from_u8(1);

    // -------------------------------------------------------------------------
    // Constant Eq
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn const_eq(&self, other: &Self) -> bool {
      $crate::intrinsics::eq::<Self, S>(*self, *other)
    }

    // Only exposed for codegen test :_(
    #[doc(hidden)]
    #[must_use]
    #[inline]
    pub const fn internal_is_zero(&self) -> bool {
      self.const_eq(&Self::ZERO)
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

    // -------------------------------------------------------------------------
    // Constant Conversion
    // -------------------------------------------------------------------------

    #[must_use]
    #[inline]
    pub(crate) const fn from_bool(other: bool) -> Self {
      Self::from_u8(other as u8)
    }

    $crate::macros::internals!(conv, u8,    from_u8,    into_u8);
    $crate::macros::internals!(conv, u16,   from_u16,   into_u16);
    $crate::macros::internals!(conv, u32,   from_u32,   into_u32);
    $crate::macros::internals!(conv, u64,   from_u64,   into_u64);
    $crate::macros::internals!(conv, u128,  from_u128,  into_u128);
    $crate::macros::internals!(conv, usize, from_usize, into_usize);

    $crate::macros::internals!(conv, i8,    from_i8,    into_i8);
    $crate::macros::internals!(conv, i16,   from_i16,   into_i16);
    $crate::macros::internals!(conv, i32,   from_i32,   into_i32);
    $crate::macros::internals!(conv, i64,   from_i64,   into_i64);
    $crate::macros::internals!(conv, i128,  from_i128,  into_i128);
    $crate::macros::internals!(conv, isize, from_isize, into_isize);

    pub(crate) const fn try_into_u32(self) -> Result<u32, $crate::errors::TryFromIntError> {
      let min: Self = $crate::macros::cast!(u32 as Self, u32::MIN);
      let max: Self = $crate::macros::cast!(u32 as Self, u32::MAX);

      if self.const_lt(&min) || self.const_gt(&max) {
        Err($crate::errors::TryFromIntError::new())
      } else {
        Ok(self.into_u32())
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
