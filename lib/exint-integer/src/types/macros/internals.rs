#[allow(unused_macro_rules, reason = "Strict type is not not always used")]
macro_rules! internals {
  (uint) => {
    $crate::types::macros::internals!(@core);

    #[inline]
    pub(crate) const fn const_cmp(&self, other: &Self) -> Ordering {
      $crate::llapi::ucmp::<Self, N>(*self, *other)
    }

    #[inline]
    const fn one_less_than_next_power_of_two(self) -> Self {
      if self.const_le(&Self::ONE) {
        return Self::ZERO;
      }

      // SAFETY: `self - 1` is at *least* 2.
      Self::MAX.const_shr(unsafe { llapi::ctlz_nonzero::<Self, N>(self.const_sub(Self::ONE)) })
    }
  };
  (int) => {
    $crate::types::macros::internals!(@core);

    pub(crate) const NEG_ONE: Self = Self::from_i8(-1);

    #[inline]
    pub(crate) const fn const_cmp(&self, other: &Self) -> Ordering {
      $crate::llapi::scmp::<Self, N>(*self, *other)
    }
  };
  (@core) => {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    pub(crate) const ZERO: Self = Self::from_u8(0);
    pub(crate) const ONE:  Self = Self::from_u8(1);
    pub(crate) const TWO:  Self = Self::from_u8(2);

    #[inline]
    const fn mask(bits: u32) -> u32 {
      if Self::BITS.is_power_of_two() {
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
      $crate::types::macros::arithmetic_select! {
        message: $crate::panic::add,
        checked: self.checked_add(rhs),
        wrapped: self.wrapping_add(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_sub(self, rhs: Self) -> Self {
      $crate::types::macros::arithmetic_select! {
        message: $crate::panic::sub,
        checked: self.checked_sub(rhs),
        wrapped: self.wrapping_sub(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_mul(self, rhs: Self) -> Self {
      $crate::types::macros::arithmetic_select! {
        message: $crate::panic::mul,
        checked: self.checked_mul(rhs),
        wrapped: self.wrapping_mul(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_div(self, rhs: Self) -> Self {
      match self.checked_div(rhs) {
        Some(result) => result,
        None => $crate::panic::div_zero(),
      }
    }

    #[inline]
    pub(crate) const fn const_rem(self, rhs: Self) -> Self {
      match self.checked_rem(rhs) {
        Some(result) => result,
        None => $crate::panic::rem_zero(),
      }
    }

    #[inline]
    pub(crate) const fn const_shl(self, rhs: u32) -> Self {
      $crate::types::macros::arithmetic_select! {
        message: $crate::panic::shl,
        checked: self.checked_shl(rhs),
        wrapped: self.wrapping_shl(rhs),
      }
    }

    #[inline]
    pub(crate) const fn const_shr(self, rhs: u32) -> Self {
      $crate::types::macros::arithmetic_select! {
        message: $crate::panic::shr,
        checked: self.checked_shr(rhs),
        wrapped: self.wrapping_shr(rhs),
      }
    }

    #[allow(dead_code, reason = "TODO: Not used by uint")]
    #[inline]
    pub(crate) const fn const_neg(self) -> Self {
      $crate::types::macros::arithmetic_select! {
        message: $crate::panic::neg,
        checked: self.checked_neg(),
        wrapped: self.wrapping_neg(),
      }
    }

    // -------------------------------------------------------------------------
    // Constant Conversion
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn from_bool(other: bool) -> Self {
      Self::from_u8(other as u8)
    }

    $crate::types::macros::internals!(@conv, u8,    from_u8,    into_u8);
    $crate::types::macros::internals!(@conv, u16,   from_u16,   into_u16);
    $crate::types::macros::internals!(@conv, u32,   from_u32,   into_u32);
    $crate::types::macros::internals!(@conv, u64,   from_u64,   into_u64);
    $crate::types::macros::internals!(@conv, u128,  from_u128,  into_u128);
    $crate::types::macros::internals!(@conv, usize, from_usize, into_usize);

    $crate::types::macros::internals!(@conv, i8,    from_i8,    into_i8);
    $crate::types::macros::internals!(@conv, i16,   from_i16,   into_i16);
    $crate::types::macros::internals!(@conv, i32,   from_i32,   into_i32);
    $crate::types::macros::internals!(@conv, i64,   from_i64,   into_i64);
    $crate::types::macros::internals!(@conv, i128,  from_i128,  into_i128);
    $crate::types::macros::internals!(@conv, isize, from_isize, into_isize);
  };
  (@conv, $type:ty, $from:ident, $into:ident) => {
    #[doc(hidden)] // Only exposed for use with `int/uint` macros.
    #[inline]
    pub const fn $from(other: $type) -> Self {
      const M: usize = size_of::<$type>();

      let input: [u8; M] = other.to_ne_bytes();
      let value: [u8; N] = $crate::llapi::cast_bytes::<$type, M, N>(input);

      Self::from_ne_bytes(value)
    }

    #[doc(hidden)] // Only exposed for use with `int/uint` macros.
    #[inline]
    pub const fn $into(self) -> $type {
      const M: usize = size_of::<$type>();

      let input: [u8; N] = self.to_ne_bytes();
      let value: [u8; M] = $crate::llapi::cast_bytes::<Self, N, M>(input);

      <$type>::from_ne_bytes(value)
    }
  };
  (Saturating<$inner:ident>) => {
    $crate::types::macros::internals!(@core, Saturating<$inner>);

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

    #[allow(dead_code, reason = "TODO: Not used by uint")]
    #[inline]
    pub(crate) const fn const_neg(self) -> Self {
      Self(self.0.saturating_neg())
    }
  };
  (Strict<$inner:ident>) => {
    $crate::types::macros::internals!(@core, Strict<$inner>);

    // -------------------------------------------------------------------------
    // Constant Ops
    // -------------------------------------------------------------------------

    #[inline]
    pub(crate) const fn const_add(self, rhs: Self) -> Self {
      Self(self.0.strict_add(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_sub(self, rhs: Self) -> Self {
      Self(self.0.strict_sub(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_mul(self, rhs: Self) -> Self {
      Self(self.0.strict_mul(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_div(self, rhs: Self) -> Self {
      Self(self.0.strict_div(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_rem(self, rhs: Self) -> Self {
      Self(self.0.strict_rem(rhs.0))
    }

    #[inline]
    pub(crate) const fn const_neg(self) -> Self {
      Self(self.0.strict_neg())
    }
  };
  (Wrapping<$inner:ident>) => {
    $crate::types::macros::internals!(@core, Wrapping<$inner>);

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
  (@core, $outer:ident<$inner:ident>) => {
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
